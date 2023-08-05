pub mod dbus;
pub mod state;
pub mod notification;
pub mod handling;
pub mod store;
pub mod todoist;


#[derive(Debug, Clone)]
pub enum InternalMessage {
    PomoStarted,
    PomoFinished,
    PomoPaused,
    PomoReseted,
    DurationChanged(u64),
    TasksRequested,
    TodoistApiKey(String),
}


pub fn seconds_to_string(seconds: u64) -> String {
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}
