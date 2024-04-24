#![feature(fs_try_exists)]
#![feature(isqrt)]

use std::{
    collections::{hash_map, HashMap},
    hash::Hash,
    ops,
};

use data_center::TaskDataCenter;
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::Error;

use crate::configurations::TaskConfigRoot;

mod configurations;
mod data_center;
mod database;
#[cfg(test)]
mod tests;

#[repr(C)]
pub struct Backend {
    data_centers: HashMap<String, TaskDataCenter>,
}

///
/// json:{id:{#TaskConfigRoot},id:{#TaskConfigRoot}...}
extern "C" fn initialize(mut json: *const String, item_count: isize) -> Backend {
    let mut hash_map = HashMap::new();
    (0..item_count)
        .map(|i| unsafe { &*(json.offset(i)) })
        .map(|s| serde_json::from_str(s) as Result<TaskConfigRoot, Error>)
        .for_each(|r_root| match r_root {
            Ok(root) => {
                hash_map.insert(root.id.clone(), TaskDataCenter::init(root));
            }
            Err(err) => {
                error!("{}", err)
            }
        });
    Backend {
        data_centers: hash_map,
    }
}

impl Backend {
    ///return a json
    ///{
    ///groups:[String]
    ///}
    extern "C" fn get_all_legally_defined_groups_in_json(&self) -> String {
        todo!()
    }

    ///return a json
    /// *serialized crate::task::Task
    extern "C" fn get_most_important_task_in_the_spec_group(&self, groupname: String) -> String {
        todo!()
    }
}
