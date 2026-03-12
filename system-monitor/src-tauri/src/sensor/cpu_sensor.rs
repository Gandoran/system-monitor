use sysinfo::System;

#[derive(serde::Serialize, Clone)]
pub struct CpuStats{
    pub cpu_usage : f32,
    pub cores_load: Vec<f32>,
    pub frequency: f32,
    pub physical_cores: usize,
}

pub struct CpuSensor {
    sys: System, 
}

impl CpuSensor{
    pub fn new() -> Self {
        Self {
            sys: System::new_all(), 
        }
    }
    
    pub fn read(&mut self) -> CpuStats {
        self.sys.refresh_cpu_all();

        let global_usage = self.sys.global_cpu_usage();
        let cores_load: Vec<f32> = self.sys.cpus().iter().map(|core| core.cpu_usage()).collect();
        let physical_cores = sysinfo::System::physical_core_count().unwrap_or(0);
        let frequency = if let Some(first_core) = self.sys.cpus().first() {first_core.frequency() as f32 / 1000.0} else {0.0};

        CpuStats {
            cpu_usage: global_usage,
            cores_load,
            frequency,
            physical_cores,
        }
    }
}