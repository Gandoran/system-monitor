import { useState } from "react";
import { useSystemMonitor } from "./hooks/useSystemMonitor";
import { C } from "./components/ui/SharedUi";
import { Header } from "./components/layout/Header";
import { CpuCard,DiskCard, GpuCard, NetCard, RamCard } from "./components/cards";
import { SysBar } from "./components/layout/SysBar";
import { SummaryPills } from "./components/cards/SummaryPills";

export default function App() {
  const { cpu, ram, gpu, disk, network } = useSystemMonitor();
  const [activeTab, setActiveTab] = useState("Overview");

  return (
    <div style={{minHeight: "100vh",background: C.bg, color: C.text, fontFamily: "monospace", padding: "20px 24px", boxSizing: "border-box"
    }}>
      <style>{`
        @keyframes pulse { 0%,100%{opacity:1} 50%{opacity:0.3} }
        * { box-sizing: border-box; }
        ::-webkit-scrollbar { width: 6px; }
        ::-webkit-scrollbar-track { background: ${C.bg}; }
        ::-webkit-scrollbar-thumb { background: ${C.border}; border-radius: 3px; }
      `}</style>
      <Header activeTab={activeTab} onTabChange={setActiveTab} />

      <SysBar cpuTemp={cpu.cpuTemp} gpuTemp={gpu.gpuTemp} />

      <div style={{ marginBottom: 20 }}>
        <SummaryPills 
          c={cpu} 
          r={ram} 
          g={gpu} 
          n={network} 
        />
      </div>
      
      {activeTab === "Overview" && (
        <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gridTemplateRows: "auto auto auto", gap: 14 }}>
            <CpuCard c={cpu}/>
            <GpuCard g={gpu}/>
            <RamCard ra={ram}/>
            <DiskCard d={disk}/> 
            <NetCard n={network}/>
        </div>
      )}

      {activeTab === "History" && (
        <div>
            <h2>Storico in costruzione...</h2>
        </div>
      )}

    </div>
  );
}