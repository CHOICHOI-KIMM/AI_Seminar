import { useState, useMemo } from 'react';
import Plot from '../charts/PlotWithCopy';
import { darkLayout, plotConfig } from '../charts/plotlyDefaults';
import type { TransientResult } from '../../types/bearing';

interface Props {
  result: TransientResult;
}

// Color palette for roller traces
const ROLLER_COLORS = [
  '#3b82f6', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6',
  '#ec4899', '#06b6d4', '#f97316', '#84cc16', '#6366f1',
  '#14b8a6', '#e11d48', '#a855f7', '#0ea5e9', '#eab308',
];

export default function RollerDynamicsChart({ result }: Props) {
  const { snapshots } = result;
  if (snapshots.length === 0) return null;

  const nRollers = snapshots[0].roller_kinematics.length;
  const [selectedRollers, setSelectedRollers] = useState<Set<number>>(() => {
    // Auto-select rollers that had slip events
    const slipRollers = new Set<number>();
    for (const snap of snapshots) {
      for (const rk of snap.roller_kinematics) {
        if (rk.in_slip) slipRollers.add(rk.j);
      }
    }
    return slipRollers.size > 0 ? slipRollers : new Set([0]);
  });

  const toggleRoller = (j: number) => {
    setSelectedRollers(prev => {
      const next = new Set(prev);
      if (next.has(j)) next.delete(j);
      else next.add(j);
      return next;
    });
  };

  const t = snapshots.map(s => s.t_s);
  const selected = Array.from(selectedRollers).sort((a, b) => a - b);

  const slipTraces: Plotly.Data[] = useMemo(() =>
    selected.map(j => ({
      x: t,
      y: snapshots.map(s => {
        const rk = s.roller_kinematics.find(r => r.j === j);
        return rk ? rk.slip_ratio * 100 : 0;
      }),
      name: `R${j}`,
      mode: 'lines' as const,
      line: { color: ROLLER_COLORS[j % ROLLER_COLORS.length], width: 1.5 },
    })),
    [snapshots, selected, t]
  );

  const slideVelTraces: Plotly.Data[] = useMemo(() =>
    selected.map(j => ({
      x: t,
      y: snapshots.map(s => {
        const rk = s.roller_kinematics.find(r => r.j === j);
        return rk ? rk.u_slide_avg : 0;
      }),
      name: `R${j}`,
      mode: 'lines' as const,
      line: { color: ROLLER_COLORS[j % ROLLER_COLORS.length], width: 1.5 },
    })),
    [snapshots, selected, t]
  );

  const tractionTraces: Plotly.Data[] = useMemo(() =>
    selected.map(j => ({
      x: t,
      y: snapshots.map(s => {
        const rk = s.roller_kinematics.find(r => r.j === j);
        return rk ? rk.tau_traction : 0;
      }),
      name: `R${j} τ`,
      mode: 'lines' as const,
      line: { color: ROLLER_COLORS[j % ROLLER_COLORS.length], width: 1.2 },
    })),
    [snapshots, selected, t]
  );

  const commonLayout = {
    ...darkLayout,
    showlegend: true,
    legend: { font: { size: 9, color: '#94a3b8' }, orientation: 'h' as const, y: 1.15 },
    margin: { l: 50, r: 15, t: 10, b: 30 },
    height: 180,
  };

  return (
    <div className="space-y-2">
      {/* Roller selector */}
      <div className="flex flex-wrap gap-1 pt-1">
        <span className="text-xs text-text-canvas mr-1 self-center">Rollers:</span>
        {Array.from({ length: nRollers }, (_, j) => (
          <button
            key={j}
            onClick={() => toggleRoller(j)}
            className={`px-1.5 py-0.5 text-[11px] rounded font-mono cursor-pointer transition-colors ${
              selectedRollers.has(j)
                ? 'text-white'
                : 'bg-white/5 text-text-canvas/50 hover:bg-white/10'
            }`}
            style={selectedRollers.has(j) ? { backgroundColor: ROLLER_COLORS[j % ROLLER_COLORS.length] + '80' } : undefined}
          >
            {j}
          </button>
        ))}
        <button
          onClick={() => setSelectedRollers(new Set(Array.from({ length: nRollers }, (_, i) => i)))}
          className="px-1.5 py-0.5 text-[11px] rounded bg-white/5 text-text-canvas hover:bg-white/10 cursor-pointer ml-1"
        >
          All
        </button>
        <button
          onClick={() => setSelectedRollers(new Set())}
          className="px-1.5 py-0.5 text-[11px] rounded bg-white/5 text-text-canvas hover:bg-white/10 cursor-pointer"
        >
          None
        </button>
      </div>

      <ChartLabel>Slip Ratio per Roller [%]</ChartLabel>
      <Plot
        data={slipTraces}
        layout={{ ...commonLayout, yaxis: { ...darkLayout.yaxis, title: { text: '%', font: { size: 10, color: '#94a3b8' } } } }}
        config={plotConfig}
        className="w-full"
      />

      <ChartLabel>Sliding Velocity [m/s]</ChartLabel>
      <Plot
        data={slideVelTraces}
        layout={{ ...commonLayout, yaxis: { ...darkLayout.yaxis, title: { text: 'm/s', font: { size: 10, color: '#94a3b8' } } } }}
        config={plotConfig}
        className="w-full"
      />

      <ChartLabel>Traction Torque [N·m]</ChartLabel>
      <Plot
        data={tractionTraces}
        layout={{ ...commonLayout, xaxis: { ...darkLayout.xaxis, title: { text: 'Time [s]', font: { size: 10, color: '#94a3b8' } } }, yaxis: { ...darkLayout.yaxis, title: { text: 'N·m', font: { size: 10, color: '#94a3b8' } } } }}
        config={plotConfig}
        className="w-full"
      />
    </div>
  );
}

function ChartLabel({ children }: { children: React.ReactNode }) {
  return <p className="text-xs font-semibold text-text-canvas uppercase tracking-wider pt-2">{children}</p>;
}
