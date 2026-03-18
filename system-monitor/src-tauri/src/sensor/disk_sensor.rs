use sysinfo::{Disks, System, ProcessesToUpdate};

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiskIdentity {
    pub disk_name: String,      
    pub file_system: String,    
    pub disk_type: String,      
    pub disk_total_memory: u64, 
}

#[derive(serde::Serialize, Clone)]
pub struct DiskMetrics {
    pub disk_used_memory: u64,
    pub disk_read: u64,
    pub disk_write: u64,
    pub disk_use: f32, 
}

pub struct DiskSensor {
    disks: Disks,
    sys: System,
    max_speed_bps: u64, 
    prev_read: u64,
    prev_write: u64,
}

impl DiskSensor {
    pub fn new() -> Self {
        let mut disks = Disks::new_with_refreshed_list();
        let sys = System::new();
        let identity = Self::extract_identity(&mut disks);
        let max_speed_bps = if identity.disk_type == "HDD" {150 * 1024 * 1024} else {1000 * 1024 * 1024};
        Self { disks, sys, max_speed_bps, prev_read: 0, prev_write: 0 }
    }

    pub fn read(&mut self) -> DiskMetrics {
        self.disks.refresh(true);
        self.sys.refresh_processes(ProcessesToUpdate::All, true);
        let used = Self::calc_used_space(&self.disks);
        let (read_per_sec, write_per_sec) = self.calc_io_rates();

        let total_io_per_sec = read_per_sec + write_per_sec;    
        let disk_use = ((total_io_per_sec as f32 / self.max_speed_bps as f32) * 100.0).clamp(0.0, 100.0);

        DiskMetrics {
            disk_used_memory: used,
            disk_read: read_per_sec,
            disk_write: write_per_sec,
            disk_use,
        }
    }

    pub fn get_static_info() -> DiskIdentity {
        let mut disks = Disks::new_with_refreshed_list();
        Self::extract_identity(&mut disks)
    }

    fn extract_identity(disks: &mut Disks) -> DiskIdentity {
        match disks.iter().next() {
            Some(disk) => {
                let kind_str = match disk.kind() {
                    sysinfo::DiskKind::SSD => "SSD",
                    sysinfo::DiskKind::HDD => "HDD",
                    _ => "Unknown",
                };
                DiskIdentity {
                    disk_name: disk.mount_point().to_string_lossy().into_owned(),
                    file_system: disk.file_system().to_string_lossy().into_owned(),
                    disk_type: kind_str.to_string(),
                    disk_total_memory: disk.total_space(),
                }
            },
            None => DiskIdentity {
                disk_name: "N/A".to_string(),
                file_system: "N/A".to_string(),
                disk_type: "Unknown".to_string(),
                disk_total_memory: 0,
            },
        }
    }

    fn calc_used_space(disks: &Disks) -> u64 {
        match disks.iter().next() {
            Some(disk) => disk.total_space().saturating_sub(disk.available_space()),
            None => 0,
        }
    }

    fn calc_io_rates(&mut self) -> (u64, u64) {
        let (current_read, current_write) = self.sys.processes().values().fold((0, 0), |(r, w), process| {
            let usage = process.disk_usage();
            (r + usage.total_read_bytes, w + usage.total_written_bytes)
        });

        let read_rate = current_read.saturating_sub(self.prev_read);
        let write_rate = current_write.saturating_sub(self.prev_write);

        self.prev_read = current_read;
        self.prev_write = current_write;

        (read_rate, write_rate)
    }
}

