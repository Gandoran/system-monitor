use crate::sensor::process::process_sensor::{ProcessMetrics};
use std::cmp::Ordering;

pub enum SortCriteria {
    Cpu,
    Ram,
}

pub struct ProcessSorter;

impl ProcessSorter {
    pub fn get_top_n(mut processes: Vec<ProcessMetrics>, n: usize, criteria: &SortCriteria) -> Vec<ProcessMetrics> {
        processes.sort_by(|a, b| {
            match criteria {
                SortCriteria::Cpu => b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(Ordering::Equal),
                SortCriteria::Ram => b.ram_usage.cmp(&a.ram_usage),
            }
        });
        processes.into_iter().take(n).collect()
    }
}