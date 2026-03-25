// File: src/views/SessionTab.tsx
import { SessionResults } from "../../types/session";
import { SessionCard } from "../cards/SessionCard";
import { C, Card } from "../ui/SharedUi";

interface SessionTabProps {
  isRunning: boolean;
  history: SessionResults[];
  startSession: () => void;
  stopSession: () => void;
}

export function SessionTab({ isRunning, history, startSession, stopSession }: SessionTabProps) {
  return (
    <div style={{ padding: "10px", fontFamily: "monospace" }}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: 24 }}>
        <div>
          <h2 style={{ color: C.text, margin: 0, letterSpacing: 1 }}>⚡ BENCHMARK</h2>
          <span style={{ fontSize: 11, color: C.muted }}>Registra le prestazioni di picco in background</span>
        </div>
        {!isRunning ? (
          <button 
            onClick={startSession}
            style={{ 
              padding: "8px 20px", background: `${C.ram}22`, color: C.ram, border: `1px solid ${C.ram}55`, 
              borderRadius: 8, cursor: "pointer", fontWeight: "bold", transition: "all 0.2s" 
            }}>▶ START RECORDING
          </button>
        ) : (
          <button 
            onClick={stopSession}
            style={{ 
              padding: "8px 20px", background: "#ff444422", color: "#ff4444", border: "1px solid #ff444455", 
              borderRadius: 8, cursor: "pointer", fontWeight: "bold", animation: "pulse 2s infinite" 
            }}
          >⏹ STOP & SAVE
          </button>
        )}
      </div>
      {isRunning && (
        <Card accent={C.ram} style={{ padding: 12, marginBottom: 24 }}>
          <span style={{ color: C.text, fontSize: 13 }}>🔴 Registrazione in corso...</span>
        </Card>
      )}
      <div style={{ display: "flex", flexDirection: "column", gap: 16 }}>
        {history.length === 0 && !isRunning && (
          <div style={{ textAlign: "center", color: C.muted, padding: 40, border: `1px dashed ${C.border}`, borderRadius: 12 }}>
            Nessun benchmark registrato. Premi Start per iniziare!
          </div>
        )}
        {history.map((session, i) => (
          <SessionCard key={i} s={session} i={i} />
        ))}
      </div>
    </div>
  );
}