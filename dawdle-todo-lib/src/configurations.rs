use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::data_center::task::Task;

pub(crate) type TaskFilePath = String;
pub(crate) type TaskFileContext = String;

#[derive(Serialize, Deserialize)]
pub(crate) struct ConfigFilePath(pub String);

#[derive(Serialize, Deserialize)]
pub struct Configurations {
    pub database_connection_path: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct TaskConfigRoot {
    pub id: String,
    pub tasks: Vec<Task>,
    ///defined_containers:Hashmap<id,json>
    pub defined_containers: HashMap<String, String>,
}

impl Configurations {
    fn new(input: &str) -> Self {
        serde_json::from_str(input).unwrap()
    }
}
