export interface CpuData{
    cpuName: string,
    cpuCoresLoad : number[],
    cpuTemp : number,
    cpuMaxTemp : number,
    cpuUse : number,
    cpuFrequency : number,
    physical_cores : number,
}
