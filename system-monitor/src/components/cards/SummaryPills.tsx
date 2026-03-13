import { C } from "../ui/SharedUi";
import { CpuData, GpuData, NetworkData, RamData } from "../../types";

interface SummaryPillsProps {
    c: CpuData;
    r: RamData;
    g: GpuData;
    n: NetworkData;
}

export function SummaryPills({c,r,g,n }: SummaryPillsProps){
    return(
      <div style={{ display: "flex", gap: 12, marginBottom: 20, flexWrap: "wrap" }}>
        {[  
        { label: "CPU",  val: `${Math.round(c.cpuUse)}%`, color: C.cpu  },
        { label: "RAM",  val: `${((r.ramUsed / r.ramTotal) * 100).toFixed(0)}%`, color: C.ram  },
        { label: "GPU",  val: `${Math.round(g.gpuLoad)}%`, color: C.gpu  },
        { label: "DISK", val: `67%`, color: C.disk },
        { label: "NET↓", val: `${n.download.toFixed(1)} MB/s`, color: C.net  },
        ].map(x => (
          <div key={x.label} style={{
            background: x.color + "10",  border: `1px solid ${x.color}33`, borderRadius: 6, 
            padding: "8px 16px",display: "flex", alignItems: "center", gap: 12
          }}>
            <span style={{ fontSize: 11, color: C.muted, fontFamily: "monospace", letterSpacing: 0.5}}>
              {x.label}
            </span>
            <span style={{ fontSize: 15, color: x.color, fontFamily: "monospace", fontWeight: 800 }}>
              {x.val}
            </span>
          </div>
        ))}
      </div>
    )
}