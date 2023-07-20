// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::get_store_path;
use async_std::task;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::Wry;
use tauri::async_runtime::handle;
use tauri_plugin_store::StoreCollection;
use std::{thread, time::Duration};
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu};
use tauri_plugin_store::with_store;
use tauri_plugin_store::StoreBuilder;

use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
struct AppState {
    kill_switch: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self { kill_switch: false }
    }
}

async fn countdown(handle: tauri::AppHandle, state: State<'_, Arc<Mutex<AppState>>>) {
    handle.emit_to("main", "pomo_started", "").unwrap();
    state.lock().unwrap().kill_switch = false;
    let stores = handle.state::<StoreCollection<Wry>>();
    let duration = with_store(handle.clone(), stores, get_store_path(), |store| {
        let dura = store.get("duration").expect("duration exist").clone();
        Ok(dura)
    }).unwrap();
    for i in (1..=duration.as_i64().expect("is int")).rev() {
        if state.lock().unwrap().kill_switch {
            return;
        }
        println!("Time remaining: {} seconds", i);
        handle.emit_to("main", "pomo_step", i).unwrap();
        task::sleep(Duration::from_secs(1)).await;
    }
    handle.emit_to("main", "pomo_step", 0).unwrap();
    handle.emit_to("main", "pomo_finished", "").unwrap();
}

#[tauri::command]
async fn run(
    app_handle: tauri::AppHandle,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<(), ()> {
    println!("Running... {:?}", state);
    countdown(app_handle, state).await;
    Ok(())
}

#[tauri::command]
async fn pause(
    app_handle: tauri::AppHandle,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<(), ()> {
    println!("Pause... {:?}", state);
    state.lock().unwrap().kill_switch = true;
    app_handle.emit_to("main", "pomo_paused", "").unwrap();
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
            let main_window = app.get_window("main").unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![run, pause])
        .build(tauri::generate_context!())
    {
        Ok(app) => {
            println!("Started");
            let mut store = StoreBuilder::new(app.handle(), get_store_path()).build();
            println!("Store: {:?}", store);
            if !store.has("duration") {
                println!("No duration");
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
