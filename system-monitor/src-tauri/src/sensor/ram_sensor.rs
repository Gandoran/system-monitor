use sysinfo::System;

pub struct RamStats{
    pub ram_used : u64,
    pub ram_total : u64,
}

pub struct RamSensor {
    sys: System, 
}

impl RamSensor {
    pub fn new() -> Self {
        Self {
            sys: System::new(), 
        }
    }

    // L'Azione: Aggiorna i dati e restituisce la nostra struct RamStats
    pub fn read(&mut self) -> RamStats {
        self.sys.refresh_memory();
        RamStats{
            ram_used : self.sys.used_memory(), 
            ram_total : self.sys.total_memory()
        }
    }
}