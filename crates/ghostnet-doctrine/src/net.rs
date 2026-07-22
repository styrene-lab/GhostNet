//! Prepared-net revision chains.

use std::collections::BTreeMap;

use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NetRevision {
    pub artifact_id: String,
    pub body_hash: String,
    pub net_id: String,
    pub revision: u64,
    pub previous_revision_hash: Option<String>,
    pub effective_at_ms: u64,
    pub expires_at_ms: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ApplyRevision {
    Applied,
    Duplicate,
    Fork {
        expected_revision: u64,
        expected_predecessor: Option<String>,
        received_revision: u64,
        received_predecessor: Option<String>,
    },
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum NetError {
    #[error("revision belongs to a different net")]
    NetMismatch,
    #[error("artifact identifier was reused with different canonical bytes")]
    ArtifactConflict,
    #[error("revision zero is invalid")]
    RevisionZero,
    #[error("initial revision must not have a predecessor")]
    InitialPredecessor,
    #[error("revision is not yet effective")]
    NotEffective,
    #[error("revision is expired")]
    Expired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NetRevisionChain {
    net_id: String,
    head: Option<NetRevision>,
    artifacts: BTreeMap<String, String>,
}

impl NetRevisionChain {
    #[must_use]
    pub fn new(net_id: impl Into<String>) -> Self {
        Self {
            net_id: net_id.into(),
            head: None,
            artifacts: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn head(&self) -> Option<&NetRevision> {
        self.head.as_ref()
    }

    /// Applies one verified revision without resolving forks by arrival time.
    ///
    /// # Errors
    ///
    /// Rejects mismatched nets, conflicting artifact IDs, invalid revision zero,
    /// invalid initial predecessors, and revisions outside their validity span.
    pub fn apply(&mut self, revision: NetRevision, now_ms: u64) -> Result<ApplyRevision, NetError> {
        if revision.net_id != self.net_id {
            return Err(NetError::NetMismatch);
        }
        if let Some(hash) = self.artifacts.get(&revision.artifact_id) {
            return if hash == &revision.body_hash {
                Ok(ApplyRevision::Duplicate)
            } else {
                Err(NetError::ArtifactConflict)
            };
        }
        if revision.revision == 0 {
            return Err(NetError::RevisionZero);
        }
        if revision.effective_at_ms > now_ms {
            return Err(NetError::NotEffective);
        }
        if revision
            .expires_at_ms
            .is_some_and(|expiry| now_ms >= expiry)
        {
            return Err(NetError::Expired);
        }

        let (expected_revision, expected_predecessor) =
            self.head.as_ref().map_or((1, None), |head| {
                (head.revision + 1, Some(head.body_hash.clone()))
            });
        if self.head.is_none()
            && revision.revision == 1
            && revision.previous_revision_hash.is_some()
        {
            return Err(NetError::InitialPredecessor);
        }
        if revision.revision != expected_revision
            || revision.previous_revision_hash != expected_predecessor
        {
            return Ok(ApplyRevision::Fork {
                expected_revision,
                expected_predecessor,
                received_revision: revision.revision,
                received_predecessor: revision.previous_revision_hash,
            });
        }

        self.artifacts
            .insert(revision.artifact_id.clone(), revision.body_hash.clone());
        self.head = Some(revision);
        Ok(ApplyRevision::Applied)
    }
}
