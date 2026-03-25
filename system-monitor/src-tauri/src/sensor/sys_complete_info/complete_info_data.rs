use serde::Serialize;

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct OsInfo {
    pub name: String,
    pub kernel_version: String,
    pub build_number: String,
    pub architecture: String,
}

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CpuSpecs {
    pub architecture: String,
    pub vendor: String,
    pub max_clock_mhz: u32,
    pub l3_cache_mb: usize,
}

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MoboInfo {
    pub vendor: String,
    pub model: String,
    pub bios_version: String,
}

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RamSpecs {
    pub total_capacity_bytes: u64,
    pub speed_mts: u32,
    pub form_factor: String,
    pub manufacturer: String,
    pub part_number: String,
}

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DisplaySpecs {
    pub name: String,
    pub resolution_x: u32,
    pub resolution_y: u32,
    pub refresh_rate_hz: u32,
}

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DiskSpecs {
    pub model: String,
    pub capacity_bytes: u64,
    pub media_type: String,
    pub interface_type: String,
}

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkSpecs {
    pub name: String,
    pub mac_address: String,
}

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SystemSpecsPayload {
    pub os: OsInfo,
    pub cpu: CpuSpecs,
    pub mobo: MoboInfo,
    pub ram: RamSpecs,
    pub displays: Vec<DisplaySpecs>,
    pub disks: Vec<DiskSpecs>,
    pub networks: Vec<NetworkSpecs>,
}