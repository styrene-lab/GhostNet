//! Deterministic incident transition state machine.

use std::collections::BTreeMap;

use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IncidentState {
    Planned,
    Active,
    Monitoring,
    Closed,
    Cancelled,
}

impl IncidentState {
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Closed | Self::Cancelled)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IncidentTransition {
    pub artifact_id: String,
    pub body_hash: String,
    pub incident_id: String,
    pub previous_transition_hash: Option<String>,
    pub state: IncidentState,
    pub occurred_at_ms: u64,
    pub expires_at_ms: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ApplyTransition {
    Applied,
    Duplicate,
    Fork {
        expected: Option<String>,
        received: Option<String>,
    },
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum IncidentError {
    #[error("transition belongs to a different incident")]
    IncidentMismatch,
    #[error("artifact identifier was reused with different canonical bytes")]
    ArtifactConflict,
    #[error("cannot transition away from terminal state {0:?}")]
    Terminal(IncidentState),
    #[error("transition {from:?} -> {to:?} is not permitted")]
    InvalidTransition {
        from: IncidentState,
        to: IncidentState,
    },
    #[error("transition expired before evaluation")]
    Expired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IncidentChain {
    incident_id: String,
    head: Option<IncidentTransition>,
    artifacts: BTreeMap<String, String>,
}

impl IncidentChain {
    #[must_use]
    pub fn new(incident_id: impl Into<String>) -> Self {
        Self {
            incident_id: incident_id.into(),
            head: None,
            artifacts: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn head(&self) -> Option<&IncidentTransition> {
        self.head.as_ref()
    }

    /// Applies a verified transition to the derived chain.
    ///
    /// # Errors
    ///
    /// Rejects incident mismatches, artifact conflicts, expired transitions,
    /// invalid lifecycle edges, and attempts to leave a terminal state.
    pub fn apply(
        &mut self,
        transition: IncidentTransition,
        now_ms: u64,
    ) -> Result<ApplyTransition, IncidentError> {
        if transition.incident_id != self.incident_id {
            return Err(IncidentError::IncidentMismatch);
        }
        if let Some(hash) = self.artifacts.get(&transition.artifact_id) {
            return if hash == &transition.body_hash {
                Ok(ApplyTransition::Duplicate)
            } else {
                Err(IncidentError::ArtifactConflict)
            };
        }
        if transition
            .expires_at_ms
            .is_some_and(|expiry| now_ms >= expiry)
        {
            return Err(IncidentError::Expired);
        }

        let expected = self.head.as_ref().map(|head| head.body_hash.clone());
        if transition.previous_transition_hash != expected {
            return Ok(ApplyTransition::Fork {
                expected,
                received: transition.previous_transition_hash,
            });
        }

        if let Some(head) = &self.head {
            if head.state.is_terminal() {
                return Err(IncidentError::Terminal(head.state));
            }
            if !valid_edge(head.state, transition.state) {
                return Err(IncidentError::InvalidTransition {
                    from: head.state,
                    to: transition.state,
                });
            }
        } else if transition.state != IncidentState::Planned
            && transition.state != IncidentState::Active
        {
            return Err(IncidentError::InvalidTransition {
                from: IncidentState::Planned,
                to: transition.state,
            });
        }

        self.artifacts
            .insert(transition.artifact_id.clone(), transition.body_hash.clone());
        self.head = Some(transition);
        Ok(ApplyTransition::Applied)
    }
}

const fn valid_edge(from: IncidentState, to: IncidentState) -> bool {
    matches!(
        (from, to),
        (
            IncidentState::Planned,
            IncidentState::Active | IncidentState::Cancelled
        ) | (
            IncidentState::Active,
            IncidentState::Monitoring | IncidentState::Closed
        ) | (
            IncidentState::Monitoring,
            IncidentState::Active | IncidentState::Closed
        )
    )
}
