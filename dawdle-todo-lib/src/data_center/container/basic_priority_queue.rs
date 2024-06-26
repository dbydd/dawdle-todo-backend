use std::sync::{Arc, RwLock};

use chrono::TimeDelta;
use serde::{Deserialize, Serialize};

use crate::data_center::{
    task::{Priority, Task},
    TaskDataCenter,
};

use super::TaskContainer;

// #[derive(Serialize, Deserialize)]
// struct SingleTask(OnceContainer);

#[derive(Serialize, Deserialize)]
pub(crate) struct BasicPriorityContainer {
    id: String,
    task_queue: Vec<String>,
    init_priority: Priority,
}

impl BasicPriorityContainer {
    fn to_task_objects<'b>(
        &self,
        data_center: &TaskDataCenter,
    ) -> Vec<Arc<RwLock<dyn TaskContainer>>> {
        self.task_queue
            .iter()
            .map(|s| data_center.container_list.get(&(s.clone())))
            .filter(Option::is_some)
            .map(|o| o.unwrap().clone())
            .collect()
    }

    pub fn all_completed(&self, data_center: &TaskDataCenter) -> bool {
        self.to_task_objects(data_center)
            .iter()
            .all(|a| a.read().unwrap().fully_completed(data_center))
    }

    pub fn pop_most_important(
        &self,
        data_center: &TaskDataCenter,
    ) -> Arc<RwLock<dyn TaskContainer>> {
        self.to_task_objects(data_center)
            .iter()
            .map(|a| (a.read().unwrap().priority(data_center), a))
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .unwrap()
            .1
            .clone()
    }
}

impl super::FromJson for BasicPriorityContainer {
    type Container = BasicPriorityContainer;

    fn from_json(json: serde_json::Value) -> Option<Self::Container> {
        serde_json::from_value(json).ok()
    }
}

impl TaskContainer for BasicPriorityContainer {
    fn id(&self) -> &str {
        &self.id
    }

    fn peek_task_inner(&self, center: &TaskDataCenter) -> Arc<Task> {
        self.pop_most_important(center)
            .read()
            .unwrap()
            .peek_task_inner(center)
    }

    fn complete_current_task_once(&mut self, center: &mut TaskDataCenter) {
        self.pop_most_important(center)
            .write()
            .unwrap()
            .complete_current_task_once(center)
    }

    fn fully_completed(&self, center: &TaskDataCenter) -> bool {
        self.all_completed(center)
    }

    fn priority(&self, center: &TaskDataCenter) -> Priority {
        self.init_priority.clone()
            * self
                .pop_most_important(center)
                .read()
                .unwrap()
                .priority(center)
    }

    fn times_remain(&self, center: &TaskDataCenter) -> TimeDelta {
        self.pop_most_important(center)
            .read()
            .unwrap()
            .times_remain(center)
    }

    fn to_json(&self, center: &TaskDataCenter) -> Option<String> {
        None
    }

    fn filters(&self) -> Vec<String> {
        self.task_queue.clone()
    }
}

impl BasicPriorityContainer {
    pub(crate) fn new(id: String, queue: Vec<String>, prio: Option<Priority>) -> Self {
        BasicPriorityContainer {
            id: id,
            task_queue: queue,
            init_priority: prio.unwrap_or(Priority::most_important()),
        }
    }
}
