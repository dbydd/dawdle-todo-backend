pub(crate) mod container;
#[macro_use]
mod marcos;
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

use crate::{configurations::TaskConfigRoot, data_center::container::FromJson};

use self::{container::TaskContainer, task::Task};

type TaskContainerList = HashMap<String, Arc<RwLock<dyn TaskContainer>>>;
type TaskList = HashMap<String, Arc<Task>>;

pub(crate) struct TaskDataCenter {
    task_list: TaskList,
    container_list: TaskContainerList,
}

impl TaskDataCenter {
    pub(crate) fn init(mut config_root: TaskConfigRoot) -> Self {
        let mut task_list: HashMap<String, Arc<Task>> = HashMap::new();
        let container_list = HashMap::new();
        config_root.tasks.iter_mut().for_each(|task| {
            task_list.insert(task.id.clone(), Arc::new(task.clone()));
        });
        config_root
            .defined_containers
            .iter_mut()
            .for_each(|container| {
                for_each_type_of_containers!(type Container,{

                })
            });

        todo!();

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

    fn get_task_containers_from_json(json: &Value, key: &str) -> TaskContainerList {
        let mut hash_map = HashMap::new();
        json.get(key)
            .map(|a| a.as_array().expect("this json object is not a array"))
            .unwrap_or(&Vec::new())
            .iter()
            .map(move |v| {
                let mut item: Option<Arc<RwLock<dyn TaskContainer>>> = None;
                for_each_type_of_containers!(type Container,{
                    if let Some(mut container) = Container::from_json(v.clone()) {
                        item = Some(Arc::new(RwLock::new(container)))
                    }
                });
                item
            })
            .for_each(|s| {
                if let Some(d) = s {
                    hash_map.insert(d.read().unwrap().id().to_string(), d.clone());
                }
            });
        hash_map
    }

    fn get_task_list_from_json(json: &Value, key: &str) -> TaskList {
        let mut hash_map = HashMap::new();
        let map = json
            .get(key)
            .map(|js| serde_json::from_value::<Vec<Task>>(js.clone()))
            .unwrap_or(Ok(Vec::new()))
            .unwrap_or(Vec::new())
            .iter()
            .map(|t| (t.id.clone(), Arc::new(t.to_owned())))
            .collect::<TaskList>();

        hash_map
    }

    pub(crate) fn from_database_json(json: Value) -> Self {
        Self {
            task_list: Self::get_task_list_from_json(&json, "task_list"),
            container_list: Self::get_task_containers_from_json(&json, "once_containers"),
        }
    }
}
