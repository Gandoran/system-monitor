import { C, Card, SectionTitle, UnitTitle } from "../ui/SharedUi";
import { SystemSpecsPayload } from "../../types/specs";

export function SpecsCard({ specs }: { specs: SystemSpecsPayload }) {
  const Row = ({ label, value, isLast = false }: { label: string, value: string | number, isLast?: boolean }) => (
    <div 
      style={{ 
        display: "flex", justifyContent: "space-between", padding: "6px 0", 
        borderBottom: isLast ? "none" : `1px solid ${C.border}33`, alignItems: "center"
      }}>
      <span style={{ fontSize: 12, color: C.muted, fontFamily: "monospace" }}>
        {label}
      </span>
      <span title={String(value)} style={{ 
        fontSize: 12, color: C.text, fontFamily: "monospace", fontWeight: 600, textAlign: "right", 
        maxWidth: "60%", overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" 
      }}>
        {value}
      </span>
    </div>
  );

  return (
    <Card accent={C.muted}>
      
      <SectionTitle label="💻 OS" />
      <Row label="Name" value={specs.os?.name || "N/A"} />
      <Row label="Architecture" value={specs.os?.architecture || "N/A"} />
      <Row label="Build" value={specs.os?.buildNumber || "N/A"} />
      <Row label="Kernel" value={specs.os?.kernelVersion || "N/A"} isLast />

      <SectionTitle label="🔌 MOTHERBOARD" />
      <Row label="Vendor" value={specs.mobo?.vendor || "N/A"} />
      <Row label="Model" value={specs.mobo?.model || "N/A"} />
      <Row label="BIOS" value={specs.mobo?.biosVersion || "N/A"} isLast />

      <SectionTitle label="🧠 CPU" />
      <Row label="Vendor" value={specs.cpu?.vendor || "N/A"} />
      <Row label="Architecture" value={specs.cpu?.architecture || "N/A"} />
      <Row label="L3 Cache" value={specs.cpu?.l3CacheMb ? `${specs.cpu.l3CacheMb} MB` : "N/A"} />
      <Row label="Max Clock" value={specs.cpu?.maxClockMhz ? `${specs.cpu.maxClockMhz} MHz` : "N/A"} isLast />

      <SectionTitle label="⚡ RAM" />
      <Row label="Capacity" value={specs.ram?.totalCapacityBytes ? `${specs.ram.totalCapacityBytes} GB` : "N/A"} />
      <Row label="Type" value={`${specs.ram?.manufacturer || ""} ${specs.ram?.formFactor || ""}`.trim() || "N/A"} />
      <Row label="Part Number" value={specs.ram?.partNumber || "N/A"} />
      <Row label="Speed" value={specs.ram?.speedMts ? `${specs.ram.speedMts} MT/s` : "N/A"} isLast />

      <SectionTitle label="💾 STORAGE" />
      {specs.disks && specs.disks.length > 0 ? (
        specs.disks.map((disk, i) => (
          <div key={`disk-${i}`} style={{ marginBottom: 12, paddingLeft: 8, borderLeft: `2px solid ${C.border}55` }}>
            <UnitTitle label={`Disk ${i}`} />
            <Row label="Model" value={disk.model || "Unknown"} />
            <Row label="Capacity" value={(disk as any).capacityFormatted || (disk.capacityBytes ? `${disk.capacityBytes} GB` : "Unknown")} />
            <Row label="Type" value={disk.mediaType || "Unknown"} />
            <Row label="Interface" value={disk.interfaceType || "Unknown"} isLast />
          </div>
        ))
      ) : (
        <Row label="Disks" value="No disks found" isLast />
      )}

      <SectionTitle label="📺 DISPLAYS" />
      {specs.displays && specs.displays.length > 0 ? (
        specs.displays.map((disp, i) => (
          <div key={`disp-${i}`} style={{ marginBottom: 12, paddingLeft: 8, borderLeft: `2px solid ${C.border}55` }}>
            <UnitTitle label={`Display ${i}`} />
            <Row label="Name" value={disp.name || "Generic"} />
            <Row label="Resolution" value={`${disp.resolutionX || 0}x${disp.resolutionY || 0}`} />
            <Row label="Refresh Rate" value={`${disp.refreshRateHz || 60} Hz`} isLast />
          </div>
        ))
      ) : (
        <Row label="Displays" value="No displays found" isLast />
      )}
      
    </Card>
  );
}