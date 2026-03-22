import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { C, Card, Title } from "../ui/SharedUi";
import { ProcessData } from "../../types/process";

export function ProcessesCard({ processes }: { processes: ProcessData[] }) {
  const MAX_DISK_MB = 100;
  const logicalCores = navigator.hardwareConcurrency || 1;
  const [totalRam,  setTotalRam] = useState(16 * 1024 * 1024 * 1024);
  useEffect(() => {
    invoke("get_static_ram_info")
      .then((res: any) => {
        if (res && res.total) {
          setTotalRam(res.total);
        }
      })
      .catch(console.error);
  }, []);
  return (
    <Card accent={C.proc} style={{ gridColumn: "1 / -1" }}>
      <Title icon="⚙️" label="TOP PROCESSES" color={C.proc} />
      <div style={{ display: "grid", gridTemplateColumns: "40px 140px 1fr 1fr 1fr", gap: 16, padding: "3px 0", borderBottom: `1px solid ${C.border}`, marginBottom: 4 }}>
        {["PID", "NAME", "CPU", "RAM", "DISK I/O"].map(h => (
          <span key={h} style={{ fontSize: 12, color: C.muted, fontFamily: "monospace", letterSpacing: 0.8 }}>{h}</span>
        ))}
      </div>
      {processes.map(p => {
        const normalizedCpu = p.cpuUsage / logicalCores;
        const ramMB = p.ramUsage / 1048576; 
        const ramPercentage = (p.ramUsage / totalRam) * 100;
        const diskTotalMB = ((p.diskRead + p.diskWrite) / 1048576);
        const diskPercentage = Math.min((diskTotalMB / MAX_DISK_MB) * 100, 100);
        return (
          <div key={p.pid} style={{
            display: "grid", gridTemplateColumns: "40px 140px 1fr 1fr 1fr",
            gap: 16, padding: "5px 0", borderBottom: `1px solid ${C.border}22`, alignItems: "center"
          }}>
            <span style={{ fontSize: 12, color: C.muted, fontFamily: "monospace" }}>{p.pid}</span>
            <span style={{ fontSize: 12, color: C.text, fontFamily: "monospace", overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }} title={p.name}>
              {p.name}
            </span>
            <div>
              <div style={{ fontSize: 12, color: C.proc, fontFamily: "monospace", fontWeight: 700, marginBottom: 2 }}>
                {normalizedCpu.toFixed(1)}%
              </div>
              <div style={{ background: C.border, borderRadius: 3, height: 4 }}>
                <div style={{ width: `${Math.min(normalizedCpu, 100)}%`, height: "100%", background: C.proc, borderRadius: 3, transition: "width 0.5s" }} />
              </div>
            </div>
            <div>
              <div style={{ fontSize: 12, color: C.ram, fontFamily: "monospace", fontWeight: 700, marginBottom: 2 }}>
                {(ramMB).toFixed(2)} MB
              </div>
              <div style={{ background: C.border, borderRadius: 3, height: 4 }}>
                <div style={{ width: `${ramPercentage}%`, height: "100%", background: C.ram, borderRadius: 3, transition: "width 0.5s" }} />
              </div>
            </div>
            <div>
              <div style={{ fontSize: 12, color: C.disk || C.text, fontFamily: "monospace", fontWeight: 700, marginBottom: 2 }}>
                {diskTotalMB.toFixed(2)} MB/s
              </div>
              <div style={{ background: C.border, borderRadius: 3, height: 4 }}>
                <div style={{ width: `${diskPercentage}%`, height: "100%", background: C.disk || C.proc, borderRadius: 3, transition: "width 0.5s" }} />
              </div>
            </div>

          </div>
        );
      })}
    </Card>
  );
}