// ── palette ──────────────────────────────────────────────────────────────────
export const C = {
  bg:      "#0d1117",
  surface: "#161b22",
  card:    "#1c2230",
  border:  "#2a3344",
  cpu:     "#4ade80",   // green
  ram:     "#60a5fa",   // blue
  gpu:     "#f472b6",   // pink
  disk:    "#fb923c",   // orange
  net:     "#a78bfa",   // violet
  temp:    "#fbbf24",   // amber
  text:    "#e2e8f0",
  muted:   "#64748b",
};

// ── mini sparkline ────────────────────────────────────────────────────────────
export function Sparkline({ data, color, height = 48 }:any) {
  const w = 200, h = height;
  const max = Math.max(...data, 1);
  const pts = data.map((v:any, i:any) => `${(i / (data.length - 1)) * w},${h - (v / max) * h}`).join(" ");
  const area = `${pts} ${w},${h} 0,${h}`;
  return (
    <svg viewBox={`0 0 ${w} ${h}`} style={{ width: "100%", height }} preserveAspectRatio="none">
      <defs>
        <linearGradient id={`sg${color.replace("#","")}`} x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stopColor={color} stopOpacity="0.35" />
          <stop offset="100%" stopColor={color} stopOpacity="0" />
        </linearGradient>
      </defs>
      <polygon points={area} fill={`url(#sg${color.replace("#","")})`} />
      <polyline points={pts} fill="none" stroke={color} strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  );
}

// ── donut chart ───────────────────────────────────────────────────────────────
export function Donut({ pct, color, size = 88, label, sub }:any) {
  const r = 36, cx = 44, cy = 44, circ = 2 * Math.PI * r;
  const dash = (pct / 100) * circ;
  return (
    <div style={{ display: "flex", flexDirection: "column", alignItems: "center", gap: 4 }}>
      <svg width={size} height={size} viewBox="0 0 88 88">
        <circle cx={cx} cy={cy} r={r} fill="none" stroke={C.border} strokeWidth="10" />
        <circle cx={cx} cy={cy} r={r} fill="none" stroke={color} strokeWidth="10"
          strokeDasharray={`${dash} ${circ}`} strokeDashoffset={circ / 4}
          strokeLinecap="round"
          style={{ transition: "stroke-dasharray 0.6s ease" }}
        />
        <text x={cx} y={cy - 4} textAnchor="middle" fill={C.text} fontSize="14" fontWeight="700" fontFamily="monospace">
          {Math.round(pct)}%
        </text>
        <text x={cx} y={cy + 12} textAnchor="middle" fill={C.muted} fontSize="9" fontFamily="monospace">
          {sub}
        </text>
      </svg>
      <span style={{ fontSize: 11, color: C.muted, fontFamily: "monospace" }}>{label}</span>
    </div>
  );
}

// ── horizontal bar ────────────────────────────────────────────────────────────
export function HBar({ pct, color, label, value }:any) {
  return (
    <div style={{ marginBottom: 8 }}>
      <div style={{ display: "flex", justifyContent: "space-between", marginBottom: 4 }}>
        <span style={{ fontSize: 11, color: C.muted, fontFamily: "monospace" }}>{label}</span>
        <span style={{ fontSize: 11, color, fontFamily: "monospace", fontWeight: 700 }}>{value}</span>
      </div>
      <div style={{ background: C.border, borderRadius: 4, height: 6, overflow: "hidden" }}>
        <div style={{
          width: `${pct}%`, height: "100%", background: color, borderRadius: 4,
          transition: "width 0.5s ease",
          boxShadow: `0 0 8px ${color}66`
        }} />
      </div>
    </div>
  );
}

// ── temp pill ─────────────────────────────────────────────────────────────────
export function TempBadge({ val }:any) {
  const hot = val > 80, warm = val > 65;
  const col = hot ? "#ef4444" : warm ? C.temp : C.cpu;
  return (
    <span style={{
      background: col + "22", color: col, border: `1px solid ${col}44`,
      borderRadius: 6, padding: "2px 8px", fontSize: 11, fontFamily: "monospace", fontWeight: 700
    }}>🌡 {val}°C</span>
  );
}

// ── card wrapper ──────────────────────────────────────────────────────────────
export function Card({ children, accent, style = {} }:any) {
  return (
    <div style={{
      background: C.card,
      border: `1px solid ${accent || C.border}33`,
      borderRadius: 14,
      padding: "16px 18px",
      boxShadow: accent ? `0 0 24px ${accent}14, 0 2px 8px #00000044` : "0 2px 8px #00000044",
      ...style
    }}>
      {children}
    </div>
  );
}

// ── section title ─────────────────────────────────────────────────────────────
export function Title({ icon, label, color, right }:any) {
  return (
    <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", marginBottom: 14 }}>
      <div style={{ display: "flex", alignItems: "center", gap: 7 }}>
        <span style={{ fontSize: 15 }}>{icon}</span>
        <span style={{ fontSize: 13, fontWeight: 700, color, fontFamily: "monospace", letterSpacing: 1 }}>{label}</span>
      </div>
      {right}
    </div>
  );
}