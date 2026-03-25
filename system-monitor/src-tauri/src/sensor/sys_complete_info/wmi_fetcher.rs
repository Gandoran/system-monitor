#[cfg(target_os = "windows")]

use wmi::{COMLibrary, WMIConnection};
use super::complete_info_data::SystemSpecsPayload;
use super::fetcher;

pub struct WmiFetcher {
    conn: Option<WMIConnection>,
}

impl WmiFetcher {
    pub fn new() -> Self { 
        unsafe {
            let com_lib = COMLibrary::assume_initialized();
            let conn = match WMIConnection::new(com_lib.into()) {
                Ok(c) => Some(c),
                Err(_e) => None
            };
            Self { conn }
        }
    }

    pub fn get_specs(&self) -> SystemSpecsPayload {
        let mut payload = SystemSpecsPayload::default();
        
        if let Some(wmi) = &self.conn {
            payload.os = fetcher::os_fetcher::fetch(wmi);
            payload.mobo = fetcher::mobo_fetcher::fetch(wmi);
            payload.cpu = fetcher::cpu_fetcher::fetch(wmi);
            payload.ram = fetcher::ram_fetcher::fetch(wmi);
            payload.disks = fetcher::disk_fetcher::fetch(wmi);
            payload.displays = fetcher::display_fetcher::fetch(wmi);
        }
        
        payload
    }
}