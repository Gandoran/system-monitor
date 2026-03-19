export * from './cpu';
export * from './disk';
export * from './gpu';
export * from './network';
export * from './ram';
export * from './uptime';

export interface RustPayload {
    cpu_stats: { cpu_usage: number, cores_load:number[],frequency:number};
    cpu_temp: { cpu_temp: number, max_temp:number };
    ram_stats: { ram_used: number, ram_available: number, swap_used: number, swap_total: number, };
    gpu_stats: { gpu_usage: number, gpu_temp: number, gpu_max_temp:number, vram_used: number, vram_total: number,
        gpu_model: string, gpu_driver:string, fan_speed_pct: number, power_draw_w:number};
    disk_stats: {disk_write:number,disk_read:number,disk_used_memory:number,disk_use:number};
    net_stats: {net_history_download:number[],net_history_upload:number[],tot_download:number,tot_upload:number,
        net_ping:number, net_interface:string, net_ip:string};
    uptime_stats: {uptime:number};
}

export interface RustPayloadProcess{
    proccess_stats: {pid: number, name:string, cpu_usage:number, ram_usage_bytes:number,
        disk_read_bytes:number, disk_write_bytes:number}
}