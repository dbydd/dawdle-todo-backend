use std::{panic::set_hook, rc::Rc, sync::Arc};

use chrono::TimeDelta;
use rusqlite::config;
use serde::{Deserialize, Serialize};

use crate::configurations::{self, Configurations, CONFIGURATIONS};

use super::{
    dataCenter::{CONTAINER_LIST, TASKLIST},
    modifiers, InternalDate, Priorty, Task, TaskContainer,
};

#[derive(Serialize, Deserialize)]
struct OnceContainer {
    task: String,
    current_time: usize,
}

#[derive(Serialize, Deserialize)]
struct DefaultContainer {}

impl TaskContainer for OnceContainer {
    fn peek_task_inner(&self) -> Arc<Task> {
        TASKLIST.read().unwrap().get(&self.task).unwrap().clone()
    }

    fn complete_current_task_once(&mut self) {
        self.current_time += 1;
    }

    fn priorty(&self) -> Priorty {
        match modifiers::simple_in_time_complete(self) {
            Some(p) => p,
            None => Priorty::most_important(),
        }
    }

    fn times_remain(&self) -> TimeDelta {
        InternalDate::current_time() - InternalDate(self.peek_task_inner().end_date.clone())
    }

    fn id(&self) -> &str {
        &self.task
    }

    fn fully_completed(&self) -> bool {
        self.current_time >= self.peek_task_inner().complete_time
    }
}

impl OnceContainer {
    pub(crate) fn init() {}
}
