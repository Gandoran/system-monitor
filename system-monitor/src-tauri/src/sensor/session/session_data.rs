use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionResults {
    pub process_name: Option<String>,
    pub duration_seconds: u64,
    pub cpu_max_temp: f32,
    pub cpu_avg_temp: f32,
    pub cpu_max_load: f32,
    pub cpu_avg_load: f32,
    pub gpu_max_temp: f32,
    pub gpu_avg_temp: f32,
    pub gpu_max_load: f32,
    pub gpu_avg_load: f32,
    pub ram_max_load: f32,
    pub ram_avg_load: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionHistory {
    #[serde(rename = "Session")]
    pub sessions: Vec<SessionResults>,
}