import { CpuData, RamData, GpuData, DiskData, NetworkData, uptimeData } from '../types';

export const INITIAL_CPU_STATE: CpuData = {
    cpuCoresLoad: Array(8).fill(0),
    cpuTemp: 0,
    cpuMaxTemp: 0,
    cpuUse: 0,
    cpuFrequency: 0,
    cpuHistory: Array(40).fill(0),
};

export const INITIAL_RAM_STATE: RamData = {
    ramTotal: 16,
    ramUsed: 0,
    ramAvailable: 0,
    swapUsed: 0,
    swapTotal: 0,
    ramHistory: Array(40).fill(0),
    swapHistory: Array(40).fill(0),
};

export const INITIAL_GPU_STATE: GpuData = {
    vramTotal: 8,
    vramUsed: 0,
    gpuLoad: 0,
    gpuTemp: 0,
    gpuMaxTemp: 0,
    gpuHistory: Array(40).fill(0),
    model: '',
    power: 0,
    fanSpeed: 0,
    driver: '',
    powerMax: 0,
    gpuMhz: 0,
    gpuMaxMhz: 0
};

export const INITIAL_DISK_STATE: DiskData = {
    diskRead: 0,
    diskWrite: 0,
    diskUsedMemory: 0,
    diskUse: 0,
    diskUseHistory: Array(40).fill(0),
};

export const INITIAL_NETWORK_STATE: NetworkData = {
    download: 0,
    upload: 0,
    netHistoryDownload: Array(60).fill(0),
    netHistoryUpload: Array(60).fill(0),
    netTotalDown: 0, 
    netTotalUp: 0,
    netInterface: "NaN",
    netIp: "0.0.0.0",
    netPing: 0,
};

export const INITIAL_UPTIME_STATE:  uptimeData = {
    uptime: 0,
}