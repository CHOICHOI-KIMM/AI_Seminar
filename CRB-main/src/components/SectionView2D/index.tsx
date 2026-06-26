import { useMemo } from 'react';
import { useAppState } from '../../store';

/**
 * TRB axial half-section view (SVG) with engineering-style dimension annotations.
 * Upper half only. x = axial, y = radial (positive up via transform).
 */
export default function SectionView2D() {
  const { state } = useAppState();
  const mg = state.input.macro_geom;
  const rg = state.input.raceway_geom;
  const rSph = state.input.roller_profile.r_sph;
  const geom = useMemo(() => computeSectionGeometry(mg, rg, rSph), [mg, rg, rSph]);

  const rOuter = mg.outer_diameter / 2;
  const rBore = mg.d / 2;
  const halfT = mg.t / 2;

  const padLeft = mg.t * 0.42;
  const padRight = mg.t * 0.52;
  const padTop = mg.t * 0.35;
  const padBot = rBore * 0.12;

  const xMin = -halfT - padLeft;
  const xMax = halfT + padRight;
  const yTop = rOuter + padTop;
  const yBot = rBore - padBot;

  return (
    <div className="w-full h-full flex items-center justify-center p-2">
      <svg
        viewBox={`${xMin} ${-yTop} ${xMax - xMin} ${yTop - yBot}`}
        className="w-full h-full"
        style={{ background: 'transparent' }}
        preserveAspectRatio="xMidYMid meet"
      >
        <g transform="scale(1,-1)">
          {/* Center line */}
          <line x1={xMin + 0.5} y1={0} x2={xMax - 0.5} y2={0}
            stroke="#475569" strokeWidth={0.12} strokeDasharray="2 0.5 0.4 0.5" />
          <SectionHalf geom={geom} />
          <Annotations mg={mg} rg={rg} geom={geom} xMin={xMin} xMax={xMax} />
        </g>
      </svg>
    </div>
  );
}

// ─── Types ──────────────────────────────────────────────────────

interface MacroG {
  d: number; outer_diameter: number; t: number; alpha: number; z: number;
  d_we_max: number; d_we_min: number; l_we: number; d_pw: number;
  h_rib: number; alpha_rib: number; g_r: number; h_c: number | null;
}
interface RacewayG {
  alpha_i: number; alpha_o: number; r_i: number; r_o: number;
  r_rib: number; r_rib_circ: number | null; d_uc: number; l_uc: number;
}
interface SectionGeom {
  roller: [number, number][];
  innerRing: [number, number][];
  outerRing: [number, number][];
  contactLine: { x1: number; y1: number; x2: number; y2: number };
  ribLine: { x1: number; y1: number; x2: number; y2: number };
  rPitch: number;
  ribContact: { x: number; y: number; r_contact: number; r_base: number; r_tip: number; r_rib_circ: number };
}

// ─── Geometry computation ───────────────────────────────────────

function computeSectionGeometry(mg: MacroG, rg: RacewayG, rSph: number): SectionGeom {
  // Roller axis at γ = (αi+αo)/2 → outer surface naturally at αo, inner at αi
  const gammaRad = ((rg.alpha_i + rg.alpha_o) / 2) * Math.PI / 180;
  const cosG = Math.cos(gammaRad), sinG = Math.sin(gammaRad);
  const rPitch = mg.d_pw / 2, rBore = mg.d / 2, rOuter = mg.outer_diameter / 2;
  const halfLwe = mg.l_we / 2, rMax = mg.d_we_max / 2, rMin = mg.d_we_min / 2;
  const rcx = 0, rcy = rPitch;
  const ax = cosG, ay = sinG, nx = -sinG, ny = cosG;
  const lx = rcx + halfLwe * ax, ly = rcy + halfLwe * ay;
  const sx = rcx - halfLwe * ax, sy = rcy - halfLwe * ay;

  const roller: [number, number][] = [
    [lx + rMax * nx, ly + rMax * ny], [lx - rMax * nx, ly - rMax * ny],
    [sx - rMin * nx, sy - rMin * ny], [sx + rMin * nx, sy + rMin * ny],
  ];

  const clr = 0.15;
  const irLargeR = ly - rMax * ny - clr * ny, irLargeX = lx - rMax * nx - clr * nx;
  const irSmallR = sy - rMin * ny - clr * ny, irSmallX = sx - rMin * nx - clr * nx;
  const orLargeR = ly + rMax * ny + clr * ny, orLargeX = lx + rMax * nx + clr * nx;
  const orSmallR = sy + rMin * ny + clr * ny, orSmallX = sx + rMin * nx + clr * nx;
  const halfT = mg.t / 2;
  const ribTopR = irLargeR + mg.h_rib * ny, ribTopX = irLargeX + mg.h_rib * nx;

  const ribBackX = Math.min(ribTopX + mg.h_rib * sinG * 0.5, halfT);
  const innerRing: [number, number][] = [
    [-halfT, rBore],           // small end face, bore
    [-halfT, irSmallR],        // small end face, raceway level
    [irSmallX, irSmallR],      // raceway small end
    [irLargeX, irLargeR],      // raceway large end
    [ribTopX, ribTopR],        // rib tip (angled face)
    [ribBackX, ribTopR],       // rib back edge (top)
    [halfT, ribTopR],          // ring face at rib height
    [halfT, rBore],            // large end face, bore
  ];
  const outerRing: [number, number][] = [
    [-halfT, orSmallR], [orSmallX, orSmallR], [orLargeX, orLargeR],
    [halfT, orLargeR], [halfT, rOuter], [-halfT, rOuter],
  ];
  const contactLine = {
    x1: sx - halfLwe * 0.3 * ax, y1: sy - halfLwe * 0.3 * ay,
    x2: lx + halfLwe * 0.3 * ax, y2: ly + halfLwe * 0.3 * ay,
  };
  const ribLine = { x1: irLargeX, y1: irLargeR, x2: ribTopX, y2: ribTopR };

  // Rib contact point position (Liu 2023)
  const alphaRib = (mg.alpha_rib * Math.PI) / 180;
  const rBase = mg.d_pw / 2 + (mg.l_we / 2) * Math.sin(gammaRad) - (mg.d_we_max / 2) * Math.cos(gammaRad);
  // h_c: contact height (user input or default h_rib/2)
  const hC = mg.h_c != null ? Math.max(0, Math.min(mg.h_rib, mg.h_c)) : mg.h_rib / 2;
  const rContact = rBase + hC;
  const rRibCirc = rg.r_rib_circ != null && rg.r_rib_circ > 0
    ? rg.r_rib_circ
    : Math.abs(Math.sin(alphaRib)) > 1e-6 ? rContact / Math.sin(alphaRib) : 1e12;
  // Interpolate along rib line to find contact point position
  const ribDr = ribTopR - irLargeR;
  const t = ribDr > 0.01 ? Math.max(0, Math.min(1, (rContact - irLargeR) / ribDr)) : 0;
  const ribContactX = irLargeX + t * (ribTopX - irLargeX);
  const ribContactY = irLargeR + t * (ribTopR - irLargeR);
  const rTip = rBase + mg.h_rib;
  const ribContact = { x: ribContactX, y: ribContactY, r_contact: rContact, r_base: rBase, r_tip: rTip, r_rib_circ: rRibCirc };

  return { roller, innerRing, outerRing, contactLine, ribLine, rPitch, ribContact };
}

// ─── Section rendering ──────────────────────────────────────────

function p2s(p: [number, number][]): string {
  return p.map(([x, y]) => `${x},${y}`).join(' ');
}

function SectionHalf({ geom }: { geom: SectionGeom }) {
  return (
    <g>
      <polygon points={p2s(geom.outerRing)} fill="#374151" stroke="#9ca3af" strokeWidth={0.2} />
      <polygon points={p2s(geom.innerRing)} fill="#4b5563" stroke="#9ca3af" strokeWidth={0.2} />
      <polygon points={p2s(geom.roller)} fill="#d97706" stroke="#fbbf24" strokeWidth={0.15} />
      <line x1={geom.contactLine.x1} y1={geom.contactLine.y1}
        x2={geom.contactLine.x2} y2={geom.contactLine.y2}
        stroke="#ef4444" strokeWidth={0.12} strokeDasharray="0.5 0.3" />
      <circle cx={0} cy={geom.rPitch} r={0.4} fill="none" stroke="#3b82f6" strokeWidth={0.12} />
      {/* Rib contact point marker */}
      <circle cx={geom.ribContact.x} cy={geom.ribContact.y} r={0.2}
        fill="#f472b6" stroke="#ec4899" strokeWidth={0.08} />
    </g>
  );
}

// ─── Formatting ─────────────────────────────────────────────────

function fmt(v: number): string {
  if (Number.isInteger(v)) return String(v);
  const s = v.toFixed(2);
  return s.replace(/\.?0+$/, '') || '0';
}

// ─── Annotations ────────────────────────────────────────────────

function Annotations({ mg, rg, geom, xMin, xMax }: {
  mg: MacroG; rg: RacewayG; geom: SectionGeom;
  xMin: number; xMax: number;
}) {
  const halfT = mg.t / 2;
  const rOuter = mg.outer_diameter / 2;
  const rBore = mg.d / 2;
  const rPitch = geom.rPitch;

  const s = mg.t * 0.013;
  const fs = Math.max(mg.t * 0.06, 1.2);
  const arrowLen = fs * 0.7;

  const gammaRad = ((rg.alpha_i + rg.alpha_o) / 2) * Math.PI / 180;
  const cosG = Math.cos(gammaRad), sinG = Math.sin(gammaRad);
  const halfLwe = mg.l_we / 2;
  const rMax = mg.d_we_max / 2, rMin = mg.d_we_min / 2;
  const nx = -sinG, ny = cosG;
  const lx = halfLwe * cosG, ly = rPitch + halfLwe * sinG;
  const sx = -halfLwe * cosG, sy = rPitch - halfLwe * sinG;

  const rightX = xMax - fs * 0.2;
  const leftX = xMin + fs * 0.2;

  // Roller outer corners (for Dmax/Dmin leaders)
  const lOuterX = lx + nx * rMax, lOuterY = ly + ny * rMax;
  const sOuterX = sx + nx * rMin, sOuterY = sy + ny * rMin;

  return (
    <g>
      <defs>
        <marker id="arr" viewBox="0 0 6 3" refX="6" refY="1.5"
          markerWidth={arrowLen} markerHeight={arrowLen * 0.5}
          orient="auto-start-reverse" fill="#94a3b8">
          <path d="M0,0 L6,1.5 L0,3 Z" />
        </marker>
      </defs>

      {/* ── LEFT: Radial dims — dot at surface + dashed line + label ── */}
      <CalloutDim
        dotX={-halfT} dotY={rOuter}
        labelX={leftX} labelY={rOuter + fs * 0.5}
        label={`D = ${fmt(mg.outer_diameter)}`}
        s={s} fs={fs} />
      <CalloutDim
        dotX={0} dotY={rPitch}
        labelX={leftX} labelY={rPitch}
        label={`d_pw = ${fmt(mg.d_pw)}`}
        s={s} fs={fs} color="#60a5fa" />
      <CalloutDim
        dotX={-halfT} dotY={rBore}
        labelX={leftX} labelY={rBore - fs * 0.5}
        label={`d = ${fmt(mg.d)}`}
        s={s} fs={fs} />

      {/* ── TOP: Width T ── */}
      <WidthDim y={rOuter + mg.t * 0.16} xLeft={-halfT} xRight={halfT}
        rOuter={rOuter} label={`T = ${fmt(mg.t)}`} s={s} fs={fs} />

      {/* ── L_we — above outer ring ── */}
      <LweDim sx={sx} sy={sy} lx={lx} ly={ly} cosA={cosG} sinA={sinG}
        label={`Lwe = ${fmt(mg.l_we)}`} s={s} fs={fs}
        offset={mg.d_we_max * 0.65 + mg.t * 0.2} />

      {/* ── Roller tilt angle γ — at roller center ── */}
      <AngleArc cx={0} cy={rPitch}
        radius={Math.min(mg.t * 0.3, (rOuter - rBore) * 0.35)}
        angleDeg={(rg.alpha_i + rg.alpha_o) / 2}
        label={`\u03B3=${fmt((rg.alpha_i + rg.alpha_o) / 2)}\u00B0`}
        s={s} fs={fs} color="#f87171" />

      {/* ── Inner raceway angle αi — at inner raceway small end ── */}
      <AngleArc cx={sx - rMin * nx} cy={sy - rMin * ny}
        radius={Math.min(mg.t * 0.2, (rOuter - rBore) * 0.2)}
        angleDeg={rg.alpha_i} label={`\u03B1i=${fmt(rg.alpha_i)}\u00B0`}
        s={s} fs={fs} color="#4ade80" />

      {/* ── Outer raceway angle αo — at outer raceway small end ── */}
      <AngleArc cx={sx + rMin * nx} cy={sy + rMin * ny}
        radius={Math.min(mg.t * 0.2, (rOuter - rBore) * 0.2)}
        angleDeg={rg.alpha_o} label={`\u03B1o=${fmt(rg.alpha_o)}\u00B0`}
        s={s} fs={fs} color="#22d3ee" />

      {/* ── RIGHT: Dmax — leader from outer corner at large end ── */}
      <CalloutDim
        dotX={lOuterX} dotY={lOuterY}
        labelX={rightX} labelY={rOuter - fs * 0.5}
        label={`Dmax = ${fmt(mg.d_we_max)}`}
        s={s} fs={fs} color="#fbbf24" anchor="end" />

      {/* ── LEFT: Dmin — leader from outer corner at small end ── */}
      <CalloutDim
        dotX={sOuterX} dotY={sOuterY}
        labelX={leftX} labelY={sOuterY + fs * 1.2}
        label={`Dmin = ${fmt(mg.d_we_min)}`}
        s={s} fs={fs} color="#fbbf24" />

      {/* ── Rib height — dimension line from raceway to rib tip ── */}
      {mg.h_rib > 0 && (
        <RibHeightDim
          x1={geom.ribLine.x1} y1={geom.ribLine.y1}
          x2={geom.ribLine.x2} y2={geom.ribLine.y2}
          label={`h_rib = ${fmt(mg.h_rib)}`}
          rightX={rightX} s={s} fs={fs}
          labelY={(geom.ribLine.y1 + geom.ribLine.y2) / 2}
        />
      )}

      {/* ── r_base — at rib base (raceway surface at large end) ── */}
      <CalloutDim
        dotX={geom.ribLine.x1} dotY={geom.ribLine.y1}
        labelX={rightX} labelY={geom.ribLine.y1 - fs * 0.8}
        label={`r_base = ${geom.ribContact.r_base.toFixed(2)}`}
        s={s} fs={fs} color="#a78bfa" anchor="end" />

      {/* ── r_c — callout at rib contact point ── */}
      <CalloutDim
        dotX={geom.ribContact.x} dotY={geom.ribContact.y}
        labelX={rightX} labelY={geom.ribContact.y}
        label={`r_c = ${geom.ribContact.r_contact.toFixed(2)}`}
        s={s} fs={fs} color="#f472b6" anchor="end" />

      {/* ── r_tip — at rib tip ── */}
      <CalloutDim
        dotX={geom.ribLine.x2} dotY={geom.ribLine.y2}
        labelX={rightX} labelY={geom.ribLine.y2 + fs * 1.2}
        label={`r_tip = ${geom.ribContact.r_tip.toFixed(2)}`}
        s={s} fs={fs} color="#c084fc" anchor="end" />

    </g>
  );
}

// ─── Primitives ─────────────────────────────────────────────────

function Txt({ x, y, text, fs, fill = '#e2e8f0', anchor = 'start' }: {
  x: number; y: number; text: string; fs: number;
  fill?: string; anchor?: 'start' | 'middle' | 'end';
}) {
  return (
    <g transform={`translate(${x}, ${y}) scale(1,-1)`}>
      <text fill={fill} fontSize={fs} fontFamily="'JetBrains Mono', monospace"
        dominantBaseline="middle" textAnchor={anchor} fontWeight={500}>
        {text}
      </text>
    </g>
  );
}

/** Callout dimension: dot at reference point + elbow leader → label */
function CalloutDim({ dotX, dotY, labelX, labelY, label, s, fs, color = '#e2e8f0', anchor = 'start' }: {
  dotX: number; dotY: number; labelX: number; labelY: number;
  label: string; s: number; fs: number;
  color?: string; anchor?: 'start' | 'end';
}) {
  const lineColor = color === '#e2e8f0' ? '#94a3b8' : color;
  const elbowX = labelX + (dotX - labelX) * 0.6;
  const textOffset = fs * 0.6; // text sits above the leader line
  return (
    <g>
      <circle cx={dotX} cy={dotY} r={s} fill={lineColor} />
      <polyline
        points={`${labelX},${labelY} ${elbowX},${labelY} ${dotX},${dotY}`}
        fill="none" stroke={lineColor} strokeWidth={s * 0.5}
        strokeDasharray={`${s * 2} ${s}`} />
      <Txt x={labelX} y={labelY + textOffset} text={label} fs={fs * 0.85} fill={color} anchor={anchor} />
    </g>
  );
}

function WidthDim({ y, xLeft, xRight, rOuter, label, s, fs }: {
  y: number; xLeft: number; xRight: number; rOuter: number;
  label: string; s: number; fs: number;
}) {
  return (
    <g>
      <line x1={xLeft} y1={rOuter + s * 2} x2={xLeft} y2={y + s * 3}
        stroke="#475569" strokeWidth={s * 0.4} />
      <line x1={xRight} y1={rOuter + s * 2} x2={xRight} y2={y + s * 3}
        stroke="#475569" strokeWidth={s * 0.4} />
      <line x1={xLeft} y1={y} x2={xRight} y2={y}
        stroke="#94a3b8" strokeWidth={s * 0.4}
        markerStart="url(#arr)" markerEnd="url(#arr)" />
      <Txt x={(xLeft + xRight) / 2} y={y + fs * 0.7} text={label} fs={fs} anchor="middle" />
    </g>
  );
}

function LweDim({ sx, sy, lx, ly, cosA, sinA, label, s, fs, offset }: {
  sx: number; sy: number; lx: number; ly: number;
  cosA: number; sinA: number; label: string;
  s: number; fs: number; offset: number;
}) {
  const nx = -sinA, ny = cosA;
  const x1 = sx + nx * offset, y1 = sy + ny * offset;
  const x2 = lx + nx * offset, y2 = ly + ny * offset;
  return (
    <g>
      <line x1={sx + nx * offset * 0.5} y1={sy + ny * offset * 0.5}
        x2={x1 + nx * fs * 0.3} y2={y1 + ny * fs * 0.3}
        stroke="#475569" strokeWidth={s * 0.4} />
      <line x1={lx + nx * offset * 0.5} y1={ly + ny * offset * 0.5}
        x2={x2 + nx * fs * 0.3} y2={y2 + ny * fs * 0.3}
        stroke="#475569" strokeWidth={s * 0.4} />
      <line x1={x1} y1={y1} x2={x2} y2={y2}
        stroke="#94a3b8" strokeWidth={s * 0.4}
        markerStart="url(#arr)" markerEnd="url(#arr)" />
      <Txt x={(x1 + x2) / 2 + nx * fs} y={(y1 + y2) / 2 + ny * fs}
        text={label} fs={fs * 0.9} anchor="middle" />
    </g>
  );
}

/** Rib height dimension: two dots + offset dim line + label */
function RibHeightDim({ x1, y1, x2, y2, label, rightX, s, fs, labelY }: {
  x1: number; y1: number; x2: number; y2: number;
  label: string; rightX: number; s: number; fs: number;
  labelY?: number;
}) {
  const color = '#a78bfa';
  const midGuideX = (x1 + x2) / 2;
  const offsetX = midGuideX + (rightX - midGuideX) * 0.5;
  const ly = labelY ?? (y1 + y2) / 2;
  return (
    <g>
      <circle cx={x1} cy={y1} r={s} fill={color} />
      <circle cx={x2} cy={y2} r={s * 1.2} fill={color} />
      <line x1={x1} y1={y1} x2={offsetX - s} y2={y1}
        stroke={color} strokeWidth={s * 0.35} strokeDasharray={`${s * 2} ${s}`} />
      <line x1={x2} y1={y2} x2={offsetX - s} y2={y2}
        stroke={color} strokeWidth={s * 0.35} strokeDasharray={`${s * 2} ${s}`} />
      <line x1={offsetX} y1={y1} x2={offsetX} y2={y2}
        stroke={color} strokeWidth={s * 0.4}
        markerStart="url(#arr)" markerEnd="url(#arr)" />
      <Txt x={offsetX + fs * 0.3} y={ly} text={label}
        fs={fs * 0.85} fill={color} anchor="start" />
    </g>
  );
}

/** Angle arc with guide lines showing which two lines form the angle */
function AngleArc({ cx, cy, radius, angleDeg, label, s, fs, color }: {
  cx: number; cy: number; radius: number; angleDeg: number;
  label: string; s: number; fs: number; color: string;
}) {
  const aRad = (angleDeg * Math.PI) / 180;
  const x0 = cx + radius, y0 = cy;
  const x1 = cx + radius * Math.cos(aRad), y1 = cy + radius * Math.sin(aRad);
  const arcPath = `M ${x0},${y0} A ${radius},${radius} 0 ${angleDeg > 180 ? 1 : 0} 1 ${x1},${y1}`;
  // Guide lines: extend beyond arc radius to show the two edges forming the angle
  const guideLen = radius * 1.4;
  // Horizontal guide (0°)
  const gx0 = cx + guideLen, gy0 = cy;
  // Angled guide (angleDeg°)
  const gx1 = cx + guideLen * Math.cos(aRad), gy1 = cy + guideLen * Math.sin(aRad);
  // Label to the right of the arc end point, vertically centered on arc
  const midA = aRad / 2;
  const lx = cx + guideLen + fs * 0.3;
  const ly = cy + guideLen * Math.sin(midA);
  return (
    <g>
      {/* guide lines showing the two edges */}
      <line x1={cx} y1={cy} x2={gx0} y2={gy0}
        stroke={color} strokeWidth={s * 0.4} strokeOpacity={0.5}
        strokeDasharray={`${s * 1.5} ${s * 0.8}`} />
      <line x1={cx} y1={cy} x2={gx1} y2={gy1}
        stroke={color} strokeWidth={s * 0.4} strokeOpacity={0.5}
        strokeDasharray={`${s * 1.5} ${s * 0.8}`} />
      {/* arc */}
      <path d={arcPath} fill="none" stroke={color} strokeWidth={s * 0.4} />
      {/* label to the right of arc */}
      <Txt x={lx} y={ly} text={label}
        fs={fs * 0.75} fill={color} anchor="start" />
    </g>
  );
}
