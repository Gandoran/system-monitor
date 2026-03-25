use wmi::WMIConnection;
use crate::sensor::sys_complete_info::complete_info_data::CpuSpecs;
use crate::sensor::sys_complete_info::wmi_data::Win32_Processor;

pub fn fetch(wmi: &WMIConnection) ->CpuSpecs{
    let mut specs = CpuSpecs::default();
    match wmi.query::<Win32_Processor>() {
        Ok(cpus) => {
            if let Some(cpu) = cpus.first() {
                specs.max_clock_mhz = cpu.max_clock_speed.unwrap_or(0);
                specs.vendor = cpu.manufacturer.clone().unwrap_or_else(|| "N/A".into());
                specs.l3_cache_mb = cpu.l3_cache_size.unwrap_or(0) / 1024;
                specs.architecture = match cpu.architecture.unwrap_or(9) {
                    0 => "x86".to_string(),
                    9 => "x64 (x86_64)".to_string(),
                    12 => "ARM64".to_string(),
                    _ => "Unknown".to_string(),
                };
            }
        },
        Err(e) => println!("Errore lettura CPU: {:?}", e),
    }
    specs
}