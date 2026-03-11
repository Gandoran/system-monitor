import { C, Donut, HBar, TempBadge, Card, Title } from "../ui/SharedUi";
import { CpuData } from "../../types";

export function CpuCard({ c }: { c: CpuData }) {
  return (
    <Card accent={C.cpu}>
      <Title icon="⚡" label="CPU" color={C.cpu} right={<TempBadge val={c.cpuTemp} />} />
      <div style={{ display: "flex", gap: 16, alignItems: "flex-start" }}>
        <Donut pct={c.cpuUse} color={C.cpu} label="Total" sub={`${c.cpuFrequency} GHz`} />
        
        <div style={{ flex: 1 }}>
          <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: "4px 12px" }}>
            {c.cpuCoresLoad.map((v, i) => (
              <HBar key={i} pct={v} color={C.cpu} label={`Core ${i}`} value={`${Math.round(v)}%`} />
            ))}
            
          </div>
        </div>
      </div>
    </Card>
  );
}