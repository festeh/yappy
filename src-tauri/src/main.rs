// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use async_std::channel::bounded;
use tauri::SystemTrayEvent;
use tauri::{SystemTray, SystemTrayMenu, SystemTrayMenuItem};
use yappy::dbus::DBus;
use yappy::handling::handle_messages;
use yappy::state::AppState;
use yappy::store::{get_settings_store_path, PersistentStore, Value};
use yappy::todoist::Task;
use yappy::{seconds_to_string, InternalMessage};

use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri::State;

fn load_duration(state: State<'_, Arc<Mutex<AppState>>>) -> u64 {
    state
        .lock()
        .unwrap()
        .settings
        .get("duration")
        .unwrap_or_else(|| &Value::Int(300))
        .to_int()
}

#[tauri::command]
fn get_duration(state: State<'_, Arc<Mutex<AppState>>>) -> String {
    let duration = load_duration(state);
    seconds_to_string(duration)
}

#[tauri::command]
fn get_duration_seconds(state: State<'_, Arc<Mutex<AppState>>>) -> u64 {
    let duration = load_duration(state);
    duration
}

fn send_message(msg: InternalMessage, state: State<'_, Arc<Mutex<AppState>>>) {
    state
        .lock()
        .unwrap()
        .s
        .try_send(msg.clone())
        .expect(&format!("Failed to send {:?}", &msg));
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
async fn reset(
    _handle: tauri::AppHandle,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<(), ()> {
    send_message(InternalMessage::PomoReseted, state);
    Ok(())
}

#[tauri::command]
fn get_tasks(state: State<'_, Arc<Mutex<AppState>>>) -> Vec<Task> {
    let res = async_std::task::block_on(async move {
        state
            .lock()
            .unwrap()
            .todoist
            .lock()
            .await
            .get_tasks()
            .unwrap()
    });
    res
}

#[tauri::command]
fn reload_tasks(state: State<'_, Arc<Mutex<AppState>>>) {
    send_message(InternalMessage::TaskReloadRequested, state);
}

#[tauri::command]
fn save_todoist_api_key(key: String, state: State<'_, Arc<Mutex<AppState>>>) {
    send_message(InternalMessage::TodoistApiKey(key), state);
}

#[tauri::command]
fn save_firebase_address(address: String, state: State<'_, Arc<Mutex<AppState>>>) {
    send_message(InternalMessage::FirebaseAddress(address), state);
}

#[tauri::command]
fn save_firebase_auth_key(key: String, state: State<'_, Arc<Mutex<AppState>>>) {
    send_message(InternalMessage::FirebaseAuthKey(key), state);
}

#[tauri::command]
fn select_task(state: State<'_, Arc<Mutex<AppState>>>, id: Option<String>) {
    match id {
        Some(id) => {
            send_message(InternalMessage::TaskSelected(id), state);
        }
        None => {
            send_message(InternalMessage::TaskUnselected, state);
        }
    }
}

#[tauri::command]
fn get_selected_task(state: State<'_, Arc<Mutex<AppState>>>) -> Result<String, String> {
    match state.lock().unwrap().settings.get("selected_task") {
        Some(Value::Text(id)) => Ok(id.into()),
        _ => Err("No selected task".into()),
    }
}

fn get_tray() -> SystemTray {
    let run_item = tauri::CustomMenuItem::new("start".to_string(), "Start");
    let pause_item = tauri::CustomMenuItem::new("pause".to_string(), "Pause");
    let reset_item = tauri::CustomMenuItem::new("reset".to_string(), "Reset");
    let pause_item = pause_item.disabled();
    let reset_item = reset_item.disabled();
    let quit_item = tauri::CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(run_item)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(pause_item)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(reset_item)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit_item);
    SystemTray::new().with_menu(tray_menu)
}

fn main() {
    let (s, r) = bounded::<InternalMessage>(256);
    let s_tray = s.clone();
    let state = Arc::new(Mutex::new(AppState::new(&s)));
    let system_tray = get_tray();
    match tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(move |_app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "start" => {
                    s_tray.try_send(InternalMessage::PomoStarted).unwrap();
                }
                "pause" => {
                    s_tray.try_send(InternalMessage::PomoPaused).unwrap();
                }
                "reset" => {
                    s_tray.try_send(InternalMessage::PomoReseted).unwrap();
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
        .manage(state.clone())
        .setup(|app| {
            app.get_window("main").unwrap();
            let handle = app.handle();
            app.listen_global("duration_changed", move |event| {
                let payload = event.payload().expect("Payload is empty");
                let duration: u64 = payload.parse::<u64>().unwrap();
                let state = handle.state::<Arc<Mutex<AppState>>>().clone();
                send_message(InternalMessage::DurationChanged(duration), state);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_duration,
            get_duration_seconds,
            run,
            pause,
            reset,
            get_tasks,
            reload_tasks,
            save_todoist_api_key,
            select_task,
            get_selected_task,
            save_firebase_address,
            save_firebase_auth_key,
        ])
        .build(tauri::generate_context!())
    {
        Ok(app) => {
            let mut store = PersistentStore::new(get_settings_store_path());
            if !store.check("duration") {
                store.set("duration".into(), yappy::store::Value::Int(300))
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
