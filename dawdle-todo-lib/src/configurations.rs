use std::{
    collections::HashMap,
    env::home_dir,
    fs::{self, DirEntry, File, FileType},
    io::Read,
    sync::Arc,
};

use clap::builder::Str;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::data_center::task::Task;

pub(crate) type TaskFilePath = String;
pub(crate) type TaskFileContext = String;

#[derive(Serialize, Deserialize)]
pub(crate) struct ConfigFilePath(pub String);

#[derive(Serialize, Deserialize)]
pub struct Configurations {
    pub task_config_path: ConfigFilePath,
    pub sql_connection_url: Option<String>,
    pub config_root_container: Vec<TaskConfigRoot>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct TaskConfigRoot {
    pub id: String,
    pub tasks: Vec<Task>,
    pub defined_containers: HashMap<String, String>,
}

impl Configurations {
    fn new(input: &str) -> Self {
        serde_json::from_str(&input).unwrap()
    }
}
