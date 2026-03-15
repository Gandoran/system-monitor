use crate::sensor::gpu_sensor::{GpuStrategy,GpuStats};
use crate::sensor::gpu_strategy::nvidia_strategy::NvidiaStrategy;
use crate::sensor::gpu_strategy::intel_strategy::IntelStrategy;

pub struct GpuChain {
    strategies: Vec<Box<dyn GpuStrategy>>,
}

impl GpuChain {
    pub fn new() -> Self {
        let mut strategies: Vec<Box<dyn GpuStrategy>> = Vec::new();

        // 1. Dedicated gpu
        if let Ok(nv) = NvidiaStrategy::new() {
            strategies.push(Box::new(nv));
        }
        
        //Amd strategy for future

        // 2. Integrated gpu
        strategies.push(Box::new(IntelStrategy::new()));

        Self { strategies }
    }

    pub fn execute(&self) -> Option<GpuStats> {
        for strat in &self.strategies {
            let stats = strat.read();
            
            if stats.identity.gpu_active {
                return Some(stats);
            }
        }
        
        None
    }
}