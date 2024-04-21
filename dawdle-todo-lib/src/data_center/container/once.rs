use std::{cmp, sync::Arc};

use chrono::TimeDelta;
use rusqlite::types::Type;
use serde::{Deserialize, Serialize};

use crate::data_center::{
    modifiers,
    task::{InternalDate, Priorty, Task},
    TaskDataCenter,
};

use super::TaskContainer;

#[derive(Serialize, Deserialize)]
pub(crate) struct OnceContainer {
    id: String,
    current_time: usize,
}

impl TaskContainer for OnceContainer {
    fn id(&self) -> &str {
        &self.id
    }

    fn peek_task_inner(&self, center: &TaskDataCenter) -> Arc<Task> {
        center.task_list.get(self.id()).unwrap().clone()
    }

    fn complete_current_task_once(&mut self, center: &mut TaskDataCenter) {
        self.current_time += 1;
    }

    fn fully_completed(&self, center: &TaskDataCenter) -> bool {
        self.current_time >= self.peek_task_inner(center).complete_time
    }

    fn priorty(&self, center: &TaskDataCenter) -> Priorty {
        match modifiers::simple_in_time_complete(self, center) {
            Some(p) => p,
            None => Priorty::most_important(),
        }
    }

    fn times_remain(&self, center: &TaskDataCenter) -> TimeDelta {
        InternalDate::current_time() - InternalDate(self.peek_task_inner(center).end_date.clone())
    }
}

impl OnceContainer {
    pub(crate) fn new(task_id: String) -> Self {
        Self {
            id: task_id,
            current_time: 0,
        }
    }
}
