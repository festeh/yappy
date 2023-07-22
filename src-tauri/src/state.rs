use async_std::channel::Sender;
use serde::{Serialize};

use crate::{dbus::DBus, InternalMessage};

#[derive(Debug, Serialize)]
pub struct AppState {
    pub pause_switch: bool,
    pub kill_switch: bool,
    pub remaining: Option<u64>,
    #[serde(skip)]
    pub dbus: DBus,
    #[serde(skip)]
    pub s: Sender<InternalMessage>
}

impl AppState {
    pub fn new(s: &Sender<InternalMessage>) -> Self {
        Self {
            pause_switch: false,
            kill_switch: false,
            remaining: None,
            dbus: DBus::new(),
            s: s.clone(),
        }
    }
}
