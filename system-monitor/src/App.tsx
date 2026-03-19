import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { C } from "./components/ui/SharedUi";
import { Header } from "./components/layout/Header";

import { OverviewTab } from "./components/tab/OverviewTab";
import { ProcessesTab } from "./components/tab/processTab";

export default function App() {
  const [activeTab, setActiveTab] = useState("Overview");

  const handleTabChange = async (newTab: string) => {
    setActiveTab(newTab);
    if (newTab === "Processes") {
        await invoke("set_app_mode", { mode: "processes" });
    } else {
        await invoke("set_app_mode", { mode: "hardware" });
    }
  };

  useEffect(() => {
    invoke("set_app_mode", { mode: "hardware" }).catch(console.error);
  }, []);

  return (
    <div style={{minHeight: "100vh",background: C.bg, color: C.text, fontFamily: "monospace", padding: "20px 24px", boxSizing: "border-box"}}>
      <style>{`
        @keyframes pulse { 0%,100%{opacity:1} 50%{opacity:0.3} }
        * { box-sizing: border-box; }
        ::-webkit-scrollbar { width: 6px; }
        ::-webkit-scrollbar-track { background: ${C.bg}; }
        ::-webkit-scrollbar-thumb { background: ${C.border}; border-radius: 3px; }
      `}</style>
      
      <Header activeTab={activeTab} onTabChange={handleTabChange} />
      {activeTab === "Overview" && <OverviewTab />}
      {activeTab === "Processes" && <ProcessesTab />}

    </div>
  );
}