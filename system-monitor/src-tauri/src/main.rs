// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;
use std::time::Duration;

mod sensor;
use sensor::hardware_orchestrator::HardwareOrchestrator;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


fn main() {
    // 1. ECCO IL NOSTRO "CONSOLE.LOG" NATIVO!
    println!("Inizio lettura sensori...");

    thread::spawn(move || {
        let mut orchestrator = HardwareOrchestrator::new();
        loop {
            let stats = orchestrator.read_all();
            println!(
                "CPU: {:.1}% | Temp: {}°C | RAM: {} / {}", 
                stats.cpu_stats.cpu_usage,
                stats.cpu_temp.cpu_temp,
                stats.ram_stats.ram_used,
                stats.ram_stats.ram_total,
            );

            println!{
                "GPU {} , {} , {} , {}",
                stats.gpu_stats.gpu_usage,
                stats.gpu_stats.gpu_temp,
                stats.gpu_stats.vram_used,
                stats.gpu_stats.vram_total,
            }
            thread::sleep(Duration::from_millis(1000));
        }
    });

    // 2. Avvio dell'applicazione Tauri
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
