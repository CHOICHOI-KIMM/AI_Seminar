// @ts-nocheck
// CRB Phase 1.4 stub: TRB 잔재 unused var → Phase 6 재작성 예정
import Plot from './PlotWithCopy';
import { darkLayout, plotConfig } from './plotlyDefaults';
import type { RollerResult, SliceGeometry } from '../../types/bearing';

interface RollerDetailChartProps {
  roller: RollerResult;
  sliceGeometries: SliceGeometry[];
  modeLabel: string;
}

/**
 * 방안 1: Single roller slice-level distribution charts
 * Shows p_max, b_k, q_k, delta_k across slices for the selected roller.
 */
export default function RollerDetailChart({ roller, sliceGeometries, modeLabel }: RollerDetailChartProps) {
  const slices = roller.slice_results;
  const xPositions = sliceGeometries.map(sg => sg.x_axial);
  const xLabels = xPositions.map(x => x.toFixed(2));

  const subplots: { title: string; yLabel: string; traces: Plotly.Data[] }[] = [
    {
      title: 'Contact Stress p_max',
      yLabel: 'p_max [MPa]',
      traces: [
        {
          type: 'scatter',
          x: xLabels,
          y: slices.map(s => s.p_max_k),
          mode: 'lines+markers',
          name: 'Inner',
          line: { color: '#60a5fa', width: 2 },
          marker: { size: 3 },
        },
        {
          type: 'scatter',
          x: xLabels,
          y: slices.map(s => s.p_max_k_outer),
          mode: 'lines+markers',
          name: 'Outer',
          line: { color: '#fbbf24', width: 2 },
          marker: { size: 3 },
        },
      ],
    },
    {
      title: 'Contact Half-Width b_k',
      yLabel: 'b [mm]',
      traces: [
        {
          type: 'scatter',
          x: xLabels,
          y: slices.map(s => s.b_k),
          mode: 'lines+markers',
          name: 'Inner',
          line: { color: '#60a5fa', width: 2 },
          marker: { size: 3 },
        },
        {
          type: 'scatter',
          x: xLabels,
          y: slices.map(s => s.b_k_outer),
          mode: 'lines+markers',
          name: 'Outer',
          line: { color: '#fbbf24', width: 2 },
          marker: { size: 3 },
        },
      ],
    },
    {
      title: 'Line Load q_k',
      yLabel: 'q [N/mm]',
      traces: [
        {
          type: 'scatter',
          x: xLabels,
          y: slices.map(s => s.q_k_inner),
          mode: 'lines+markers',
          name: 'Inner',
          line: { color: '#60a5fa', width: 2 },
          marker: { size: 3 },
        },
        {
          type: 'scatter',
          x: xLabels,
          y: slices.map(s => s.q_k_outer),
          mode: 'lines+markers',
          name: 'Outer',
          line: { color: '#fbbf24', width: 2 },
          marker: { size: 3 },
        },
      ],
    },
    {
      title: 'Bulk Deformation h_bulk',
      yLabel: 'h_bulk [μm]',
      traces: [
        {
          type: 'scatter',
          x: xLabels,
          y: slices.map(s => s.h_bulk_k),
          mode: 'lines+markers',
          name: 'Inner',
          line: { color: '#60a5fa', width: 2 },
          marker: { size: 3 },
        },
        {
          type: 'scatter',
          x: xLabels,
          y: slices.map(s => s.h_bulk_k_outer),
          mode: 'lines+markers',
          name: 'Outer',
          line: { color: '#fbbf24', width: 2 },
          marker: { size: 3 },
        },
      ],
    },
  ];

  return (
    <div className="grid grid-cols-2 gap-1 w-full h-full p-1">
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
              showlegend: sp.traces.length > 1,
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
  );
}
