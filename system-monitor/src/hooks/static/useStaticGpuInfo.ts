import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface StaticGpuInfo {
  gpuModel: string,
  gpuDriver: string,
  vramTotal: number,
  powerMaxW: number,
  maxMhz: number,
}

export function useStaticGpuInfo(needsUpdate: boolean = false) {
  const [gpuInfo, setGpuInfo] = useState<StaticGpuInfo>({
    gpuModel: '..',      
    gpuDriver: '..',         
    vramTotal: 0, 
    powerMaxW: 0,
    maxMhz: 0,
  });
  const fetchGpuInfo = () => {
    invoke<StaticGpuInfo>("get_static_gpu_info")
      .then((data) => {
        setGpuInfo({
          ...data,
          vramTotal: Math.round(data.vramTotal / 1073741824),
          powerMaxW: Math.round(data.powerMaxW / 1000)
        });
      })
      .catch((err) => console.error("Errore nel recupero della gpu", err));
  };
  useEffect(() => {
    fetchGpuInfo();
  }, []);
  useEffect(() => {
    if (needsUpdate) {
      fetchGpuInfo();
    }
  }, [needsUpdate]);
  return gpuInfo;
}