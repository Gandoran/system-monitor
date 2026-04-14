#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code, unused_imports)]

use tauri::Manager;
use std::sync::{Arc, Mutex};

mod sensor;
mod command;
mod app_mode;
mod background_worker;

use app_mode::AppMode;
use crate::sensor::gpu_sensor::GpuIdentity;
use crate::sensor::session::tracker::SessionTracker;
use crate::sensor::ram_sensor::RamSensor;

fn main() {
    let app_mode = Arc::new(Mutex::new(AppMode::Hardware));
    let current_gpu_identity = Arc::new(Mutex::new(GpuIdentity {
        gpu_model: "Caricamento...".to_string(),
        gpu_driver: "...".to_string(),
        vram_total: 0,
        power_max_w: 0.0,
        max_mhz: 0,
    }));
    
    let session_tracker = Arc::new(Mutex::new(SessionTracker::new()));
    let mode_for_thread = app_mode.clone();
    let gpu_for_thread = current_gpu_identity.clone();
    let tracker_for_thread = session_tracker.clone();
    let ram_total = RamSensor::get_static_info().ram_total as f32;

    tauri::Builder::default()
        .manage(app_mode) 
        .manage(current_gpu_identity)
        .manage(session_tracker)
        .invoke_handler(tauri::generate_handler![
            command::specs_command::get_static_specs_info,
            command::system_command::get_static_sys_info,
            command::ram_command::get_static_ram_info,
            command::cpu_command::get_static_cpu_info,
            command::disk_command::get_static_disk_info,
            command::gpu_command::get_static_gpu_info,
            command::session_command::get_session_history,
            command::session_command::start_session,
            command::session_command::stop_session,
            command::session_command::delete_session,
            app_mode::set_app_mode,
        ])
        .setup(move |app| {
            background_worker::spawn_monitoring_thread(
                app.handle().clone(),
                mode_for_thread,
                gpu_for_thread,
                tracker_for_thread,
                ram_total
            );
            Ok(()) 
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}