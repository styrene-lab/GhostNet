//! Application-local storage ports and rebuildable `GhostNet` views.
//!
//! This crate owns drafts, preferences, cursors, policy audit, and derived
//! views—not Styrene messages, receipts, identities, or propagation queues.

/// A retained-event cursor opaque to doctrine code.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventCursor(String);

impl EventCursor {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::EventCursor;

    #[test]
    fn cursor_round_trips_opaque_value() {
        let cursor = EventCursor::new("event:42");
        assert_eq!(cursor.as_str(), "event:42");
    }
}
