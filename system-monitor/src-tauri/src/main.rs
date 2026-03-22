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
use crate::sensor::session_tracker::SessionTracker;

fn main() {
    let app_mode = Arc::new(Mutex::new(AppMode::Hardware));
    let mode_for_thread = app_mode.clone();

    let session_tracker = Arc::new(Mutex::new(SessionTracker::new()));
    let tracker_for_thread = session_tracker.clone();
    tauri::Builder::default()
        .manage(app_mode) 
        .manage(session_tracker)
        .invoke_handler(tauri::generate_handler![
            command::specs_command::get_static_specs_info,
            command::system_command::get_static_sys_info,
            command::ram_command::get_static_ram_info,
            command::cpu_command::get_static_cpu_info,
            command::disk_command::get_static_disk_info,
            command::session_command::start_session,
            command::session_command::stop_session,
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
                    let is_session_running = { tracker_for_thread.lock().unwrap().is_running };
                    match current_mode {
                        AppMode::Hardware => {
                            let stats = orchestrator.read_all();
                            app_handle.emit("system-stats", &stats);
                            thread::sleep(Duration::from_millis(1000));
                            if is_session_running {
                                let mut tracker = tracker_for_thread.lock().unwrap();
                                tracker.update(
                                    stats.cpu_temp.cpu_temp, 
                                    stats.cpu_stats.cpu_usage,
                                    stats.gpu_stats.metrics.gpu_temp, 
                                    stats.gpu_stats.metrics.gpu_usage, 
                                    stats.ram_stats.ram_used as f32
                                );
                            }
                        }
                        AppMode::Processes => {
                            let top_processes = process.read_top_processes(20);
                            app_handle.emit("process-stats", &top_processes);
                            let light_stats = orchestrator.read_session_only();
                            let mut tracker = tracker_for_thread.lock().unwrap();
                            tracker.update(
                                light_stats.cpu_temp.cpu_temp, 
                                light_stats.cpu_stats.cpu_usage,
                                light_stats.gpu_stats.metrics.gpu_temp, 
                                light_stats.gpu_stats.metrics.gpu_usage, 
                                light_stats.ram_stats.ram_used as f32
                            );
                            thread::sleep(Duration::from_millis(1000));
                        }
                        AppMode::Info | AppMode::Session => {
                                let light_stats = orchestrator.read_session_only();
                                let mut tracker = tracker_for_thread.lock().unwrap();
                                tracker.update(
                                    light_stats.cpu_temp.cpu_temp, 
                                    light_stats.cpu_stats.cpu_usage,
                                    light_stats.gpu_stats.metrics.gpu_temp, 
                                    light_stats.gpu_stats.metrics.gpu_usage, 
                                    light_stats.ram_stats.ram_used as f32
                                );
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

