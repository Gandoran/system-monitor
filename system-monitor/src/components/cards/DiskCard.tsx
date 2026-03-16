import { DiskData } from "../../types";
import { C, Card, Title, HBar, StatRow, Sparkline } from "../ui/SharedUi";

export function DiskCard({ d }:{d:DiskData}) {
  const p = (d.diskUsedMemory / d.diskTotalMemory) * 100;

  return (
    <Card accent={C.disk}>
      <Title icon="💾" label="DISK" color={C.disk} />
      <div style={{ display: "flex", flexDirection: "column", gap: 8, marginBottom: 8 }}>
        <div>
          <div style={{ display: "flex", justifyContent: "space-between", marginBottom: 3 }}>
            <div style={{ display: "flex", gap: 5, alignItems: "center" }}>
              <span style={{ fontSize: 10, color: C.text, fontFamily: "monospace", fontWeight: 700 }}>{d.diskName}</span>
              <span style={{ fontSize: 9, color: C.muted, background: C.border, borderRadius: 4, padding: "1px 5px", fontFamily: "monospace" }}>{d.diskType}</span>
              <span style={{ fontSize: 9, color: C.muted, background: C.border, borderRadius: 4, padding: "1px 5px", fontFamily: "monospace" }}>{d.fileSystem}</span>
            </div>
            <span style={{ fontSize: 10, color: C.disk, fontFamily: "monospace" }}>{d.diskUsedMemory.toFixed(1)} / {d.diskTotalMemory.toFixed(1)} GB</span>
          </div>
          <div style={{ background: C.border, borderRadius: 5, height: 8, overflow: "hidden" }}>
            <div style={{
              width: `${p}%`, height: "100%",
              background: `linear-gradient(90deg,${C.disk},${C.temp})`,
              borderRadius: 5, transition: "width 0.5s", boxShadow: `0 0 8px ${C.disk}44`
            }} />
          </div>
        </div>
      </div>
      {/*TODO HISTORY */}
      {<Sparkline data={[1,2,3,44,5,6,7,8]} color={C.disk} height={36} uid="dskh" />}
      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: "2px 8px", marginTop: 7 }}>
        <HBar pct={(d.diskRead  / 500) * 100} color={C.disk}  label="Read"  value={`${Math.round(d.diskRead)} MB/s`} />
        <HBar pct={(d.diskWrite / 500) * 100} color={C.temp}  label="Write" value={`${Math.round(d.diskWrite)} MB/s`} />
      </div>
      <StatRow items={[
        { label: "ACTIVITY", value: `${Math.round(d.diskUse)}%`, color: C.disk },
        { label: "FILE SYSTEM", value: d.fileSystem, color: C.muted },
      ]} />
    </Card>
  );
}