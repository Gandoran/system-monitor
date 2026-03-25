use wmi::WMIConnection;
use crate::sensor::sys_complete_info::complete_info_data::DiskSpecs;
use crate::sensor::sys_complete_info::wmi_data::Win32_DiskDrive;

pub fn fetch(wmi: &WMIConnection) ->Vec<DiskSpecs>{
    let mut specs: Vec<DiskSpecs> = Vec::default();
    match wmi.query::<Win32_DiskDrive>(){
        Ok(wmi_disks) => {
            for d in wmi_disks {
                if let Some(capacity) = d.size {
                    let new_disk = DiskSpecs {
                        model: d.model.unwrap_or_else(|| "Unknown Disk".to_string()),
                        capacity_bytes: capacity,
                        media_type: d.media_type.unwrap_or_else(|| "Unknown".to_string()),
                        interface_type: d.interface_type.unwrap_or_else(|| "Unknown".to_string()),
                    };
                    specs.push(new_disk);
                }
            }
        },
        Err(e) => println!("Errore lettura Dischi: {:?}", e),
    }
    specs
}