import { C, Donut, HBar, TempBadge, Card, Title, StatRow, Sparkline } from "../ui/SharedUi";
import { CpuData } from "../../types";
import { useStaticCpuInfo } from "../../hooks/static/useStaticCpuInfo";

export function CpuCard({ c }: { c: CpuData }) {
  const cu = useStaticCpuInfo();
  const HeaderBadges = (
    <div style={{ display: "flex", gap: 8, alignItems: "center" }}>
      <span title="Max Temperature">
        <TempBadge val={c.cpuMaxTemp} baseColor={C.cpu} label="MAX" />
      </span>
      <span title="Current Temperature">
        <TempBadge val={c.cpuTemp} baseColor={C.cpu} />
      </span>
    </div>
  );
  return (
    <Card accent={C.cpu}>
      <Title icon="⚡" label="CPU" color={C.cpu} right={HeaderBadges} />
      <div style={{ display: "flex", gap: 16, alignItems: "center" }}>
        <Donut pct={c.cpuUse} color={C.cpu} label="Total" sub={`${c.cpuFrequency} GHz`} />
        <div style={{ flex: 1 }}>
          <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: "4px 12px" }}>
            {c.cpuCoresLoad.map((v, i) => (
              <HBar key={i} pct={v} color={C.cpu} label={`Thread ${i}`} value={`${Math.round(v)}%`} />
            ))}
          </div>
        </div>
      </div>
      <div>
      <Sparkline data={c.cpuHistory} color={C.cpu} height={42} uid="cpuh" />
      </div>
      <StatRow items={[
        { label: "MODEL", value: cu.cpuName, color: C.text },
        { label: "FREQUENCY", value: c.cpuFrequency, color: C.text },
        { label: "CORE", value: cu.physicalCores, color: C.cpu },
        { label: "THREADS", value: c.cpuCoresLoad.length, color: C.cpu },
      ]}/>
    </Card>
  );
}