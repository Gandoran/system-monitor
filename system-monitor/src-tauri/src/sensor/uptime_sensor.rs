use sysinfo::System;

#[derive(serde::Serialize, Clone)]
pub struct UptimeStats{
    pub uptime: u64,
}

pub struct UptimeSensor;

impl UptimeSensor {
    pub fn new() -> Self {
        Self
    }

    pub fn read(&self) -> UptimeStats {
        UptimeStats{
            uptime: System::uptime(),
        }   
    }
}