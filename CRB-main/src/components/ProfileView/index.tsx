import { useMemo, useState, useEffect, useCallback } from 'react';
import { useAppState } from '../../store';
import type { RollerProfile, RacewayProfile, MacroGeometry, CrownType } from '../../types/bearing';

// ─── Context menu state ──────────────────────────────────────────
interface ContextMenuState {
  x: number;
  y: number;
  title: string;
  data: ProfilePoint[];
}

type CopyFormat = 'tsv' | 'csv' | 'json';

/**
 * Micro-profile view: roller crown + raceway profiles along the effective length.
 * Shows Δz (profile correction) vs axial position with inline parameter annotations.
 */
export default function ProfileView() {
  const { state } = useAppState();
  const { roller_profile, raceway_profile_inner, raceway_profile_outer, macro_geom, solver } = state.input;
  const nSlices = solver.n_slices;

  const profiles = useMemo(
    () => computeProfiles(roller_profile, raceway_profile_inner, raceway_profile_outer, macro_geom, nSlices),
    [roller_profile, raceway_profile_inner, raceway_profile_outer, macro_geom, nSlices],
  );

  const crownDrop = profiles.roller.length > 0
    ? Math.max(...profiles.roller.map((p: ProfilePoint) => p.dz)) - Math.min(...profiles.roller.map((p: ProfilePoint) => p.dz))
    : 0;

  // ── Context menu ──
  const [ctxMenu, setCtxMenu] = useState<ContextMenuState | null>(null);
  const [copiedFmt, setCopiedFmt] = useState<CopyFormat | null>(null);

  const handleContextMenu = useCallback((e: React.MouseEvent, title: string, data: ProfilePoint[]) => {
    e.preventDefault();
    setCopiedFmt(null);
    setCtxMenu({ x: e.clientX, y: e.clientY, title, data });
  }, []);

  const closeMenu = useCallback(() => { setCtxMenu(null); setCopiedFmt(null); }, []);

  const handleCopy = useCallback(async (fmt: CopyFormat) => {
    if (!ctxMenu) return;
    const { data } = ctxMenu;
    let text = '';
    if (fmt === 'tsv') {
      text = `Position [mm]\tΔz [μm]\n` + data.map(p => `${p.x}\t${p.dz}`).join('\n');
    } else if (fmt === 'csv') {
      text = `Position [mm],Δz [μm]\n` + data.map(p => `${p.x},${p.dz}`).join('\n');
    } else {
      text = JSON.stringify(data.map(p => ({ position_mm: p.x, dz_um: p.dz })), null, 2);
    }
    await navigator.clipboard.writeText(text);
    setCopiedFmt(fmt);
    setTimeout(() => { setCtxMenu(null); setCopiedFmt(null); }, 600);
  }, [ctxMenu]);

  useEffect(() => {
    if (!ctxMenu) return;
    const onKey = (e: KeyboardEvent) => { if (e.key === 'Escape') closeMenu(); };
    const onClick = () => closeMenu();
    window.addEventListener('keydown', onKey);
    window.addEventListener('click', onClick);
    return () => { window.removeEventListener('keydown', onKey); window.removeEventListener('click', onClick); };
  }, [ctxMenu, closeMenu]);

  return (
    <div className="w-full h-full grid grid-cols-2 gap-2 p-2 overflow-y-auto custom-scrollbar relative">
      {/* Left column: Input profiles */}
      <div className="flex flex-col gap-1">
        <div className="text-xs text-text-canvas font-semibold px-1 pb-0.5 border-b border-white/10">
          Input Profiles
        </div>
        <ProfileChart
          title="Roller Crown Profile"
          subtitle={getCrownDescription(roller_profile.crown_type, roller_profile.delta_c, macro_geom.l_we)}
          data={profiles.roller}
          lWe={macro_geom.l_we}
          color="#d97706"
          fillColor="rgba(217,119,6,0.15)"
          rollerProfile={roller_profile}
          crownDrop={crownDrop}
          onContextMenu={handleContextMenu}
        />
        <ProfileChart
          title="Inner Raceway Profile"
          subtitle={getRacewayDescription(raceway_profile_inner)}
          data={profiles.inner}
          lWe={macro_geom.l_we}
          color="#3b82f6"
          fillColor="rgba(59,130,246,0.15)"
          racewayProfile={raceway_profile_inner}
          onContextMenu={handleContextMenu}
        />
        <ProfileChart
          title="Outer Raceway Profile"
          subtitle={getRacewayDescription(raceway_profile_outer)}
          data={profiles.outer}
          lWe={macro_geom.l_we}
          color="#10b981"
          fillColor="rgba(16,185,129,0.15)"
          racewayProfile={raceway_profile_outer}
          onContextMenu={handleContextMenu}
        />
      </div>

      {/* Right column: Computed contact profiles */}
      <div className="flex flex-col gap-1">
        <div className="text-xs text-text-canvas font-semibold px-1 pb-0.5 border-b border-white/10">
          Contact Profiles (Superposed)
        </div>
        <ProfileChart
          title="Inner Contact Profile"
          subtitle="Δz_inner_total = Δz_roller + Δz_inner"
          data={profiles.totalInner}
          lWe={macro_geom.l_we}
          color="#818cf8"
          fillColor="rgba(129,140,248,0.1)"
          onContextMenu={handleContextMenu}
        />
        <ProfileChart
          title="Outer Contact Profile"
          subtitle="Δz_outer_total = Δz_roller + Δz_outer"
          data={profiles.totalOuter}
          lWe={macro_geom.l_we}
          color="#f472b6"
          fillColor="rgba(244,114,182,0.1)"
          onContextMenu={handleContextMenu}
        />
      </div>

      {/* ── Context menu overlay ── */}
      {ctxMenu && (
        <div
          className="fixed z-50 min-w-[180px] bg-gray-900 border border-white/15 rounded-lg shadow-xl py-1 text-sm"
          style={{ left: ctxMenu.x, top: ctxMenu.y }}
          onClick={e => e.stopPropagation()}
        >
          <div className="px-3 py-1.5 text-[11px] text-gray-400 border-b border-white/10 truncate">
            {ctxMenu.title} ({ctxMenu.data.length} pts)
          </div>
          {(['tsv', 'csv', 'json'] as CopyFormat[]).map(fmt => (
            <button
              key={fmt}
              className="w-full text-left px-3 py-1.5 text-gray-200 hover:bg-white/10 transition-colors flex items-center gap-2"
              onClick={() => handleCopy(fmt)}
            >
              <span className="text-gray-500 text-xs w-4">📋</span>
              {copiedFmt === fmt
                ? <span className="text-emerald-400">Copied!</span>
                : `Copy as ${fmt.toUpperCase()}`}
            </button>
          ))}
        </div>
      )}
    </div>
  );
}

// ─── Description helpers ──────────────────────────────────────────

function getCrownDescription(ct: CrownType, deltaC: number, lWe: number): string {
  const halfL = lWe / 2;
  if ('Logarithmic' in ct) {
    const aLog = deltaC > 0 ? (deltaC / Math.log(1 / (1 - 0.9 * 0.9))).toFixed(4) : '0';
    return `Logarithmic (Reusner):  δ_c=${deltaC}μm → A_log=${aLog}`;
  }
  if ('Circular' in ct) {
    const rCrown = deltaC > 0 ? (halfL * halfL / (2 * deltaC / 1000)).toFixed(1) : '∞';
    return `Circular arc:  δ_c=${deltaC}μm → R_crown=${rCrown} mm`;
  }
  if ('Parabolic' in ct) {
    const c2 = deltaC > 0 ? (deltaC / (halfL * halfL)).toFixed(6) : '0';
    return `Parabolic:  δ_c=${deltaC}μm → c₂=${c2}`;
  }
  if ('Polynomial' in ct) {
    const c = ct.Polynomial.coeffs;
    const fmt = (v: number) => v.toFixed(4);
    return `Polynomial: ${fmt(c[0]??0)}x⁴ + ${fmt(c[1]??0)}x³ + ${fmt(c[2]??0)}x² + ${fmt(c[3]??0)}x + ${fmt(c[4]??0)}`;
  }
  return 'Custom measured profile (user data points)';
}

function getRacewayDescription(rp: RacewayProfile): string {
  const parts: string[] = [];
  if (rp.custom_profile && rp.custom_profile.length >= 2) {
    parts.push('Custom profile');
  } else if (rp.delta_rw > 0) {
    parts.push(`Parabolic: δ_rw = ${rp.delta_rw} μm`);
  } else {
    parts.push('Flat (no crowning)');
  }
  if (rp.ra > 0) parts.push(`Ra = ${rp.ra} μm`);
  if (rp.w_a > 0) parts.push(`W_a = ${rp.w_a} μm`);
  return parts.join('  |  ');
}

// ─── Profile computation ────────────────────────────────────────────

interface ProfilePoint {
  x: number; // axial position [mm] from center
  dz: number; // profile correction [um]
}

interface AllProfiles {
  roller: ProfilePoint[];
  inner: ProfilePoint[];
  outer: ProfilePoint[];
  totalInner: ProfilePoint[];  // Δz_roller + Δz_inner (inner contact)
  totalOuter: ProfilePoint[];  // Δz_roller + Δz_outer (outer contact)
}

function computeRollerProfile(rp: RollerProfile, lWe: number, n: number): ProfilePoint[] {
  const dx = lWe / n;
  const points: ProfilePoint[] = [];

  for (let k = 0; k < n; k++) {
    const x = -lWe / 2 + (k + 0.5) * dx;
    const absX = Math.abs(x);
    const halfLwe = lWe / 2;

    let dz = 0;
    dz += computeCrownDz(rp.crown_type, x, lWe, rp.delta_c);

    // End relief (dub) - large end (positive x)
    if (x > 0) {
      const dubStart = halfLwe - rp.l_dub_l;
      if (x > dubStart && rp.l_dub_l > 0) {
        const t = (x - dubStart) / rp.l_dub_l;
        dz += rp.delta_dub_l * t * t;
      }
    }
    // End relief - small end (negative x)
    if (x < 0) {
      const dubStart = halfLwe - rp.l_dub_s;
      if (absX > dubStart && rp.l_dub_s > 0) {
        const t = (absX - dubStart) / rp.l_dub_s;
        dz += rp.delta_dub_s * t * t;
      }
    }

    points.push({ x, dz });
  }
  return points;
}

/** Crown correction using δ_c as master parameter.
 *  Type-specific parameters are derived from δ_c. */
function computeCrownDz(crown: CrownType, x: number, lWe: number, deltaC: number): number {
  const halfL = lWe / 2;
  const hl2 = halfL * halfL;

  if (deltaC <= 0 && !('Custom' in crown) && !('Polynomial' in crown)) return 0;

  if ('Parabolic' in crown) {
    const c2 = deltaC / hl2;
    return c2 * x * x;
  }
  if ('Circular' in crown) {
    const rCrown = hl2 / (2 * deltaC / 1000); // mm
    const x2 = x * x;
    const r2 = rCrown * rCrown;
    if (x2 >= r2) return deltaC;
    return (rCrown - Math.sqrt(r2 - x2)) * 1000;
  }
  if ('Logarithmic' in crown) {
    const refRatio = 0.9;
    const aLog = deltaC / Math.log(1 / (1 - refRatio * refRatio));
    const ratio2 = (x / halfL) ** 2;
    if (ratio2 >= 0.999) return deltaC;
    return aLog * Math.log(1 / (1 - ratio2));
  }
  if ('Custom' in crown) {
    const pts = crown.Custom.profile;
    if (pts.length < 2) return 0;
    for (let i = 0; i < pts.length - 1; i++) {
      const [x0, z0] = pts[i];
      const [x1, z1] = pts[i + 1];
      if (x >= x0 && x <= x1) {
        const t = (x - x0) / (x1 - x0);
        return z0 + t * (z1 - z0);
      }
    }
    return 0;
  }
  if ('Polynomial' in crown) {
    const c = crown.Polynomial.coeffs;
    const p1 = c[0] ?? 0, p2 = c[1] ?? 0, p3 = c[2] ?? 0, p4 = c[3] ?? 0, p5 = c[4] ?? 0;
    return ((((p1 * x + p2) * x + p3) * x + p4) * x + p5);
  }
  return 0;
}

function computeRacewayProfilePoints(rp: RacewayProfile, lWe: number, n: number): ProfilePoint[] {
  const dx = lWe / n;
  const points: ProfilePoint[] = [];

  for (let k = 0; k < n; k++) {
    const x = -lWe / 2 + (k + 0.5) * dx;
    let dz = 0;

    if (rp.custom_profile && rp.custom_profile.length >= 2) {
      const pts = rp.custom_profile;
      for (let i = 0; i < pts.length - 1; i++) {
        const [x0, z0] = pts[i];
        const [x1, z1] = pts[i + 1];
        if (x >= x0 && x <= x1) {
          const t = (x - x0) / (x1 - x0);
          dz = z0 + t * (z1 - z0);
          break;
        }
      }
    } else {
      dz = rp.delta_rw * (2 * x / lWe) * (2 * x / lWe);
    }

    // Polynomial profile overlay
    if (rp.polynomial_coeffs && rp.polynomial_coeffs.length > 0) {
      const c = rp.polynomial_coeffs;
      const p1 = c[0] ?? 0, p2 = c[1] ?? 0, p3 = c[2] ?? 0, p4 = c[3] ?? 0, p5 = c[4] ?? 0;
      dz += ((((p1 * x + p2) * x + p3) * x + p4) * x + p5);
    }

    points.push({ x, dz });
  }
  return points;
}

function computeProfiles(
  rp: RollerProfile,
  rpInner: RacewayProfile,
  rpOuter: RacewayProfile,
  mg: MacroGeometry,
  n: number,
): AllProfiles {
  const roller = computeRollerProfile(rp, mg.l_we, n);
  const inner = computeRacewayProfilePoints(rpInner, mg.l_we, n);
  const outer = computeRacewayProfilePoints(rpOuter, mg.l_we, n);

  const totalInner: ProfilePoint[] = roller.map((p, i) => ({
    x: p.x,
    dz: p.dz + inner[i].dz,
  }));
  const totalOuter: ProfilePoint[] = roller.map((p, i) => ({
    x: p.x,
    dz: p.dz + outer[i].dz,
  }));

  return { roller, inner, outer, totalInner, totalOuter };
}

// ─── SVG chart component with inline annotations ─────────────────

function ProfileChart({
  title,
  subtitle,
  data,
  lWe,
  color,
  fillColor,
  rollerProfile,
  racewayProfile,
  crownDrop,
  onContextMenu,
}: {
  title: string;
  subtitle?: string;
  data: ProfilePoint[];
  lWe: number;
  color: string;
  fillColor: string;
  rollerProfile?: RollerProfile;
  racewayProfile?: RacewayProfile;
  crownDrop?: number;
  onContextMenu?: (e: React.MouseEvent, title: string, data: ProfilePoint[]) => void;
}) {
  const margin = { top: 32, right: 45, bottom: 28, left: 50 };
  const width = 700;
  const height = 160;
  const innerW = width - margin.left - margin.right;
  const innerH = height - margin.top - margin.bottom;

  const dzValues = data.map(p => p.dz);
  const dzMin = Math.min(0, ...dzValues);
  const dzMax = Math.max(...dzValues, 0.01);
  const dzRange = dzMax - dzMin || 1;
  const dzPad = dzRange * 0.15;

  const scaleX = (x: number) => margin.left + ((x + lWe / 2) / lWe) * innerW;
  const scaleY = (dz: number) => margin.top + innerH - ((dz - (dzMin - dzPad)) / (dzRange + 2 * dzPad)) * innerH;

  // Build path
  const linePath = data.map((p, i) => `${i === 0 ? 'M' : 'L'}${scaleX(p.x).toFixed(2)},${scaleY(p.dz).toFixed(2)}`).join(' ');
  const areaPath = linePath
    + ` L${scaleX(data[data.length - 1].x).toFixed(2)},${scaleY(0).toFixed(2)}`
    + ` L${scaleX(data[0].x).toFixed(2)},${scaleY(0).toFixed(2)} Z`;

  // Y-axis ticks
  const nTicksY = 4;
  const yTicks: number[] = [];
  for (let i = 0; i <= nTicksY; i++) {
    yTicks.push(dzMin - dzPad + (dzRange + 2 * dzPad) * (i / nTicksY));
  }

  // X-axis ticks
  const nTicksX = 6;
  const xTicks: number[] = [];
  for (let i = 0; i <= nTicksX; i++) {
    xTicks.push(-lWe / 2 + (lWe * i) / nTicksX);
  }

  const halfL = lWe / 2;

  // Find center and edge values for crown drop dimension
  const centerIdx = Math.floor(data.length / 2);
  const edgeIdx = data.length - 1;
  const centerDz = data[centerIdx]?.dz ?? 0;
  const edgeDz = data[edgeIdx]?.dz ?? 0;

  return (
    <div className="bg-canvas rounded-lg border border-white/10 flex-1 min-h-0"
      onContextMenu={onContextMenu ? (e) => onContextMenu(e, title, data) : undefined}>
      <svg viewBox={`0 0 ${width} ${height}`} className="w-full h-full" preserveAspectRatio="xMidYMid meet">
        {/* Arrow marker */}
        <defs>
          <marker id="dimArrow" viewBox="0 0 6 3" refX="6" refY="1.5"
            markerWidth={4} markerHeight={3} orient="auto-start-reverse" fill="#94a3b8">
            <path d="M0,0 L6,1.5 L0,3 Z" />
          </marker>
        </defs>

        {/* Title */}
        <text x={width / 2} y={13} textAnchor="middle" fill="#e2e8f0" fontSize={11} fontWeight={600}>
          {title}
        </text>
        {/* Subtitle (formula / params) */}
        {subtitle && (
          <text x={width / 2} y={25} textAnchor="middle" fill="#64748b" fontSize={8}
            fontFamily="'JetBrains Mono', monospace">
            {subtitle}
          </text>
        )}

        {/* Grid */}
        {yTicks.map((v, i) => (
          <line key={`yg${i}`} x1={margin.left} y1={scaleY(v)} x2={width - margin.right} y2={scaleY(v)}
            stroke="#334155" strokeWidth={0.5} />
        ))}
        {xTicks.map((v, i) => (
          <line key={`xg${i}`} x1={scaleX(v)} y1={margin.top} x2={scaleX(v)} y2={height - margin.bottom}
            stroke="#334155" strokeWidth={0.5} />
        ))}

        {/* Zero line */}
        <line x1={margin.left} y1={scaleY(0)} x2={width - margin.right} y2={scaleY(0)}
          stroke="#64748b" strokeWidth={0.8} strokeDasharray="3 2" />

        {/* ── Roller Profile Annotations ── */}
        {rollerProfile && (
          <>
            {/* Dub-off zone: Small end */}
            {rollerProfile.l_dub_s > 0 && (
              <g>
                <rect x={scaleX(-halfL)} y={margin.top} width={scaleX(-halfL + rollerProfile.l_dub_s) - scaleX(-halfL)} height={innerH}
                  fill="#a855f7" fillOpacity={0.06} stroke="#a855f7" strokeWidth={0.5} strokeDasharray="2 1" />
                {/* Boundary line */}
                <line x1={scaleX(-halfL + rollerProfile.l_dub_s)} y1={margin.top}
                  x2={scaleX(-halfL + rollerProfile.l_dub_s)} y2={margin.top + innerH}
                  stroke="#a855f7" strokeWidth={0.5} strokeDasharray="2 1.5" />
                {/* Label at top */}
                <text x={(scaleX(-halfL) + scaleX(-halfL + rollerProfile.l_dub_s)) / 2} y={margin.top + 11}
                  textAnchor="middle" fill="#c084fc" fontSize={7.5} fontWeight={500}>
                  Dub-off S
                </text>
                <text x={(scaleX(-halfL) + scaleX(-halfL + rollerProfile.l_dub_s)) / 2} y={margin.top + 21}
                  textAnchor="middle" fill="#c084fc" fontSize={7}>
                  {rollerProfile.delta_dub_s}μm / {rollerProfile.l_dub_s}mm
                </text>
              </g>
            )}

            {/* Dub-off zone: Large end */}
            {rollerProfile.l_dub_l > 0 && (
              <g>
                <rect x={scaleX(halfL - rollerProfile.l_dub_l)} y={margin.top}
                  width={scaleX(halfL) - scaleX(halfL - rollerProfile.l_dub_l)} height={innerH}
                  fill="#a855f7" fillOpacity={0.06} stroke="#a855f7" strokeWidth={0.5} strokeDasharray="2 1" />
                <line x1={scaleX(halfL - rollerProfile.l_dub_l)} y1={margin.top}
                  x2={scaleX(halfL - rollerProfile.l_dub_l)} y2={margin.top + innerH}
                  stroke="#a855f7" strokeWidth={0.5} strokeDasharray="2 1.5" />
                <text x={(scaleX(halfL - rollerProfile.l_dub_l) + scaleX(halfL)) / 2} y={margin.top + 11}
                  textAnchor="middle" fill="#c084fc" fontSize={7.5} fontWeight={500}>
                  Dub-off L
                </text>
                <text x={(scaleX(halfL - rollerProfile.l_dub_l) + scaleX(halfL)) / 2} y={margin.top + 21}
                  textAnchor="middle" fill="#c084fc" fontSize={7}>
                  {rollerProfile.delta_dub_l}μm / {rollerProfile.l_dub_l}mm
                </text>
              </g>
            )}
          </>
        )}

        {/* ── Raceway Profile Annotations ── */}
        {racewayProfile && racewayProfile.delta_rw > 0 && (
          <g>
            {/* Vertical dimension at right edge showing δ_rw */}
            <line x1={width - margin.right + 4} y1={scaleY(0)}
              x2={width - margin.right + 4} y2={scaleY(racewayProfile.delta_rw)}
              stroke="#60a5fa" strokeWidth={0.7}
              markerStart="url(#dimArrow)" markerEnd="url(#dimArrow)" />
            <text x={width - margin.right + 8} y={(scaleY(0) + scaleY(racewayProfile.delta_rw)) / 2 + 3}
              textAnchor="start" fill="#60a5fa" fontSize={7.5} fontWeight={500}>
              δ_rw={racewayProfile.delta_rw}μm
            </text>
          </g>
        )}

        {/* Filled area */}
        <path d={areaPath} fill={fillColor} />

        {/* Profile line */}
        <path d={linePath} fill="none" stroke={color} strokeWidth={1.5} />

        {/* Data points */}
        {data.length <= 60 && data.map((p, i) => (
          <circle key={i} cx={scaleX(p.x)} cy={scaleY(p.dz)} r={1.5} fill={color} />
        ))}

        {/* ── Crown drop dimension (vertical arrow from center to edge level) ── */}
        {rollerProfile && crownDrop !== undefined && crownDrop > 0.01 && (
          <g>
            {/* Vertical dimension line at right side of chart */}
            <line x1={width - margin.right + 4} y1={scaleY(centerDz)}
              x2={width - margin.right + 4} y2={scaleY(edgeDz)}
              stroke="#fbbf24" strokeWidth={0.7}
              markerStart="url(#dimArrow)" markerEnd="url(#dimArrow)" />
            {/* Horizontal tick from center point to dimension */}
            <line x1={scaleX(0)} y1={scaleY(centerDz)}
              x2={width - margin.right + 4} y2={scaleY(centerDz)}
              stroke="#fbbf24" strokeWidth={0.3} strokeDasharray="2 1.5" />
            <line x1={scaleX(data[edgeIdx].x)} y1={scaleY(edgeDz)}
              x2={width - margin.right + 4} y2={scaleY(edgeDz)}
              stroke="#fbbf24" strokeWidth={0.3} strokeDasharray="2 1.5" />
            {/* Label */}
            <text x={width - margin.right + 8} y={(scaleY(centerDz) + scaleY(edgeDz)) / 2 + 3}
              textAnchor="start" fill="#fbbf24" fontSize={7.5} fontWeight={500}>
              Δ={crownDrop.toFixed(1)}μm
            </text>
            {/* R_sph label */}
            {rollerProfile.r_sph > 0 && (
              <text x={width - margin.right + 8} y={(scaleY(centerDz) + scaleY(edgeDz)) / 2 + 13}
                textAnchor="start" fill="#94a3b8" fontSize={7}>
                R_sph={rollerProfile.r_sph}mm
              </text>
            )}
          </g>
        )}

        {/* Y-axis labels */}
        {yTicks.map((v, i) => (
          <text key={`yl${i}`} x={margin.left - 4} y={scaleY(v) + 3} textAnchor="end" fill="#94a3b8" fontSize={8}>
            {v.toFixed(1)}
          </text>
        ))}

        {/* X-axis labels */}
        {xTicks.map((v, i) => (
          <text key={`xl${i}`} x={scaleX(v)} y={height - margin.bottom + 12} textAnchor="middle" fill="#94a3b8" fontSize={8}>
            {v.toFixed(1)}
          </text>
        ))}

        {/* Axis titles */}
        <text x={width / 2} y={height - 2} textAnchor="middle" fill="#94a3b8" fontSize={8}>
          Axial position [mm]
        </text>
        <text x={10} y={height / 2} textAnchor="middle" fill="#94a3b8" fontSize={8}
          transform={`rotate(-90,10,${height / 2})`}>
          {'\u0394'}z [{'\u03BC'}m]
        </text>
      </svg>
    </div>
  );
}
