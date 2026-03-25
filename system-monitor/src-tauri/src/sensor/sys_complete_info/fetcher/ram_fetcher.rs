use wmi::WMIConnection;
use crate::sensor::sys_complete_info::complete_info_data::RamSpecs;
use crate::sensor::sys_complete_info::wmi_data::Win32_PhysicalMemory;

pub fn fetch(wmi: &WMIConnection) -> RamSpecs {
    let mut specs = RamSpecs::default();   
    match wmi.query::<Win32_PhysicalMemory>() {
        Ok(rams) => {
            let mut total_capacity = 0;
            for ram in &rams {
                if let Some(cap) = ram.capacity {
                    total_capacity += cap;
                }
            }
            specs.total_capacity_bytes = total_capacity;
            if let Some(ram) = rams.first() {
                specs.speed_mts = ram.speed.unwrap_or(0);
                specs.manufacturer = ram.manufacturer.clone().unwrap_or_else(|| "N/A".into());
                specs.part_number = ram.part_number.clone().unwrap_or_else(|| "N/A".into()).trim().to_string();
                specs.form_factor = match ram.form_factor.unwrap_or(0) {
                    8 => "DIMM".to_string(),
                    12 => "SO-DIMM".to_string(),
                    _ => "Unknown".to_string(),
                };
            }
        },
        Err(e) => println!("Errore lettura RAM: {:?}", e),
    }
    specs
}