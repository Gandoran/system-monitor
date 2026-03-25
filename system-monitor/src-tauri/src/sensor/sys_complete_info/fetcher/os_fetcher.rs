use wmi::WMIConnection;
use crate::sensor::sys_complete_info::complete_info_data::OsInfo;
use crate::sensor::sys_complete_info::wmi_data::Win32_OperatingSystem;

pub fn fetch(wmi: &WMIConnection) ->OsInfo{
    let mut specs = OsInfo::default();
    match wmi.query::<Win32_OperatingSystem>() {
        Ok(oses) => {
            if let Some(os) = oses.first() {
                specs.name = os.caption.clone().unwrap_or_else(|| "Unknown OS".into());
                specs.kernel_version = os.version.clone().unwrap_or_else(|| "N/A".into());
                specs.build_number = os.build_number.clone().unwrap_or_else(|| "N/A".into());
                specs.architecture = os.os_architecture.clone().unwrap_or_else(|| "N/A".into());
            }
        },
        Err(e) => println!("Errore lettura OS: {:?}", e),
    }
    specs
}