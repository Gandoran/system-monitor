export interface GpuData {
    vramUsed: number;
    gpuLoad: number; 
    gpuTemp: number;
    gpuMaxTemp:number;
    gpuHistory: number[];
    power: number;
    fanSpeed: number;
    gpuMhz:number;
    needUpdate: boolean;
}