import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { SessionResults } from "../../types/session";

export function SessionTab() {
  const [isRunning, setIsRunning] = useState(false);
  const [results, setResults] = useState<SessionResults | null>(null);

  // Azione del bottone START
  const handleStart = async () => {
    try {
      await invoke("start_session");
      setIsRunning(true);
      setResults(null); // Puliamo i risultati della sessione precedente
    } catch (e) {
      console.error("Errore avvio sessione:", e);
    }
  };

  // Azione del bottone STOP
  const handleStop = async () => {
    try {
      const data = await invoke<SessionResults>("stop_session");
      setIsRunning(false);
      setResults(data); // Salviamo i calcoli di Rust nello stato
    } catch (e) {
      console.error("Errore stop sessione:", e);
    }
  };

  return (
    <div style={{ padding: "20px", color: "white", fontFamily: "monospace" }}>
      <h2 style={{ color: "#00ffcc" }}>⚡ Benchmark Session</h2>
      
      {/* BOTTONE START / STOP */}
      <div style={{ marginBottom: "20px" }}>
        {!isRunning ? (
          <button 
            onClick={handleStart}
            style={{ padding: "10px 20px", background: "green", color: "white", border: "none", cursor: "pointer", fontWeight: "bold" }}
          >
            ▶ START SESSION
          </button>
        ) : (
          <button 
            onClick={handleStop}
            style={{ padding: "10px 20px", background: "red", color: "white", border: "none", cursor: "pointer", fontWeight: "bold", animation: "pulse 1s infinite" }}
          >
            ⏹ STOP RECORDING
          </button>
        )}
      </div>

      {/* STATO IN CORSO */}
      {isRunning && (
        <div style={{ color: "orange" }}>
          <p>Registrazione in background attiva...</p>
          <p style={{ fontSize: "12px", color: "gray" }}>Prova a cambiare Tab e poi torna qui per fermarla!</p>
        </div>
      )}

      {/* VISUALIZZAZIONE RISULTATI GREZZI */}
      {results && !isRunning && (
        <div style={{ background: "#1e1e1e", padding: "15px", borderRadius: "8px", border: "1px solid #333" }}>
          <h3 style={{ color: "violet", marginTop: 0 }}>📊 Risultati Finali</h3>
          
          <p><strong>Durata Sessione:</strong> {results.durationSeconds} secondi</p>
          
          <hr style={{ borderColor: "#333" }} />
          
          <p><strong>CPU Load Medio:</strong> {results.cpuAvgLoad.toFixed(2)} %</p>
          <p><strong>CPU Temp Media:</strong> {results.cpuAvgTemp.toFixed(1)} °C</p>
          <p><strong>CPU Picco Temp:</strong> <span style={{ color: "red" }}>{results.cpuMaxTemp.toFixed(1)} °C</span></p>
          
          <hr style={{ borderColor: "#333" }} />
          
          <p><strong>GPU Load Medio:</strong> {results.gpuAvgLoad.toFixed(2)} %</p>
          <p><strong>GPU Temp Media:</strong> {results.gpuAvgTemp.toFixed(1)} °C</p>
          <p><strong>GPU Picco Temp:</strong> <span style={{ color: "red" }}>{results.gpuMaxTemp.toFixed(1)} °C</span></p>
          
          <hr style={{ borderColor: "#333" }} />
          
          <p><strong>RAM Load Medio:</strong> {results.ramAvgLoad.toFixed(2)} %</p>
        </div>
      )}
    </div>
  );
}