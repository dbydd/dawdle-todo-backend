#![feature(fs_try_exists)]

use std::ops;

mod configurations;
mod database;
mod task;

#[macro_use]
extern crate diesel;
