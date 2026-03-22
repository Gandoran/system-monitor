use crate::sensor::process::process_sensor::ProcessMetrics;
use std::cmp::Ordering;

pub struct ProcessSorter;

impl ProcessSorter {
    pub fn get_top_n(mut processes: Vec<ProcessMetrics>, n: usize, total_ram: u64) -> Vec<ProcessMetrics> {
        let total_ram_f32 = if total_ram > 0 { total_ram as f32 } else { 1.0 };
        processes.sort_by(|a, b| {
            let a_cpu = if a.cpu_usage.is_nan() { 0.0 } else { a.cpu_usage };
            let b_cpu = if b.cpu_usage.is_nan() { 0.0 } else { b.cpu_usage };
            let a_ram_pct = (a.ram_usage as f32 / total_ram_f32) * 100.0;
            let b_ram_pct = (b.ram_usage as f32 / total_ram_f32) * 100.0;
            let a_score = a_cpu + a_ram_pct;
            let b_score = b_cpu + b_ram_pct;
            b_score.partial_cmp(&a_score).unwrap_or(Ordering::Equal)
        });
        processes.into_iter().take(n).collect()
    }
}