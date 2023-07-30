use serde::{Deserialize, Serialize};
use std::string::String;
use std::time::Duration;
use surf::Url;
use surf::{Client, Config};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub project_id: String,
    pub content: String,
}

fn get_api_key() -> String {
    dotenv::from_filename(".env.local").ok();
    std::env::var("TODOIST_API_KEY").unwrap()
}

pub async fn get_tasks() -> surf::Result<Vec<Task>> {
    let client: Client = Config::new()
        .set_base_url(Url::parse("https://api.todoist.com")?)
        .set_timeout(Some(Duration::from_secs(15)))
        .add_header("Authorization", format!("Bearer {}", get_api_key()))?
        .try_into()?;
    let mut res = client.get("/rest/v2/tasks?filter=today").await?;
    println!("Res {:?}", res.status());
    let tasks: Vec<Task> = res.body_json().await?;
    Ok(tasks)
}

#[test]
fn test_get_api_key() {
    let key = get_api_key();
    assert_ne!(key, "");
}

#[test]
fn test_get_tasks() {
    let res = async_std::task::block_on(get_tasks());
    println!("{:?}", res.unwrap());
}
