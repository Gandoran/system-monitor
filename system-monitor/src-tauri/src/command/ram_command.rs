use tauri::command;
use crate::sensor::ram_sensor::{RamSensor,RamIdentity};

#[command]
pub fn get_static_ram_info() -> RamIdentity {
    RamSensor::get_static_info()
}