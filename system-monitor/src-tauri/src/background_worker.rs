use tauri::{AppHandle, Emitter};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use crate::app_mode::AppMode;
use crate::sensor::gpu_sensor::GpuIdentity;
use crate::sensor::hardware_orchestrator::HardwareOrchestrator;
use crate::sensor::process::process_sensor::ProcessSensor;
use crate::sensor::session_tracker::SessionTracker;

pub fn spawn_monitoring_thread(
    app_handle: AppHandle,
    mode_state: Arc<Mutex<AppMode>>,
    gpu_identity_state: Arc<Mutex<GpuIdentity>>,
    session_tracker_state: Arc<Mutex<SessionTracker>>,
) {
    thread::spawn(move || {
        let mut orchestrator = HardwareOrchestrator::new();
        let mut process = ProcessSensor::new();
        {
            let mut locked_identity = gpu_identity_state.lock().unwrap();
            *locked_identity = orchestrator.get_current_gpu_identity();
        }
        loop {
            let current_mode = { mode_state.lock().unwrap().clone() };
            let is_session_running = { session_tracker_state.lock().unwrap().is_running };
            match current_mode {
                AppMode::Hardware => {
                    let stats = orchestrator.read_all(); 
                    if stats.gpu_stats.needs_static_update {
                        let mut locked_identity = gpu_identity_state.lock().unwrap();
                        *locked_identity = orchestrator.get_current_gpu_identity();
                    }
                    let _ = app_handle.emit("system-stats", &stats);     
                    if is_session_running {
                        let mut tracker = session_tracker_state.lock().unwrap();
                        tracker.update(
                            stats.cpu_temp.cpu_temp, 
                            stats.cpu_stats.cpu_usage,
                            stats.gpu_stats.gpu_temp, 
                            stats.gpu_stats.gpu_usage, 
                            stats.ram_stats.ram_used as f32
                        );
                    }
                }
                AppMode::Processes => {
                    let top_processes = process.read_top_processes(20);
                    let _ = app_handle.emit("process-stats", &top_processes);
                    if is_session_running {
                        update_session_light(&mut orchestrator, &session_tracker_state);
                    }
                }
                AppMode::Info | AppMode::Session => {
                    if is_session_running {
                        update_session_light(&mut orchestrator, &session_tracker_state);
                    }
                }
            }
            thread::sleep(Duration::from_millis(1000));
        }
    });
}

fn update_session_light(
    orchestrator: &mut HardwareOrchestrator, 
    session_tracker_state: &Arc<Mutex<SessionTracker>>
) {
    let light_stats = orchestrator.read_session_only();
    let mut tracker = session_tracker_state.lock().unwrap();
    tracker.update(
        light_stats.cpu_temp.cpu_temp, 
        light_stats.cpu_stats.cpu_usage,
        light_stats.gpu_stats.gpu_temp, 
        light_stats.gpu_stats.gpu_usage, 
        light_stats.ram_stats.ram_used as f32
    );
}