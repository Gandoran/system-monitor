import { C,Sparkline, TempBadge, Donut, HBar, Card, Title, StatRow } from "../ui/SharedUi";
import { GpuData } from "../../types";
import { useStaticGpuInfo } from "../../hooks/static/useStaticGpuInfo";

export function GpuCard({g}:{g:GpuData}){
  const gu = useStaticGpuInfo(g.needUpdate);  
  const vramPct  = (g.vramUsed / (gu.vramTotal || 1)) * 100;
  const powerPct = (g.power / gu.powerMaxW) * 100;
  const mhzPct = (g.gpuMhz/gu.maxMhz)*100;
  const HeaderBadges = (
    <div style={{ display: "flex", gap: 8, alignItems: "center" }}>
       <TempBadge val={g.gpuMaxTemp} baseColor={C.gpu} label="MAX" />
       <TempBadge val={g.gpuTemp} baseColor={C.gpu} />
    </div>
  );
  return (
    <Card accent={C.gpu}>
      <Title icon="🎮" label="GPU" color={C.gpu} right={HeaderBadges}/>
      <div style={{ display: "flex", gap: 12, alignItems: "flex-start" }}>
        <div style={{ display: "flex", flexDirection: "column", gap: 6, alignItems: "center" }}>
          <Donut pct={g.gpuLoad} color={C.gpu} label="Load" sub="render" />
          <Donut pct={vramPct} color={C.net} size={72} label="VRAM" sub={`${g.vramUsed.toFixed(1)}G`} />
        </div>
        <div style={{ flex: 1, minWidth: 0 }}>
          <Sparkline data={g.gpuHistory} color={C.gpu} height={42} uid="gpuh" />
          <div style={{ marginTop: 7 }}>
            <HBar pct={g.gpuLoad} color={C.gpu} label="GPU Load" value={`${Math.round(g.gpuLoad)}%`} />
            <HBar pct={vramPct} color={C.net} label="VRAM" value={`${g.vramUsed.toFixed(2)} / ${gu.vramTotal} GB`} />
            <HBar pct={powerPct} color={C.temp} label="Power" value={`${Math.round(g.power)} / ${gu.powerMaxW} W`} />
            <HBar pct={mhzPct} color={C.disk} label="Mhz" value={`${Math.round(g.gpuMhz)} / ${gu.maxMhz} Mhz`} />
            <HBar pct={g.fanSpeed} color={C.fan} label="Fan" value={`${Math.round(g.fanSpeed)}%`} />
          </div>
        </div>
      </div>
      <div style={{ marginTop: 12 }}>
        <StatRow items={[
          { label: "MODEL",  value: gu.gpuModel || "Unknown GPU",  color: C.text },
          { label: "DRIVER", value: gu.gpuDriver, color: C.muted },
          { label: "POWER",  value: `${Math.round(g.power)}W`, color: C.temp },
        ]} />
      </div>
    </Card>
  );
}