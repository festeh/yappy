use std::sync::Arc;

use async_std::channel::Sender;
use async_std::sync::Mutex;
use serde::Serialize;

use crate::firebase::FirebaseClient;
use crate::store::{get_settings_store_path, PersistentStore};
use crate::todoist::Todoist;
use crate::{dbus::DBus, InternalMessage};

#[derive(Debug, Serialize)]
pub struct AppState {
    pub pause_switch: bool,
    pub kill_switch: bool,
    pub remaining: Option<u64>,
    pub settings: PersistentStore,
    #[serde(skip)]
    pub todoist: Arc<Mutex<Todoist>>,
    #[serde(skip)]
    pub firebase: FirebaseClient,
    #[serde(skip)]
    pub dbus: DBus,
    #[serde(skip)]
    pub s: Sender<InternalMessage>,
}

impl AppState {
    pub fn new(s: &Sender<InternalMessage>) -> Self {
        let settings = PersistentStore::new(get_settings_store_path());
        let todoist = Arc::new(Mutex::new(Todoist::new(
            settings.get("api_key").map(|s| s.to_string()),
        )));
        let firebase = FirebaseClient::new(
            settings.get("firebase_address").map(|s| s.to_string()),
            settings.get("firebase_auth_key").map(|s| s.to_string()),
        );
        Self {
            pause_switch: false,
            kill_switch: false,
            remaining: None,
            dbus: DBus::new(),
            settings,
            todoist,
            firebase,
            s: s.clone(),
        }
    }
}
