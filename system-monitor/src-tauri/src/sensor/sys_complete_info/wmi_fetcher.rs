#[cfg(target_os = "windows")] //Only for windows

//TODO FIX DELLA MADOONA E AGGIUNTA ROBA 
use wmi::{COMLibrary, WMIConnection};
use serde::Deserialize;
use super::complete_info_data::{SystemSpecsPayload, MoboInfo, RamSpecs, CpuSpecs};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32_BaseBoard {
    Manufacturer: Option<String>,
    Product: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32_BIOS {
    SMBIOSBIOSVersion: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32_PhysicalMemory {
    Speed: Option<u32>,
    FormFactor: Option<u16>,
    Manufacturer: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32_Processor {
    MaxClockSpeed: Option<u32>,
    Manufacturer: Option<String>,
    Architecture: Option<u16>,
}

pub struct WmiFetcher {
    conn: Option<WMIConnection>,
}

impl WmiFetcher {
    pub fn new() -> Self {
        println!("🔄 Inizializzazione libreria COM per WMI...");
        
        // Non usiamo 'match' perché assume_initialized() restituisce direttamente la libreria.
        // Diciamo a Rust: "Fidati, Tauri ha già aperto le porte per Windows!"
        unsafe{let com_lib = COMLibrary::assume_initialized();

        // Usiamo la libreria di Tauri per connetterci al vero database WMI
        let conn = match WMIConnection::new(com_lib.into()) {
            Ok(c) => {
                println!("✅ Connessione al database WMI stabilita (grazie a Tauri)!");
                Some(c)
            },
            Err(e) => {
                println!("❌ Errore WMIConnection: {:?}", e);
                None
            }
        };

        Self { conn }}
    }

    // ... la tua get_specs(&self) rimane qui sotto identica ...

    pub fn get_specs(&self) -> SystemSpecsPayload {
        let mut payload = SystemSpecsPayload::default();
        
        if let Some(wmi) = &self.conn {
            // Sezione BaseBoard (Scheda Madre)
            match wmi.query::<Win32_BaseBoard>() {
                Ok(boards) => {
                    if let Some(board) = boards.first() {
                        payload.mobo.vendor = board.Manufacturer.clone().unwrap_or_else(|| "Sconosciuto".into());
                        payload.mobo.model = board.Product.clone().unwrap_or_else(|| "Sconosciuto".into());
                    }
                },
                Err(e) => println!("⚠️ Errore lettura BaseBoard: {:?}", e),
            }

            // Sezione BIOS
            match wmi.query::<Win32_BIOS>() {
                Ok(bios) => {
                    if let Some(b) = bios.first() {
                        payload.mobo.bios_version = b.SMBIOSBIOSVersion.clone().unwrap_or_else(|| "N/A".into());
                    }
                },
                Err(e) => println!("⚠️ Errore lettura BIOS: {:?}", e),
            }

            // Sezione RAM
            match wmi.query::<Win32_PhysicalMemory>() {
                Ok(rams) => {
                    if let Some(ram) = rams.first() {
                        payload.ram.speed_mts = ram.Speed.unwrap_or(0);
                        payload.ram.manufacturer = ram.Manufacturer.clone().unwrap_or_else(|| "N/A".into());
                        payload.ram.form_factor = match ram.FormFactor.unwrap_or(0) {
                            8 => "DIMM".to_string(),
                            12 => "SO-DIMM".to_string(),
                            _ => "Sconosciuto".to_string(),
                        };
                    }
                },
                Err(e) => println!("⚠️ Errore lettura RAM: {:?}", e),
            }

            // Sezione CPU
            match wmi.query::<Win32_Processor>() {
                Ok(cpus) => {
                    if let Some(cpu) = cpus.first() {
                        payload.cpu.max_clock_mhz = cpu.MaxClockSpeed.unwrap_or(0);
                        payload.cpu.vendor = cpu.Manufacturer.clone().unwrap_or_else(|| "N/A".into());
                        payload.cpu.architecture = match cpu.Architecture.unwrap_or(9) {
                            0 => "x86".to_string(),
                            9 => "x64 (x86_64)".to_string(),
                            12 => "ARM64".to_string(),
                            _ => "Unknown".to_string(),
                        }
                    }
                },
                Err(e) => println!("⚠️ Errore lettura CPU: {:?}", e),
            }
        } else {
            println!("🛑 RICHIESTA BLOCCATA: Nessuna connessione WMI disponibile.");
        }

        payload
    }
}