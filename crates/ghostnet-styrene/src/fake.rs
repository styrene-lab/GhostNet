//! Deterministic fault-injectable implementation of [`crate::StyrenePort`].

use std::collections::VecDeque;
use std::sync::Mutex;

use async_trait::async_trait;

use crate::contract::TopicEvents;
use crate::{
    ArtifactEvent, ArtifactPage, CapabilitySnapshot, CapabilityState, DetachedSignature,
    IdentityRef, PollRequest, PublishReceipt, PublishRequest, SignRequest, StyreneError,
    StyrenePort,
};

/// A fault consumed by the next matching fake operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Fault {
    TimeoutBeforeAcceptance,
    TimeoutAfterAcceptance,
    Deny(String),
    DuplicateNextPoll,
    ReverseNextPoll,
    RetentionGapNextPoll,
}

#[derive(Debug)]
struct State {
    events: TopicEvents,
    faults: VecDeque<Fault>,
    next_event: u64,
    now_ms: u64,
}

/// In-memory Styrene contract double with deterministic fault injection.
#[derive(Debug)]
pub struct FakeStyrenePort {
    capabilities: CapabilitySnapshot,
    identity: IdentityRef,
    state: Mutex<State>,
}

impl FakeStyrenePort {
    #[must_use]
    pub fn new(identity: IdentityRef) -> Self {
        Self {
            capabilities: CapabilitySnapshot {
                contract_version: 1,
                generated_at_ms: 0,
                identity: CapabilityState::Supported,
                detached_signing: CapabilityState::Unsupported,
                topics: CapabilityState::Supported,
                maximum_publication_bytes: 65_536,
            },
            identity,
            state: Mutex::new(State {
                events: TopicEvents::new(),
                faults: VecDeque::new(),
                next_event: 1,
                now_ms: 0,
            }),
        }
    }

    /// Queues a fault for the next applicable operation.
    ///
    /// # Panics
    ///
    /// Panics if an earlier fake operation poisoned the internal test lock.
    pub fn inject(&self, fault: Fault) {
        self.state
            .lock()
            .expect("fake lock poisoned")
            .faults
            .push_back(fault);
    }

    /// Advances the deterministic fake clock.
    ///
    /// # Panics
    ///
    /// Panics if an earlier fake operation poisoned the internal test lock.
    pub fn set_now_ms(&self, now_ms: u64) {
        self.state.lock().expect("fake lock poisoned").now_ms = now_ms;
    }
}

#[async_trait]
impl StyrenePort for FakeStyrenePort {
    async fn capabilities(&self) -> Result<CapabilitySnapshot, StyreneError> {
        Ok(self.capabilities.clone())
    }

    async fn local_identity(&self) -> Result<IdentityRef, StyreneError> {
        Ok(self.identity.clone())
    }

    async fn sign(&self, _request: SignRequest) -> Result<DetachedSignature, StyreneError> {
        Err(StyreneError::CapabilityUnavailable("detached_signing"))
    }

    async fn publish(&self, request: PublishRequest) -> Result<PublishReceipt, StyreneError> {
        if request.body.len() > self.capabilities.maximum_publication_bytes {
            return Err(StyreneError::InvalidRequest(
                "publication exceeds negotiated limit".into(),
            ));
        }

        let mut state = self.state.lock().expect("fake lock poisoned");
        match state.faults.front() {
            Some(Fault::TimeoutBeforeAcceptance) => {
                state.faults.pop_front();
                return Err(StyreneError::Timeout);
            }
            Some(Fault::Deny(_)) => {
                let Some(Fault::Deny(reason)) = state.faults.pop_front() else {
                    unreachable!();
                };
                return Err(StyreneError::Denied(reason));
            }
            _ => {}
        }

        if let Some(existing) = state.events.get(&request.topic).and_then(|events| {
            events
                .iter()
                .find(|event| event.artifact == request.artifact)
        }) {
            return Ok(PublishReceipt {
                event_id: existing.event_id.clone(),
                artifact: existing.artifact.clone(),
                accepted_at_ms: state.now_ms,
                duplicate: true,
            });
        }

        let event_id = format!("event:{}", state.next_event);
        state.next_event += 1;
        let event = ArtifactEvent {
            event_id: event_id.clone(),
            artifact: request.artifact.clone(),
            body: request.body,
        };
        state.events.entry(request.topic).or_default().push(event);
        let receipt = PublishReceipt {
            event_id,
            artifact: request.artifact,
            accepted_at_ms: state.now_ms,
            duplicate: false,
        };

        if state.faults.front() == Some(&Fault::TimeoutAfterAcceptance) {
            state.faults.pop_front();
            Err(StyreneError::Timeout)
        } else {
            Ok(receipt)
        }
    }

    async fn poll(&self, request: PollRequest) -> Result<ArtifactPage, StyreneError> {
        if request.limit == 0 {
            return Err(StyreneError::InvalidRequest(
                "poll limit must be positive".into(),
            ));
        }

        let mut state = self.state.lock().expect("fake lock poisoned");
        if let Some(Fault::Deny(_)) = state.faults.front() {
            let Some(Fault::Deny(reason)) = state.faults.pop_front() else {
                unreachable!();
            };
            return Err(StyreneError::Denied(reason));
        }

        let start = request.after.as_ref().map_or(0, |cursor| {
            state
                .events
                .get(&request.topic)
                .and_then(|events| events.iter().position(|event| &event.event_id == cursor))
                .map_or(0, |index| index + 1)
        });
        let mut events: Vec<_> = state
            .events
            .get(&request.topic)
            .into_iter()
            .flatten()
            .skip(start)
            .take(request.limit)
            .cloned()
            .collect();
        let mut retention_gap = false;

        match state.faults.front() {
            Some(Fault::DuplicateNextPoll) => {
                state.faults.pop_front();
                if let Some(first) = events.first().cloned() {
                    events.insert(0, first);
                }
            }
            Some(Fault::ReverseNextPoll) => {
                state.faults.pop_front();
                events.reverse();
            }
            Some(Fault::RetentionGapNextPoll) => {
                state.faults.pop_front();
                retention_gap = true;
                if !events.is_empty() {
                    events.remove(0);
                }
            }
            _ => {}
        }

        Ok(ArtifactPage {
            next_cursor: events.last().map(|event| event.event_id.clone()),
            events,
            retention_gap,
        })
    }
}
