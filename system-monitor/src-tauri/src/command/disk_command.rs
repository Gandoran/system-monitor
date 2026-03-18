use tauri::command;
use crate::sensor::disk_sensor::{DiskSensor,DiskIdentity};

#[command]
pub fn get_static_disk_info() -> DiskIdentity {
    DiskSensor::get_static_info()
}