use sysinfo::System;

#[derive(serde::Serialize, Clone)]
pub struct CpuStats{
    pub cpu_usage : f32,
}

pub struct CpuSensor {
    sys: System, 
}

impl CpuSensor{
    pub fn new() -> Self {
        Self {
            sys: System::new(), 
        }
    }

    // L'Azione: Aggiorna i dati e restituisce la nostra struct RamStats
    pub fn read(&mut self) -> CpuStats {
        self.sys.refresh_cpu_usage();
        CpuStats{
            cpu_usage : self.sys.global_cpu_usage(), 
        }
    }
}