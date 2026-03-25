use crate::sensor::gpu_sensor::{GpuStrategy, GpuIdentity, GpuMetrics};
use wmi::{COMLibrary, WMIConnection};
use serde::Deserialize;
use winreg::enums::*;
use winreg::RegKey;
use sysinfo::System;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32VideoController {
    name: String,
    driver_version: Option<String>,
    adapter_r_a_m: Option<u32>, // Windows restituisce la VRAM totale qui
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct GPUEngine {
    name: String,
    utilization_percentage: u32, // Il carico % in tempo reale
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct GPUAdapterMemory {
    name: String,
    dedicated_usage: u64,
    shared_usage: u64, // La VRAM usata in tempo reale
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32Fan {
    name: Option<String>,
    desired_speed: Option<u32>,
}

pub struct IntelStrategy {
    identity: GpuIdentity,
}

impl IntelStrategy {
    pub fn new() -> Self {
        let mut identity = GpuIdentity {
            gpu_model: "Intel GPU (Sconosciuta)".to_string(),
            gpu_driver: "Sconosciuto".to_string(),
            vram_total: 0,
            power_max_w: 30.0, // Fallback di sicurezza 
            max_mhz: 1200,     // Fallback di sicurezza
        };
        let com_con = match COMLibrary::new() {
            Ok(con) => con,
            Err(_) => unsafe { COMLibrary::assume_initialized() },
        };
        if let Ok(wmi_con) = WMIConnection::new(com_con.into()) {
            let query = "SELECT Name, DriverVersion, AdapterRAM FROM Win32_VideoController";
            if let Ok(controllers) = wmi_con.raw_query::<Win32VideoController>(query) {
                if let Some(intel_gpu) = controllers.into_iter().find(|c| c.name.to_lowercase().contains("intel")) {
                    identity.gpu_model = intel_gpu.name.clone();
                    let (max_w, max_mhz) = guess_intel_specs(&identity.gpu_model);
                    identity.power_max_w = max_w;
                    identity.max_mhz = max_mhz;
                    if let Some(driver) = intel_gpu.driver_version {
                        identity.gpu_driver = driver;
                    }
                    // 2. VRAM: Le integrate Intel spesso non hanno VRAM dedicata nel registro,
                    // quindi il fallback su AdapterRAM di WMI è vitale.
                    let (dedicata, _) = get_real_vram_from_registry("intel");
                    if dedicata > 0 {
                        identity.vram_total = dedicata;
                    } else if let Some(ram) = intel_gpu.adapter_r_a_m {
                        // AdapterRAM è in Bytes. (1GB = 1073741824 bytes)
                        let ram_u64 = ram as u64;  
                        // Se WMI dice che la VRAM è 1GB o meno, sta mentendo. È un'integrata!
                        if ram_u64 <= 1073741824 {
                            // La VRAM massima reale delle integrate è la metà della RAM del PC
                            let mut sys = System::new();
                            sys.refresh_memory();
                            identity.vram_total = sys.total_memory() / 2;
                        } else {
                            // Se è più di 1GB (es. una Intel Arc dedicata), fidiamoci di WMI
                            identity.vram_total = ram_u64;
                        }
                    }
                }
            }
        }
        Self { identity }
    }
}

impl GpuStrategy for IntelStrategy {
    fn read(&self) -> GpuMetrics {
        let mut gpu_usage = 0.0;
        let mut vram_used = 0;
        let mut fan_speed_pct = 0;
        let mut gpu_active = false;
        // LO STESSO FIX PER IL LOOP DI LETTURA
        let com_con = match COMLibrary::new() {
            Ok(con) => con,
            Err(_) => unsafe { COMLibrary::assume_initialized() },
        };
        if let Ok(wmi_con) = WMIConnection::new(com_con.into()) {
            // 1. Utilizzo
            let query_engine = "SELECT Name, UtilizationPercentage FROM Win32_PerfFormattedData_GPUPerformanceCounters_GPUEngine";
            if let Ok(engines) = wmi_con.raw_query::<GPUEngine>(query_engine) {
                let mut total_utilization = 0;
                for eng in engines {
                    if eng.name.contains("engtype_3D") {
                        total_utilization += eng.utilization_percentage;
                    }
                }
                gpu_usage = (total_utilization as f32).min(100.0);
                if gpu_usage > 0.0 { gpu_active = true; }
            }
            // 2. VRAM
            let query_mem = "SELECT Name, DedicatedUsage, SharedUsage FROM Win32_PerfFormattedData_GPUPerformanceCounters_GPUAdapterMemory";
            if let Ok(memories) = wmi_con.raw_query::<GPUAdapterMemory>(query_mem) {
                for mem in memories {
                    vram_used += mem.dedicated_usage + mem.shared_usage;
                }
                if vram_used > 500 * 1024 * 1024 { gpu_active = true; } 
            }
            let query_fan = "SELECT Name, DesiredSpeed FROM Win32_Fan";
            if let Ok(fans) = wmi_con.raw_query::<Win32Fan>(query_fan) {
                for fan in fans {
                    if let Some(name) = fan.name {
                        if name.to_lowercase().contains("gpu") || name.to_lowercase().contains("arc") {
                            fan_speed_pct = fan.desired_speed.unwrap_or(0);
                        }
                    }
                }
            }
        }
        let power_draw_w = if gpu_usage > 0.0 {
            5.0 + ((gpu_usage / 100.0) * (self.identity.power_max_w - 5.0))
        } else {2.0 };
        let gpu_mhz = if gpu_usage > 0.0 {
            300 + ((gpu_usage / 100.0) * (self.identity.max_mhz as f32 - 300.0)) as u32
        } else {300 };
        GpuMetrics {
            gpu_usage, vram_used, power_draw_w,   gpu_mhz,        
            fan_speed_pct, gpu_temp: 0.0, gpu_max_temp: 0.0,
            gpu_active,needs_static_update: false,
        }   
    }

    fn get_static_gpu_info(&self) -> GpuIdentity {
        self.identity.clone()
    }
}

pub fn get_real_vram_from_registry(gpu_name_filter: &str) -> (u64, u64) {
    let mut dedicated_vram: u64 = 0;
    let shared_vram: u64 = 0;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    // Questa è la chiave universale di Windows per tutte le schede video
    let path = "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e968-e325-11ce-bfc1-08002be10318}";
    if let Ok(class_key) = hklm.open_subkey(path) {
        // Cicliamo tra le sottocartelle (0000, 0001, ecc.)
        for name in class_key.enum_keys().filter_map(|x| x.ok()) {
            if let Ok(device_key) = class_key.open_subkey(&name) {
                // Leggiamo la descrizione del driver (Es. "Intel(R) Arc(TM) A770")
                if let Ok(desc) = device_key.get_value::<String, _>("DriverDesc") {
                    if desc.to_lowercase().contains(&gpu_name_filter.to_lowercase()) {        
                        // 1. VRAM DEDICATA (Spesso salvata come QWORD o come array di Byte BINARY)
                        // Proviamo prima a leggerla come QWORD (64-bit)
                        if let Ok(mem_qw) = device_key.get_value::<u64, _>("HardwareInformation.qwMemorySize") {
                            dedicated_vram = mem_qw;
                        } 
                        // Se fallisce, proviamo a leggerla come array di Byte e la convertiamo
                        else if let Ok(mem_bin) = device_key.get_raw_value("HardwareInformation.MemorySize") {
                            if mem_bin.bytes.len() >= 8 {
                                let mut arr = [0u8; 8];
                                arr.copy_from_slice(&mem_bin.bytes[0..8]);
                                dedicated_vram = u64::from_le_bytes(arr);
                            } else if mem_bin.bytes.len() >= 4 {
                                let mut arr = [0u8; 4];
                                arr.copy_from_slice(&mem_bin.bytes[0..4]);
                                dedicated_vram = u32::from_le_bytes(arr) as u64;
                            }
                        }
                        // Trovata la scheda giusta, possiamo fermare il ciclo
                        break; 
                    }
                }
            }
        }
    }
    (dedicated_vram, shared_vram)
}

// Aggiungiamo questa funzione helper fuori o dentro il file intel_strategy.rs
fn guess_intel_specs(gpu_name: &str) -> (f32, u32) {
    let name = gpu_name.to_lowercase();  
    // Restituisce (Power_Max_W, Max_MHz)
    if name.contains("arc a770") {
        (225.0, 2400)
    } else if name.contains("arc a750") {
        (225.0, 2400)
    } else if name.contains("arc a380") {
        (75.0, 2000)
    } else if name.contains("iris xe") {
        // Valori medi per processori mobile serie P/U
        (28.0, 1300) 
    } else if name.contains("uhd") {
        // Valori medi per vecchie integrate
        (15.0, 1100) 
    } else {
        // Fallback generico se non la riconosciamo
        (30.0, 1200) 
    }
}