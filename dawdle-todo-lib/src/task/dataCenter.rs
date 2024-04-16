use std::{
    cell::OnceCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, OnceLock},
};

use lazy_static::lazy_static;

use super::{Task, TaskContainer};

lazy_static! {
    pub(crate) static ref TASKLIST: OnceLock<HashMap<String, Arc<Task>>> = OnceLock::new();
    pub(crate) static ref CONTAINER_LIST: OnceLock<HashMap<String, Arc<dyn TaskContainer>>> =
        OnceLock::new();
}
