#![feature(fs_try_exists)]

use std::{collections::HashMap, ops};

use data_center::TaskDataCenter;

mod configurations;
mod data_center;
mod database;

#[macro_use]
extern crate diesel;

//#[repr("C")]

pub struct Backend {
    data_centers: HashMap<String, TaskDataCenter>,
}
///return a json
///{
///groups:[String]
///}
extern "C" fn get_all_legally_defined_groups_in_json() -> String {}

///return a json
/// *serialized crate::task::Task
extern "C" fn get_most_important_task_in_the_spec_group(groupname: String) -> String {}
