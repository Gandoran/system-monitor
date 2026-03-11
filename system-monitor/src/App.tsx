import { useSystemMonitor } from "./hooks/useSystemMonitor";
import { CpuCard, DiskCard, GpuCard, NetCard, RamCard } from "./components/cards";

// --- IL COMPONENTE GENITORE ---
export default function App() {
  // 1. Chiamiamo il nostro hook! Ci restituisce tutto già diviso e pronto.
  const { cpu, ram, gpu, disk, network } = useSystemMonitor();

  return (
    <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gridTemplateRows: "auto auto auto", gap: 14 }}>
        <CpuCard c={cpu}/>
        <RamCard ra={ram}/>
        <GpuCard g={gpu}/>
        <DiskCard d={disk}/> 
        <NetCard n={network}/>
      </div>
  );
}