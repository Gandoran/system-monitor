import { C,DoubleSparkline, Card, Title, StatRow } from "../ui/SharedUi";
import { NetworkData } from "../../types";


export function NetCard({n}: {n: NetworkData}) {
  const maxNet = 100;
  const pingCol = n.netPing < 10 ? C.cpu : n.netPing < 40 ? C.temp : "#ef4444";
  return (
    <Card accent={C.net}>
      <Title icon="🌐" label="NETWORK" color={C.net}
      right={
          <div style={{ display: "flex", gap: 7, alignItems: "center" }}>
            <span style={{ fontSize: 10, color: C.muted, fontFamily: "monospace" }}>
              {n.netInterface} · {n.netIp}
            </span>
            <span style={{ background: pingCol + "22", color: pingCol, border: `1px solid ${pingCol}44`, borderRadius: 6, padding: "2px 7px", fontSize: 10, fontFamily: "monospace", fontWeight: 700 
            }}>
              {n.netPing} ms
            </span>
          </div>} />
      <DoubleSparkline data1={n.netHistoryDownload} data2={n.netHistoryUpload} color1={C.net} color2={C.gpu} height={52} />
      <div style={{ display: "flex", gap: 12, marginTop: 12 }}>
        {[
          { l: "↓ Download", v: `${n.download.toFixed(1)} MB/s`, pct: (n.download / maxNet) * 100, c: C.net },
          { l: "↑ Upload",   v: `${n.upload.toFixed(1)} MB/s`,   pct: (n.upload   / maxNet) * 100, c: C.gpu },
        ].map(x => (
          <div key={x.l} style={{ flex: 1 }}>
            <div style={{ display: "flex", justifyContent: "space-between", marginBottom: 4 }}>
              <span style={{ fontSize: 10, color: C.muted, fontFamily: "monospace" }}>{x.l}</span>
              <span style={{ fontSize: 10, color: x.c, fontFamily: "monospace", fontWeight: 700 }}>{x.v}</span>
            </div>
            <div style={{ background: C.border, borderRadius: 4, height: 5, overflow: "hidden" }}>
              <div style={{
                width: `${x.pct}%`, height: "100%", background: x.c,
                borderRadius: 4, transition: "width 0.5s",
                boxShadow: `0 0 6px ${x.c}66`
              }} />
            </div>
          </div>
        ))}
      </div>
      <div style={{ marginTop: 12 }}>
        <StatRow items={[
          //TODO SISTEMARE IL TOFIXED (Primo avvio crasha..)
          { label: "TOTAL ↓", value: `${(n.netTotalDown || 0).toFixed(2)} GB`, color: C.net },
          { label: "TOTAL ↑", value: `${(n.netTotalUp || 0).toFixed(2)} GB`,   color: C.gpu },
          { label: "PING",    value: `${n.netPing} ms`, color: pingCol },
          { label: "IFACE",   value: n.netInterface, color: C.muted },
        ]} />
      </div>
    </Card>
  );
}