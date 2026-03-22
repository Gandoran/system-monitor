use sysinfo::{System, ProcessesToUpdate};
use crate::sensor::process::sorter::ProcessSorter;
use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProcessMetrics {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub ram_usage: u64,
    pub disk_read: u64,
    pub disk_write: u64, 
}

pub struct ProcessSensor {
    sys: System,
}

impl ProcessSensor {
    pub fn new() -> Self {
        let mut sys = System::new();
        sys.refresh_processes(ProcessesToUpdate::All, true);
        sys.refresh_memory();
        Self { sys }
    }

    pub fn read_top_processes(&mut self, limit: usize) -> Vec<ProcessMetrics> {
        self.sys.refresh_processes(ProcessesToUpdate::All, true);
        let mut metrics_list = Vec::with_capacity(self.sys.processes().len());
        let total_ram = self.sys.total_memory(); //violazione single responsability      
        for (pid, process) in self.sys.processes() {
            let disk_usage = process.disk_usage();
            metrics_list.push(ProcessMetrics {
                pid: pid.as_u32(),
                name: process.name().to_string_lossy().into_owned(),
                cpu_usage: process.cpu_usage(),
                ram_usage: process.memory(),
                disk_read: disk_usage.read_bytes,
                disk_write: disk_usage.written_bytes,
            });
        }
        ProcessSorter::get_top_n(metrics_list, limit, total_ram)    
    }
}