use std::{
    env::home_dir,
    fs::{self, DirEntry, File, FileType},
    io::Read,
};

use clap::builder::Str;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Configurations {
    sql_connection_url: Option<String>,
    task_config_path: Option<String>,
}

impl Configurations {
    pub fn new() -> Self {
        toml::from_str(read_config_at("", "config.toml").unwrap().as_str())
            .expect("error config file")
    }
}

pub(crate) fn save_to(path: &str, json: &str) {
    let mut userhome = dirs::config_dir().expect("error on locate config dir");
    userhome.push("dawdle_todo");
    userhome.push(path);
    if std::fs::try_exists(userhome.as_path()).is_err() {
        std::fs::create_dir_all(userhome.as_path());
    }

    std::fs::write(path, json);
}

pub(crate) fn get_task_config_at<FM, R>(s: &str, f: FM) -> Vec<R>
where
    FM: FnMut((String, String)) -> R,
{
    read_configs_at(s)
        .flat_map(|dir| match dir {
            Ok(entry) => match entry.file_type() {
                Ok(file_type) if file_type.is_file() => {
                    vec![(
                        entry.path().to_string_lossy().to_string(),
                        std::fs::read_to_string(entry.path()).unwrap(),
                    )]
                }
                Ok(file_type) if file_type.is_dir() => {
                    let mut vec = Vec::new();
                    solve_dir(entry, &mut vec);
                    vec
                }
                Err(err) => {
                    panic!("err!: {err}");
                }
                _ => Vec::new(),
            },
            Err(err) => panic!("err at file:{}", err),
        })
        .map(f)
        .collect()
}

fn solve_dir(dir: DirEntry, ret: &mut Vec<(String, String)>) {
    std::fs::read_dir(dir.path())
        .unwrap()
        .for_each(|f| match f {
            Ok(entry) => match entry.file_type() {
                Ok(file_type) if file_type.is_file() => ret.push((
                    entry.path().to_string_lossy().to_string(),
                    std::fs::read_to_string(entry.path()).unwrap(),
                )),
                Ok(file_type) if file_type.is_dir() => solve_dir(entry, ret),
                Err(err) => {
                    panic!("err!: {err}");
                }
                _ => (),
            },
            Err(err) => panic!("err at file:{}", err),
        });
}

fn read_configs_at(location: &str) -> fs::ReadDir {
    let mut userhome = dirs::config_dir().expect("error on locate config dir");
    userhome.push("dawdle_todo");
    userhome.push(location);
    if std::fs::try_exists(userhome.as_path()).is_err() {
        std::fs::create_dir_all(userhome.as_path());
    }

    std::fs::read_dir(userhome.as_path()).unwrap()
}

fn read_config_at(location: &str, fname: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(
        read_configs_at(location)
            .filter(|e| {
                e.as_ref().is_ok_and(|e| {
                    e.file_type().is_ok_and(|t| t.is_file()) && e.file_name() == fname
                })
            })
            .last()
            .unwrap()
            .unwrap()
            .path(),
    )
}
