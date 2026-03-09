use nvml_wrapper::Nvml;
use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use crate::sensor::gpu_sensor::GpuStrategy;
use crate::sensor::gpu_sensor::GpuStats;

pub struct NvidiaStrategy {
    nvml: Nvml,
}

impl NvidiaStrategy {
    // Inizializza le API di Nvidia. Restituisce un Result perché potrebbe fallire 
    // (es. se i driver Nvidia non sono installati)
    pub fn new() -> Result<Self, nvml_wrapper::error::NvmlError> {
        let nvml = Nvml::init()?;
        Ok(Self { nvml })
    }
}

// Implementiamo il "Contratto" per la nostra strategia Nvidia!
impl GpuStrategy for NvidiaStrategy {
    fn read(&self) -> GpuStats {
        let device = self.nvml.device_by_index(0).unwrap();
        let mem_info = device.memory_info().unwrap();
        GpuStats{
            gpu_usage: device.utilization_rates().unwrap().gpu as f32,
            gpu_temp: device.temperature(TemperatureSensor::Gpu).unwrap() as f32,
            vram_used: mem_info.used,
            vram_total: mem_info.total,
        }
    }
}