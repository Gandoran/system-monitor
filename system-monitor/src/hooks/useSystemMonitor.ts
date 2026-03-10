import { useState, useEffect } from 'react';
import { CpuData, RamData, GpuData, DiskData, NetworkData } from '../types';
import { listen } from "@tauri-apps/api/event";

interface RustPayload {
    cpu_stats: { cpu_usage: number };
    cpu_temp: { cpu_temp: number };
    ram_stats: { ram_used: number; ram_total: number };
    gpu_stats: { gpu_usage: number; gpu_temp: number; vram_used: number; vram_total: number };
}

export function useSystemMonitor() {
    const [cpu, setCpu] = useState<CpuData>({
        cpuCoresLoad: Array(8).fill(0),
        cpuTemp: 0,
        cpuUse: 0,
        cpuFrequency: 0
    });

    const [ram, setRam] = useState<RamData>({
        ramTotal: 16, 
        ramUsed: 0,
        ramHistory: Array(40).fill(0)
    });

    const [gpu, setGpu] = useState<GpuData>({
        vramTotal: 8,
        vramUsed: 0,
        gpuLoad: 0,
        gpuTemp: 0,
        gpuHistory: Array(40).fill(0)
    });

    const [disk, setDisk] = useState<DiskData>({
        diskRead: 0,
        diskWrite: 0,
        diskTotalMemory: 512,
        diskUsedMemory: 0,
        diskUse: 0
    });

    const [network, setNetwork] = useState<NetworkData>({
        download: 0,
        upload: 0,
        netHistory: Array(40).fill(0)
    });

useEffect(() => {
        let unlisten: () => void;
        async function setupListener() {
            unlisten = await listen<RustPayload>("system-stats", (event) => {
                const data = event.payload;
                setCpu(prev => ({
                    ...prev, // Ricopia tutti i vecchi dati che non stiamo toccando
                    cpuUse: data.cpu_stats.cpu_usage,
                    cpuTemp: data.cpu_temp.cpu_temp,
                    // Placeholder finto: muoviamo un po' i core basandoci sull'uso totale per estetica
                    cpuCores: prev.cpuCoresLoad.map(() => data.cpu_stats.cpu_usage * (0.8 + Math.random() * 0.4)),
                    cpuFrequency: 3.6
                }));
                const ramUsedGB = data.ram_stats.ram_used / 1073741824;
                const ramTotalGB = data.ram_stats.ram_total / 1073741824;
                const ramPct = (ramUsedGB / ramTotalGB) * 100;

                setRam(prev => ({
                    ...prev,
                    ramUsed: ramUsedGB,
                    ramTotal: ramTotalGB,
                    ramHistory: [...prev.ramHistory.slice(1), ramPct]
                }));-
                setGpu(prev => ({
                    ...prev,
                    vramUsed: data.gpu_stats.vram_used / 1073741824,
                    vramTotal: data.gpu_stats.vram_total / 1073741824,
                    gpuLoad: data.gpu_stats.gpu_usage,
                    gpuTemp: data.gpu_stats.gpu_temp,
                    gpuHistory: [...prev.gpuHistory.slice(1), data.gpu_stats.gpu_usage]
                }));

                // (Rete e Disco li lasciamo fermi a zero finché non li implementiamo nel backend)
            });
        }
        setupListener();
        // Funzione di "Clean-up": quando l'utente chiude l'app o cambia pagina, 
        // smettiamo di ascoltare per non intasare la memoria.
        return () => {
            if (unlisten) unlisten();
        };
    }, []);
    return {
        cpu,
        ram,
        gpu,
        disk,
        network
    };
}