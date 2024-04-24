#![feature(fs_try_exists)]
#![feature(isqrt)]
#![allow(warnings)]

use std::collections::HashMap;

use configurations::Configurations;
use data_center::TaskDataCenter;
use database::HistoryData;
use log::error;
use serde_json::Error;

use crate::configurations::TaskConfigRoot;

mod configurations;
mod data_center;
mod database;
#[cfg(test)]
mod test_lib;

#[repr(C)]
pub struct Backend {
    data_centers: HashMap<String, TaskDataCenter>,
    configuration: Configurations,
    database_cache: HistoryData,
}

///
/// json:{id:{#TaskConfigRoot},id:{#TaskConfigRoot}...}
extern "C" fn initialize(
    json: *const String,
    item_count: isize,
    config: Configurations,
) -> Backend {
    let mut history_data = HistoryData::new(&config);

    let mut hash_map = HashMap::new();
    (0..item_count)
        .map(|i| unsafe { &*(json.offset(i)) })
        .map(|s| serde_json::from_str(s) as Result<TaskConfigRoot, Error>)
        .for_each(|r_root| match r_root {
            Ok(root) => {
                hash_map.insert(root.id.clone(), {
                    let mut rhs = history_data.read_from_database(root.id.clone());
                    let mut lhs = TaskDataCenter::init(root);
                    lhs + rhs
                });
            }
            Err(err) => {
                error!("{}", err)
            }
        });
    Backend {
        data_centers: hash_map,
        configuration: config,
        database_cache: todo!(),
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
