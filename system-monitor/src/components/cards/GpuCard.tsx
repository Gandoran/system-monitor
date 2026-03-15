import { C,Sparkline, TempBadge, Donut, HBar, Card, Title } from "../ui/SharedUi";
import { GpuData } from "../../types";

export function GpuCard({g}:{g:GpuData}){
  const vramPct = (g.vramUsed / g.vramTotal) * 100;
  const HeaderBadges = (
    <div style={{ display: "flex", gap: 8, alignItems: "center" }}>
       <TempBadge val={g.gpuMaxTemp} baseColor={C.gpu} label="MAX" />
       <TempBadge val={g.gpuTemp} baseColor={C.gpu} />
    </div>
  );
  return (
    <Card accent={C.gpu}>
      <Title icon="🎮" label="GPU" color={C.gpu} right={HeaderBadges}/>
      <div style={{ display: "flex", gap: 16, alignItems: "flex-start" }}>
        <div style={{ display: "flex", flexDirection: "column", gap: 8, alignItems: "center" }}>
          <Donut pct={g.gpuLoad} color={C.gpu} label="Load" sub="render" />
          <Donut pct={vramPct} color={C.net} size={72} label="VRAM"
            sub={`${g.vramUsed.toFixed(1)}G`} />
        </div>
        <div style={{ flex: 1, minWidth: 0 }}>
          <Sparkline data={g.gpuHistory} color={C.gpu} height={64} />
          <div style={{ marginTop: 10 }}>
            <HBar pct={g.gpuLoad} color={C.gpu} label="GPU Load" value={`${Math.round(g.gpuLoad)}%`} />
            <HBar pct={vramPct} color={C.net} label="VRAM" value={`${g.vramUsed.toFixed(2)} / ${g.vramTotal} GB`} />
          </div>
        </div>
      </div>
    </Card>
  );
}