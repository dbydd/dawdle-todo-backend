mod container;
mod dataCenter;
mod modifiers;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use toml::value::Datetime;

extern crate chrono;
extern crate serde;

#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub(crate) struct Priorty(usize);

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
    fn times_remain(&self) -> usize;
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
