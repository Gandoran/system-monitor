//I don't even know what this class do, the intel gpu doesnt have a properly library
use crate::sensor::gpu_sensor::{GpuStrategy, GpuIdentity, GpuMetrics};
use wmi::{COMLibrary, WMIConnection};
use serde::Deserialize;
use winreg::enums::*;
use winreg::RegKey;
use sysinfo::System;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct GpuEngine {
    name: String,
    utilization_percentage: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct GpuAdapterMemory {
    dedicated_usage: u64,
    shared_usage: u64,
}

pub struct IntelStrategy {
    identity: GpuIdentity,
}

impl IntelStrategy {
    pub fn new() -> Self {
        Self {
            identity: build_identity_from_registry(),
        }
    }
}

impl GpuStrategy for IntelStrategy {
    fn read(&self) -> GpuMetrics {
        let com_con = unsafe { COMLibrary::assume_initialized() };
        let Ok(wmi) = WMIConnection::new(com_con.into()) else {return empty_stats_m()};
        let mut gpu_usage = read_gpu_usage(&wmi);
        let vram_used = read_vram_used(&wmi);
        let gpu_active = gpu_usage > 0.0 || vram_used > 500 * 1024 * 1024;
        gpu_usage = gpu_usage.clamp(0.0, 100.0);
        let power_draw_w = if gpu_usage > 0.0 {5.0 + (gpu_usage / 100.0) * (self.identity.power_max_w - 5.0)} else {2.0};
        let gpu_mhz = if gpu_usage > 0.0 {300 + ((gpu_usage / 100.0) * (self.identity.max_mhz as f32 - 300.0)) as u32} else {300};
        GpuMetrics {
            gpu_usage,
            vram_used,
            power_draw_w,
            gpu_mhz,
            fan_speed_pct: 0,
            gpu_temp: 0.0,
            gpu_max_temp: 0.0,
            gpu_active,
            needs_static_update: false,
        }
    }

    fn get_static_gpu_info(&self) -> GpuIdentity {
        self.identity.clone()
    }
}


fn empty_stats_m() -> GpuMetrics {
    GpuMetrics {
        gpu_usage: 0.0, gpu_temp: 0.0, vram_used: 0, power_draw_w: 0.0, 
        fan_speed_pct: 0, gpu_mhz: 0, gpu_max_temp: 0.0, gpu_active:false, needs_static_update:false,
    }
}

fn build_identity_from_registry() -> GpuIdentity {
    let mut identity = GpuIdentity {
        gpu_model: "Intel GPU (unknown)".to_string(),
        gpu_driver: "Unknown".to_string(),
        vram_total: 0,
        power_max_w: 30.0,
        max_mhz: 1200,
    };
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let gpu_class = "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e968-e325-11ce-bfc1-08002be10318}";
    let Ok(class_key) = hklm.open_subkey(gpu_class) else {
        return identity;
    };
    for subkey_name in class_key.enum_keys().filter_map(|r| r.ok()) {
        let Ok(device_key) = class_key.open_subkey(&subkey_name) else {continue;};
        let Ok(desc) = device_key.get_value::<String, _>("DriverDesc") else {continue;};
        if !desc.to_lowercase().contains("intel") {continue;}
        identity.gpu_model = desc.clone();
        if let Ok(ver) = device_key.get_value::<String, _>("DriverVersion") {identity.gpu_driver = ver;}
        identity.vram_total = read_vram_from_device_key(&device_key, &desc);
        let (max_w, max_mhz) = specs_for_model(&desc);
        identity.power_max_w = max_w  * 1000.0;
        print!("{}",max_w);
        identity.max_mhz = max_mhz;
        break;
    }

    identity
}

fn read_vram_from_device_key(key: &RegKey, desc: &str) -> u64 {
    if let Ok(qw) = key.get_value::<u64, _>("HardwareInformation.qwMemorySize") {
        if qw > 0 { return qw; }
    }
    if let Ok(raw) = key.get_raw_value("HardwareInformation.MemorySize") {
        let vram = match raw.bytes.len() {
            n if n >= 8 => {
                let mut arr = [0u8; 8];
                arr.copy_from_slice(&raw.bytes[..8]);
                u64::from_le_bytes(arr)
            }
            n if n >= 4 => {
                let mut arr = [0u8; 4];
                arr.copy_from_slice(&raw.bytes[..4]);
                u32::from_le_bytes(arr) as u64
            }
            _ => 0,
        };
        if vram > 0 { return vram; }
    }
    let desc_lower = desc.to_lowercase();
    let is_discrete = desc_lower.contains("arc a");
    if is_discrete {
        if desc_lower.contains("a770") { return 16 * 1073741824; } // 16 GB
        if desc_lower.contains("a750") || desc_lower.contains("a580") { return 8 * 1073741824; } // 8 GB
        if desc_lower.contains("a380") { return 6 * 1073741824; } // 6 GB
        return 8 * 1073741824; // Fallback generico per Arc (8GB)
    }
    let mut sys = System::new();
    sys.refresh_memory();
    sys.total_memory() / 2
}

fn read_gpu_usage(wmi: &WMIConnection) -> f32 {
    const QUERY: &str =
        "SELECT Name, UtilizationPercentage \
         FROM Win32_PerfFormattedData_GPUPerformanceCounters_GPUEngine";

    let Ok(engines) = wmi.raw_query::<GpuEngine>(QUERY) else {
        return 0.0;
    };

    let total: u32 = engines
        .into_iter()
        .filter(|e| e.name.contains("engtype_3D"))
        .map(|e| e.utilization_percentage)
        .sum();

    (total as f32).min(100.0)
}

fn read_vram_used(wmi: &WMIConnection) -> u64 {
    const QUERY: &str =
        "SELECT DedicatedUsage, SharedUsage \
         FROM Win32_PerfFormattedData_GPUPerformanceCounters_GPUAdapterMemory";

    let Ok(memories) = wmi.raw_query::<GpuAdapterMemory>(QUERY) else {
        return 0;
    };

    memories
        .into_iter()
        .map(|m| m.dedicated_usage + m.shared_usage)
        .sum()
}

fn specs_for_model(name: &str) -> (f32, u32) {
    let n = name.to_lowercase();
    match () {
        _ if n.contains("arc a770") => (225.0, 2400),
        _ if n.contains("arc a750") => (225.0, 2400),
        _ if n.contains("arc a580") => (185.0, 2000),
        _ if n.contains("arc a380") => (75.0,  2000),
        _ if n.contains("arc a310") => (30.0,  2000),
        _ if n.contains("iris xe") =>  (28.0,  1300),
        _ if n.contains("uhd") =>      (15.0,  1100),
        _ =>                           (30.0,  1200),
    }
}

pub fn get_real_vram_from_registry(gpu_name_filter: &str) -> (u64, u64) {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e968-e325-11ce-bfc1-08002be10318}";

    let Ok(class_key) = hklm.open_subkey(path) else {
        return (0, 0);
    };

    for name in class_key.enum_keys().filter_map(|x| x.ok()) {
        let Ok(device_key) = class_key.open_subkey(&name) else {
            continue;
        };
        let Ok(desc) = device_key.get_value::<String, _>("DriverDesc") else {
            continue;
        };
        if desc.to_lowercase().contains(&gpu_name_filter.to_lowercase()) {
            let vram = read_vram_from_device_key(&device_key, &desc);
            return (vram, 0);
        }
    }

    (0, 0)
}