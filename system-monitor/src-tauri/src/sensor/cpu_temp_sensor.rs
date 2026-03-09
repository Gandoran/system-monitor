use std::ffi::c_void;

// --- 1. ROBA DI WINDOWS ---
#[repr(C)]
struct CoreTempSharedData {
    ui_load: [u32; 256],
    ui_tj_max: [u32; 128],
    ui_core_cnt: u32,
    ui_cpu_cnt: u32,
    f_temp: [f32; 256],
}

#[link(name = "kernel32")]
extern "system" {
    fn OpenFileMappingA(dwDesiredAccess: u32, bInheritHandle: i32, lpName: *const u8) -> *mut c_void;
    fn MapViewOfFile(hFile: *mut c_void, dwDesiredAccess: u32, dwFileOffsetHigh: u32, dwFileOffsetLow: u32, dwNumberOfBytesToMap: usize) -> *mut c_void;
    fn UnmapViewOfFile(lpBaseAddress: *mut c_void) -> i32;
    fn CloseHandle(hObject: *mut c_void) -> i32;
}
// ------------------------------------------

pub struct TempStats {
    pub cpu_temp: f32,
}

pub struct TempSensor;

impl TempSensor {
    pub fn new() -> Self {
        Self 
    }

    pub fn read(&self) -> TempStats {
        let mut final_temp;
        unsafe {
            const FILE_MAP_READ: u32 = 4;
            // Il nome in codice dell'area di memoria creata da Core Temp
            let name = b"CoreTempMappingObject\0";
            
            // 1. Chiediamo a Windows di aprire l'area di memoria
            let handle = OpenFileMappingA(FILE_MAP_READ, 0, name.as_ptr());
            if handle.is_null() {
                return TempStats { cpu_temp: 0.0 }; // Core Temp probabilmente è chiuso
            }

            // 2. Mappiamo la memoria nella nostra applicazione
            let map_ptr = MapViewOfFile(handle, FILE_MAP_READ, 0, 0, 0);
            final_temp = 0.0;
            
            if !map_ptr.is_null() {
                // 3. IL VERO HACK: Facciamo finta che quel blocco di RAM grezza sia la nostra struct!
                let data = &*(map_ptr as *const CoreTempSharedData);
                final_temp = data.f_temp[0]; // Prendiamo la temperatura del Core 0 
                // Chiudiamo l'accesso
                UnmapViewOfFile(map_ptr);
            }
            CloseHandle(handle);
            TempStats {
                cpu_temp: final_temp,
            }
        }
    }
}