import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface StaticDiskInfo {
    diskName: string,      
    fileSystem: string,    
    diskType: string,      
    diskTotalMemory: number, 
}

export function useStaticDiskInfo() {
  const [diskInfo, setDiskInfo] = useState<StaticDiskInfo>({
    diskName: '..',      
    fileSystem: '..',    
    diskType: '..',      
    diskTotalMemory: 0, 
  });
  useEffect(() => {
    invoke<StaticDiskInfo>("get_static_disk_info")
      .then((data) => setDiskInfo(data))
      .catch((err) => console.error("Errore nel recupero del disco statico:", err));
  }, []);

  return diskInfo;
}