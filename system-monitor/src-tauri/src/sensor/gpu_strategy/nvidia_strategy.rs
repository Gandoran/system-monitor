use nvml_wrapper::Nvml;
use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use crate::sensor::gpu_sensor::{GpuStrategy,GpuStats,GpuMetrics,GpuIdentity};

pub struct NvidiaStrategy {
    nvml: Nvml,
    model: String,
    driver: String,
}

impl NvidiaStrategy {
    pub fn new() -> Result<Self, nvml_wrapper::error::NvmlError> {
        let nvml = Nvml::init()?;
        let model = nvml.device_by_index(0)?.name().unwrap_or_else(|_| "Nvidia GPU".to_string());
        let driver = nvml.sys_driver_version().unwrap_or_else(|_| "Unknown".to_string());
        Ok(Self { nvml, model, driver })
    }
    fn empty_stats(&self) -> GpuStats {
        GpuStats {     
            identity: GpuIdentity {
                gpu_model: self.model.clone(), gpu_driver: self.driver.clone(),
                gpu_active: false, vram_total: 0,
            },
            metrics: GpuMetrics {
                gpu_usage: 0.0, gpu_temp: 0.0, vram_used: 0, power_draw_w: 0.0, fan_speed_pct: 0,
            },
            gpu_max_temp: 0.0,
        }
    }
}

impl GpuStrategy for NvidiaStrategy {
    fn read(&self) -> GpuStats {
        let Ok(device) = self.nvml.device_by_index(0) else {return self.empty_stats();};
        let Ok(usage) = device.utilization_rates() else {return self.empty_stats();};
        let mem_info = device.memory_info().unwrap();
        GpuStats {
            identity: GpuIdentity {
                gpu_model: self.model.clone(),
                gpu_driver: self.driver.clone(),
                gpu_active: usage.gpu > 1 || mem_info.used > 350 * 1024 * 1024,
                vram_total: mem_info.total,
            },
            metrics: GpuMetrics {
                gpu_usage: usage.gpu as f32,
                gpu_temp: device.temperature(TemperatureSensor::Gpu).unwrap_or(0) as f32,
                vram_used: mem_info.used,
                power_draw_w: device.power_usage().unwrap_or(0) as f32 / 1000.0,
                fan_speed_pct: device.fan_speed(0).unwrap_or(0),
            },
            gpu_max_temp: 0.0,
        }
    }
}