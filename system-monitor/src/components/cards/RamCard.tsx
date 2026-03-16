import { C, Card, Title, HBar, StatRow, DoubleSparkline } from "../ui/SharedUi";
import { RamData } from "../../types";

export function RamCard({ra}:{ra:RamData}){
  const pct   = (ra.ramUsed  / ra.ramTotal||1)  * 100;
  const swPct = (ra.swapUsed / ra.swapTotal||1) * 100;
  const free  = ra.ramTotal - ra.ramUsed;
  const r = 26, cx = 30, cy = 30, circ = 2 * Math.PI * r;
  const segs = [{ pct, color: C.ram }, { pct: 100 - pct, color: C.border }];
  let off = circ / 4;
  const arcs = segs.map(s => { const dash = (s.pct / 100) * circ; const o = off; off -= dash; return { ...s, dash, off: o }; });

  return (
    <Card accent={C.ram}>
      <Title icon="🧠" label="RAM / SWAP" color={C.ram}/>
      <div style={{ display: "flex", gap: 12, alignItems: "center" }}>
        <svg width={60} height={60} viewBox="0 0 60 60" style={{ flexShrink: 0 }}>
          {arcs.map((a, i) => (
            <circle key={i} cx={cx} cy={cy} r={r} fill="none" stroke={a.color} strokeWidth="10"
              strokeDasharray={`${a.dash} ${circ}`} strokeDashoffset={a.off}
              style={{ transition: "all 0.6s" }} />
          ))}
          <text x={cx} y={cy + 4} textAnchor="middle" fill={C.text} fontSize="10" fontWeight="700" fontFamily="monospace">{Math.round(pct)}%</text>
        </svg>
        <div style={{ flex: 1, minWidth: 0 }}>
          <DoubleSparkline data1={ra.ramHistory} data2={ra.swapHistory} color1={C.ram} color2={C.swap} height={38} uid="ramh" />
          <div style={{ marginTop: 7 }}>
            <HBar pct={pct}   color={C.ram}  label="RAM"  value={`${ra.ramUsed.toFixed(2)} / ${ra.ramTotal.toFixed(2)} GB`} />
            <HBar pct={swPct} color={C.swap} label="SWAP" value={`${ra.swapUsed.toFixed(2)} / ${ra.swapTotal.toFixed(2)} GB`} />
          </div>
        </div>
      </div>
      <StatRow items={[
        { label: "AVAILABLE", value: `${ra.ramAvailable.toFixed(2)} GB`, color: C.ram },
        { label: "FREE", value: `${free.toFixed(2)} GB`, color: C.ram },
        { label: "SWAP", value: `${swPct.toFixed(0)}%`,  color: C.swap },
      ]} />
    </Card>
  );
}
