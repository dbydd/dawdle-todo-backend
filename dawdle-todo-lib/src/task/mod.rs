mod container;
mod dataCenter;
mod modifiers;
use std::{ops::Sub, sync::Arc};

use chrono::{DateTime, Datelike, Local, TimeDelta, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use toml::value::Datetime;

extern crate chrono;
extern crate serde;

#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub(crate) struct Priorty(usize);
pub(crate) struct InternalDate(toml::value::Datetime);

pub(crate) fn init() {
    dataCenter::init()
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Task {
    id: String,
    init_priorty: Priorty,
    complete_time: usize,
    begin_date: Datetime,
    end_date: Datetime,
}

pub(crate) trait TaskContainer: Sync + Send {
    fn id(&self) -> &str;
    fn peek_task_inner(&self) -> Arc<Task>; //考虑实现为iter?
    fn complete_current_task_once(&mut self);
    fn fully_completed(&self) -> bool;
    fn priorty(&self) -> Priorty;
    fn times_remain(&self) -> TimeDelta;
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: "error_occured".to_owned(),
            init_priorty: Priorty::most_important(),
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

impl PartialOrd for Priorty {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match self.0.cmp(&other.0) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
        })
    }
}

impl Priorty {
    pub(crate) fn most_important() -> Self {
        Priorty(usize::min_value())
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
        return InternalDate::from(chrono::Local::now());
    }
}

impl Sub<InternalDate> for InternalDate {
    type Output = TimeDelta;

    fn sub(self, rhs: InternalDate) -> TimeDelta {
        (DateTime::from(rhs) - DateTime::from(self))
    }
}
