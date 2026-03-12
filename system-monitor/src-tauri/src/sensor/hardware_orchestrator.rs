use super::gpu_sensor::{GpuSensor, GpuStats};
use super::cpu_sensor::{CpuSensor, CpuStats};
use super::ram_sensor::{RamSensor, RamStats};
use super::disk_sensor::{DiskSensor, DiskStats};
use super::net_sensor::{NetSensor,NetStats};
use super::cpu_temp_sensor::{TempSensor, TempStats};

#[derive(serde::Serialize, Clone)]
pub struct SystemStats {
    pub gpu_stats: GpuStats,
    pub cpu_stats: CpuStats,
    pub cpu_temp: TempStats,
    pub ram_stats: RamStats,
    pub disk_stats : DiskStats,
    pub net_stats : NetStats
}

pub struct HardwareOrchestrator{
    cpu_sensor : CpuSensor,
    cpu_temp_sensor : TempSensor,
    gpu_sensor : GpuSensor,
    ram_sensor : RamSensor,
    disk_sensor : DiskSensor,
    net_sensor : NetSensor
}

impl HardwareOrchestrator{
    pub fn new()->Self{
        Self {
            cpu_sensor : CpuSensor::new(),
            cpu_temp_sensor : TempSensor::new(),
            gpu_sensor : GpuSensor::new(),
            ram_sensor : RamSensor::new(),
            disk_sensor : DiskSensor::new(), 
            net_sensor : NetSensor::new()
        }
    }

    pub fn read_all(&mut self) -> SystemStats{
        SystemStats{
            gpu_stats : self.gpu_sensor.read(),
            cpu_stats : self.cpu_sensor.read(),
            cpu_temp : self.cpu_temp_sensor.read(),
            ram_stats : self.ram_sensor.read(),
            disk_stats : self.disk_sensor.read(),
            net_stats : self.net_sensor.read()
        }
    }
}
