// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Emitter;
use std::thread;
use std::time::Duration;
mod sensor;
mod command;
use sensor::hardware_orchestrator::HardwareOrchestrator;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::system_command::get_static_sys_info,
            command::ram_command::get_static_ram_info,
            command::cpu_command::get_static_cpu_info,
            command::disk_command::get_static_disk_info,
        ])
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
