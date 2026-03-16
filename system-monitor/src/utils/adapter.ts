import { CpuData, DiskData, GpuData, NetworkData, RamData, RustPayload, uptimeData} from '../types';

export function updateRamData(prev: RamData, rustData: RustPayload):RamData{
    const ramUsedGB = rustData.ram_stats.ram_used / 1073741824;
    const ramTotalGB = rustData.ram_stats.ram_total / 1073741824;
    const swapUsedGB =  rustData.ram_stats.swap_used / 1073741824;
    const swapTotalGB = rustData.ram_stats.swap_total / 1073741824;;
    return {
        ...prev,
        ramUsed: ramUsedGB,
        ramTotal: ramTotalGB,
        ramAvailable: rustData.ram_stats.ram_available / 1073741824,
        swapUsed: swapUsedGB,
        swapTotal: swapTotalGB,
        ramHistory: [...prev.ramHistory.slice(1), (ramUsedGB / ramTotalGB) * 100],
        swapHistory:  [...prev.swapHistory.slice(1), (swapUsedGB / swapTotalGB) * 100],
    };
}

export function updateCpuData(prev: CpuData,rustData:RustPayload):CpuData{
    return {
        ...prev,
        cpuName: rustData.cpu_stats.cpu_name,
        cpuUse: rustData.cpu_stats.cpu_usage,
        cpuTemp: rustData.cpu_temp.cpu_temp,
        cpuMaxTemp : rustData.cpu_temp.max_temp,
        cpuCoresLoad: rustData.cpu_stats.cores_load,
        cpuFrequency: Number(rustData.cpu_stats.frequency.toFixed(2)),
        physical_cores: rustData.cpu_stats.physical_cores,
    };
} 

export function updateGpuData(prev: GpuData,rustData:RustPayload):GpuData{
    return{
        ...prev,
        model: rustData.gpu_stats.gpu_model,
        driver: rustData.gpu_stats.gpu_driver,
        vramUsed: rustData.gpu_stats.vram_used / 1073741824,
        vramTotal: rustData.gpu_stats.vram_total / 1073741824,
        gpuLoad: rustData.gpu_stats.gpu_usage,
        gpuTemp: rustData.gpu_stats.gpu_temp,
        gpuMaxTemp: rustData.gpu_stats.gpu_max_temp,
        gpuHistory: [...prev.gpuHistory.slice(1), rustData.gpu_stats.gpu_usage],
        power: rustData.gpu_stats.power_draw_w,
        fanSpeed: rustData.gpu_stats.fan_speed_pct,
    };
}

export function updateDiskData(prev: DiskData,rustData:RustPayload):DiskData{
    return {
        ...prev,
        diskName: rustData.disk_stats.disk_name,
        fileSystem: rustData.disk_stats.file_system,
        diskType: rustData.disk_stats.disk_type,
        diskRead: Number((rustData.disk_stats.disk_read / 1048576).toFixed(1)),
        diskWrite: Number((rustData.disk_stats.disk_write / 1048576).toFixed(1)),
        diskTotalMemory: Math.round(rustData.disk_stats.disk_total_memory / 1073741824),
        diskUsedMemory: Math.round(rustData.disk_stats.disk_used_memory / 1073741824),
        diskUse: rustData.disk_stats.disk_use,
    }
}

export function updateNetworkData(prev:NetworkData,rustData:RustPayload):NetworkData{
    return {
        ...prev,
        download : rustData.net_stats.net_history_download[rustData.net_stats.net_history_download.length-1] || 0, 
        upload : rustData.net_stats.net_history_upload[rustData.net_stats.net_history_upload.length-1] || 0,
        netHistoryDownload: rustData.net_stats.net_history_download,
        netHistoryUpload : rustData.net_stats.net_history_upload,
        netTotalDown: rustData.net_stats.tot_download,
        netTotalUp: rustData.net_stats.tot_upload,
        netInterface: rustData.net_stats.net_interface || "N/A",
        netIp: rustData.net_stats.net_ip || "127.0.0.1",
        netPing: rustData.net_stats.net_ping,
    }
}

export function updateUptime(prev:uptimeData,rustData:RustPayload):uptimeData{
    return{
        ...prev,
        uptime : rustData.uptime_stats.uptime,
    }
}