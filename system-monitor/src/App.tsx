import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";

// 1. Creiamo l'Interfaccia TypeScript (la "copia" della tua SystemStats di Rust)
interface SystemStats {
  cpu_stats: { cpu_usage: number };
  cpu_temp: { cpu_temp: number };
  ram_stats: { ram_used: number; ram_total: number };
  gpu_stats: { 
    gpu_usage: number; 
    gpu_temp: number; 
    vram_used: number; 
    vram_total: number;
  };
}

function App() {
  // 2. Diciamo a React: "Questa variabile inizierà come 'null', 
  // ma poi diventerà un oggetto di tipo 'SystemStats'!"
  const [stats, setStats] = useState<SystemStats | null>(null);

  useEffect(() => {
    const unlistenPromise = listen("system-stats", (event) => {
      // 3. Forziamo il tipo dicendo a TypeScript: 
      // "Tranquillo, so per certo che questo payload sconosciuto è un SystemStats"
      setStats(event.payload as SystemStats); 
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  return (
    <div style={{ padding: "20px", fontFamily: "monospace" }}>
      <h2>Monitoraggio Sistema in Tempo Reale 🚀</h2>
      
      <div style={{ background: "#1e1e1e", color: "#00ff00", padding: "15px", borderRadius: "8px" }}>
        {stats ? (
          <pre>{JSON.stringify(stats, null, 2)}</pre>
        ) : (
          <p>In attesa dei dati da Rust...</p>
        )}
      </div>
    </div>
  );
}

export default App;