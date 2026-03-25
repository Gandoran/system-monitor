import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { SystemSpecsPayload } from "../types/specs";

export function useSpecsMonitor() {
    const [specs, setSpecs] = useState<SystemSpecsPayload>();
    useEffect(() => {
        invoke<SystemSpecsPayload>("get_static_specs_info")
            .then((data) => {
                setSpecs({
                    ...data,
                    disks: data.disks.map(disk => ({
                        ...disk,
                        capacityFormatted: formatBytes(disk.capacityBytes)
                    })),
                    ram: {
                        ...data.ram,
                        totalCapacityBytes: Math.round(data.ram.totalCapacityBytes / 1073741824),
                    }
                });
            })
            .catch((err) => console.log("Errore nel recupero dei dati", err) );
    }, []);
    return specs;
}

function formatBytes(bytes: number) {
  if (!bytes || bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}