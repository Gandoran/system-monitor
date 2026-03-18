use tauri::command;
use crate::sensor::cpu_sensor::{CpuSensor,CpuIdentity};

#[command]
pub fn get_static_cpu_info() -> CpuIdentity {
    CpuSensor::get_static_info()
}