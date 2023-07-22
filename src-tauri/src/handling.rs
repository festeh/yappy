use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use async_std::channel::Receiver;
use async_std::channel::Sender;
use async_std::task;
use tauri::Manager;
use tauri::State;
use tauri::Wry;
use tauri_plugin_store::with_store;
use tauri_plugin_store::StoreCollection;

use crate::get_store_path;
use crate::notification::send_notification;
use crate::seconds_to_string;
use crate::state::AppState;
use crate::InternalMessage;

fn set_tray_menu_item(handle: &tauri::AppHandle, id: &str, enabled: bool) {
    handle
        .tray_handle()
        .get_item(id)
        .set_enabled(enabled)
        .expect("Unable to change menu item");
}

async fn countdown(handle: tauri::AppHandle, state: Arc<Mutex<AppState>>) {
    set_tray_menu_item(&handle, "start", false);
    set_tray_menu_item(&handle, "pause", true);
    set_tray_menu_item(&handle, "reset", true);
    handle.emit_to("main", "pomo_started", "").unwrap();
    send_notification("Pomodoro started!");
    state.lock().unwrap().pause_switch = false;
    state.lock().unwrap().kill_switch = false;
    let stores = handle.state::<StoreCollection<Wry>>();
    let duration = match state.lock().unwrap().remaining {
        Some(d) => d,
        None => with_store(handle.clone(), stores, get_store_path(), |store| {
            let dura = store
                .get("duration")
                .expect("duration does not exist")
                .clone();
            Ok(dura.as_u64())
        })
        .expect("Failed to get duration from store")
        .expect("Duration is None"),
    };
    for i in (1..=duration).rev() {
        let seconds_str = seconds_to_string(i);
        if state.lock().unwrap().pause_switch {
            handle.emit_to("main", "pomo_step", &seconds_str).unwrap();
            state
                .lock()
                .unwrap()
                .dbus
                .send(&format!("{} (paused)", seconds_str));
            handle.emit_to("main", "pomo_paused", "").unwrap();
            return;
        }
        if state.lock().unwrap().kill_switch {
            handle.emit_to("main", "pomo_reseted", "").unwrap();
            state.lock().unwrap().remaining = None;
            state.lock().unwrap().dbus.send("Waiting");
            send_notification("Pomodoro abandoned!");
            return;
        }
        println!("Time remaining: {} seconds", i);
        handle.emit_to("main", "pomo_step", &seconds_str).unwrap();
        state.lock().unwrap().dbus.send(&seconds_str);
        state.lock().unwrap().remaining = Some(i);
        task::sleep(Duration::from_secs(1)).await;
    }
    handle.emit_to("main", "pomo_step", 0).unwrap();
    handle.emit_to("main", "pomo_finished", "").unwrap();
    state.lock().unwrap().dbus.send("Waiting");
    send_notification("Pomodoro finished!");
    state.lock().unwrap().remaining = None;
}

pub fn handle_messages(
    handle: tauri::AppHandle,
    state: Arc<Mutex<AppState>>,
    s: Sender<InternalMessage>,
    r: Receiver<InternalMessage>,
) {
    async_std::task::spawn(async move {
        while let Ok(msg) = r.recv().await {
            match msg {
                InternalMessage::PomoStarted => {
                    let handle = handle.clone();
                    let state = state.clone();
                    task::spawn(async move {
                        countdown(handle, state).await;
                    });
                }
                InternalMessage::PomoPaused => {
                    state.lock().unwrap().pause_switch = true;
                    set_tray_menu_item(&handle, "start", true);
                    set_tray_menu_item(&handle, "pause", false);
                    set_tray_menu_item(&handle, "reset", true);
                }
                InternalMessage::PomoReseted => {
                    if state.lock().unwrap().pause_switch {
                        handle.emit_to("main", "pomo_reseted", "").unwrap();
                        state.lock().unwrap().remaining = None;
                        state.lock().unwrap().dbus.send("Waiting");
                        state.lock().unwrap().pause_switch = false;
                    } else {
                        state.lock().unwrap().kill_switch = true;
                    }
                    set_tray_menu_item(&handle, "start", true);
                    set_tray_menu_item(&handle, "pause", false);
                    set_tray_menu_item(&handle, "reset", false);
                }
                InternalMessage::PomoFinished => {}
            }
        }
    });
}
