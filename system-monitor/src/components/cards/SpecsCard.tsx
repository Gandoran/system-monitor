import { C, Card, Title } from "../ui/SharedUi";
import {SystemSpecsPayload} from "../../types/specs";

export function SpecsCard({ specs }: { specs: SystemSpecsPayload }) {
  const rows = [
    { l: "OS",            v: specs.os.name },
    { l: "Kernel",        v: specs.os.kernelVersion },
    { l: "Motherboard",   v: `${specs.mobo.vendor} ${specs.mobo.model}`.trim() },
    { l: "BIOS Version",  v: specs.mobo.biosVersion },
    { l: "CPU Vendor",    v: specs.cpu.vendor },
    { l: "CPU Arch",      v: specs.cpu.architecture },
    { l: "Max Clock",     v: specs.cpu.maxClockMhz > 0 ? `${specs.cpu.maxClockMhz} MHz` : "N/A" },
    { l: "RAM Type",      v: `${specs.ram.manufacturer} ${specs.ram.formFactor}`.trim() },
    { l: "RAM Speed",     v: specs.ram.speedMts > 0 ? `${specs.ram.speedMts} MT/s` : "N/A" },
  ];

  return (
    <Card accent={C.muted}>
      <Title icon="🖥" label="SYSTEM INFO" color={C.text} />
      
      {rows.map((r, i) => (
        <div 
          key={r.l} 
          style={{ 
            display: "flex",  justifyContent: "space-between",  padding: "5px 0", 
            borderBottom: i === rows.length - 1 ? "none" : `1px solid ${C.border}33`, alignItems: "center"
          }}
        >
          <span style={{ fontSize: 10, color: C.muted, fontFamily: "monospace" }}>
            {r.l}
          </span>
          <span title={r.v} style={{ fontSize: 10, color: C.text, fontFamily: "monospace", fontWeight: 600,  textAlign: "right", 
            maxWidth: "60%",  overflow: "hidden",  textOverflow: "ellipsis",  whiteSpace: "nowrap" }}>
            {r.v}
          </span>
        </div>
      ))}
    </Card>
  );
}