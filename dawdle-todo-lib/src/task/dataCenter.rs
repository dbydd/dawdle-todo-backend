use std::{
    cell::OnceCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, OnceLock, RwLock},
};

use lazy_static::lazy_static;

use crate::configurations::{self, CONFIGURATIONS};

use super::{Task, TaskContainer};

lazy_static! {
    pub(crate) static ref TASKLIST: Arc<RwLock<HashMap<String, Arc<Task>>>> =
        Arc::new(RwLock::new(HashMap::new()));
    pub(crate) static ref CONTAINER_LIST: Arc<RwLock<HashMap<String, Arc<dyn TaskContainer>>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

pub(crate) fn init() {
    configurations::get_configs_at(&CONFIGURATIONS.task_config_path.0, |(path, ctx)| {
        let task: Task = serde_json::from_str(&ctx).unwrap_or_default();
        TASKLIST.write().unwrap().insert(path, Arc::new(task));
        ()
    });
}
