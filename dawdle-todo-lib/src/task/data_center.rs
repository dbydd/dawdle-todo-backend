use std::{
    cell::OnceCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, OnceLock, RwLock},
};

use lazy_static::lazy_static;

use crate::configurations::{self, TaskConfigRoot, CONFIGURATIONS};

use super::{Task, TaskContainer};

pub(crate) struct TaskDataCenter {
    task_list: HashMap<String, Arc<Task>>,
    container_list: HashMap<String, Arc<dyn TaskContainer>>,
}

impl TaskDataCenter {
    pub(crate) fn init(config_root: TaskConfigRoot) -> Self {
        let mut task_list = HashMap::new();
        let mut container_list = HashMap::new();
        config_root
            .tasks
            .iter_mut()
            .for_each(move |task| task_list.insert(task.id.clone(), Arc::new(task)));

        Self {
            task_list,
            container_list,
        }
    }
}
