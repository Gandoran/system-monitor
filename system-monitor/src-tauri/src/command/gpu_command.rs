use tauri::State;
use std::sync::{Arc, Mutex};
use crate::sensor::gpu_sensor::GpuIdentity;

#[tauri::command]
pub fn get_static_gpu_info(state: State<'_, Arc<Mutex<GpuIdentity>>>) -> GpuIdentity {
    state.lock().unwrap().clone()
}