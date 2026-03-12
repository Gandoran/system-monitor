// FILE: App.tsx
import { useState } from "react";
import { useSystemMonitor } from "./hooks/useSystemMonitor";
import { C } from "./components/ui/SharedUi";
import { Header } from "./components/layout/Header";
import { CpuCard,DiskCard, GpuCard, NetCard, RamCard } from "./components/cards";
import { SysBar } from "./components/layout/SysBar";

export default function App() {
  const { cpu, ram, gpu, disk, network } = useSystemMonitor();
  const [activeTab, setActiveTab] = useState("Overview");

  return (
    <div style={{
      minHeight: "100vh",
      background: C.bg,      
      color: C.text,         
      fontFamily: "monospace", 
      padding: "20px 24px",
      boxSizing: "border-box"
    }}>
      <style>{`
        @keyframes pulse { 0%,100%{opacity:1} 50%{opacity:0.3} }
        * { box-sizing: border-box; }
        ::-webkit-scrollbar { width: 6px; }
        ::-webkit-scrollbar-track { background: ${C.bg}; }
        ::-webkit-scrollbar-thumb { background: ${C.border}; border-radius: 3px; }
      `}</style>

      {/* 2. INSERIAMO L'HEADER IN ALTO */}
      <Header activeTab={activeTab} onTabChange={setActiveTab} />

      <SysBar 
         hostname="Rust-PC" 
         os="Windows 11" 
         uptime="2h 14m" 
         cpuTemp={cpu.cpuTemp} 
         gpuTemp={gpu.gpuTemp} 
      />

      {/* 3. SOTTO L'HEADER, MOSTRA LE CARD SOLO SE SIAMO SU OVERVIEW */}
      {activeTab === "Overview" && (
        <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gridTemplateRows: "auto auto auto", gap: 14 }}>
            <CpuCard c={cpu}/>
            <RamCard ra={ram}/>
            <GpuCard g={gpu}/>
            <DiskCard d={disk}/> 
            <NetCard n={network}/>
        </div>
      )}

      {/* Se clicchi su History, la griglia scompare e appare questo! */}
      {activeTab === "History" && (
        <div>
            <h2>Storico in costruzione...</h2>
        </div>
      )}

    </div>
  );
}