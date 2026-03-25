use crate::sensor::gpu_sensor::{GpuStrategy, GpuMetrics, GpuIdentity};
use crate::sensor::gpu_strategy::nvidia_strategy::NvidiaStrategy;
use crate::sensor::gpu_strategy::intel_strategy::IntelStrategy;

pub struct GpuChain {
    strategies: Vec<Box<dyn GpuStrategy>>,
    pub active_index: usize,
}

impl GpuChain {
    pub fn new() -> Self {
        let mut strategies: Vec<Box<dyn GpuStrategy>> = Vec::new();
        if let Ok(nv) = NvidiaStrategy::new() {
            strategies.push(Box::new(nv));
        }
        strategies.push(Box::new(IntelStrategy::new()));
        Self { strategies, active_index: 0 }
    }

    pub fn execute(&mut self) -> Option<GpuMetrics> {
        for (i, strat) in self.strategies.iter().enumerate() {
            let stats = strat.read();
            if stats.gpu_active || i == self.strategies.len() - 1 {
                self.active_index = i;
                return Some(stats);
            }
        }
        None
    }

    pub fn get_static_gpu_info(&self) -> Option<GpuIdentity> {
        if self.strategies.is_empty() { return None; }
        Some(self.strategies[self.active_index].get_static_gpu_info())
    }
}