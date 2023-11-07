use chrono::{DateTime, Local};

pub type Time = DateTime<Local>;

/// Defines a time interval (start <= end).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Interval {
    pub start: Time,
    pub end: Time,
}
