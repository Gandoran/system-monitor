use std::fs;

#[derive(serde::Serialize, Clone)]
pub struct TempStats {
    pub cpu_temp: f32,
    pub max_temp: f32,
}

pub struct TempSensor {
    max_temp: f32,
}

impl TempSensor {
    pub fn new() -> Self {
        Self { max_temp: 0.0 }
    }

    // --- IMPLEMENTAZIONE PER LINUX ---
    #[cfg(target_os = "linux")]
    pub fn read(&mut self) -> TempStats {
        let mut final_temp = 0.0;
        //First Sensor 
        if let Ok(temp_str) = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
            if let Ok(milli_celsius) = temp_str.trim().parse::<f32>() {
                final_temp = milli_celsius / 1000.0;
            }
        } 
        //Fallback with second sensor
        else if let Ok(temp_str) = fs::read_to_string("/sys/class/hwmon/hwmon0/temp1_input") {
            if let Ok(milli_celsius) = temp_str.trim().parse::<f32>() {
                final_temp = milli_celsius / 1000.0;
            }
        }
        self.temp_calc(final_temp)
    }

    // --- IMPLEMENTAZIONE PER WINDOWS ---
    #[cfg(target_os = "windows")]
    pub fn read(&mut self) -> TempStats {
        use std::ffi::c_void;  
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
        let mut final_temp = 0.0;
        unsafe {
            const FILE_MAP_READ: u32 = 4;
            let name = b"CoreTempMappingObject\0";  
            let handle = OpenFileMappingA(FILE_MAP_READ, 0, name.as_ptr());    
            if !handle.is_null() {
                let map_ptr = MapViewOfFile(handle, FILE_MAP_READ, 0, 0, 0);
                if !map_ptr.is_null() {
                    let data = &*(map_ptr as *const CoreTempSharedData);
                    final_temp = data.f_temp[0]; 
                    UnmapViewOfFile(map_ptr);
                }
                CloseHandle(handle);
            }
        }
        self.temp_calc(final_temp)
    }

    fn temp_calc(&mut self, final_temp:f32)->TempStats{
        if final_temp > 0.0 {
            self.max_temp = self.max_temp.max(final_temp);
        }
        TempStats {
            cpu_temp: final_temp,
            max_temp: self.max_temp,
        }
    }
}

