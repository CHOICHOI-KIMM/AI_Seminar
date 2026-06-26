import { useState, useMemo } from 'react';
import Plot from '../charts/PlotWithCopy';
import { darkLayout, plotConfig } from '../charts/plotlyDefaults';
import type { TransientResult } from '../../types/bearing';

interface Props {
  result: TransientResult;
}

export default function SliceSlidingContour({ result }: Props) {
  const { snapshots } = result;

  // Find snapshots that have slice_srr_map data
  const validSnapshots = useMemo(
    () => snapshots.filter(s => s.slice_srr_map && s.slice_srr_map.length > 0 && s.slice_srr_map[0]?.length > 0),
    [snapshots]
  );

  const [snapIdx, setSnapIdx] = useState(Math.max(0, validSnapshots.length - 1));

  if (validSnapshots.length === 0) {
    return (
      <div className="flex items-center justify-center h-48">
        <p className="text-text-canvas/60 text-xs">
          No per-slice SRR data available. Enable roller dynamics with profile modifications.
        </p>
      </div>
    );
  }

  const snap = validSnapshots[Math.min(snapIdx, validSnapshots.length - 1)];
  const srrMap = snap.slice_srr_map!;
  const nSlices = srrMap[0]?.length ?? 0;

  // X axis: roller angular positions (ψ)
  const psiDeg = snap.roller_kinematics.map(rk => rk.psi_deg);

  // Y axis: slice positions (normalized % of effective length)
  const slicePositions = Array.from({ length: nSlices }, (_, k) => (k + 0.5) / nSlices * 100);

  // Sort by psi for proper spatial ordering
  const sortedIndices = psiDeg.map((_, i) => i).sort((a, b) => psiDeg[a] - psiDeg[b]);
  const sortedPsi = sortedIndices.map(i => psiDeg[i]);

  // z[slice_k][roller_j] — Plotly heatmap wants z[y][x]
  const z: number[][] = [];
  for (let k = 0; k < nSlices; k++) {
    const row: number[] = [];
    for (const j of sortedIndices) {
      row.push((srrMap[j]?.[k] ?? 0) * 100); // convert to %
    }
    z.push(row);
  }

  // Time-history max SRR per snapshot
  const timeData = useMemo(() => {
    const times = validSnapshots.map(s => s.t_s);
    const maxSrrPerTime = validSnapshots.map(s => {
      const map = s.slice_srr_map ?? [];
      let maxVal = 0;
      for (const row of map) {
        for (const v of row) {
          const abs = Math.abs(v);
          if (abs > maxVal) maxVal = abs;
        }
      }
      return maxVal * 100; // %
    });
    return { times, maxSrrPerTime };
  }, [validSnapshots]);

  const heatmapTrace: Plotly.Data = {
    x: sortedPsi,
    y: slicePositions,
    z,
    type: 'heatmap',
    colorscale: [
      [0, '#1e293b'],
      [0.2, '#1e40af'],
      [0.4, '#0891b2'],
      [0.6, '#10b981'],
      [0.8, '#f59e0b'],
      [1.0, '#ef4444'],
    ],
    colorbar: {
      title: { text: 'SRR [%]', font: { size: 10, color: '#94a3b8' } },
      tickfont: { size: 9, color: '#94a3b8' },
      thickness: 12,
      len: 0.9,
    },
    hovertemplate: 'ψ=%{x:.1f}°<br>Slice=%{y:.1f}%<br>SRR=%{z:.4f}%<extra></extra>',
  };

  return (
    <div className="space-y-3">
      {/* Time slider */}
      <div className="flex items-center gap-3 px-1">
        <span className="text-xs text-text-canvas/70 shrink-0">
          t = {snap.t_s.toFixed(4)} s
        </span>
        <input
          type="range"
          min={0}
          max={validSnapshots.length - 1}
          value={snapIdx}
          onChange={e => setSnapIdx(Number(e.target.value))}
          className="flex-1 h-1 accent-accent"
        />
        <span className="text-xs text-text-canvas/50 shrink-0">
          {snapIdx + 1}/{validSnapshots.length}
        </span>
      </div>

      {/* Heatmap: roller position × slice position */}
      <p className="text-xs font-semibold text-text-canvas uppercase tracking-wider">
        Per-Slice SRR [%] — Roller × Slice
      </p>
      <Plot
        data={[heatmapTrace]}
        layout={{
          ...darkLayout,
          margin: { l: 55, r: 80, t: 10, b: 40 },
          height: 280,
          xaxis: {
            ...darkLayout.xaxis,
            title: { text: 'Roller Position ψ [°]', font: { size: 10, color: '#94a3b8' } },
          },
          yaxis: {
            ...darkLayout.yaxis,
            title: { text: 'Slice Position [%L]', font: { size: 10, color: '#94a3b8' } },
          },
        }}
        config={plotConfig}
        className="w-full"
      />

      {/* Time history of max slice SRR */}
      <p className="text-xs font-semibold text-text-canvas uppercase tracking-wider pt-2">
        Max Slice SRR over Time [%]
      </p>
      <Plot
        data={[{
          x: timeData.times,
          y: timeData.maxSrrPerTime,
          mode: 'lines',
          line: { color: '#f59e0b', width: 1.5 },
          name: 'Max Slice SRR',
        }]}
        layout={{
          ...darkLayout,
          showlegend: false,
          margin: { l: 50, r: 15, t: 10, b: 30 },
          height: 140,
          xaxis: {
            ...darkLayout.xaxis,
            title: { text: 'Time [s]', font: { size: 10, color: '#94a3b8' } },
          },
          yaxis: {
            ...darkLayout.yaxis,
            title: { text: '%', font: { size: 10, color: '#94a3b8' } },
          },
        }}
        config={plotConfig}
        className="w-full"
      />
    </div>
  );
}
