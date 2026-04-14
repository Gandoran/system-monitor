import { C } from "../../ui/SharedUi"

interface SessionHeaderProps {
  isRunning: boolean;
  onStart: () => void;
  onStop: () => void;
}

export function SessionHeader({ isRunning, onStart, onStop }: SessionHeaderProps) {
  return (
    <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: 24 }}>
      <div>
        <h2 style={{ color: C.text, margin: 0, letterSpacing: 1 }}>⚡ BENCHMARK</h2>
      </div>
      {!isRunning ? (
        <button onClick={onStart}
          style={{ 
            padding: "8px 20px", background: `${C.ram}22`, color: C.ram, border: `1px solid ${C.ram}55`, 
            borderRadius: 8, cursor: "pointer", fontWeight: "bold", transition: "all 0.2s" 
          }}>▶ START RECORDING
        </button>
      ) : (
        <button onClick={onStop}
          style={{ 
            padding: "8px 20px", background: "#ff444422", color: "#ff4444", border: "1px solid #ff444455", 
            borderRadius: 8, cursor: "pointer", fontWeight: "bold", animation: "pulse 2s infinite" 
          }}>⏹ STOP & SAVE
        </button>
      )}
    </div>
  );
}