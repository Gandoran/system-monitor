import { C, Donut, HBar, Card, Title } from "../ui/SharedUi";
import { DiskData } from "../../types";

export function DiskCard({d}:{d:DiskData}){
  const usedPct = (d.diskUsedMemory / d.diskTotalMemory) * 100;
  const maxIO = 500;
  return (
    <Card accent={C.disk}>
      <Title icon="💾" label="DISK" color={C.disk} right={
        <span style={{ fontSize: 11, color: C.muted, fontFamily: "monospace" }}>
          {d.diskUsedMemory} / {d.diskTotalMemory} GB
        </span>
      } />
      <div style={{ display: "flex", gap: 16, alignItems: "center" }}>
        <Donut pct={usedPct} color={C.disk} label="Used" sub={`${d.diskTotalMemory}GB`} />
        <div style={{ flex: 1 }}>
          <HBar pct={(d.diskRead / maxIO) * 100} color={C.disk}
            label="Read" value={`${Math.round(d.diskRead)} MB/s`} />
          <HBar pct={(d.diskWrite / maxIO) * 100} color={C.temp}
            label="Write" value={`${Math.round(d.diskWrite)} MB/s`} />
          <div style={{ marginTop: 12 }}>
            {/* storage bar */}
            <div style={{ background: C.border, borderRadius: 6, height: 10, overflow: "hidden" }}>
              <div style={{
                width: `${usedPct}%`, height: "100%",
                background: `linear-gradient(90deg, ${C.disk}, ${C.temp})`,
                borderRadius: 6,
                transition: "width 0.5s",
                boxShadow: `0 0 10px ${C.disk}55`
              }} />
            </div>
            <div style={{ display: "flex", justifyContent: "space-between", marginTop: 4 }}>
              <span style={{ fontSize: 9, color: C.muted, fontFamily: "monospace" }}>0</span>
              <span style={{ fontSize: 9, color: C.muted, fontFamily: "monospace" }}>{d.diskTotalMemory} GB</span>
            </div>
          </div>
        </div>
      </div>
    </Card>
  );
}