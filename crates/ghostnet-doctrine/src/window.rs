//! Restricted UTC communication-window evaluation.

use thiserror::Error;
use time::{Date, Duration, Month, OffsetDateTime, Time, UtcOffset, Weekday};

use crate::{ClockQuality, ClockReading};

const MS_PER_SECOND: u64 = 1_000;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OneShotWindow {
    pub starts_at_ms: u64,
    pub duration_ms: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WeeklyWindow {
    pub start_date: Date,
    pub end_date: Option<Date>,
    pub weekdays: Vec<Weekday>,
    pub utc_time: Time,
    pub interval_weeks: u32,
    pub duration_ms: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NetWindow {
    OneShot(OneShotWindow),
    Weekly(WeeklyWindow),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WindowOccurrence {
    pub starts_at_ms: u64,
    pub ends_at_ms: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowEvaluation {
    Outside,
    Current(WindowOccurrence),
    Indeterminate {
        candidate: WindowOccurrence,
        maximum_skew_ms: u64,
    },
    Unsynchronized,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum WindowError {
    #[error("window duration must be positive")]
    ZeroDuration,
    #[error("weekly recurrence interval must be positive")]
    ZeroInterval,
    #[error("weekly recurrence requires at least one weekday")]
    NoWeekdays,
    #[error("window timestamp is outside the supported epoch range")]
    TimestampRange,
}

impl NetWindow {
    /// Finds the first occurrence whose end is after `from_ms`.
    ///
    /// # Errors
    ///
    /// Rejects malformed recurrence constraints and unsupported timestamps.
    pub fn next_occurrence(&self, from_ms: u64) -> Result<Option<WindowOccurrence>, WindowError> {
        match self {
            Self::OneShot(window) => one_shot(window, from_ms),
            Self::Weekly(window) => weekly(window, from_ms),
        }
    }

    /// Evaluates a window using explicit clock quality.
    ///
    /// # Errors
    ///
    /// Returns validation or timestamp conversion errors from occurrence lookup.
    pub fn evaluate(&self, reading: ClockReading) -> Result<WindowEvaluation, WindowError> {
        let Some(candidate) =
            self.next_occurrence(reading.unix_ms.saturating_sub(max_skew(reading.quality)))?
        else {
            return Ok(WindowEvaluation::Outside);
        };
        match reading.quality {
            ClockQuality::Unsynchronized => Ok(WindowEvaluation::Unsynchronized),
            ClockQuality::Synchronized => {
                if contains(candidate, reading.unix_ms) {
                    Ok(WindowEvaluation::Current(candidate))
                } else {
                    Ok(WindowEvaluation::Outside)
                }
            }
            ClockQuality::Uncertain { maximum_skew_ms } => {
                let earliest = reading.unix_ms.saturating_sub(maximum_skew_ms);
                let latest = reading.unix_ms.saturating_add(maximum_skew_ms);
                if contains(candidate, earliest) && contains(candidate, latest) {
                    Ok(WindowEvaluation::Current(candidate))
                } else if latest >= candidate.starts_at_ms && earliest < candidate.ends_at_ms {
                    Ok(WindowEvaluation::Indeterminate {
                        candidate,
                        maximum_skew_ms,
                    })
                } else {
                    Ok(WindowEvaluation::Outside)
                }
            }
        }
    }
}

fn one_shot(window: &OneShotWindow, from_ms: u64) -> Result<Option<WindowOccurrence>, WindowError> {
    if window.duration_ms == 0 {
        return Err(WindowError::ZeroDuration);
    }
    let occurrence = WindowOccurrence {
        starts_at_ms: window.starts_at_ms,
        ends_at_ms: window
            .starts_at_ms
            .checked_add(window.duration_ms)
            .ok_or(WindowError::TimestampRange)?,
    };
    Ok((occurrence.ends_at_ms > from_ms).then_some(occurrence))
}

fn weekly(window: &WeeklyWindow, from_ms: u64) -> Result<Option<WindowOccurrence>, WindowError> {
    validate_weekly(window)?;
    let from = timestamp(from_ms)?;
    let mut date = from.date();
    if date < window.start_date {
        date = window.start_date;
    }
    let search_end = window
        .end_date
        .unwrap_or_else(|| date + Duration::days(3_700));
    while date <= search_end {
        if window.weekdays.contains(&date.weekday()) && on_interval(window, date) {
            let start = date
                .with_time(window.utc_time)
                .assume_offset(UtcOffset::UTC);
            let starts_at_ms = milliseconds(start)?;
            let ends_at_ms = starts_at_ms
                .checked_add(window.duration_ms)
                .ok_or(WindowError::TimestampRange)?;
            if ends_at_ms > from_ms {
                return Ok(Some(WindowOccurrence {
                    starts_at_ms,
                    ends_at_ms,
                }));
            }
        }
        date = date.next_day().ok_or(WindowError::TimestampRange)?;
    }
    Ok(None)
}

fn validate_weekly(window: &WeeklyWindow) -> Result<(), WindowError> {
    if window.duration_ms == 0 {
        return Err(WindowError::ZeroDuration);
    }
    if window.interval_weeks == 0 {
        return Err(WindowError::ZeroInterval);
    }
    if window.weekdays.is_empty() {
        return Err(WindowError::NoWeekdays);
    }
    Ok(())
}

fn on_interval(window: &WeeklyWindow, date: Date) -> bool {
    let anchor = week_monday(window.start_date);
    let candidate = week_monday(date);
    let weeks = (candidate - anchor).whole_days() / 7;
    weeks >= 0 && weeks % i64::from(window.interval_weeks) == 0
}

fn week_monday(date: Date) -> Date {
    date - Duration::days(i64::from(date.weekday().number_days_from_monday()))
}

fn timestamp(ms: u64) -> Result<OffsetDateTime, WindowError> {
    let seconds = i64::try_from(ms / MS_PER_SECOND).map_err(|_| WindowError::TimestampRange)?;
    let nanos =
        i64::try_from((ms % MS_PER_SECOND) * 1_000_000).map_err(|_| WindowError::TimestampRange)?;
    OffsetDateTime::from_unix_timestamp(seconds)
        .map_err(|_| WindowError::TimestampRange)?
        .checked_add(Duration::nanoseconds(nanos))
        .ok_or(WindowError::TimestampRange)
}

fn milliseconds(value: OffsetDateTime) -> Result<u64, WindowError> {
    u64::try_from(value.unix_timestamp_nanos() / 1_000_000).map_err(|_| WindowError::TimestampRange)
}

const fn contains(occurrence: WindowOccurrence, at_ms: u64) -> bool {
    at_ms >= occurrence.starts_at_ms && at_ms < occurrence.ends_at_ms
}

const fn max_skew(quality: ClockQuality) -> u64 {
    match quality {
        ClockQuality::Uncertain { maximum_skew_ms } => maximum_skew_ms,
        ClockQuality::Synchronized | ClockQuality::Unsynchronized => 0,
    }
}

/// UTC date helper for contract tests and callers without direct `time` usage.
///
/// # Errors
///
/// Rejects invalid Gregorian dates.
pub fn utc_date(year: i32, month: u8, day: u8) -> Result<Date, WindowError> {
    let month = Month::try_from(month).map_err(|_| WindowError::TimestampRange)?;
    Date::from_calendar_date(year, month, day).map_err(|_| WindowError::TimestampRange)
}
