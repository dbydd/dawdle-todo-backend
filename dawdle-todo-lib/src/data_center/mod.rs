pub(crate) mod container;
mod modifiers;
pub(crate) mod task;

extern crate chrono;
extern crate serde;

use std::{
    cell::OnceCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, OnceLock, RwLock},
};

use lazy_static::lazy_static;

use crate::configurations::{self, TaskConfigRoot};

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
}
