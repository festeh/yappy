use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use crate::notification::send_notification;
use crate::seconds_to_string;
use crate::state::AppState;
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
            println!("Settings: {:?}", sett);
            sett.get("duration").unwrap().to_int()
        }
    };
    println!("Duration: {}", duration);
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
            }
        }
    });
}
