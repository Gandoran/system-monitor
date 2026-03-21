struct SessionTracker {
    is_running: bool,
    target_process: Option<String>, // Per il futuro menu a tendina
    
    // Contatori per le medie
    ticks: u64, // Quanti secondi sono passati

    cpu_max_temp: f32,
    cpu_sum_temp: f32,
    cpu_sum_load: f32,

    gpu_max_temp: f32,
    gpu_sum_temp: f32,
    gpu_sum_load: f32,

    // RAM
    ram_sum_load: f32,
}