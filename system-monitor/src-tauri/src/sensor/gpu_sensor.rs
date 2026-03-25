use crate::sensor::gpu_strategy::gpu_chain::GpuChain;

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GpuIdentity {
    pub gpu_model: String,
    pub gpu_driver: String,
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
    pub gpu_max_temp: f32,
    pub gpu_active: bool,
    pub needs_static_update: bool,
}

pub trait GpuStrategy {
    fn read(&self) -> GpuMetrics;
    fn get_static_gpu_info(&self)->GpuIdentity;
}

pub struct GpuSensor {
    chain: GpuChain,
    gpu_max_temp: f32,
    last_active_index: usize,
}

impl GpuSensor {
    pub fn new() -> Self {
        Self {
            chain: GpuChain::new(),
            gpu_max_temp: 0.0,
            last_active_index: 999,
        }
    }

    pub fn read(&mut self) -> GpuMetrics {
        let mut final_stats = self.chain.execute().unwrap_or_else(|| self.empty_stats_m());
        if final_stats.gpu_temp > 0.0 {
            self.gpu_max_temp = self.gpu_max_temp.max(final_stats.gpu_temp);
        }
        final_stats.gpu_max_temp = self.gpu_max_temp;
        if self.chain.active_index != self.last_active_index {
            final_stats.needs_static_update = true;
            self.last_active_index = self.chain.active_index;
        } else {
            final_stats.needs_static_update = false;
        }
        final_stats
    }

    pub fn get_static_gpu_info(&self)->GpuIdentity{
        self.chain.get_static_gpu_info().unwrap_or_else(|| self.empty_stats_i())
    }

    fn empty_stats_m(&self) -> GpuMetrics {
        GpuMetrics {
            gpu_usage: 0.0, gpu_temp: 0.0, vram_used: 0, power_draw_w: 0.0, fan_speed_pct: 0, gpu_mhz:0, gpu_max_temp: 0.0,
            gpu_active: false, needs_static_update: false,
        }
    }

    fn empty_stats_i(&self) -> GpuIdentity{
        GpuIdentity{
            gpu_model: ".".to_string(), gpu_driver: ".".to_string(), vram_total: 0, power_max_w: 0.0, max_mhz: 0,
        }
    }
}