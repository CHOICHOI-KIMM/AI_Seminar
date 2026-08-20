// @ts-nocheck
// CRB Phase 1.4 stub: TRB 잔재 unused var → Phase 6 재작성 예정
import Plot from './PlotWithCopy';
import { darkLayout, plotConfig } from './plotlyDefaults';
import type { RollerResult, SliceGeometry } from '../../types/bearing';

interface RollerComparisonChartProps {
  gen1Roller: RollerResult;
  gen3Roller: RollerResult;
  sliceGeometries: SliceGeometry[];
}

/**
 * 방안 3: Gen1 vs Gen3 single roller comparison overlay
 * Overlays Gen1 (independent slice) vs Gen3 (beam-coupled) results for the same roller.
 */
export default function RollerComparisonChart({ gen1Roller, gen3Roller, sliceGeometries }: RollerComparisonChartProps) {
  const g1 = gen1Roller.slice_results;
  const g3 = gen3Roller.slice_results;
  const xPos = sliceGeometries.map(sg => sg.x_axial.toFixed(2));

  const subplots: { title: string; yLabel: string; traces: Plotly.Data[] }[] = [
    {
      title: 'p_max Inner — Gen1 vs Gen3',
      yLabel: 'p_max [MPa]',
      traces: [
        {
          type: 'scatter', x: xPos, y: g1.map(s => s.p_max_k),
          mode: 'lines+markers', name: 'Gen1',
          line: { color: '#60a5fa', width: 2, dash: 'dash' }, marker: { size: 3 },
        },
        {
          type: 'scatter', x: xPos, y: g3.map(s => s.p_max_k),
          mode: 'lines+markers', name: 'Gen3',
          line: { color: '#f472b6', width: 2 }, marker: { size: 3 },
        },
      ],
    },
    {
      title: 'p_max Outer — Gen1 vs Gen3',
      yLabel: 'p_max [MPa]',
      traces: [
        {
          type: 'scatter', x: xPos, y: g1.map(s => s.p_max_k_outer),
          mode: 'lines+markers', name: 'Gen1',
          line: { color: '#60a5fa', width: 2, dash: 'dash' }, marker: { size: 3 },
        },
        {
          type: 'scatter', x: xPos, y: g3.map(s => s.p_max_k_outer),
          mode: 'lines+markers', name: 'Gen3',
          line: { color: '#f472b6', width: 2 }, marker: { size: 3 },
        },
      ],
    },
    {
      title: 'Line Load q_k — Gen1 vs Gen3',
      yLabel: 'q [N/mm]',
      traces: [
        {
          type: 'scatter', x: xPos, y: g1.map(s => s.q_k),
          mode: 'lines+markers', name: 'Gen1',
          line: { color: '#60a5fa', width: 2, dash: 'dash' }, marker: { size: 3 },
        },
        {
          type: 'scatter', x: xPos, y: g3.map(s => s.q_k),
          mode: 'lines+markers', name: 'Gen3',
          line: { color: '#f472b6', width: 2 }, marker: { size: 3 },
        },
      ],
    },
    {
      title: 'Approach δ_k — Gen1 vs Gen3',
      yLabel: 'δ [μm]',
      traces: [
        {
          type: 'scatter', x: xPos, y: g1.map(s => s.delta_k),
          mode: 'lines+markers', name: 'Gen1',
          line: { color: '#60a5fa', width: 2, dash: 'dash' }, marker: { size: 3 },
        },
        {
          type: 'scatter', x: xPos, y: g3.map(s => s.delta_k),
          mode: 'lines+markers', name: 'Gen3',
          line: { color: '#f472b6', width: 2 }, marker: { size: 3 },
        },
      ],
    },
  ];

  // Summary statistics
  const g1QTotal = g1.reduce((sum, s) => sum + s.q_k * (sliceGeometries[0]?.slice_width ?? 1), 0);
  const g3QTotal = g3.reduce((sum, s) => sum + s.q_k * (sliceGeometries[0]?.slice_width ?? 1), 0);
  const g1PMaxI = Math.max(...g1.map(s => s.p_max_k));
  const g3PMaxI = Math.max(...g3.map(s => s.p_max_k));
  const g1PMaxO = Math.max(...g1.map(s => s.p_max_k_outer));
  const g3PMaxO = Math.max(...g3.map(s => s.p_max_k_outer));

  const pctDiff = (a: number, b: number) => b !== 0 ? ((a - b) / b * 100).toFixed(1) : '-';

  return (
    <div className="flex flex-col w-full h-full">
      {/* Summary bar */}
      <div className="flex items-center gap-4 px-3 py-1.5 text-xs font-mono border-b border-white/10 shrink-0">
        <span className="text-slate-400">ψ = {gen1Roller.psi_deg.toFixed(1)}°</span>
        <span className="text-slate-400">
          Q: <span className="text-blue-400">{gen1Roller.q_normal.toFixed(0)}</span>
          {' → '}
          <span className="text-pink-400">{gen3Roller.q_normal.toFixed(0)}</span> N
          <span className="text-slate-500"> ({pctDiff(gen3Roller.q_normal, gen1Roller.q_normal)}%)</span>
        </span>
        <span className="text-slate-400">
          p_max,i: <span className="text-blue-400">{g1PMaxI.toFixed(0)}</span>
          {' → '}
          <span className="text-pink-400">{g3PMaxI.toFixed(0)}</span> MPa
          <span className="text-slate-500"> ({pctDiff(g3PMaxI, g1PMaxI)}%)</span>
        </span>
        <span className="text-slate-400">
          p_max,o: <span className="text-blue-400">{g1PMaxO.toFixed(0)}</span>
          {' → '}
          <span className="text-pink-400">{g3PMaxO.toFixed(0)}</span> MPa
          <span className="text-slate-500"> ({pctDiff(g3PMaxO, g1PMaxO)}%)</span>
        </span>
      </div>

      {/* 2x2 chart grid */}
      <div className="grid grid-cols-2 gap-1 flex-1 min-h-0 p-1">
        {subplots.map((sp, i) => (
          <div key={i} className="min-h-0">
            <Plot
              data={sp.traces}
              layout={{
                ...darkLayout,
                title: { text: sp.title, font: { size: 13, color: '#e2e8f0' } },
                xaxis: {
                  ...darkLayout.xaxis,
                  title: { text: 'Axial position [mm]', font: { size: 11, color: '#94a3b8' } },
                  tickfont: { size: 10, family: 'JetBrains Mono' },
                },
                yaxis: {
                  ...darkLayout.yaxis,
                  title: { text: sp.yLabel, font: { size: 11, color: '#94a3b8' } },
                  tickfont: { size: 10, family: 'JetBrains Mono' },
                },
                margin: { l: 55, r: 10, t: 28, b: 35 },
                showlegend: true,
                legend: {
                  x: 1, y: 1, xanchor: 'right',
                  font: { size: 11, color: '#94a3b8' },
                  bgcolor: 'rgba(0,0,0,0.3)',
                },
              }}
              config={plotConfig}
              useResizeHandler
              style={{ width: '100%', height: '100%' }}
            />
          </div>
        ))}
      </div>
    </div>
  );
}
