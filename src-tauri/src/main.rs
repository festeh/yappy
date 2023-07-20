// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use async_std::task;
use serde::{Deserialize, Serialize};
use std::{thread, time::Duration};
// use tauri
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu};

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
    for i in (1..=5).rev() {
        if state.lock().unwrap().kill_switch {
            state.lock().unwrap().kill_switch = false;
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
            app.emit_to("main", "start", "Booyaka").unwrap();
            println!("emited");
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
