use sysinfo::System;

#[derive(serde::Serialize, Clone)]
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

#[derive(serde::Serialize, Clone)]
pub struct RamStats {
    #[serde(flatten)]
    pub identity: RamIdentity,
    #[serde(flatten)]
    pub metrics: RamMetrics,
}

pub struct RamSensor {
    sys: System,
    identity: RamIdentity,
}

impl RamSensor {
    pub fn new() -> Self {
        let mut sys= System::new();
        sys.refresh_memory();
        let identity = RamIdentity {ram_total: sys.total_memory(),};
        Self { sys, identity }
    }
    
    pub fn read(&mut self) -> RamStats {
        self.sys.refresh_memory();
        RamStats {
            identity: self.identity.clone(), 
            metrics: RamMetrics {
                ram_used: self.sys.used_memory(),
                ram_available: self.sys.available_memory(), 
                swap_used: self.sys.used_swap(),
                swap_total: self.sys.total_swap(), 
            },
        }
    }
}