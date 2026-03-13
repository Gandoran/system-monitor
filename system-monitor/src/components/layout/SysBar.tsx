import { C } from "../ui/SharedUi";
import { useStaticSystemInfo } from "../../hooks/useStaticSystemInfo.ts";
import { useSystemMonitor } from "../../hooks/useSystemMonitor.ts";
import { formatUptime } from "../../utils/timeFormatters.ts";

interface SysBarProps {
  cpuTemp: number;
  gpuTemp: number;
}

export function SysBar({cpuTemp, gpuTemp }: SysBarProps) {
 const { hostname, os_name } = useStaticSystemInfo();
  const { uptime } = useSystemMonitor();
  
  return (
    <div style={{ 
      background: C.surface, border: `1px solid ${C.border}`, borderRadius: 10, padding: "10px 18px",
      display: "flex", alignItems: "center", justifyContent: "space-between", marginBottom: 18
    }}>
      <div style={{ display: "flex", gap: 24 }}>
        {[
          { icon: "🖥", label: hostname, sub: os_name }, 
          { icon: "⏱", label: "Uptime", sub: formatUptime(uptime.uptime as any) },
          { icon: "🔥", label: "CPU Temp", sub: `${cpuTemp}°C` },
          { icon: "🎮", label: "GPU Temp", sub: `${gpuTemp}°C` },
        ].map(x => (
          <div key={x.label} style={{ display: "flex", alignItems: "center", gap: 8 }}>
            <span style={{ fontSize: 16 }}>{x.icon}</span>
            <div>
              <div style={{ fontSize: 10, color: C.muted, fontFamily: "monospace" }}>{x.label}</div>
              <div style={{ fontSize: 12, color: C.text, fontFamily: "monospace", fontWeight: 700 }}>{x.sub}</div>
            </div>
          </div>
        ))}
      </div>
      <div style={{ display: "flex", alignItems: "center", gap: 6, marginLeft: "auto" }}>
        <div style={{width: 7, height: 7, borderRadius: "50%", background: C.cpu, boxShadow: `0 0 6px ${C.cpu}`, animation: "pulse 1.2s infinite"}} />
          <span style={{ fontSize: 10, color: C.cpu, fontFamily: "monospace", letterSpacing: 1 }}>LIVE</span>
        </div>
    </div>
  )
}