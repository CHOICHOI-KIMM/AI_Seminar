import { useState } from 'react';
import Plot from './PlotWithCopy';
import { useAppState } from '../../store';
import { darkLayout, plotConfig, viridisScale } from './plotlyDefaults';
import type { BearingResult } from '../../types/bearing';

type CompView = 'load' | 'stress' | 'life';

const viewTabs: { key: CompView; label: string }[] = [
  { key: 'load', label: 'Load' },
  { key: 'stress', label: 'Stress' },
  { key: 'life', label: 'Life' },
];

export default function ComparisonChart() {
  const { state } = useAppState();
  const { dualResult } = state;
  const [view, setView] = useState<CompView>('load');

  if (!dualResult) {
    return (
      <div className="flex items-center justify-center h-full text-text-canvas text-sm">
        Run in Dual mode to compare Gen1 vs Gen3
      </div>
    );
  }

  const gen1 = dualResult.gen1_result;
  const gen3 = dualResult.gen3_result;

  return (
    <div className="w-full h-full flex flex-col">
      {/* Sub-tab bar */}
      <div className="flex items-center gap-1 px-3 pt-2 shrink-0">
        {viewTabs.map(t => (
          <button
            key={t.key}
            onClick={() => setView(t.key)}
            className={`px-2.5 py-1 text-[13px] font-medium rounded transition-colors cursor-pointer ${
              view === t.key
                ? 'bg-white/10 text-text-light'
                : 'text-text-canvas hover:text-text-light'
            }`}
          >
            {t.label}
          </button>
        ))}
        <span className="ml-auto text-xs font-mono text-text-canvas">
          Δp_max: {dualResult.delta_p_max_pct.toFixed(1)}% | ΔQ_max: {dualResult.delta_q_max_pct.toFixed(1)}% | ΔL₁₀: {dualResult.delta_l10_pct.toFixed(1)}%
        </span>
      </div>

      {/* Chart area */}
      <div className="flex-1 min-h-0">
        {view === 'load' && <LoadComparison gen1={gen1} gen3={gen3} />}
        {view === 'stress' && <StressComparison gen1={gen1} gen3={gen3} />}
        {view === 'life' && <LifeComparison gen1={gen1} gen3={gen3} />}
      </div>
    </div>
  );
}

// ── Load Comparison (existing polar overlay) ──
function LoadComparison({ gen1, gen3 }: { gen1: BearingResult; gen3: BearingResult }) {
  const psi = gen1.equilibrium.roller_results.map(r => r.psi_deg);
  const q1 = gen1.equilibrium.roller_results.map(r => r.q_normal / 1000);
  const q3 = gen3.equilibrium.roller_results.map(r => r.q_normal / 1000);

  const data: Plotly.Data[] = [
    {
      type: 'scatterpolar',
      r: [...q1, q1[0]],
      theta: [...psi, psi[0]],
      name: 'Gen1',
      line: { color: '#3b82f6', width: 2 },
      mode: 'lines',
      fill: 'toself',
      fillcolor: 'rgba(59,130,246,0.1)',
    },
    {
      type: 'scatterpolar',
      r: [...q3, q3[0]],
      theta: [...psi, psi[0]],
      name: 'Gen3',
      line: { color: '#22c55e', width: 2 },
      mode: 'lines',
      fill: 'toself',
      fillcolor: 'rgba(34,197,94,0.1)',
    },
  ];

  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: { text: 'Gen1 vs Gen3 — Load Distribution', font: { size: 15, color: '#e2e8f0' } },
    showlegend: true,
    legend: { x: 0.02, y: 0.98, font: { size: 12, color: '#94a3b8' } },
    polar: {
      bgcolor: 'transparent',
      angularaxis: {
        direction: 'clockwise',
        gridcolor: '#334155',
        linecolor: '#334155',
        tickfont: { size: 11, family: 'JetBrains Mono', color: '#94a3b8' },
      },
      radialaxis: {
        gridcolor: '#334155',
        linecolor: '#334155',
        tickfont: { size: 11, family: 'JetBrains Mono', color: '#94a3b8' },
        title: { text: 'Q [kN]', font: { size: 12, color: '#94a3b8' } },
      },
    },
    margin: { l: 40, r: 40, t: 50, b: 40 },
  };

  return (
    <Plot data={data} layout={layout} config={plotConfig}
      useResizeHandler style={{ width: '100%', height: '100%' }} />
  );
}

// ── Stress Contour Comparison (side-by-side heatmaps) ──
function StressComparison({ gen1, gen3 }: { gen1: BearingResult; gen3: BearingResult }) {
  const build = (result: BearingResult) => {
    const loaded = result.equilibrium.roller_results.filter(r => r.q_normal > 0);
    if (loaded.length === 0) return null;
    const nSlices = loaded[0].slice_results.length;
    return {
      psi: loaded.map(r => `${r.psi_deg.toFixed(0)}°`),
      slices: Array.from({ length: nSlices }, (_, k) => `${k + 1}`),
      z: loaded.map(r => r.slice_results.map(s => s.p_max_k)),
    };
  };

  const d1 = build(gen1);
  const d3 = build(gen3);
  if (!d1 || !d3) return null;

  // Find shared color range
  const allVals = [...d1.z.flat(), ...d3.z.flat()];
  const zMin = Math.min(...allVals);
  const zMax = Math.max(...allVals);

  const makeTrace = (d: NonNullable<ReturnType<typeof build>>, xaxis: string, yaxis: string): Plotly.Data => ({
    type: 'heatmap',
    z: d.z,
    x: d.slices,
    y: d.psi,
    colorscale: viridisScale,
    zmin: zMin,
    zmax: zMax,
    showscale: xaxis === 'x2', // colorbar only on right
    colorbar: {
      title: { text: 'MPa', font: { size: 12, color: '#94a3b8' } },
      tickfont: { size: 11, family: 'JetBrains Mono', color: '#94a3b8' },
      len: 0.8,
      thickness: 12,
    },
    xaxis,
    yaxis,
  } as Plotly.Data);

  const data: Plotly.Data[] = [
    makeTrace(d1, 'x', 'y'),
    makeTrace(d3, 'x2', 'y'),
  ];

  const axisStyle = {
    gridcolor: '#1e293b',
    zerolinecolor: '#334155',
    tickfont: { family: 'JetBrains Mono, monospace', size: 12, color: '#94a3b8' },
  };

  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    showlegend: false,
    annotations: [
      { x: 0.22, y: 1.06, xref: 'paper', yref: 'paper', text: 'Gen1', showarrow: false, font: { size: 14, color: '#3b82f6' } },
      { x: 0.78, y: 1.06, xref: 'paper', yref: 'paper', text: 'Gen3', showarrow: false, font: { size: 14, color: '#22c55e' } },
    ],
    xaxis: { ...axisStyle, domain: [0, 0.45], title: { text: 'Slice', font: { size: 12, color: '#94a3b8' } } },
    xaxis2: { ...axisStyle, domain: [0.55, 1], title: { text: 'Slice', font: { size: 12, color: '#94a3b8' } } },
    yaxis: { ...axisStyle, title: { text: 'Roller ψ', font: { size: 12, color: '#94a3b8' } } },
    margin: { l: 50, r: 20, t: 40, b: 40 },
  };

  return (
    <Plot data={data} layout={layout} config={plotConfig}
      useResizeHandler style={{ width: '100%', height: '100%' }} />
  );
}

// ── Life Comparison (grouped bar) ──
function LifeComparison({ gen1, gen3 }: { gen1: BearingResult; gen3: BearingResult }) {
  const categories = ['L₁₀ Inner', 'L₁₀ Outer', 'L₁₀ Combined', 'L_nm (hrs)'];
  const g1Vals = [gen1.life.l_10_inner, gen1.life.l_10_outer, gen1.life.l_10_basic, gen1.life.l_nm_hours];
  const g3Vals = [gen3.life.l_10_inner, gen3.life.l_10_outer, gen3.life.l_10_basic, gen3.life.l_nm_hours];

  const data: Plotly.Data[] = [
    {
      type: 'bar',
      name: 'Gen1',
      x: categories,
      y: g1Vals,
      marker: { color: '#3b82f6' },
      text: g1Vals.map(v => v.toFixed(1)),
      textposition: 'outside',
      textfont: { family: 'JetBrains Mono', size: 11, color: '#93c5fd' },
    },
    {
      type: 'bar',
      name: 'Gen3',
      x: categories,
      y: g3Vals,
      marker: { color: '#22c55e' },
      text: g3Vals.map(v => v.toFixed(1)),
      textposition: 'outside',
      textfont: { family: 'JetBrains Mono', size: 11, color: '#86efac' },
    },
  ];

  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: { text: 'Gen1 vs Gen3 — Fatigue Life', font: { size: 15, color: '#e2e8f0' } },
    barmode: 'group',
    showlegend: true,
    legend: { x: 0.02, y: 0.98, font: { size: 12, color: '#94a3b8' } },
    yaxis: {
      ...darkLayout.yaxis,
      title: { text: 'Life [10⁶ rev / hrs]', font: { size: 13, color: '#94a3b8' } },
      type: 'log',
    },
    margin: { l: 50, r: 20, t: 40, b: 60 },
  };

  return (
    <Plot data={data} layout={layout} config={plotConfig}
      useResizeHandler style={{ width: '100%', height: '100%' }} />
  );
}
