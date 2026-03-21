#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code, unused_imports)]

use tauri::{Emitter, Manager};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

mod sensor;
mod command;
mod app_mode;

use app_mode::AppMode;
use crate::sensor::hardware_orchestrator::HardwareOrchestrator;
use crate::sensor::process::process_sensor::ProcessSensor;
use crate::sensor::process::sorter::SortCriteria;

fn main() {
    let app_mode = Arc::new(Mutex::new(AppMode::Hardware));
    let mode_for_thread = app_mode.clone();

    tauri::Builder::default()
        .manage(app_mode) 
        .invoke_handler(tauri::generate_handler![
            command::specs_command::get_static_specs_info,
            command::system_command::get_static_sys_info,
            command::ram_command::get_static_ram_info,
            command::cpu_command::get_static_cpu_info,
            command::disk_command::get_static_disk_info,
            app_mode::set_app_mode,
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
                            let _ = app_handle.emit("system-stats", &stats);
                            thread::sleep(Duration::from_millis(1000));
                        }
                        AppMode::Processes => {
                            let top_processes = process.read_top_processes(20, SortCriteria::Cpu);
                            let _ = app_handle.emit("process-stats", &top_processes);
                            thread::sleep(Duration::from_millis(2000));
                        }
                        AppMode::Info | AppMode::Session => {
                            thread::sleep(Duration::from_millis(1000));
                        }
                    }
                }
            });
            Ok(()) 
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}