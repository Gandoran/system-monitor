export * from './cpu';
export * from './disk';
export * from './gpu';
export * from './network';
export * from './ram';
export * from './uptime';

export interface RustPayload {
    cpu_stats: { cpu_usage: number, cores_load:number[],frequency:number,physical_cores:number };
    cpu_temp: { cpu_temp: number, max_temp:number };
    ram_stats: { ram_used: number; ram_total: number };
    gpu_stats: { gpu_usage: number; gpu_temp: number; gpu_max_temp:number; vram_used: number; vram_total: number };
    disk_stats: {disk_write:number,disk_read:number,disk_total_memory:number,disk_used_memory:number,disk_use:number};
    net_stats: {net_history_download:number[],net_history_upload:number[],tot_download:number,tot_upload:number,net_ping:number};
    uptime_stats: {uptime:number};
}