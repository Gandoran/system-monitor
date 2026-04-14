import { SessionResults } from "../../types/session";
import { C, Card, ResultPill } from "../ui/SharedUi";

interface SessionCardProps {
  s: SessionResults;
  i: number;
  absoluteIndex: number;
  onDelete: (index: number) => void;
}

export function SessionCard({ s, i, absoluteIndex, onDelete }: SessionCardProps)    {
return(
    <Card accent={i === 0 ? C.cpu : C.border} style={{ opacity: i === 0 ? 1 : 0.7, transition: "opacity 0.3s" }}>
    <div style={{ fontSize: 12, color: C.muted, marginBottom: 12, display: "flex", justifyContent: "space-between" }}>
    <span>SESSIONE #{absoluteIndex} <span style={{color: C.text}}>• 🎯 {s.processName || "Sistema"}</span></span>
    <span>Durata: <strong>{s.durationSeconds}s</strong></span>
    <button onClick={() => onDelete(i)} onMouseEnter={(e) => e.currentTarget.style.opacity = "1"} onMouseLeave={(e) => e.currentTarget.style.opacity = "0.6"}
            style={{ 
              background: "transparent", border: "none", color: "#ff4444", cursor: "pointer", fontSize: 16, padding: "0 4px", 
              opacity: 0.6, transition: "opacity 0.2s",display: "flex", alignItems: "center", justifyContent: "center"
            }}>✕</button>
    </div>
    <div style={{ display: "flex", gap: 12, flexWrap: "wrap" }}>
    <ResultPill icon="🧠" label="CPU Load" avg={`${s.cpuAvgLoad.toFixed(1)}%`} peak={`${s.cpuMaxLoad.toFixed(1)}%`} color={C.cpu} />
    <ResultPill icon="🌡️" label="CPU Temp" avg={`${s.cpuAvgTemp.toFixed(1)}°C`} peak={`${s.cpuMaxTemp.toFixed(1)}°C`} color={C.cpu} />
    <ResultPill icon="🎮" label="GPU Load" avg={`${s.gpuAvgLoad.toFixed(1)}%`} peak={`${s.gpuMaxLoad.toFixed(1)}%`} color={C.gpu} />
    <ResultPill icon="🌡️" label="GPU Temp" avg={`${s.gpuAvgTemp.toFixed(1)}°C`} peak={`${s.gpuMaxTemp.toFixed(1)}°C`} color={C.gpu} />
    <ResultPill icon="⚡" label="RAM Load" avg={`${s.ramAvgLoad.toFixed(1)}%`} peak={`${s.ramMaxLoad.toFixed(1)}%`} color={C.ram} />
    </div>
    </Card>
)
}