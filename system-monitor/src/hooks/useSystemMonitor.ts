import { useState, useEffect } from 'react';
import { RustPayload } from '../types';
import { listen } from "@tauri-apps/api/event";
import { updateCpuData, updateDiskData, updateGpuData, updateNetworkData, updateRamData, updateUptime } from '../utils/adapter';
import { INITIAL_CPU_STATE, INITIAL_RAM_STATE, INITIAL_GPU_STATE, INITIAL_DISK_STATE, INITIAL_NETWORK_STATE, INITIAL_UPTIME_STATE } from '../constants/defaultStates'
import { useStaticRamInfo } from './static/useStaticRamInfo';

export function useSystemMonitor() {
    const [cpu, setCpu] = useState(INITIAL_CPU_STATE);
    const [ram, setRam] = useState(INITIAL_RAM_STATE);
    const [gpu, setGpu] = useState(INITIAL_GPU_STATE);
    const [disk, setDisk] = useState(INITIAL_DISK_STATE);
    const [network, setNetwork] = useState(INITIAL_NETWORK_STATE);
    const [uptime, setUptime] = useState(INITIAL_UPTIME_STATE);
    //static values
    const staticRam = useStaticRamInfo();
    useEffect(() => {
        const unlistenPromise = listen<RustPayload>("system-stats", (event) => {
            const data = event.payload;
            setCpu(prev => updateCpuData(prev,data));
            setRam(prev => updateRamData(prev,data,staticRam));
            setGpu(prev => updateGpuData(prev,data));
            setDisk(prev => updateDiskData(prev,data));
            setNetwork(prev => updateNetworkData(prev,data));
            setUptime(prev => updateUptime(prev,data));
        });

        return () => {
            unlistenPromise.then((unlisten) => unlisten());
        };
    }, [staticRam]);

    return { cpu, ram, gpu, disk, network, uptime };
}