use nvml_wrapper::Nvml;
use nvml_wrapper::enum_wrappers::device::{TemperatureSensor,Clock};
use crate::sensor::gpu_sensor::{GpuStrategy,GpuMetrics,GpuIdentity};

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

    fn empty_stats_m(&self) -> GpuMetrics {
        GpuMetrics {
            gpu_usage: 0.0, gpu_temp: 0.0, vram_used: 0, power_draw_w: 0.0, 
            fan_speed_pct: 0, gpu_mhz: 0, gpu_max_temp: 0.0, gpu_active:false, needs_static_update:false,
        }
    }

    fn empty_stats_i(&self) -> GpuIdentity{
        GpuIdentity {
            gpu_model: self.model.clone(), gpu_driver: self.driver.clone(),
            vram_total: 0, power_max_w: 0.0, max_mhz: 0,
        }
    }
}

impl GpuStrategy for NvidiaStrategy {
    fn read(&self) -> GpuMetrics {
        let Ok(device) = self.nvml.device_by_index(0) else {return self.empty_stats_m();};
        let Ok(usage) = device.utilization_rates() else {return self.empty_stats_m();};
        let mem_info = device.memory_info().unwrap();
        GpuMetrics {
            gpu_usage: usage.gpu as f32,
            gpu_temp: device.temperature(TemperatureSensor::Gpu).unwrap_or(0) as f32,
            vram_used: mem_info.used,
            power_draw_w: device.power_usage().unwrap_or(0) as f32 / 1000.0,
            fan_speed_pct: device.fan_speed(0).unwrap_or(0),
            gpu_mhz: device.clock_info(Clock::Graphics).unwrap_or(0),
            gpu_max_temp: 0.0,
            gpu_active: usage.gpu > 0 || mem_info.used > 800 * 1024 * 1024,
            needs_static_update:false
        }
    }

    fn get_static_gpu_info(&self) -> GpuIdentity{
        let Ok(device) = self.nvml.device_by_index(0) else {return self.empty_stats_i();};
        GpuIdentity {
            gpu_model: self.model.clone(),
            gpu_driver: self.driver.clone(),
            vram_total: device.memory_info().map(|m| m.total).unwrap_or(0),
            power_max_w: device.enforced_power_limit().unwrap_or(0) as f32,
            max_mhz: device.max_clock_info(Clock::Graphics).unwrap_or(0),
        }
    }
}