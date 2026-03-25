use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Win32_OperatingSystem {
    pub caption: Option<String>,
    pub version: Option<String>,
    pub build_number: Option<String>,
    pub os_architecture: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Win32_BaseBoard {
    pub manufacturer: Option<String>,
    pub product: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Win32_BIOS {
    #[serde(rename = "SMBIOSBIOSVersion")]
    pub smbiosbios_version: Option<String>,
}   

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Win32_PhysicalMemory {
    pub capacity: Option<u64>,
    pub speed: Option<u32>,
    pub form_factor: Option<u16>,
    pub manufacturer: Option<String>,
    pub part_number: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Win32_Processor {
    pub max_clock_speed: Option<u32>,
    pub manufacturer: Option<String>,
    pub architecture: Option<u16>,
    pub l3_cache_size: Option<usize>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Win32_DiskDrive {
    pub model: Option<String>,
    pub size: Option<u64>, 
    pub media_type: Option<String>,
    pub interface_type: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Win32_VideoController {
    pub description: Option<String>,
    pub current_horizontal_resolution: Option<u32>,
    pub current_vertical_resolution: Option<u32>,
    pub current_refresh_rate: Option<u32>,
}