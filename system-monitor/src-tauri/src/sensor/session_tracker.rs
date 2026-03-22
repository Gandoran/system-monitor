use serde::Serialize;

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SessionResults {
    pub process_name: Option<String>,
    pub duration_seconds: u64,
    pub cpu_max_temp: f32,
    pub cpu_avg_temp: f32,
    pub cpu_avg_load: f32,
    pub gpu_max_temp: f32,
    pub gpu_avg_temp: f32,
    pub gpu_avg_load: f32,
    pub ram_avg_load: f32,
}

pub struct SessionTracker {
    pub is_running: bool,
    pub target_process: Option<String>,
    ticks: u64,
    cpu_max_temp: f32,
    cpu_sum_temp: f32,
    cpu_sum_load: f32,
    gpu_max_temp: f32,
    gpu_sum_temp: f32,
    gpu_sum_load: f32,
    ram_sum_load: f32,
}

impl SessionTracker {
    pub fn new() -> Self {
        Self {
            is_running: false, target_process: None, ticks: 0, cpu_max_temp: 0.0,
            cpu_sum_temp: 0.0, cpu_sum_load: 0.0, gpu_max_temp: 0.0,
            gpu_sum_temp: 0.0, gpu_sum_load: 0.0, ram_sum_load: 0.0,
        }
    }
    pub fn start(&mut self, process: Option<String>) {
        Self::new();
        self.is_running = true;
        self.target_process = process;
    }

    pub fn update(&mut self, cpu_temp: f32, cpu_load: f32, gpu_temp: f32, gpu_load: f32, ram_load: f32) {
        if !self.is_running {return;}
        self.ticks += 1;
        if cpu_temp > self.cpu_max_temp { self.cpu_max_temp = cpu_temp; }
        if gpu_temp > self.gpu_max_temp { self.gpu_max_temp = gpu_temp; }
        self.cpu_sum_temp += cpu_temp;
        self.cpu_sum_load += cpu_load;
        self.gpu_sum_temp += gpu_temp;
        self.gpu_sum_load += gpu_load;
        self.ram_sum_load += ram_load;
    }

    pub fn stop(&mut self) -> SessionResults {
        self.is_running = false;
        let divider = if self.ticks > 0 { self.ticks as f32 } else { 1.0 };
        SessionResults {
            process_name: self.target_process.clone(),
            duration_seconds: self.ticks,
            cpu_max_temp: self.cpu_max_temp,
            cpu_avg_temp: self.cpu_sum_temp / divider,
            cpu_avg_load: self.cpu_sum_load / divider,
            gpu_max_temp: self.gpu_max_temp,
            gpu_avg_temp: self.gpu_sum_temp / divider,
            gpu_avg_load: self.gpu_sum_load / divider,
            ram_avg_load: self.ram_sum_load / divider,
        }
    }
}