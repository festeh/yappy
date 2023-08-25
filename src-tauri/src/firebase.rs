use anyhow::bail;
use chrono::{Datelike, Local, Timelike};
use firebase_rs::Firebase;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

fn current_time_formatted() -> String {
    let now = Local::now();

    format!(
        "{:02}-{:02}-{:02}-{:02}-{}-{}",
        now.hour(),
        now.minute(),
        now.second(),
        now.day(),
        now.month(),
        now.year()
    )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirebaseClient {
    pub address: Option<String>,
    pub auth_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PomodoroStatus {
    Started,
    Finished,
    Cancelled,
}

#[derive(Serialize, Deserialize, Debug)]
struct PomodoroInfo {
    id: String,
    time: String,
    task: String,
    status: PomodoroStatus,
}

impl FirebaseClient {
    pub fn new(address: Option<String>, auth_key: Option<String>) -> Self {
        Self { address, auth_key }
    }

    pub fn send_pomodoro_info(
        &self,
        id: &str,
        task: &str,
        status: PomodoroStatus,
    ) -> Result<(), anyhow::Error> {
        let Some(address) = &self.address else {
            bail!("No Firebase address");
        };
        let Some(auth_key) = &self.auth_key else {
            bail!("No Firebase auth key");
        };

        let firebase = Firebase::auth(address, auth_key).unwrap();
        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            let now = current_time_formatted();
            let cursor = firebase.at(&now);
            let pomo_info = PomodoroInfo {
                id: id.to_owned(),
                time: now,
                task: task.to_owned(),
                status,
            };
            let _ = cursor.set(&pomo_info).await;
        });
        Ok(())
    }
}
