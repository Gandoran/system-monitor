import { SessionResults } from "../../types/session";
import { C, Card, ResultPill } from "../ui/SharedUi";

export function SessionCard({ s, i }: { s: SessionResults; i: number }){
return(
    <Card key={i} accent={i === 0 ? C.cpu : C.border}style={{ opacity: i === 0 ? 1 : 0.7, transition: "opacity 0.3s"}}>
    <div style={{ fontSize: 12, color: C.muted, marginBottom: 12, display: "flex", justifyContent: "space-between" }}>
    <span>SESSIONE #{history.length - i} - ${s.processName}</span>
    <span>Durata: <strong>{s.durationSeconds}s</strong></span>
    </div>
    <div style={{ display: "flex", gap: 12, flexWrap: "wrap" }}>
    <ResultPill icon="🧠" label="CPU Load" avg={`${s.cpuAvgLoad.toFixed(1)}%`} color={C.cpu} />
    <ResultPill icon="🌡️" label="CPU Temp" avg={`${s.cpuAvgTemp.toFixed(1)}°C`} peak={`${s.cpuMaxTemp.toFixed(1)}°C`} color={C.cpu} />
    <ResultPill icon="🎮" label="GPU Load" avg={`${s.gpuAvgLoad.toFixed(1)}%`} color={C.gpu} />
    <ResultPill icon="🌡️" label="GPU Temp" avg={`${s.gpuAvgTemp.toFixed(1)}°C`} peak={`${s.gpuMaxTemp.toFixed(1)}°C`} color={C.gpu} />
    <ResultPill icon="⚡" label="RAM Load" avg={`${s.ramAvgLoad.toFixed(1)}%`} color={C.ram} />
    </div>
    </Card>
)
}