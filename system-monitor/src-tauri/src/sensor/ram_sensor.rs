use sysinfo::System;

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RamIdentity {
    pub ram_total: u64,
}

#[derive(serde::Serialize, Clone)]
pub struct RamMetrics {
    pub ram_used: u64,
    pub ram_available: u64,
    pub swap_used: u64,
    pub swap_total: u64,
}

pub struct RamSensor {
    sys: System,
}

impl RamSensor {
    pub fn new() -> Self {
        let mut sys= System::new();
        sys.refresh_memory();
        Self { sys }
    }
    
    pub fn read(&mut self) -> RamMetrics {
        self.sys.refresh_memory();
        RamMetrics {
            ram_used: self.sys.used_memory(),
            ram_available: self.sys.available_memory(), 
            swap_used: self.sys.used_swap(),
            swap_total: self.sys.total_swap(), 
        }
    }

    pub fn get_static_info() -> RamIdentity {
        let mut sys = System::new();
        sys.refresh_memory();
        RamIdentity {
            ram_total: sys.total_memory(),
        }
    }
}