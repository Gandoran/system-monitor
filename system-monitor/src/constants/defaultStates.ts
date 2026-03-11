import { CpuData, RamData, GpuData, DiskData, NetworkData } from '../types';

export const INITIAL_CPU_STATE: CpuData = {
    cpuCoresLoad: Array(8).fill(0),
    cpuTemp: 0,
    cpuUse: 0,
    cpuFrequency: 0
};

export const INITIAL_RAM_STATE: RamData = {
    ramTotal: 16, 
    ramUsed: 0,
    ramHistory: Array(40).fill(0)
};

export const INITIAL_GPU_STATE: GpuData = {
    vramTotal: 8,
    vramUsed: 0,
    gpuLoad: 0,
    gpuTemp: 0,
    gpuHistory: Array(40).fill(0)
};

export const INITIAL_DISK_STATE: DiskData = {
    diskRead: 0,
    diskWrite: 0,
    diskTotalMemory: 512,
    diskUsedMemory: 0,
    diskUse: 0
};

export const INITIAL_NETWORK_STATE: NetworkData = {
    download: 0,
    upload: 0,
    netHistory: Array(40).fill(0)
};