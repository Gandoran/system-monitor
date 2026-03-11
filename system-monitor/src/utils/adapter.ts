import { CpuData, GpuData, RamData, RustPayload} from '../types';

export function updateRamData(prev: RamData, rustData: RustPayload):RamData{
    const ramUsedGB = rustData.ram_stats.ram_used / 1073741824;
    const ramTotalGB = rustData.ram_stats.ram_total / 1073741824;

    return {
        ...prev,
        ramUsed: ramUsedGB,
        ramTotal: ramTotalGB,
        ramHistory: [...prev.ramHistory.slice(1), (ramUsedGB / ramTotalGB) * 100]
    };
}

export function updateCpuData(prev: CpuData,rustData:RustPayload):CpuData{
    return {
        ...prev,
        cpuUse: rustData.cpu_stats.cpu_usage,
        cpuTemp: rustData.cpu_temp.cpu_temp,
        cpuCoresLoad: prev.cpuCoresLoad.map(() => rustData.cpu_stats.cpu_usage * (0.8 + Math.random() * 0.4)),
        cpuFrequency: 3.6
    };
} 

export function updateGpuData(prev: GpuData,rustData:RustPayload):GpuData{
    return{
        ...prev,
        vramUsed: rustData.gpu_stats.vram_used / 1073741824,
        vramTotal: rustData.gpu_stats.vram_total / 1073741824,
        gpuLoad: rustData.gpu_stats.gpu_usage,
        gpuTemp: rustData.gpu_stats.gpu_temp,
        gpuHistory: [...prev.gpuHistory.slice(1), rustData.gpu_stats.gpu_usage]
    };
}