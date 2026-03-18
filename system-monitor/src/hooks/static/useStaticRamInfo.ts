import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface StaticRamInfo {
  ramTotal: number;
}

export function useStaticRamInfo() {
  const [ramInfo, setRamInfo] = useState<StaticRamInfo>({
    ramTotal: 1,
  });
  useEffect(() => {
    invoke<StaticRamInfo>("get_static_ram_info")
      .then((data) => setRamInfo(data))
      .catch((err) => console.error("Errore nel recupero della RAM statica:", err));
  }, []);

  return ramInfo;
}