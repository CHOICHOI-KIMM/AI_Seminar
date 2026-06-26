import Plot from '../charts/PlotWithCopy';
import { darkLayout, plotConfig } from '../charts/plotlyDefaults';
import type { TransientResult } from '../../types/bearing';

interface Props {
  result: TransientResult;
}

export default function TransientTimeChart({ result }: Props) {
  const { snapshots } = result;
  if (snapshots.length === 0) return null;

  const t = snapshots.map(s => s.t_s);

  // Load traces: F_x, F_y, F_a from operating conditions
  const fx = snapshots.map(s => s.operating.f_x);
  const fy = snapshots.map(s => s.operating.f_y);
  const fa = snapshots.map(s => s.operating.f_a);
  const rpm = snapshots.map(s => Math.abs(s.operating.n_inner_rpm - s.operating.n_outer_rpm));

  // Sliding metrics (convert SRR to %)
  const maxSlip = snapshots.map(s => s.sliding_metrics.max_slip_ratio * 100);
  const nSlip = snapshots.map(s => s.sliding_metrics.n_rollers_in_slip);
  const fricPower = snapshots.map(s => s.sliding_metrics.instantaneous_friction_power);

  const loadTraces: Plotly.Data[] = [
    { x: t, y: fx, name: 'F_x', mode: 'lines', line: { color: '#3b82f6', width: 1.5 } },
    { x: t, y: fy, name: 'F_y', mode: 'lines', line: { color: '#10b981', width: 1.5 } },
    { x: t, y: fa, name: 'F_a', mode: 'lines', line: { color: '#f59e0b', width: 1.5 } },
  ];

  const rpmTrace: Plotly.Data[] = [
    { x: t, y: rpm, name: 'Speed', mode: 'lines', line: { color: '#a78bfa', width: 1.5 } },
  ];

  const slipTraces: Plotly.Data[] = [
    { x: t, y: maxSlip, name: 'Max SRR', mode: 'lines', line: { color: '#ef4444', width: 1.5 } },
  ];

  const slipCountTraces: Plotly.Data[] = [
    { x: t, y: nSlip, name: 'Rollers in slip', mode: 'lines', line: { color: '#f97316', width: 1.5 }, fill: 'tozeroy', fillcolor: 'rgba(249,115,22,0.1)' },
  ];

  const powerTraces: Plotly.Data[] = [
    { x: t, y: fricPower, name: 'Friction power', mode: 'lines', line: { color: '#ec4899', width: 1.5 }, fill: 'tozeroy', fillcolor: 'rgba(236,72,153,0.1)' },
  ];

  const commonLayout = {
    ...darkLayout,
    showlegend: true,
    legend: { font: { size: 9, color: '#94a3b8' }, orientation: 'h' as const, y: 1.12 },
    margin: { l: 50, r: 15, t: 10, b: 30 },
    height: 160,
  };

  return (
    <div className="space-y-2">
      <ChartLabel>Applied Loads [kN]</ChartLabel>
      <Plot
        data={loadTraces}
        layout={{ ...commonLayout, xaxis: { ...darkLayout.xaxis, title: '' }, yaxis: { ...darkLayout.yaxis, title: { text: 'kN', font: { size: 10, color: '#94a3b8' } } } }}
        config={plotConfig}
        className="w-full"
      />

      <ChartLabel>Speed [rpm]</ChartLabel>
      <Plot
        data={rpmTrace}
        layout={{ ...commonLayout, xaxis: { ...darkLayout.xaxis, title: '' }, yaxis: { ...darkLayout.yaxis, title: { text: 'rpm', font: { size: 10, color: '#94a3b8' } } } }}
        config={plotConfig}
        className="w-full"
      />

      <ChartLabel>Max Slip Ratio (SRR) [%]</ChartLabel>
      <Plot
        data={slipTraces}
        layout={{ ...commonLayout, xaxis: { ...darkLayout.xaxis, title: '' }, yaxis: { ...darkLayout.yaxis, title: { text: '%', font: { size: 10, color: '#94a3b8' } } } }}
        config={plotConfig}
        className="w-full"
      />

      <ChartLabel>Rollers in Slip</ChartLabel>
      <Plot
        data={slipCountTraces}
        layout={{ ...commonLayout, xaxis: { ...darkLayout.xaxis, title: '' }, yaxis: { ...darkLayout.yaxis, title: { text: 'count', font: { size: 10, color: '#94a3b8' } } } }}
        config={plotConfig}
        className="w-full"
      />

      <ChartLabel>Instantaneous Friction Power [W]</ChartLabel>
      <Plot
        data={powerTraces}
        layout={{ ...commonLayout, xaxis: { ...darkLayout.xaxis, title: { text: 'Time [s]', font: { size: 10, color: '#94a3b8' } } }, yaxis: { ...darkLayout.yaxis, title: { text: 'W', font: { size: 10, color: '#94a3b8' } } } }}
        config={plotConfig}
        className="w-full"
      />
    </div>
  );
}

function ChartLabel({ children }: { children: React.ReactNode }) {
  return <p className="text-xs font-semibold text-text-canvas uppercase tracking-wider pt-2">{children}</p>;
}
