use wmi::WMIConnection;
use crate::sensor::sys_complete_info::complete_info_data::MoboInfo;
use crate::sensor::sys_complete_info::wmi_data::{Win32_BaseBoard,Win32_BIOS};

pub fn fetch(wmi: &WMIConnection) ->MoboInfo{
    let mut specs = MoboInfo::default();
    match wmi.query::<Win32_BIOS>() {
        Ok(bios) => {
            if let Some(b) = bios.first() {
                specs.bios_version = b.smbiosbios_version.clone().unwrap_or_else(|| "N/A".into());
            }
        },
        Err(e) => println!("Errore lettura BIOS: {:?}", e),
    }
    match wmi.query::<Win32_BaseBoard>() {
        Ok(boards) => {
            if let Some(board) = boards.first() {
                specs.vendor = board.manufacturer.clone().unwrap_or_else(|| "Sconosciuto".into());
                specs.model = board.product.clone().unwrap_or_else(|| "Sconosciuto".into());
            }
        },
        Err(e) => println!("Errore lettura BaseBoard: {:?}", e),
    }
    specs
}