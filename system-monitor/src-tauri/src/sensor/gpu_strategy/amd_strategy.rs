use crate::sensor::gpu_sensor::{GpuStrategy, GpuStats, GpuIdentity, GpuMetrics};

pub struct AmdStrategy {
    model: String,
}

impl AmdStrategy {
    pub fn new() -> Self {
        Self {
            model: "Amd graphic card".to_string(), 
        }
    }
}

impl GpuStrategy for AmdStrategy {
    fn read(&self) -> GpuStats {
        // Restituiamo dati fittizi
        GpuStats {
            identity: GpuIdentity {
                gpu_model: self.model.clone(),
                gpu_driver : "666.xxx".to_string(),
                gpu_active: true, // Sempre VERO sui laptop!
                vram_total: 8192 * 1024 * 1024, // 8 GB condivisi
                power_max_w: 150.0,
                max_mhz: 1500,
            },
            metrics: GpuMetrics {
                gpu_usage: 12.0, // 12% di utilizzo finto
                gpu_temp: 45.0,  // 45 gradi
                vram_used: 512 * 1024 * 1024, // 512 MB di RAM di sistema condivisa
                power_draw_w: 15.0, // Consuma pochissimo (15W)
                fan_speed_pct: 0,   // Niente ventola dedicata
                gpu_mhz: 1250,
            },
            gpu_max_temp: 0.0,
        }
    }
}