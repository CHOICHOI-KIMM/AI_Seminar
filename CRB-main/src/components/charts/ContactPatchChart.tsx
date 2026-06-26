import Plot from './PlotWithCopy';
import { darkLayout, plotConfig, viridisScale } from './plotlyDefaults';
import type { RollerResult, SliceGeometry } from '../../types/bearing';

interface ContactPatchChartProps {
  roller: RollerResult;
  sliceGeometries: SliceGeometry[];
  raceway: 'inner' | 'outer';
}

/**
 * 방안 2: 2D Contact Patch Contour
 * Visualizes the Hertz pressure distribution across the roller surface.
 * X: axial slice position, Y: contact width direction [-b_k, +b_k], Z(color): Hertz pressure
 * p(y) = p_max * sqrt(1 - (y/b)^2)
 */
export default function ContactPatchChart({ roller, sliceGeometries, raceway }: ContactPatchChartProps) {
  const slices = roller.slice_results;
  const nSlices = slices.length;

  // Y resolution: contact width direction discretization
  const nY = 41; // odd number for symmetry around 0
  const yNorm = Array.from({ length: nY }, (_, i) => -1 + (2 * i) / (nY - 1)); // [-1, ..., 0, ..., 1]

  // Find max b for Y-axis scaling
  const bValues = slices.map(s => raceway === 'inner' ? s.b_k : s.b_k_outer);
  const bMax = Math.max(...bValues, 0.001);

  // Build 2D pressure matrix: z[yIdx][sliceIdx]
  const z: number[][] = [];
  const yLabels: string[] = [];

  for (let j = 0; j < nY; j++) {
    const yN = yNorm[j]; // normalized [-1, 1]
    const yPhysical = yN * bMax;
    yLabels.push(yPhysical.toFixed(4));

    const row: number[] = [];
    for (let k = 0; k < nSlices; k++) {
      const s = slices[k];
      const bk = raceway === 'inner' ? s.b_k : s.b_k_outer;
      const pmax = raceway === 'inner' ? s.p_max_k : s.p_max_k_outer;

      if (!s.in_contact || bk <= 0) {
        row.push(0);
        continue;
      }

      // Physical y coordinate for this slice
      const yAbs = Math.abs(yPhysical);
      if (yAbs > bk) {
        row.push(0);
      } else {
        // Hertz semi-elliptical distribution: p = p_max * sqrt(1 - (y/b)^2)
        const ratio = yAbs / bk;
        row.push(pmax * Math.sqrt(1 - ratio * ratio));
      }
    }
    z.push(row);
  }

  const xLabels = sliceGeometries.map(sg => sg.x_axial.toFixed(2));
  const racewayLabel = raceway === 'inner' ? 'Inner' : 'Outer';

  const data: Plotly.Data[] = [
    {
      type: 'heatmap',
      z,
      x: xLabels,
      y: yLabels,
      colorscale: viridisScale,
      colorbar: {
        title: { text: 'MPa', font: { size: 12, color: '#94a3b8' } },
        tickfont: { size: 11, family: 'JetBrains Mono', color: '#94a3b8' },
        len: 0.8,
        thickness: 12,
      },
      zsmooth: 'best',
      hovertemplate:
        'x = %{x} mm<br>y = %{y} mm<br>p = %{z:.0f} MPa<extra></extra>',
    },
  ];

  // Overlay contact boundary lines
  const xBoundary: string[] = [];
  const yUpper: string[] = [];
  const yLower: string[] = [];
  for (let k = 0; k < nSlices; k++) {
    const bk = raceway === 'inner' ? slices[k].b_k : slices[k].b_k_outer;
    if (slices[k].in_contact && bk > 0) {
      xBoundary.push(sliceGeometries[k].x_axial.toFixed(2));
      yUpper.push(bk.toFixed(4));
      yLower.push((-bk).toFixed(4));
    }
  }

  // Upper boundary
  data.push({
    type: 'scatter',
    x: xBoundary,
    y: yUpper,
    mode: 'lines',
    line: { color: '#ffffff', width: 1.5, dash: 'dot' },
    showlegend: false,
    hoverinfo: 'skip',
  });
  // Lower boundary
  data.push({
    type: 'scatter',
    x: xBoundary,
    y: yLower,
    mode: 'lines',
    line: { color: '#ffffff', width: 1.5, dash: 'dot' },
    showlegend: false,
    hoverinfo: 'skip',
  });

  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: {
      text: `Contact Patch — ${racewayLabel} Raceway (ψ=${roller.psi_deg.toFixed(1)}°, Q=${roller.q_normal.toFixed(0)} N)`,
      font: { size: 14, color: '#e2e8f0' },
    },
    xaxis: {
      ...darkLayout.xaxis,
      title: { text: 'Axial position [mm]', font: { size: 12, color: '#94a3b8' } },
    },
    yaxis: {
      ...darkLayout.yaxis,
      title: { text: 'Contact width [mm]', font: { size: 12, color: '#94a3b8' } },
      scaleanchor: undefined,
    },
    margin: { l: 60, r: 20, t: 35, b: 45 },
  };

  return (
    <Plot
      data={data}
      layout={layout}
      config={plotConfig}
      useResizeHandler
      style={{ width: '100%', height: '100%' }}
    />
  );
}
