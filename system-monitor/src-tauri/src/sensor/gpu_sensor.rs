use crate::sensor::gpu_strategy::gpu_chain::GpuChain;

#[derive(serde::Serialize, Clone)]
pub struct GpuIdentity {
    pub gpu_model: String,
    pub gpu_driver: String,
    pub gpu_active: bool,
    pub vram_total: u64,
    pub power_max_w: f32,
    pub max_mhz: u32,
}

#[derive(serde::Serialize, Clone)]
pub struct GpuMetrics {
    pub gpu_usage: f32,
    pub gpu_temp: f32,
    pub vram_used: u64,
    pub power_draw_w: f32,
    pub fan_speed_pct: u32,
    pub gpu_mhz: u32,
}

#[derive(serde::Serialize, Clone)]
pub struct GpuStats {
    #[serde(flatten)]
    pub identity: GpuIdentity,
    #[serde(flatten)]
    pub metrics: GpuMetrics,
    pub gpu_max_temp: f32,
}

pub trait GpuStrategy {
    fn read(&self) -> GpuStats;
}

pub struct GpuSensor {
    chain: GpuChain,
    gpu_max_temp: f32, 
}

impl GpuSensor {
    pub fn new() -> Self {
        Self {
            chain: GpuChain::new(),
            gpu_max_temp: 0.0,
        }
    }

    pub fn read(&mut self) -> GpuStats {
        let mut final_stats = self.chain.execute().unwrap_or_else(|| self.empty_stats());
        if final_stats.metrics.gpu_temp > 0.0 {
            self.gpu_max_temp = self.gpu_max_temp.max(final_stats.metrics.gpu_temp);
        }
        final_stats.gpu_max_temp = self.gpu_max_temp;
        final_stats
    }

fn empty_stats(&self) -> GpuStats {
        GpuStats {
            identity: GpuIdentity {
                gpu_model: "No Active GPU".to_string(),gpu_driver:"No Driver found".to_string(),
                gpu_active: false, vram_total: 0, power_max_w:0.0, max_mhz:0,
            },
            metrics: GpuMetrics {
                gpu_usage: 0.0, gpu_temp: 0.0, vram_used: 0, power_draw_w: 0.0, fan_speed_pct: 0, gpu_mhz:0,
            },
            gpu_max_temp: 0.0,
        }
    }
}