use ghostnet_doctrine::{
    ClockQuality, ClockReading, NetWindow, OneShotWindow, UtcTime, Weekday, WeeklyWindow,
    WindowEvaluation, utc_date,
};

fn timestamp(year: i32, month: u8, day: u8, hour: u8, minute: u8) -> u64 {
    let date = utc_date(year, month, day).expect("date");
    let time = UtcTime::from_hms(hour, minute, 0).expect("time");
    u64::try_from(date.with_time(time).assume_utc().unix_timestamp_nanos() / 1_000_000)
        .expect("positive timestamp")
}

#[test]
fn weekly_window_crosses_year_boundary() {
    let window = NetWindow::Weekly(WeeklyWindow {
        start_date: utc_date(2025, 12, 29).expect("start"),
        end_date: None,
        weekdays: vec![Weekday::Thursday],
        utc_time: UtcTime::from_hms(1, 30, 0).expect("time"),
        interval_weeks: 1,
        duration_ms: 30 * 60 * 1_000,
    });
    let occurrence = window
        .next_occurrence(timestamp(2025, 12, 31, 23, 0))
        .expect("valid window")
        .expect("occurrence");
    assert_eq!(occurrence.starts_at_ms, timestamp(2026, 1, 1, 1, 30));
}

#[test]
fn biweekly_interval_uses_anchor_week_across_iso_year() {
    let window = NetWindow::Weekly(WeeklyWindow {
        start_date: utc_date(2025, 12, 29).expect("start"),
        end_date: None,
        weekdays: vec![Weekday::Monday],
        utc_time: UtcTime::MIDNIGHT,
        interval_weeks: 2,
        duration_ms: 60_000,
    });
    let occurrence = window
        .next_occurrence(timestamp(2026, 1, 5, 0, 0))
        .expect("valid window")
        .expect("occurrence");
    assert_eq!(occurrence.starts_at_ms, timestamp(2026, 1, 12, 0, 0));
}

#[test]
fn uncertain_clock_near_boundary_is_indeterminate() {
    let start = timestamp(2026, 1, 1, 12, 0);
    let window = NetWindow::OneShot(OneShotWindow {
        starts_at_ms: start,
        duration_ms: 60_000,
    });
    assert_eq!(
        window
            .evaluate(ClockReading {
                unix_ms: start - 5_000,
                quality: ClockQuality::Uncertain {
                    maximum_skew_ms: 10_000,
                },
            })
            .expect("evaluation"),
        WindowEvaluation::Indeterminate {
            candidate: window
                .next_occurrence(start - 15_000)
                .expect("lookup")
                .expect("window"),
            maximum_skew_ms: 10_000,
        }
    );
}

#[test]
fn synchronized_clock_distinguishes_half_open_boundary() {
    let start = timestamp(2026, 1, 1, 12, 0);
    let window = NetWindow::OneShot(OneShotWindow {
        starts_at_ms: start,
        duration_ms: 60_000,
    });
    assert!(matches!(
        window
            .evaluate(ClockReading {
                unix_ms: start,
                quality: ClockQuality::Synchronized,
            })
            .expect("start"),
        WindowEvaluation::Current(_)
    ));
    assert_eq!(
        window
            .evaluate(ClockReading {
                unix_ms: start + 60_000,
                quality: ClockQuality::Synchronized,
            })
            .expect("end"),
        WindowEvaluation::Outside
    );
}

#[test]
fn unsynchronized_clock_never_claims_current_window() {
    let start = timestamp(2026, 1, 1, 12, 0);
    let window = NetWindow::OneShot(OneShotWindow {
        starts_at_ms: start,
        duration_ms: 60_000,
    });
    assert_eq!(
        window
            .evaluate(ClockReading {
                unix_ms: start,
                quality: ClockQuality::Unsynchronized,
            })
            .expect("evaluation"),
        WindowEvaluation::Unsynchronized
    );
}
