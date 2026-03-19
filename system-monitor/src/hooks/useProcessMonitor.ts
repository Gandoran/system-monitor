import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { ProcessData } from "../types/process";


export function useProcessMonitor() {
    const [processes, setProcesses] = useState<ProcessData[]>([]);
    useEffect(() => {
        const unlistenPromise = listen<ProcessData[]>("process-stats", (event) => {
            setProcesses(event.payload);
        });
        return () => {
            unlistenPromise.then((unlisten) => unlisten());
        };
    }, []);

    return processes;
}