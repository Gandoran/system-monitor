import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface StaticCpuInfo {
    cpuName: string,
    physicalCores: number,
}

export function useStaticCpuInfo() {
  const [cpuInfo, setCpuInfo] = useState<StaticCpuInfo>({
    cpuName: '..',
    physicalCores: 0,
  });
  useEffect(() => {
    invoke<StaticCpuInfo>("get_static_cpu_info")
      .then((data) => setCpuInfo(data))
      .catch((err) => console.error("Errore nel recupero della CPU statica:", err));
  }, []);

  return cpuInfo;
}