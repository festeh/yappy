// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use async_std::task;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{thread, time::Duration};
use tauri::Wry;
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu};
use tauri_plugin_store::with_store;
use tauri_plugin_store::StoreBuilder;
use tauri_plugin_store::StoreCollection;
use yappy::dbus::DBus;
use yappy::notification::send_notification;
use yappy::{get_store_path, seconds_to_string};

use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
struct AppState {
    pause_switch: bool,
    kill_switch: bool,
    #[serde(skip_serializing, skip_deserializing)]
    dbus: DBus,
    remaining: Option<u64>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            pause_switch: false,
            kill_switch: false,
            remaining: None,
            dbus: DBus::new(),
        }
    }
}

async fn countdown(handle: tauri::AppHandle, state: State<'_, Arc<Mutex<AppState>>>) {
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
            handle.emit_to("main", "pomo_reset", "").unwrap();
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

#[tauri::command]
fn get_duration(handle: tauri::AppHandle) -> String {
                    let stores = handle.state::<StoreCollection<Wry>>();
    let duration = with_store(handle.clone(), stores, get_store_path(), |store| {
        let dura = store
            .get("duration")
            .expect("duration does not exist")
            .clone();
        Ok(dura.as_u64())
    })
    .expect("Duration is None")
    .expect("Duration is None");
    seconds_to_string(duration)
}

#[tauri::command]
async fn run(
    app_handle: tauri::AppHandle,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<(), ()> {
    countdown(app_handle, state).await;
    Ok(())
}

#[tauri::command]
async fn pause(state: State<'_, Arc<Mutex<AppState>>>) -> Result<(), ()> {
    state.lock().unwrap().pause_switch = true;
    Ok(())
}

#[tauri::command]
async fn reset(state: State<'_, Arc<Mutex<AppState>>>) -> Result<(), ()> {
    state.lock().unwrap().kill_switch = true;
    Ok(())
}

fn main() {
    let state = Arc::new(Mutex::new(AppState::default()));
    let tray_menu = SystemTrayMenu::new(); // insert the menu items here
    let system_tray = SystemTray::new().with_menu(tray_menu);
    match tauri::Builder::default()
        .system_tray(system_tray)
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(state)
        .setup(|app| {
            app.get_window("main").unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_duration, run, pause, reset])
        .build(tauri::generate_context!())
    {
        Ok(app) => {
            let mut store = StoreBuilder::new(app.handle(), get_store_path()).build();
            if !store.has("duration") {
                store.insert("duration".to_string(), json!(300)).unwrap();
                store.save().unwrap();
            }
            app.run(|_app, event| match event {
                tauri::RunEvent::ExitRequested { api, .. } => {
                    api.prevent_exit();
                }
                _ => {}
            })
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
