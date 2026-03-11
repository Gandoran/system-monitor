export * from './cpu';
export * from './disk';
export * from './gpu';
export * from './network';
export * from './ram';

export interface RustPayload {
    cpu_stats: { cpu_usage: number };
    cpu_temp: { cpu_temp: number };
    ram_stats: { ram_used: number; ram_total: number };
    gpu_stats: { gpu_usage: number; gpu_temp: number; vram_used: number; vram_total: number };
}