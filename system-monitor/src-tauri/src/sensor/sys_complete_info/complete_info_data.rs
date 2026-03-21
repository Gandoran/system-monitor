use serde::Serialize;

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct OsInfo {
    pub name: String,
    pub kernel_version: String,
}

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CpuSpecs {
    pub architecture: String,
    pub vendor: String,
    pub max_clock_mhz: u32,
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
    pub speed_mts: u32,
    pub form_factor: String,
    pub manufacturer: String,
}

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DisplaySpecs{

}

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DiskSpecs{
    
}

//COMPLETE
#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SystemSpecsPayload {
    pub os: OsInfo,
    pub cpu: CpuSpecs,
    pub mobo: MoboInfo,
    pub ram: RamSpecs,
    //pub disk: DiskSpecs,
    //pib display: DisplaySpecs,
}