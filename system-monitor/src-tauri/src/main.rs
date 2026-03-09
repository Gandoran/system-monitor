// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sysinfo::System;
use std::thread;
use std::time::Duration;
use std::ffi::c_void;

// Ricreiamo esattamente la struttura di memoria C++ che usa Core Temp!
#[repr(C)]
struct CoreTempSharedData {
    ui_load: [u32; 256],
    ui_tj_max: [u32; 128],
    ui_core_cnt: u32,
    ui_cpu_cnt: u32,
    f_temp: [f32; 256], // A noi interessa solo questo array!
}

struct SystemStats{
    cpu_usage : f32,
    ram_used : u64,
    ram_total : u64,
    cpu_temp : f32,
    //for future purposes
    gpu_usage : f32,
    gpu_temp : f32,
    gpu_vram : u64
}

// Interfaccia diretta col Kernel di Windows (niente crate esterni!)
#[link(name = "kernel32")]
extern "system" {
    fn OpenFileMappingA(dwDesiredAccess: u32, bInheritHandle: i32, lpName: *const u8) -> *mut c_void;
    fn MapViewOfFile(hFile: *mut c_void, dwDesiredAccess: u32, dwFileOffsetHigh: u32, dwFileOffsetLow: u32, dwNumberOfBytesToMap: usize) -> *mut c_void;
    fn UnmapViewOfFile(lpBaseAddress: *mut c_void) -> i32;
    fn CloseHandle(hObject: *mut c_void) -> i32;
}


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn read_core_temp() -> f32 {
    unsafe {
        const FILE_MAP_READ: u32 = 4;
        // Il nome in codice dell'area di memoria creata da Core Temp
        let name = b"CoreTempMappingObject\0";
        
        // 1. Chiediamo a Windows di aprire l'area di memoria
        let handle = OpenFileMappingA(FILE_MAP_READ, 0, name.as_ptr());
        if handle.is_null() {
            return 0.0; // Core Temp probabilmente è chiuso
        }

        // 2. Mappiamo la memoria nella nostra applicazione
        let map_ptr = MapViewOfFile(handle, FILE_MAP_READ, 0, 0, 0);
        let mut temp = 0.0;
        
        if !map_ptr.is_null() {
            // 3. IL VERO HACK: Facciamo finta che quel blocco di RAM grezza sia la nostra struct!
            let data = &*(map_ptr as *const CoreTempSharedData);
            temp = data.f_temp[0]; // Prendiamo la temperatura del Core 0
            
            // Chiudiamo l'accesso
            UnmapViewOfFile(map_ptr);
        }
        
        CloseHandle(handle);
        temp
    }
}

fn main() {
    // 1. ECCO IL NOSTRO "CONSOLE.LOG" NATIVO!
    println!("Inizio lettura sensori...");

    thread::spawn(move || {
        // SPOSTIAMO L'INIZIALIZZAZIONE QUI DENTRO!
        // Così non diamo fastidio al thread principale di Tauri
        let mut sys = System::new_all();

        loop {
            sys.refresh_cpu_usage();
            sys.refresh_memory();
            let cpu_temp = read_core_temp();

            println!(
                "CPU: {:.1}% | Temp: {}°C | RAM: {} / {}", 
                sys.global_cpu_usage(), 
                cpu_temp, 
                sys.used_memory(), 
                sys.total_memory()
            );
            
            thread::sleep(Duration::from_millis(1000));
        }
    });

    // 2. Avvio dell'applicazione Tauri
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
