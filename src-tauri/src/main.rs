// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use async_std::channel::bounded;
use async_std::task;
use serde::{Deserialize, Serialize};
use serde_json::json;
use yappy::handling::handle_messages;
use std::time::Duration;
use tauri::{SystemTray, SystemTrayMenu, SystemTrayMenuItem};
use tauri::{SystemTrayEvent, Wry};
use tauri_plugin_store::with_store;
use tauri_plugin_store::StoreBuilder;
use tauri_plugin_store::StoreCollection;
use yappy::dbus::DBus;
use yappy::notification::send_notification;
use yappy::state::AppState;
use yappy::{get_store_path, seconds_to_string, InternalMessage};

use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri::State;

#[tauri::command]
fn get_duration(handle: tauri::AppHandle, state: State<'_, Arc<Mutex<AppState>>>) -> String {
    let stores = handle.state::<StoreCollection<Wry>>();
    let duration = with_store(handle.clone(), stores, get_store_path(), |store| {
        let dura = store
            .get("duration")
            .expect("duration does not exist")
            .clone();
        Ok(dura.as_u64())
    })
    .expect("Internal error in get_duration")
    .expect("Duration is None");
    seconds_to_string(duration)
}

fn send_message(msg: InternalMessage, state: State<'_, Arc<Mutex<AppState>>>) {
    state.lock().unwrap().s.try_send(msg.clone()).expect(&format!("Failed to send {:?}", &msg));
}

#[tauri::command]
async fn run(state: State<'_, Arc<Mutex<AppState>>>) -> Result<(), ()> {
    send_message(InternalMessage::PomoStarted, state);
    Ok(())
}

#[tauri::command]
async fn pause(state: State<'_, Arc<Mutex<AppState>>>) -> Result<(), ()> {
    send_message(InternalMessage::PomoPaused, state);
    Ok(())
}

#[tauri::command]
async fn reset(handle: tauri::AppHandle, state: State<'_, Arc<Mutex<AppState>>>) -> Result<(), ()> {
    send_message(InternalMessage::PomoReseted, state);
    Ok(())
}

fn main() {
    let (s, r) = bounded::<InternalMessage>(256);
    let s_tray = s.clone();
    let state = Arc::new(Mutex::new(AppState::new(&s)));
    let quit_item = tauri::CustomMenuItem::new("quit".to_string(), "Quit");
    let run_item = tauri::CustomMenuItem::new("run".to_string(), "Run");
    let pause_item = tauri::CustomMenuItem::new("pause".to_string(), "Pause");
    let reset_item = tauri::CustomMenuItem::new("reset".to_string(), "Reset");
    let tray_menu = SystemTrayMenu::new()
        .add_item(run_item)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(pause_item)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(reset_item)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit_item);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    match tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(move |app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "run" => {
                    s_tray.try_send(InternalMessage::PomoStarted).unwrap();
                }
                "quit" => {
                    let dbus = DBus::new();
                    dbus.send("Waiting");
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(state.clone())
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
            handle_messages(app.handle(), state, s, r);
            app.run(|_app, event| match event {
                tauri::RunEvent::ExitRequested { api, .. } => {
                    let dbus = DBus::new();
                    dbus.send("Waiting");
                    api.prevent_exit();
                }
                tauri::RunEvent::Exit => {
                    let dbus = DBus::new();
                    dbus.send("Waiting");
                }
                _ => {}
            });
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };
}

