use tauri::{command, State};
use std::sync::{Arc, Mutex};

#[derive(Clone, PartialEq, Debug)]
pub enum AppMode {
    Hardware,
    Processes,
    Info,
    Session,
}

#[command]
pub fn set_app_mode(mode: String, state: State<'_, Arc<Mutex<AppMode>>>) {
    let mut current_mode = state.lock().unwrap();
    *current_mode = match mode.as_str() {
        "processes" => AppMode::Processes,
        "hardware" => AppMode::Hardware,
        "info" => AppMode::Info,
        "session" => AppMode::Session,
        _ => AppMode::Hardware,
    };
}