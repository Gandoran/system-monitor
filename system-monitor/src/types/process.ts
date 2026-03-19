export interface ProcessData{
    pid: number;
    name: string;
    cpuUsage: number;
    ramUsage: number;
    diskRead: number;
    diskWrite: number;
}