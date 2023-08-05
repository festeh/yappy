use serde::{Deserialize, Serialize};
use std::string::String;
use std::time::Duration;
use surf::Url;
use surf::{Client, Config};

use crate::store::{get_tasks_store_path, PersistentStore, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todoist {
    pub api_key: Option<String>,
    pub store: PersistentStore,
}

impl Todoist {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            api_key,
            store: PersistentStore::new(get_tasks_store_path()),
        }
    }
    pub fn update_api_key(&mut self, settings: &PersistentStore) {
        let key = settings.get("api_key").map(|s| s.to_string());
        self.set_api_key(key);
    }

    pub fn set_api_key(&mut self, api_key: Option<String>) {
        self.api_key = api_key;
    }

    pub fn get_tasks(&self) -> anyhow::Result<Vec<Task>> {
        match self.store.get("tasks") {
            Some(Value::Tasks(tasks)) => Ok(tasks.clone()),
            None => {
                eprintln!("No tasks found");
                Ok(Vec::new())
            }
            _ => {
                eprintln!("Shoudl not happen {:?}", self.store.get("tasks"));
                Ok(Vec::new())
            }
        }
    }

    pub async fn update_tasks(&mut self) -> Result<(), surf::Error> {
        let Some(api_key) = &self.api_key else {
            eprintln!("No Todoist API key found in settings");
            return Ok(());
        };
        let client: Client = Config::new()
            .set_base_url(Url::parse("https://api.todoist.com")?)
            .set_timeout(Some(Duration::from_secs(15)))
            .add_header("Authorization", format!("Bearer {}", api_key))?
            .try_into()?;
        let mut res = client.get("/rest/v2/tasks?filter=today").await?;
        println!("Res {:?}", res.status());
        if !res.status().is_success() {
            return Ok(());
        }
        let tasks: Vec<Task> = res.body_json().await?;
        self.store
            .set("tasks".into(), crate::store::Value::Tasks(tasks));
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub project_id: String,
    pub content: String,
}

pub struct FullTask {
    pub id: String,
    pub project: String,
    pub content: String,
}
