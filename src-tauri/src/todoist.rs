use serde::{Deserialize, Serialize};
use std::string::String;
use std::time::Duration;
use surf::Url;
use surf::{Client, Config};

use crate::store::PersistentStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub project_id: String,
    pub content: String,
}

fn get_api_key(settings: &PersistentStore) -> Option<String> {
    settings.get("api_key").map(|s| s.to_string())
}

pub async fn get_tasks(api_key: Option<String>) -> surf::Result<Vec<Task>> {
    let Some(api_key) = api_key else {
        eprintln!("No Todoist API key found in settings");
        return Ok(Vec::new());
    };
    // TODO: remove
    println!("API key (sending): {}", api_key);
    let client: Client = Config::new()
        .set_base_url(Url::parse("https://api.todoist.com")?)
        .set_timeout(Some(Duration::from_secs(15)))
        .add_header("Authorization", format!("Bearer {}", api_key))?
        .try_into()?;
    let mut res = client.get("/rest/v2/tasks?filter=today").await?;
    println!("Res {:?}", res.status());
    if !res.status().is_success() {
        return Ok(Vec::new());
    }
    let tasks: Vec<Task> = res.body_json().await?;
    Ok(tasks)
}

#[test]
fn test_get_tasks() {
    let settings = PersistentStore::new_settings();
    let res = async_std::task::block_on(get_tasks(&settings));
    println!("{:?}", res.unwrap());
}
