import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface StaticSysInfo {
  hostname: string;
  os_name: string;
}

export function useStaticSystemInfo() {
  const [sysInfo, setSysInfo] = useState<StaticSysInfo>({
    hostname: "...",
    os_name: "...",
  });

  useEffect(() => {
    invoke<StaticSysInfo>("get_static_sys_info")
      .then((data) => setSysInfo(data))
      .catch((err) => console.error("Errore nel recupero dei dati di sistema:", err));
  }, []);

  return sysInfo;
}