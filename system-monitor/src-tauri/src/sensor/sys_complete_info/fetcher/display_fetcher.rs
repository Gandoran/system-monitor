use wmi::WMIConnection;
use crate::sensor::sys_complete_info::complete_info_data::DisplaySpecs;
use crate::sensor::sys_complete_info::wmi_data::Win32_VideoController;

pub fn fetch(wmi: &WMIConnection) ->Vec<DisplaySpecs>{
    let mut specs: Vec<DisplaySpecs> = Vec::default();
    match wmi.query::<Win32_VideoController>(){
        Ok(wmi_displays) => {
            for disp in wmi_displays {
                if let (Some(res_x), Some(res_y)) = (disp.current_horizontal_resolution, disp.current_vertical_resolution) {
                    let new_display = DisplaySpecs {
                        name: disp.description.unwrap_or_else(|| "Generic Display".to_string()),
                        resolution_x: res_x,
                        resolution_y: res_y,
                        refresh_rate_hz: disp.current_refresh_rate.unwrap_or(60),
                    };
                    specs.push(new_display);
                }
            }
        },
        Err(e) => println!("Errore lettura Display: {:?}", e),
    }
    specs
}