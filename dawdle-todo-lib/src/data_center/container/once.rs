use std::sync::Arc;

use chrono::TimeDelta;
use serde::{Deserialize, Serialize};

use crate::data_center::{
    modifiers,
    task::{InternalDate, Priority, Task},
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

    fn priority(&self, center: &TaskDataCenter) -> Priority {
        match modifiers::simple_in_time_complete(self, center) {
            Some(p) => p,
            None => Priority::most_important(),
        }
    }

    fn times_remain(&self, center: &TaskDataCenter) -> TimeDelta {
        InternalDate::current_time() - InternalDate(self.peek_task_inner(center).end_date)
    }

    fn to_json(&self, center: &TaskDataCenter) -> Option<String> {
        serde_json::to_string(self).ok()
    }

    fn filters(&self) -> Vec<String> {
        Vec::new()
    }
}

impl super::FromJson for OnceContainer {
    type Container = Self;

    fn from_json(json: serde_json::Value) -> Option<Self::Container> {
        serde_json::from_value(json).ok()
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
