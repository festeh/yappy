use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::Write;
use std::{collections::HashMap, path::PathBuf};

use crate::todoist::Task;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value {
    Int(u64),
    Text(String),
    Tasks(Vec<Task>),
    Projects(HashMap<String, Value>),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Text(t) => t.clone(),
            Value::Tasks(t) => serde_json::to_string(t).unwrap(),
            Value::Projects(p) => serde_json::to_string(p).unwrap(),
        }
    }

    pub fn to_int(&self) -> u64 {
        match self {
            Value::Int(i) => *i,
            _ => panic!("not an integer"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PersistentStore {
    cache: HashMap<String, Value>,
    file_path: PathBuf,
}

pub fn get_settings_store_path() -> PathBuf {
    let mut path = PathBuf::from(env!("XDG_DATA_HOME"));
    path.push("yappy");
    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }
    path.push("settings.json");
    path
}

pub fn get_tasks_store_path() -> PathBuf {
    let mut path = PathBuf::from(env!("XDG_DATA_HOME"));
    path.push("yappy");
    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }
    path.push("tasks.json");
    path
}

impl PersistentStore {
    pub fn new(file_path: PathBuf) -> Self {
        let data = fs::read_to_string(&file_path).unwrap_or_else(|_| "{}".into());
        let cache: HashMap<String, Value> = serde_json::from_str(&data).unwrap();
        PersistentStore { cache, file_path }
    }

    pub fn new_settings() -> Self {
        Self::new(get_settings_store_path())
    }

    pub fn new_tasks() -> Self {
        Self::new(get_tasks_store_path())
    }

    pub fn check(&self, key: &str) -> bool {
        self.cache.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.cache.get(key)
    }

    pub fn set(&mut self, key: &str, value: Value) {
        self.cache.insert(key.into(), value.clone());
        let file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.file_path)
            .expect("Failed to open file.");
        let mut file = std::io::BufWriter::new(file);
        let data = serde_json::to_string(&self.cache).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }
}
