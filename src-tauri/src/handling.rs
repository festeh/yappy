use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use crate::notification::send_notification;
use crate::seconds_to_string;
use crate::state::AppState;
use crate::store::Value;

use crate::InternalMessage;
use async_std::channel::Receiver;
use async_std::channel::Sender;
use async_std::task;
use tauri::Manager;

fn set_tray_menu_item(handle: &tauri::AppHandle, id: &str, enabled: bool) {
    handle
        .tray_handle()
        .get_item(id)
        .set_enabled(enabled)
        .expect("Unable to change menu item");
}

fn sliding_window(s: &str, window_size: usize) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= window_size {
        return vec![s.to_string()];
    }
    let mut strs: Vec<String> = chars
        .windows(window_size)
        .map(|window| window.iter().collect())
        .collect();
    strs.reverse();
    strs
}

async fn countdown(handle: tauri::AppHandle, state: Arc<Mutex<AppState>>) {
    set_tray_menu_item(&handle, "start", false);
    set_tray_menu_item(&handle, "pause", true);
    set_tray_menu_item(&handle, "reset", true);
    handle.emit_to("main", "pomo_started", "").unwrap();
    send_notification("Pomodoro started!");
    state.lock().unwrap().pause_switch = false;
    state.lock().unwrap().kill_switch = false;
    let remaining = state.lock().unwrap().remaining;

    let duration: u64 = match remaining {
        Some(d) => d,
        None => {
            println!("No duration");
            let sett = &state.lock().unwrap().settings;
            sett.get("duration").unwrap().to_int()
        }
    };
    println!("Duration: {}", duration);
    let selected_task = state
        .lock()
        .unwrap()
        .settings
        .get("selected_task")
        .map(|s| s.to_string())
        .unwrap_or("Not selected".into());
    let windows = sliding_window(&selected_task, 24);
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
        handle.emit_to("main", "pomo_step", &seconds_str).unwrap();
        let trunctated = windows.get(i as usize % windows.len()).unwrap();
        let dbus_msg = format!("{} ({})", seconds_str, trunctated);
        state.lock().unwrap().dbus.send(&dbus_msg);
        state.lock().unwrap().remaining = Some(i);
        task::sleep(Duration::from_secs(1)).await;
    }
    handle
        .emit_to("main", "pomo_step", seconds_to_string(0))
        .unwrap();
    handle.emit_to("main", "pomo_finished", "").unwrap();
    state.lock().unwrap().dbus.send("Waiting");
    send_notification("Pomodoro finished!");
    state.lock().unwrap().remaining = None;
}

pub fn handle_messages(
    handle: tauri::AppHandle,
    state: Arc<Mutex<AppState>>,
    _s: Sender<InternalMessage>,
    r: Receiver<InternalMessage>,
) {
    let todoist = state.lock().unwrap().todoist.clone();
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
                InternalMessage::DurationChanged(d) => state
                    .lock()
                    .unwrap()
                    .settings
                    .set("duration".into(), crate::store::Value::Int(d)),
                InternalMessage::TaskReloadRequested => {
                    let _ = todoist.lock().await.update_tasks().await;
                    let tasks = todoist.lock().await.get_tasks();
                    match tasks {
                        Ok(tasks) => {
                            handle.emit_to("main", "tasks_loaded", tasks).unwrap();
                        }
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            let empty_res: Vec<String> = Vec::new();
                            handle.emit_to("main", "tasks_loaded", empty_res).unwrap();
                        }
                    }
                }
                InternalMessage::TodoistApiKey(api_key) => {
                    state
                        .lock()
                        .unwrap()
                        .settings
                        .set("api_key".into(), crate::store::Value::Text(api_key.clone()));
                    todoist.lock().await.set_api_key(Some(api_key));
                }
                InternalMessage::TaskUnselected => {
                    state
                        .lock()
                        .unwrap()
                        .settings
                        .set("selected_task", Value::Text("".into()));
                }
                InternalMessage::TaskSelected(id) => {
                    let tasks = todoist.lock().await.get_tasks().unwrap_or(vec![]);
                    for task in tasks {
                        if task.id == id {
                            state
                                .lock()
                                .unwrap()
                                .settings
                                .set("selected_task", Value::Text(task.content));
                            break;
                        }
                    }
                }
                InternalMessage::FirebaseAddress(address) => {
                    state
                        .lock()
                        .unwrap()
                        .settings
                        .set("firebase_address".into(), crate::store::Value::Text(address.clone()));
                }
                InternalMessage::FirebaseAuthKey(key) => {
                    state
                        .lock()
                        .unwrap()
                        .settings
                        .set("firebase_auth_key".into(), crate::store::Value::Text(key.clone()));
                }
            }
        }
    });
}
