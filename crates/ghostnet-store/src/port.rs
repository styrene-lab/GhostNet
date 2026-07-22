//! Application-local atomic state contract and deterministic memory store.

use std::collections::BTreeMap;
use std::sync::Mutex;

use async_trait::async_trait;
use thiserror::Error;

/// Opaque application view bytes associated with one retained-event cursor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewCommit {
    pub view: String,
    pub cursor: String,
    pub bytes: Vec<u8>,
}

/// Stored cursor and view loaded atomically.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredView {
    pub cursor: String,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum StoreError {
    #[error("local storage failed: {0}")]
    Failed(String),
}

/// Stores only application decisions, drafts, cursors, audit, and rebuildable views.
#[async_trait]
pub trait LocalStorePort: Send + Sync {
    async fn commit_view(&self, commit: ViewCommit) -> Result<(), StoreError>;
    async fn load_view(&self, view: &str) -> Result<Option<StoredView>, StoreError>;
}

/// Deterministic memory implementation for application and conformance tests.
#[derive(Debug, Default)]
pub struct MemoryLocalStore {
    views: Mutex<BTreeMap<String, StoredView>>,
}

#[async_trait]
impl LocalStorePort for MemoryLocalStore {
    async fn commit_view(&self, commit: ViewCommit) -> Result<(), StoreError> {
        self.views
            .lock()
            .expect("memory store lock poisoned")
            .insert(
                commit.view,
                StoredView {
                    cursor: commit.cursor,
                    bytes: commit.bytes,
                },
            );
        Ok(())
    }

    async fn load_view(&self, view: &str) -> Result<Option<StoredView>, StoreError> {
        Ok(self
            .views
            .lock()
            .expect("memory store lock poisoned")
            .get(view)
            .cloned())
    }
}
