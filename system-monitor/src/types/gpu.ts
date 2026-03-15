export interface GpuData {
    vramTotal: number,
    vramUsed: number,
    gpuLoad: number,  
    gpuTemp: number,
    gpuMaxTemp:number,
    gpuHistory: number[]
    model: string,
    driver: string,
    power: number,
    fanSpeed: number,
}