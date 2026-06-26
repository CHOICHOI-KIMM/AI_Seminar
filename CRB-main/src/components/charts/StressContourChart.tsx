import { useState } from 'react';
import Plot from './PlotWithCopy';
import { useActiveResult } from '../../hooks/useActiveResult';
import { useAppState } from '../../store';
import { darkLayout, plotConfig, viridisScale } from './plotlyDefaults';
import RollerDetailChart from './RollerDetailChart';
import ContactPatchChart from './ContactPatchChart';
import RollerComparisonChart from './RollerComparisonChart';
import RibContactDetailChart from './RibContactDetailChart';
import type { BearingResult } from '../../types/bearing';

type RacewayView = 'inner' | 'outer' | 'rib';
type RollerViewMode = 'distributions' | 'patch' | 'comparison';

export default function StressContourChart() {
  const result = useActiveResult();
  const { state } = useAppState();
  const [raceway, setRaceway] = useState<RacewayView>('inner');
  const [selectedRollerIdx, setSelectedRollerIdx] = useState<number | null>(null);
  const [rollerViewMode, setRollerViewMode] = useState<RollerViewMode>('distributions');

  if (!result) return null;

  const modeLabel = result.mode === 'Gen1' ? 'Gen1' : 'Gen3';
  const racewayLabel = raceway === 'inner' ? 'Inner' : raceway === 'outer' ? 'Outer' : 'Rib';
  const rollers = result.equilibrium.roller_results;
  const loadedRollers = rollers.filter(r => r.q_normal > 0);

  // Dual mode data for Gen1/Gen3 comparison
  const dualResult = state.dualResult;
  const hasDual = !!dualResult;

  // Find matching gen1/gen3 rollers for comparison
  const getComparisonRollers = (idx: number) => {
    if (!dualResult) return null;
    const g1Rollers = dualResult.gen1_result.equilibrium.roller_results;
    const g3Rollers = dualResult.gen3_result.equilibrium.roller_results;
    if (idx < g1Rollers.length && idx < g3Rollers.length) {
      return { gen1: g1Rollers[idx], gen3: g3Rollers[idx] };
    }
    return null;
  };

  const selectedRoller = selectedRollerIdx !== null ? rollers[selectedRollerIdx] : null;

  // Toggle buttons
  const RacewayToggle = () => (
    <div className="flex justify-center gap-1 mb-1">
      <button
        onClick={() => setRaceway('inner')}
        className={`px-3 py-1 text-[13px] rounded-l transition-colors ${
          raceway === 'inner'
            ? 'bg-blue-600 text-white'
            : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
        }`}
      >
        Inner Raceway
      </button>
      <button
        onClick={() => setRaceway('outer')}
        className={`px-3 py-1 text-xs transition-colors ${
          raceway === 'outer'
            ? 'bg-blue-600 text-white'
            : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
        }`}
      >
        Outer Raceway
      </button>
      <button
        onClick={() => setRaceway('rib')}
        className={`px-3 py-1 text-[13px] rounded-r transition-colors ${
          raceway === 'rib'
            ? 'bg-amber-600 text-white'
            : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
        }`}
      >
        Rib Contact
      </button>
    </div>
  );

  // Roller selector component
  const RollerSelector = () => (
    <div className="flex items-center gap-2 px-3 py-1 border-b border-white/10 shrink-0">
      <label className="text-[13px] text-slate-400">Roller:</label>
      <select
        value={selectedRollerIdx ?? ''}
        onChange={e => {
          const v = e.target.value;
          setSelectedRollerIdx(v === '' ? null : Number(v));
        }}
        className="bg-slate-800 text-slate-200 text-[13px] px-2 py-1 rounded border border-white/10 focus:outline-none focus:border-blue-500"
      >
        <option value="">Bearing Overview</option>
        {rollers.map((r, i) => (
          r.q_normal > 0 && (
            <option key={i} value={i}>
              #{i + 1} — ψ={r.psi_deg.toFixed(1)}° — Q={r.q_normal.toFixed(0)} N
            </option>
          )
        ))}
      </select>

      {selectedRollerIdx !== null && (
        <>
          <div className="h-4 w-px bg-white/10" />
          <div className="flex gap-0.5">
            <button
              onClick={() => setRollerViewMode('distributions')}
              className={`px-2 py-0.5 text-xs rounded transition-colors ${
                rollerViewMode === 'distributions'
                  ? 'bg-emerald-600 text-white'
                  : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
              }`}
            >
              Distributions
            </button>
            <button
              onClick={() => setRollerViewMode('patch')}
              className={`px-2 py-0.5 text-xs rounded transition-colors ${
                rollerViewMode === 'patch'
                  ? 'bg-emerald-600 text-white'
                  : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
              }`}
            >
              Contact Patch
            </button>
            {hasDual && (
              <button
                onClick={() => setRollerViewMode('comparison')}
                className={`px-2 py-0.5 text-xs rounded transition-colors ${
                  rollerViewMode === 'comparison'
                    ? 'bg-pink-600 text-white'
                    : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
                }`}
              >
                Gen1 vs Gen3
              </button>
            )}
          </div>
          <button
            onClick={() => setSelectedRollerIdx(null)}
            className="ml-auto px-2 py-0.5 text-xs text-slate-400 hover:text-white transition-colors"
          >
            ✕ Close
          </button>
        </>
      )}
    </div>
  );

  // ── Single Roller Detail Views ──────────────────────────────────
  if (selectedRollerIdx !== null && selectedRoller) {
    const sliceGeometries = result.geometry.slice_geometries;

    return (
      <div className="w-full h-full flex flex-col">
        <RollerSelector />
        <div className="flex-1 min-h-0">
          {rollerViewMode === 'distributions' && (
            <RollerDetailChart
              roller={selectedRoller}
              sliceGeometries={sliceGeometries}
              modeLabel={modeLabel}
            />
          )}
          {rollerViewMode === 'patch' && (
            <div className="w-full h-full flex flex-col">
              <RacewayToggle />
              <div className="flex-1 min-h-0">
                {raceway === 'rib' ? (
                  <RibContactDetailChart key="rib" roller={selectedRoller} />
                ) : (
                  <ContactPatchChart
                    key={`patch-${raceway}`}
                    roller={selectedRoller}
                    sliceGeometries={sliceGeometries}
                    raceway={raceway}
                  />
                )}
              </div>
            </div>
          )}
          {rollerViewMode === 'comparison' && hasDual && (() => {
            const pair = getComparisonRollers(selectedRollerIdx);
            if (!pair) return (
              <div className="flex items-center justify-center h-full text-text-canvas text-sm">
                No matching roller data for comparison
              </div>
            );
            return (
              <RollerComparisonChart
                gen1Roller={pair.gen1}
                gen3Roller={pair.gen3}
                sliceGeometries={sliceGeometries}
              />
            );
          })()}
        </div>
      </div>
    );
  }

  // ── Bearing Overview (Original Contour) ─────────────────────────

  // Rib contact view
  if (raceway === 'rib') {
    return (
      <div className="w-full h-full flex flex-col">
        <RollerSelector />
        <RibContactView result={result} modeLabel={modeLabel} RacewayToggle={RacewayToggle} />
      </div>
    );
  }

  const angDist = result.equilibrium.angular_distribution;

  // angular_distribution now includes split-interpolated data from Rust backend
  // (when split is active, all loaded points have interpolated p_max from roller_results)
  if (angDist && angDist.length > 0) {
    const loaded = angDist.filter(p => p.q_total > 0);
    if (loaded.length === 0) return <Empty />;

    const nSlices = loaded[0].slice_p_max.length;
    const psiLabels = loaded.map(p => `${p.psi_deg.toFixed(1)}°`);
    const sliceLabels = Array.from({ length: nSlices }, (_, k) => `${k + 1}`);

    const z = loaded.map(p =>
      raceway === 'inner' ? p.slice_p_max : (p.slice_p_max_outer ?? p.slice_p_max)
    );

    const data: Plotly.Data[] = [
      {
        type: 'heatmap',
        z,
        x: sliceLabels,
        y: psiLabels,
        colorscale: viridisScale,
        colorbar: {
          title: { text: 'MPa', font: { size: 12, color: '#94a3b8' } },
          tickfont: { size: 11, family: 'JetBrains Mono', color: '#94a3b8' },
          len: 0.8,
          thickness: 12,
        },
        zsmooth: 'best' as const,
      },
    ];

    const layout: Partial<Plotly.Layout> = {
      ...darkLayout,
      title: {
        text: `Contact Stress — ${racewayLabel} p_max (${modeLabel})`,
        font: { size: 15, color: '#e2e8f0' },
      },
      xaxis: {
        ...darkLayout.xaxis,
        title: { text: 'Slice', font: { size: 13, color: '#94a3b8' } },
      },
      yaxis: {
        ...darkLayout.yaxis,
        title: { text: 'ψ [deg]', font: { size: 13, color: '#94a3b8' } },
      },
    };

    return (
      <div className="w-full h-full flex flex-col">
        <RollerSelector />
        <RacewayToggle />
        <div className="flex-1">
          <Plot
            data={data}
            layout={layout}
            config={plotConfig}
            useResizeHandler
            style={{ width: '100%', height: '100%' }}
          />
        </div>
      </div>
    );
  }

  // Fallback: use roller_results
  if (loadedRollers.length === 0) return <Empty />;

  const nSlices = loadedRollers[0].slice_results.length;
  const psiLabels = loadedRollers.map(r => `${r.psi_deg.toFixed(0)}°`);
  const sliceLabels = Array.from({ length: nSlices }, (_, k) => `${k + 1}`);

  const z = loadedRollers.map(r =>
    r.slice_results.map(s => raceway === 'inner' ? s.p_max_k : (s.p_max_k_outer ?? s.p_max_k))
  );

  const data: Plotly.Data[] = [
    {
      type: 'heatmap',
      z,
      x: sliceLabels,
      y: psiLabels,
      colorscale: viridisScale,
      colorbar: {
        title: { text: 'MPa', font: { size: 10, color: '#94a3b8' } },
        tickfont: { size: 9, family: 'JetBrains Mono', color: '#94a3b8' },
        len: 0.8,
        thickness: 12,
      },
    },
  ];

  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: {
      text: `Contact Stress — ${racewayLabel} p_max (${modeLabel})`,
      font: { size: 13, color: '#e2e8f0' },
    },
    xaxis: {
      ...darkLayout.xaxis,
      title: { text: 'Slice', font: { size: 11, color: '#94a3b8' } },
    },
    yaxis: {
      ...darkLayout.yaxis,
      title: { text: 'Roller ψ', font: { size: 13, color: '#94a3b8' } },
    },
  };

  return (
    <div className="w-full h-full flex flex-col">
      <RollerSelector />
      <RacewayToggle />
      <div className="flex-1">
        <Plot
          data={data}
          layout={layout}
          config={plotConfig}
          useResizeHandler
          style={{ width: '100%', height: '100%' }}
        />
      </div>
    </div>
  );
}

function Empty() {
  return (
    <div className="flex items-center justify-center h-full text-text-canvas text-sm">
      No loaded rollers
    </div>
  );
}

function RibContactView({
  result,
  modeLabel,
  RacewayToggle,
}: {
  result: BearingResult;
  modeLabel: string;
  RacewayToggle: React.FC;
}) {
  const rollers = result.equilibrium.roller_results;
  const ribRollers = rollers.filter(r => r.rib_result && r.rib_result.p_max_rib > 0);

  if (ribRollers.length === 0) {
    return (
      <div className="w-full h-full flex flex-col">
        <RacewayToggle />
        <div className="flex items-center justify-center flex-1 text-text-canvas text-sm">
          No rib contact detected
        </div>
      </div>
    );
  }

  // Find max-loaded roller for ellipse display
  const maxRib = ribRollers.reduce((best, r) =>
    r.rib_result!.p_max_rib > best.rib_result!.p_max_rib ? r : best
  );
  const maxRibRes = maxRib.rib_result!;

  // Bar chart data — all rollers
  const psiLabels = rollers.map(r => `${r.psi_deg.toFixed(0)}°`);
  const pMaxValues = rollers.map(r => r.rib_result?.p_max_rib ?? 0);
  const fRibValues = rollers.map(r => r.rib_result?.f_rib ?? 0);
  const aValues = rollers.map(r => r.rib_result?.a_ellipse ?? 0);
  const bValues = rollers.map(r => r.rib_result?.b_ellipse ?? 0);
  const spinValues = rollers.map(r => r.rib_result?.spin_moment ?? 0);

  const pMaxMax = Math.max(...pMaxValues, 1);

  // Color: stress ratio mapped to blue→yellow→red
  const colors = pMaxValues.map(p => {
    const t = p / pMaxMax;
    if (t < 0.5) return `rgb(${Math.round(t * 2 * 255)}, ${Math.round(180 + t * 150)}, ${Math.round(255 - t * 2 * 255)})`;
    return `rgb(255, ${Math.round(255 - (t - 0.5) * 2 * 255)}, 0)`;
  });

  const barData: Plotly.Data[] = [
    {
      type: 'bar',
      x: psiLabels,
      y: pMaxValues,
      marker: { color: colors },
      hovertemplate: psiLabels.map((_, i) =>
        `ψ = ${psiLabels[i]}<br>` +
        `p_max = ${pMaxValues[i].toFixed(0)} MPa<br>` +
        `F_rib = ${fRibValues[i].toFixed(1)} N<br>` +
        `a = ${aValues[i].toFixed(4)} mm<br>` +
        `b = ${bValues[i].toFixed(4)} mm<br>` +
        `Spin M = ${spinValues[i].toFixed(2)} N·mm<extra></extra>`
      ),
    },
  ];

  const barLayout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: {
      text: `Rib Contact Stress — p_max (${modeLabel})`,
      font: { size: 13, color: '#e2e8f0' },
    },
    xaxis: {
      ...darkLayout.xaxis,
      title: { text: 'Roller position ψ', font: { size: 13, color: '#94a3b8' } },
    },
    yaxis: {
      ...darkLayout.yaxis,
      title: { text: 'p_max [MPa]', font: { size: 13, color: '#94a3b8' } },
    },
    margin: { l: 60, r: 20, t: 35, b: 50 },
  };

  // Contact ellipse SVG for max-loaded roller
  // SVG x = circumferential (b), y = meridional (a) — matches individual roller view
  const ellipseScale = 120;
  const bScaled = Math.max(maxRibRes.b_ellipse * ellipseScale, 2);
  const aScaled = Math.max(maxRibRes.a_ellipse * ellipseScale, 2);
  const svgW = 200;
  const svgH = 140;
  const cx = svgW / 2;
  const cy = svgH / 2;

  return (
    <div className="w-full h-full flex flex-col">
      <RacewayToggle />
      <div className="flex-1 flex min-h-0">
        {/* Bar chart */}
        <div className="flex-1 min-w-0">
          <Plot
            data={barData}
            layout={barLayout}
            config={plotConfig}
            useResizeHandler
            style={{ width: '100%', height: '100%' }}
          />
        </div>

        {/* Contact ellipse panel */}
        <div className="w-52 shrink-0 flex flex-col items-center justify-center border-l border-white/10 px-3 gap-2">
          <div className="text-[13px] text-slate-400 font-medium">
            Contact Ellipse (#{rollers.indexOf(maxRib) + 1}, ψ={maxRib.psi_deg.toFixed(0)}°)
          </div>
          <svg width={svgW} height={svgH} className="shrink-0">
            {/* Grid lines */}
            <line x1={0} y1={cy} x2={svgW} y2={cy} stroke="#334155" strokeWidth={0.5} />
            <line x1={cx} y1={0} x2={cx} y2={svgH} stroke="#334155" strokeWidth={0.5} />
            {/* Pressure gradient ellipse (filled rings) */}
            {[1.0, 0.75, 0.5, 0.25].map((f, i) => (
              <ellipse
                key={i}
                cx={cx}
                cy={cy}
                rx={bScaled * f}
                ry={aScaled * f}
                fill={`rgba(${Math.round(255 * (1 - f * 0.3))}, ${Math.round(100 * f)}, ${Math.round(50 * f)}, ${0.15 + f * 0.2})`}
                stroke={f === 1 ? '#f59e0b' : '#f59e0b44'}
                strokeWidth={f === 1 ? 1.5 : 0.5}
              />
            ))}
            {/* Dimension lines */}
            <line x1={cx - bScaled} y1={cy + aScaled + 12} x2={cx + bScaled} y2={cy + aScaled + 12} stroke="#94a3b8" strokeWidth={0.8} markerStart="url(#arrowL)" markerEnd="url(#arrowR)" />
            <line x1={cx + bScaled + 12} y1={cy - aScaled} x2={cx + bScaled + 12} y2={cy + aScaled} stroke="#94a3b8" strokeWidth={0.8} markerStart="url(#arrowL)" markerEnd="url(#arrowR)" />
            <text x={cx} y={cy + aScaled + 24} textAnchor="middle" fill="#e2e8f0" fontSize={11} fontFamily="JetBrains Mono">
              2b = {(maxRibRes.b_ellipse * 2).toFixed(4)} mm (circ.)
            </text>
            <text x={cx + bScaled + 16} y={cy + 3} textAnchor="start" fill="#e2e8f0" fontSize={11} fontFamily="JetBrains Mono">
              2a = {(maxRibRes.a_ellipse * 2).toFixed(4)} mm (merid.)
            </text>
            <defs>
              <marker id="arrowL" markerWidth="6" markerHeight="6" refX="6" refY="3" orient="auto">
                <path d="M6,0 L0,3 L6,6" fill="none" stroke="#94a3b8" strokeWidth="0.8" />
              </marker>
              <marker id="arrowR" markerWidth="6" markerHeight="6" refX="0" refY="3" orient="auto">
                <path d="M0,0 L6,3 L0,6" fill="none" stroke="#94a3b8" strokeWidth="0.8" />
              </marker>
            </defs>
          </svg>
          {/* Summary table */}
          <table className="text-xs text-slate-300 w-full">
            <tbody>
              <tr><td className="text-slate-500 pr-2">p_max</td><td className="text-right font-mono">{maxRibRes.p_max_rib.toFixed(0)} MPa</td></tr>
              <tr><td className="text-slate-500 pr-2">F_rib</td><td className="text-right font-mono">{maxRibRes.f_rib.toFixed(1)} N</td></tr>
              <tr><td className="text-slate-500 pr-2">a / b</td><td className="text-right font-mono">{(maxRibRes.a_ellipse / maxRibRes.b_ellipse).toFixed(3)}</td></tr>
              <tr><td className="text-slate-500 pr-2">Spin M</td><td className="text-right font-mono">{maxRibRes.spin_moment.toFixed(2)} N·mm</td></tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
