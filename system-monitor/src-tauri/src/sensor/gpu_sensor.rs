use super::gpu_strategy::nvidia_strategy::NvidiaStrategy;

#[derive(serde::Serialize, Clone)]
pub struct GpuStats {
    pub gpu_usage: f32,
    pub gpu_temp: f32,
    pub gpu_max_temp: f32,
    pub vram_used: u64,
    pub vram_total: u64,
}

pub trait GpuStrategy {
    fn read(&self) -> GpuStats;
}

pub struct GpuSensor {
    strategy: Option<Box<dyn GpuStrategy>>,
    gpu_max_temp: f32, 
}

impl GpuSensor {
    pub fn new() -> Self {
        // Qui dentro, in futuro, proveremo a inizializzare prima Nvidia, poi AMD, ecc.
        // Per ora forziamo Nvidia. Se fallisce (es. PC senza Nvidia), mettiamo None.
        let strategy: Option<Box<dyn GpuStrategy>> = match NvidiaStrategy::new() {
            Ok(nv) => Some(Box::new(nv)),
            Err(_) => None,
        };
        Self { strategy, gpu_max_temp: 0.0 }
    }

    pub fn read(&mut self) -> GpuStats {
        let mut stats=match &self.strategy {
            Some(strat) => strat.read(),
            None => GpuStats { gpu_usage: 0.0, gpu_temp: 0.0, gpu_max_temp: 0.0, vram_used: 0, vram_total: 0 },
        };
        if stats.gpu_temp > 0.0 {
            self.gpu_max_temp = self.gpu_max_temp.max(stats.gpu_temp);
        }
        stats.gpu_max_temp = self.gpu_max_temp;
        stats
    }
}