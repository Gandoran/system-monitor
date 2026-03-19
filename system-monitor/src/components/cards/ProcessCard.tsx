import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { C, Card, Title } from "../ui/SharedUi";
import { ProcessData } from "../../types/process";

export function ProcessesCard({ processes }: { processes: ProcessData[] }) {
  //TODO SMALL FIX
  const [totalRam, setTotalRam] = useState(16 * 1024 * 1024 * 1024);
  useEffect(() => {
    invoke("get_static_ram_info")
      .then((res: any) => {
        if (res && res.total) {
          setTotalRam(res.total);
        }
      })
      .catch(console.error);
  }, []);
  const maxDisk = Math.max(...processes.map(p => p.diskRead + p.diskWrite), 1);

  return (
    <Card accent={C.proc} style={{ gridColumn: "1 / -1" }}>
      <Title icon="⚙️" label="TOP PROCESSES" color={C.proc} />
      <div style={{ display: "grid", gridTemplateColumns: "40px 140px 1fr 1fr 1fr", gap: 16, padding: "3px 0", borderBottom: `1px solid ${C.border}`, marginBottom: 4 }}>
        {["PID", "NAME", "CPU", "RAM", "DISK I/O"].map(h => (
          <span key={h} style={{ fontSize: 9, color: C.muted, fontFamily: "monospace", letterSpacing: 0.8 }}>{h}</span>
        ))}
      </div>
      {processes.map(p => {
        const diskTotal = p.diskRead + p.diskWrite;

        return (
          <div key={p.pid} style={{
            display: "grid", gridTemplateColumns: "40px 140px 1fr 1fr 1fr",
            gap: 16, padding: "5px 0", borderBottom: `1px solid ${C.border}22`, alignItems: "center"
          }}>
            <span style={{ fontSize: 9, color: C.muted, fontFamily: "monospace" }}>{p.pid}</span>
            <span style={{ fontSize: 11, color: C.text, fontFamily: "monospace", overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }} title={p.name}>
              {p.name}
            </span>

            {/* BLOCCO CPU */}
            <div>
              <div style={{ fontSize: 9, color: C.proc, fontFamily: "monospace", fontWeight: 700, marginBottom: 2 }}>
                {p.cpuUsage.toFixed(1)}%
              </div>
              <div style={{ background: C.border, borderRadius: 3, height: 4 }}>
                <div style={{ width: `${Math.min(p.cpuUsage, 100)}%`, height: "100%", background: C.proc, borderRadius: 3, transition: "width 0.5s" }} />
              </div>
            </div>

            {/* BLOCCO RAM */}
            <div>
              <div style={{ fontSize: 9, color: C.ram, fontFamily: "monospace", fontWeight: 700, marginBottom: 2 }}>
                {(p.ramUsage / 1073741824).toFixed(1)} MB
              </div>
              <div style={{ background: C.border, borderRadius: 3, height: 4 }}>
                <div style={{ width: `${(p.ramUsage / totalRam) * 100}%`, height: "100%", background: C.ram, borderRadius: 3, transition: "width 0.5s" }} />
              </div>
            </div>

            {/* BLOCCO DISCO */}
            <div>
              <div style={{ fontSize: 9, color: C.disk || C.text, fontFamily: "monospace", fontWeight: 700, marginBottom: 2 }}>
                {(diskTotal / 1073741824).toFixed(2)} MB/s
              </div>
              <div style={{ background: C.border, borderRadius: 3, height: 4 }}>
                <div style={{ width: `${(diskTotal / maxDisk) * 100}%`, height: "100%", background: C.disk || C.proc, borderRadius: 3, transition: "width 0.5s" }} />
              </div>
            </div>

          </div>
        );
      })}
    </Card>
  );
}