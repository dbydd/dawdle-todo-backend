pub(crate) mod container;
mod modifiers;
pub(crate) mod task;

extern crate chrono;
extern crate serde;

use std::{
    collections::HashMap,
    ops::Deref,
    sync::{Arc, RwLock},
};

use serde_json::{json, Value};

use crate::configurations::{TaskConfigRoot};

use self::{container::TaskContainer, task::Task};

pub(crate) struct TaskDataCenter {
    task_list: HashMap<String, Arc<Task>>,
    container_list: HashMap<String, Arc<RwLock<dyn TaskContainer>>>,
}

impl TaskDataCenter {
    pub(crate) fn init(mut config_root: TaskConfigRoot) -> Self {
        let mut task_list: HashMap<String, Arc<Task>> = HashMap::new();
        let container_list = HashMap::new();
        config_root.tasks.iter_mut().for_each(|task| {
            task_list.insert(task.id.clone(), Arc::new(task.clone()));
        });

        Self {
            task_list,
            container_list,
        }
    }

    pub(crate) fn to_json(&self) -> Value {
        let collect: Vec<Task> = self
            .task_list
            .values()
            .map(|t| {
                let arc = t.clone().deref().clone();
                arc
            })
            .collect();
        let task_list = serde_json::to_string(&collect).unwrap();
        let once_containers: Vec<String> = self
            .container_list
            .values()
            .filter_map(|c| c.read().unwrap().to_json(self))
            .collect();

        json!({
            "task_list": task_list,
            "once_containers":once_containers
        })
    }
}
