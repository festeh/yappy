use std::path::PathBuf;

pub mod dbus;
pub mod state;
pub mod notification;

pub fn get_store_path() -> PathBuf {
    let mut path = PathBuf::from(env!("XDG_DATA_HOME"));
    path.push("yappy");
    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }
    path.push("store.json");
    path
}

pub fn seconds_to_string(seconds: u64) -> String {
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}
