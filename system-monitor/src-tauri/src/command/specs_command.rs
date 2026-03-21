use tauri::command;
use crate::sensor::sys_complete_info::wmi_fetcher::{WmiFetcher};
use crate::sensor::sys_complete_info::complete_info_data::{SystemSpecsPayload};


#[command]
pub fn get_static_specs_info() -> SystemSpecsPayload {
    let fetcher = WmiFetcher::new();
    WmiFetcher::get_specs(&fetcher)
}