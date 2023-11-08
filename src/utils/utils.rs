use std::hash::Hash;

use chrono::{DateTime, Duration, Local};

pub type Time = DateTime<Local>;

/// Defines a time interval (start <= end).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Interval {
    pub start: Time,
    pub end: Time,
}

impl Default for Interval {
    fn default() -> Self {
        Self::new(
            Time::MIN_UTC.into(),
            (Time::MAX_UTC - Duration::days(1)).into(),
        )
    }
}

impl Interval {
    pub fn new(start: Time, end: Time) -> Self {
        Self { start, end }
    }
}
