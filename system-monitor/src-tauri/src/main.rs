// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Emitter, State};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
mod sensor;
mod command;
use sensor::hardware_orchestrator::HardwareOrchestrator;
use sensor::process::process_sensor::ProcessSensor;
use sensor::process::sorter::SortCriteria;

#[derive(Clone, PartialEq)]
pub enum AppMode {
    Hardware,
    Processes,
}

#[tauri::command]
fn set_app_mode(mode: String, state: State<'_, Arc<Mutex<AppMode>>>) {
    let mut current_mode = state.lock().unwrap();
    match mode.as_str() {
        "processes" => *current_mode = AppMode::Processes,
        _ => *current_mode = AppMode::Hardware,
    }
}

fn main() {
    let app_mode = Arc::new(Mutex::new(AppMode::Hardware));
    let mode_for_thread = app_mode.clone();

    tauri::Builder::default().manage(app_mode) 
        .invoke_handler(tauri::generate_handler![
            command::system_command::get_static_sys_info,
            command::ram_command::get_static_ram_info,
            command::cpu_command::get_static_cpu_info,
            command::disk_command::get_static_disk_info,
            set_app_mode,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone(); 
            thread::spawn(move || {
                let mut orchestrator = HardwareOrchestrator::new();
                let mut process = ProcessSensor::new(); 
                loop {
                    let current_mode = {
                        let lock = mode_for_thread.lock().unwrap();
                        lock.clone()
                    };
                    match current_mode {
                        AppMode::Hardware => {
                            let stats = orchestrator.read_all();
                            app_handle.emit("system-stats", &stats).unwrap();
                            thread::sleep(Duration::from_millis(1000));
                        }
                        AppMode::Processes => {
                            let top_processes = process.read_top_processes(20, SortCriteria::Cpu);
                            app_handle.emit("process-stats", &top_processes).unwrap();
                            thread::sleep(Duration::from_millis(2000));
                        }
                    }
                }
            });
            Ok(()) 
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
