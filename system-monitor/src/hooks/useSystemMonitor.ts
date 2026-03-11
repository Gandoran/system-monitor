import { useState, useEffect } from 'react';
import { RustPayload } from '../types';
import { listen } from "@tauri-apps/api/event";
import { updateCpuData, updateGpuData, updateRamData } from '../utils/adapter';
import { INITIAL_CPU_STATE, INITIAL_RAM_STATE, INITIAL_GPU_STATE, INITIAL_DISK_STATE, INITIAL_NETWORK_STATE } from '../constants/defaultStates'

export function useSystemMonitor() {
    const [cpu, setCpu] = useState(INITIAL_CPU_STATE);
    const [ram, setRam] = useState(INITIAL_RAM_STATE);
    const [gpu, setGpu] = useState(INITIAL_GPU_STATE);
    const [disk, setDisk] = useState(INITIAL_DISK_STATE);
    const [network, setNetwork] = useState(INITIAL_NETWORK_STATE);
    useEffect(() => {
        let unlisten: () => void;

        async function setupListener() {
            unlisten = await listen<RustPayload>("system-stats", (event) => {
                const data = event.payload;
                setCpu(prev => updateCpuData(prev, data));
                setRam(prev => updateRamData(prev, data));
                setGpu(prev => updateGpuData(prev, data));
            });
        }

        setupListener();

        return () => {
            if (unlisten) unlisten();
        };
    }, []);

    return { cpu, ram, gpu, disk, network };
}