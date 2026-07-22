//! Application-owned time contract and deterministic test clock.

use std::sync::atomic::{AtomicU64, Ordering};

/// Time quality supplied alongside every policy-relevant timestamp.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClockQuality {
    Synchronized,
    Uncertain { maximum_skew_ms: u64 },
    Unsynchronized,
}

/// A timestamp and its explicit quality.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClockReading {
    pub unix_ms: u64,
    pub quality: ClockQuality,
}

/// Supplies time to deterministic application and doctrine decisions.
pub trait Clock: Send + Sync {
    fn now(&self) -> ClockReading;
}

/// Deterministic clock for tests and scripted exercises.
#[derive(Debug)]
pub struct FakeClock {
    unix_ms: AtomicU64,
    quality: ClockQuality,
}

impl FakeClock {
    #[must_use]
    pub const fn new(unix_ms: u64, quality: ClockQuality) -> Self {
        Self {
            unix_ms: AtomicU64::new(unix_ms),
            quality,
        }
    }

    pub fn set(&self, unix_ms: u64) {
        self.unix_ms.store(unix_ms, Ordering::SeqCst);
    }

    pub fn advance(&self, duration_ms: u64) {
        self.unix_ms.fetch_add(duration_ms, Ordering::SeqCst);
    }
}

impl Clock for FakeClock {
    fn now(&self) -> ClockReading {
        ClockReading {
            unix_ms: self.unix_ms.load(Ordering::SeqCst),
            quality: self.quality,
        }
    }
}
