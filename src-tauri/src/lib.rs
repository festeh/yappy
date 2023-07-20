use std::path::PathBuf;

pub mod state;


pub fn get_store_path() -> PathBuf {
    let mut path = PathBuf::from(env!("XDG_DATA_HOME"));
    path.push("yappy");
    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }
    path.push("store.json");
    path
}