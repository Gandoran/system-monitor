use tauri::command;
use crate::sensor::sys_info_sensor::{SysInfoSensor, StaticSysInfo};

#[command]
pub fn get_static_sys_info() -> StaticSysInfo {
    SysInfoSensor::get_static_info()
}