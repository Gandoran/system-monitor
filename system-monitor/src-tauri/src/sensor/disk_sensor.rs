use sysinfo::{Disks, System, ProcessesToUpdate};

#[derive(serde::Serialize, Clone)]
pub struct DiskStats {
    pub disk_write: u64,
    pub disk_read: u64,
    pub disk_total_memory: u64,
    pub disk_used_memory: u64,
    pub disk_use: f32,
}

pub struct DiskSensor {
    disks: Disks,
    sys: System,
    prev_read: u64,
    prev_write: u64,
}

impl DiskSensor {
    pub fn new() -> Self {
        Self {
            disks: Disks::new_with_refreshed_list(),
            sys: System::new(),
            prev_read: 0,
            prev_write: 0,
        }
    }

    pub fn read(&mut self) -> DiskStats {
        self.disks.refresh(true);
        self.sys.refresh_processes(ProcessesToUpdate::All, true);

        let (total, used) = self.disks.iter().fold((0, 0), |(t, u), disk| {
            (t + disk.total_space(), u + (disk.total_space() - disk.available_space()))
        });

        let (current_read, current_write) = self.sys.processes().values().fold((0, 0), |(r, w), process| {
            let usage = process.disk_usage();
            (r + usage.total_read_bytes, w + usage.total_written_bytes)
        });

        let read_per_sec = if self.prev_read == 0 { 0 } else { current_read.saturating_sub(self.prev_read) };
        let write_per_sec = if self.prev_write == 0 { 0 } else { current_write.saturating_sub(self.prev_write) };

        self.prev_read = current_read;
        self.prev_write = current_write;

        let disk_use = if total > 0 { (used as f32 / total as f32) * 100.0 } else { 0.0 };

        DiskStats {
            disk_write: write_per_sec,
            disk_read: read_per_sec,
            disk_total_memory: total,
            disk_used_memory: used,
            disk_use,
        }
    }
}