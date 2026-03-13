use sysinfo::System;
use serde::Serialize;


#[derive(Serialize, Clone)]
pub struct StaticSysInfo {
    pub hostname: String,
    pub os_name: String,
}

pub struct SysInfoSensor;

impl SysInfoSensor {
    pub fn get_static_info() -> StaticSysInfo {
        StaticSysInfo {
            hostname: System::host_name().unwrap_or_else(|| "Unknown Host".to_string()),
            os_name: System::long_os_version().unwrap_or_else(|| "Unknown OS".to_string()),
        }
    }
}