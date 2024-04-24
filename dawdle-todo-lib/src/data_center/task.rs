use std::{
    ops::{Mul, Sub},
};

use chrono::{DateTime, Datelike, Local, TimeDelta, TimeZone};
use serde::{Deserialize, Serialize};
use toml::value::Datetime;


#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub(crate) struct Priority(pub(crate) usize);
pub(crate) struct InternalDate(pub(crate) toml::value::Datetime);

pub(crate) fn init() {}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Task {
    pub id: String,
    pub init_priority: Priority,
    pub complete_time: usize,
    pub begin_date: Datetime,
    pub end_date: Datetime,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: "error_occurred".to_owned(),
            init_priority: Priority::most_important(),
            complete_time: Default::default(),
            begin_date: Datetime {
                date: Some(toml::value::Date {
                    year: 0,
                    month: 0,
                    day: 0,
                }),
                time: None,
                offset: None,
            },
            end_date: Datetime {
                date: Some(toml::value::Date {
                    year: 0,
                    month: 0,
                    day: 0,
                }),
                time: None,
                offset: None,
            },
        }
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match self.0.cmp(&other.0) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
        })
    }
}

impl Priority {
    pub(crate) fn most_important() -> Self {
        Priority(usize::min_value())
    }
}

impl<T> From<chrono::DateTime<T>> for InternalDate
where
    T: TimeZone,
{
    fn from(value: chrono::DateTime<T>) -> Self {
        InternalDate(Datetime {
            date: Some(toml::value::Date {
                year: value.year() as u16,
                month: value.month() as u8,
                day: value.day() as u8,
            }),
            time: None,
            offset: None,
        })
    }
}

impl From<InternalDate> for chrono::DateTime<Local> {
    fn from(value: InternalDate) -> Self {
        DateTime::parse_from_str(
            &format!("{}", &value.0.date.expect("error while parsing date")),
            "yyyy-mm-dd",
        )
        .expect("error oh parsing date, this should not even able to be happened")
        .to_utc()
        .with_timezone(&Local)
    }
}

impl InternalDate {
    pub fn current_time() -> Self {
        InternalDate::from(chrono::Local::now())
    }
}

impl Sub<InternalDate> for InternalDate {
    type Output = TimeDelta;

    fn sub(self, rhs: InternalDate) -> TimeDelta {
        DateTime::from(rhs) - DateTime::from(self)
    }
}

impl Mul<Priority> for Priority {
    type Output = Priority;

    fn mul(self, rhs: Priority) -> Self::Output {
        Priority((self.0 * rhs.0).isqrt())
    }
}
