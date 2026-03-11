import { C,Sparkline, Card, Title } from "../ui/SharedUi";
import { RamData } from "../../types";

export function RamCard({ra}:{ra:RamData}){
  const pct = (ra.ramUsed / ra.ramTotal) * 100;
  const free = ra.ramTotal - ra.ramUsed;
  const ar = 30, cx = 36, cy = 36, circ = 2 * Math.PI * ar;
  const segs = [
    { label: "Used",   pct, color: C.ram },
    { label: "Free",   pct: (free / ra.ramTotal) * 100, color: C.border },
  ];
  let offset = circ / 4;
  const arcs = segs.map(s => {
    const dash = (s.pct / 100) * circ;
    const o = offset;
    offset -= dash;
    return { ...s, dash, offset: o };
  });

  return (
    <Card accent={C.ram}>
      <Title icon="🧠" label="RAM" color={C.ram} right={
        <span style={{ fontSize: 11, color: C.muted, fontFamily: "monospace" }}>
          {ra.ramUsed.toFixed(1)} / {ra.ramTotal} GB
        </span>
      } />
      <div style={{ display: "flex", gap: 16, alignItems: "center" }}>
        {/* pie */}
        <svg width={72} height={72} viewBox="0 0 72 72" style={{ flexShrink: 0 }}>
          {arcs.map((a, i) => (
            <circle key={i} cx={cx} cy={cy} r={ar} fill="none"
              stroke={a.color} strokeWidth="12"
              strokeDasharray={`${a.dash} ${circ}`}
              strokeDashoffset={a.offset}
              style={{ transition: "stroke-dasharray 0.6s ease, stroke-dashoffset 0.6s ease" }}
            />
          ))}
          <text x={cx} y={cy + 4} textAnchor="middle" fill={C.text} fontSize="11" fontWeight="700" fontFamily="monospace">
            {Math.round(pct)}%
          </text>
        </svg>
        {/* history */}
        <div style={{ flex: 1, minWidth: 0 }}>
          <div style={{ marginBottom: 6 }}>
            <Sparkline data={ra.ramHistory} color={C.ram} height={44} />
          </div>
          <div style={{ display: "flex", gap: 12 }}>
            {[
              { l: "Used", v: `${ra.ramUsed.toFixed(1)} GB`, c: C.ram },
              { l: "Free", v: `${free.toFixed(1)} GB`, c: C.muted },
            ].map(x => (
              <div key={x.l}>
                <div style={{ fontSize: 9, color: C.muted, fontFamily: "monospace" }}>{x.l}</div>
                <div style={{ fontSize: 13, color: x.c, fontFamily: "monospace", fontWeight: 700 }}>{x.v}</div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </Card>
  );
}
