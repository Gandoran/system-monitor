import { useSystemMonitor } from "../../hooks/useSystemMonitor";
import { CpuCard, DiskCard, GpuCard, NetCard, RamCard, SummaryPills } from "../cards";
import { SysBar } from "../layout/SysBar";

export function OverviewTab() {
  const { cpu, ram, gpu, disk, network } = useSystemMonitor();
  return (
    <div>
      <SysBar cpuTemp={cpu.cpuTemp} gpuTemp={gpu.gpuTemp} />
      
      <div style={{ marginBottom: 20 }}>
        <SummaryPills c={cpu} r={ram} g={gpu} n={network} />
      </div>
      
      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gridTemplateRows: "auto auto auto", gap: 14 }}>
          <CpuCard c={cpu}/>
          <GpuCard g={gpu}/>
          <RamCard ra={ram}/>
          <DiskCard d={disk}/> 
          <NetCard n={network}/>
      </div>
    </div>
  );
}