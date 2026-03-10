// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Emitter;
use std::thread;
use std::time::Duration;
mod sensor;
use sensor::hardware_orchestrator::HardwareOrchestrator;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone(); 
            thread::spawn(move || {
                let mut orchestrator = HardwareOrchestrator::new();
                loop {
                    let stats = orchestrator.read_all();
                    app_handle.emit("system-stats", &stats).unwrap();
                    thread::sleep(Duration::from_millis(1000));
                }
            });
            Ok(()) 
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
