#![feature(fs_try_exists)]
#![feature(isqrt)]
#![allow(warnings)]

use std::{collections::HashMap, ffi::CString};

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
/// json:["{#TaskConfigRoot}","{#TaskConfigRoot}"...]
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
    extern "C" fn get_all_legally_defined_groups_in_json(&self) -> CString {
        std::ffi::CString::new(
            serde_json::to_string(
                &self
                    .data_centers
                    .keys()
                    .map(|s| s.to_string().clone())
                    .collect::<Vec<String>>(),
            )
            .unwrap(),
        )
        .unwrap()
    }

    ///return a json
    /// *serialized crate::task::Task
    extern "C" fn get_most_important_task_in_the_spec_group(&self, groupname: &str) -> CString {
        CString::new(
            self.data_centers
                .get(groupname)
                .map(|t| t.solve_task_containers().read().unwrap().peek_task_inner(t))
                .map(|a| {
                    serde_json::to_string({
                        let task = a.as_ref();
                        &task.clone()
                    })
                    .unwrap()
                })
                .unwrap_or("".to_string()),
        )
        .unwrap()
    }
}
