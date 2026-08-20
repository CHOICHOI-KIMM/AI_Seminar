// @ts-nocheck
// CRB Phase 1.4 stub: 이 컴포넌트는 TRB 데이터 모델을 참조 중 → Phase 6 (Frontend UI 변경) 에서 CRB 로 정식 재작성 예정
import Plot from './PlotWithCopy';
import { darkLayout, plotConfig, viridisScale } from './plotlyDefaults';
import type { RollerResult } from '../../types/bearing';

interface RibContactDetailChartProps {
  roller: RollerResult;
}

/**
 * Single roller rib contact detail:
 * - 2D elliptical Hertz pressure heatmap p(x,y) = p0 * sqrt(1 - (x/a)^2 - (y/b)^2)
 * - Contact ellipse dimensions and summary table
 */
export default function RibContactDetailChart({ roller }: RibContactDetailChartProps) {
  const rib = roller.rib_result;

  if (!rib || rib.f_rib <= 0) {
    return (
      <div className="flex items-center justify-center h-full text-text-canvas text-sm">
        No rib contact for this roller (Q = {roller.q_normal.toFixed(0)} N)
      </div>
    );
  }

  const { a_ellipse: a, b_ellipse: b, p_max_rib: pMax, f_rib, spin_moment, ehl } = rib;

  // Build 2D elliptical Hertz pressure distribution
  // x-axis = circumferential (tangential, semi-axis b)
  // y-axis = meridional (normal, semi-axis a)
  // p(x,y) = p_max * sqrt(1 - (x/b)^2 - (y/a)^2)
  const nX = 61;
  const nY = 61;
  const xRange = b * 1.3; // circumferential direction
  const yRange = a * 1.3; // meridional direction

  const xVals = Array.from({ length: nX }, (_, i) => -xRange + (2 * xRange * i) / (nX - 1));
  const yVals = Array.from({ length: nY }, (_, j) => -yRange + (2 * yRange * j) / (nY - 1));

  const z: number[][] = [];
  for (let j = 0; j < nY; j++) {
    const row: number[] = [];
    for (let i = 0; i < nX; i++) {
      const xn = xVals[i] / b; // circumferential
      const yn = yVals[j] / a; // meridional
      const r2 = xn * xn + yn * yn;
      if (r2 <= 1) {
        row.push(pMax * Math.sqrt(1 - r2));
      } else {
        row.push(0);
      }
    }
    z.push(row);
  }

  // Contact boundary ellipse overlay
  const nTheta = 100;
  const ellipseX: number[] = [];
  const ellipseY: number[] = [];
  for (let t = 0; t <= nTheta; t++) {
    const theta = (2 * Math.PI * t) / nTheta;
    ellipseX.push(b * Math.cos(theta)); // circumferential
    ellipseY.push(a * Math.sin(theta)); // meridional
  }

  // Use mm scale if ellipse is large enough, else μm
  const useUm = a < 0.1; // < 0.1mm → display in μm
  const xDisplay = useUm ? xVals.map(v => v * 1000) : xVals;
  const yDisplay = useUm ? yVals.map(v => v * 1000) : yVals;
  const ellipseXd = useUm ? ellipseX.map(v => v * 1000) : ellipseX;
  const ellipseYd = useUm ? ellipseY.map(v => v * 1000) : ellipseY;
  const unitLabel = useUm ? 'μm' : 'mm';

  const data: Plotly.Data[] = [
    {
      type: 'heatmap',
      z,
      x: xDisplay,
      y: yDisplay,
      colorscale: viridisScale,
      colorbar: {
        title: { text: 'MPa', font: { size: 12, color: '#94a3b8' } },
        tickfont: { size: 11, family: 'JetBrains Mono', color: '#94a3b8' },
        len: 0.8,
        thickness: 12,
      },
      zsmooth: 'best',
      hovertemplate:
        `x = %{x:.1f} ${unitLabel}<br>y = %{y:.1f} ${unitLabel}<br>p = %{z:.0f} MPa<extra></extra>`,
    },
    // Contact boundary
    {
      type: 'scatter',
      x: ellipseXd,
      y: ellipseYd,
      mode: 'lines',
      line: { color: '#ffffff', width: 1.5, dash: 'dot' },
      showlegend: false,
      hoverinfo: 'skip',
    },
  ];

  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    paper_bgcolor: '#0f172a',
    plot_bgcolor: '#0f172a',
    title: {
      text: `Rib Contact — Roller #${Math.round(roller.psi_deg / (360 / 14)) + 1} (ψ=${roller.psi_deg.toFixed(1)}°)`,
      font: { size: 14, color: '#e2e8f0' },
    },
    xaxis: {
      ...darkLayout.xaxis,
      type: 'linear',
      title: { text: `Circumferential b [${unitLabel}]`, font: { size: 13, color: '#94a3b8' } },
      constrain: 'domain',
      nticks: 7,
      tickformat: '.2f',
    },
    yaxis: {
      ...darkLayout.yaxis,
      type: 'linear',
      title: { text: `Meridional a [${unitLabel}]`, font: { size: 13, color: '#94a3b8' } },
      scaleanchor: 'x',
      scaleratio: 1,
      constrain: 'domain',
      nticks: 7,
      tickformat: '.2f',
    },
    margin: { l: 60, r: 20, t: 35, b: 50 },
  };

  const ellipseArea = Math.PI * a * b;
  const pMean = f_rib / ellipseArea;
  const ellipticity = a > 0 && b > 0 ? (a > b ? a / b : b / a) : 0;

  return (
    <div className="w-full h-full flex min-h-0">
      {/* Heatmap */}
      <div className="flex-1 min-w-0">
        <Plot
          data={data}
          layout={layout}
          config={plotConfig}
          useResizeHandler
          style={{ width: '100%', height: '100%' }}
        />
      </div>

      {/* Summary panel */}
      <div className="w-56 shrink-0 flex flex-col justify-center border-l border-white/10 px-4 gap-3">
        <h4 className="text-sm font-semibold text-text-light uppercase tracking-wider">
          Rib Contact Summary
          <span className="block normal-case text-[11px] text-slate-500 mt-0.5 tracking-normal">리브 접촉 요약</span>
        </h4>
        <table className="text-xs text-slate-300 w-full">
          <tbody>
            <tr className="border-b border-white/[0.05]">
              <td className="text-slate-500 pr-2 py-0.5">F_rib</td>
              <td className="text-right font-mono">{f_rib.toFixed(1)} N</td>
            </tr>
            <tr className="border-b border-white/[0.05]">
              <td className="text-slate-500 pr-2 py-0.5">p_max</td>
              <td className="text-right font-mono">{pMax.toFixed(0)} MPa</td>
            </tr>
            <tr className="border-b border-white/[0.05]">
              <td className="text-slate-500 pr-2 py-0.5">p_mean</td>
              <td className="text-right font-mono">{pMean.toFixed(0)} MPa</td>
            </tr>
            <tr className="border-b border-white/[0.05]">
              <td className="text-slate-500 pr-2 py-0.5">p_max / p_mean</td>
              <td className="text-right font-mono">{(pMax / pMean).toFixed(2)}</td>
            </tr>
            <tr className="border-b border-white/[0.05]">
              <td className="text-slate-500 pr-2 py-0.5">a (semi-axis)</td>
              <td className="text-right font-mono">{a.toFixed(4)} mm</td>
            </tr>
            <tr className="border-b border-white/[0.05]">
              <td className="text-slate-500 pr-2 py-0.5">b (semi-axis)</td>
              <td className="text-right font-mono">{b.toFixed(4)} mm</td>
            </tr>
            <tr className="border-b border-white/[0.05]">
              <td className="text-slate-500 pr-2 py-0.5">2a × 2b</td>
              <td className="text-right font-mono">{(2*a).toFixed(4)} × {(2*b).toFixed(4)} mm</td>
            </tr>
            <tr className="border-b border-white/[0.05]">
              <td className="text-slate-500 pr-2 py-0.5">Ellipticity (a/b)</td>
              <td className="text-right font-mono">{ellipticity.toFixed(3)}</td>
            </tr>
            <tr className="border-b border-white/[0.05]">
              <td className="text-slate-500 pr-2 py-0.5">Contact area</td>
              <td className="text-right font-mono">{ellipseArea.toFixed(4)} mm²</td>
            </tr>
            <tr className="border-b border-white/[0.05]">
              <td className="text-slate-500 pr-2 py-0.5">Spin moment</td>
              <td className="text-right font-mono">{spin_moment.toFixed(3)} N·mm</td>
            </tr>
            <tr>
              <td className="text-slate-500 pr-2 py-0.5">Roller Q</td>
              <td className="text-right font-mono">{roller.q_normal.toFixed(0)} N</td>
            </tr>
          </tbody>
        </table>

        {/* EHL/TEHL block */}
        {ehl ? (
          <>
            <div className="mt-2 pt-2 border-t border-white/10">
              <h4 className="text-sm font-semibold text-text-light uppercase tracking-wider">
                Rib EHL / TEHL
                <span className="block normal-case text-[11px] text-slate-500 mt-0.5 tracking-normal">
                  유막·마찰·플래시 온도
                </span>
              </h4>
            </div>
            <table className="text-xs text-slate-300 w-full">
              <tbody>
                <tr className="border-b border-white/[0.05]">
                  <td className="text-slate-500 pr-2 py-0.5">h_c</td>
                  <td className="text-right font-mono">{ehl.h_c_um.toFixed(3)} μm</td>
                </tr>
                <tr className="border-b border-white/[0.05]">
                  <td className="text-slate-500 pr-2 py-0.5">h_min</td>
                  <td className="text-right font-mono">{ehl.h_min_um.toFixed(3)} μm</td>
                </tr>
                <tr className="border-b border-white/[0.05]">
                  <td className="text-slate-500 pr-2 py-0.5">σ_c (composite)</td>
                  <td className="text-right font-mono">{ehl.sigma_composite_um.toFixed(3)} μm</td>
                </tr>
                <tr className="border-b border-white/[0.05]">
                  <td className="text-slate-500 pr-2 py-0.5">Λ</td>
                  <td className="text-right">
                    <span className="font-mono mr-1.5">{ehl.lambda_ratio.toFixed(2)}</span>
                    <span
                      className={`inline-block px-1.5 rounded text-[10px] font-medium ${
                        ehl.regime === 'FullEhl'
                          ? 'bg-emerald-500/20 text-emerald-300'
                          : ehl.regime === 'Mixed'
                          ? 'bg-amber-500/20 text-amber-300'
                          : 'bg-red-500/20 text-red-300'
                      }`}
                    >
                      {ehl.regime === 'FullEhl' ? 'Full' : ehl.regime}
                    </span>
                  </td>
                </tr>
                <tr className="border-b border-white/[0.05]">
                  <td className="text-slate-500 pr-2 py-0.5">μ_eff</td>
                  <td className="text-right font-mono">{ehl.mu_eff.toFixed(4)}</td>
                </tr>
                <tr className="border-b border-white/[0.05]">
                  <td className="text-slate-500 pr-2 py-0.5 text-[11px]">μ_ehl (fluid)</td>
                  <td className="text-right font-mono text-slate-400">{ehl.mu_ehl.toFixed(4)}</td>
                </tr>
                <tr className="border-b border-white/[0.05]">
                  <td className="text-slate-500 pr-2 py-0.5 text-[11px]">f_a (asperity)</td>
                  <td className="text-right font-mono text-slate-400">
                    {(ehl.asperity_load_ratio * 100).toFixed(1)}%
                  </td>
                </tr>
                <tr className="border-b border-white/[0.05]">
                  <td className="text-slate-500 pr-2 py-0.5">ΔT_flash</td>
                  <td
                    className={`text-right font-mono ${
                      ehl.flash_temp_c > 50
                        ? 'text-red-400'
                        : ehl.flash_temp_c > 20
                        ? 'text-amber-400'
                        : ''
                    }`}
                  >
                    {ehl.flash_temp_c.toFixed(1)} °C
                  </td>
                </tr>
                <tr className="border-b border-white/[0.05]">
                  <td className="text-slate-500 pr-2 py-0.5 text-[11px]">SRR</td>
                  <td className="text-right font-mono text-slate-400">{ehl.srr.toFixed(2)}</td>
                </tr>
                <tr className="border-b border-white/[0.05]">
                  <td className="text-slate-500 pr-2 py-0.5 text-[11px]">u_entrain</td>
                  <td className="text-right font-mono text-slate-400">
                    {ehl.u_entrain_m_s.toFixed(3)} m/s
                  </td>
                </tr>
                <tr>
                  <td className="text-slate-500 pr-2 py-0.5 text-[11px]">φ_T (thermal)</td>
                  <td className="text-right font-mono text-slate-400">
                    {ehl.thermal_factor.toFixed(3)}
                  </td>
                </tr>
              </tbody>
            </table>
          </>
        ) : (
          <div className="mt-2 pt-2 border-t border-white/10 text-[11px] text-slate-500 leading-relaxed">
            <span className="text-slate-400">EHL not evaluated</span>
            <br />
            <span className="text-slate-600">
              EHL은 회전 중 + 축하중 &gt; 0 + α_i ≠ α_o 조건에서만 계산됩니다.
            </span>
          </div>
        )}

        {/* Visual ellipse */}
        <div className="mt-2">
          <div className="text-[11px] text-slate-500 text-center mb-1">
            Contact Ellipse (scaled)<br />
            <span className="text-slate-600">접촉 타원 형상 (압력 분포 등고선)</span>
          </div>
          <svg width="100%" viewBox="0 0 160 140" className="mx-auto">
            <line x1="0" y1="60" x2="160" y2="60" stroke="#334155" strokeWidth="0.5" />
            <line x1="80" y1="0" x2="80" y2="120" stroke="#334155" strokeWidth="0.5" />
            {[1.0, 0.75, 0.5, 0.25].map((f, i) => {
              const maxR = 55;
              // SVG x = circumferential (b), y = meridional (a)
              const finalRx = maxR * (b / Math.max(a, b)) * f;
              const finalRy = maxR * (a / Math.max(a, b)) * f;
              const label = `${Math.round(f * 100)}%`;
              return (
                <g key={i}>
                  <ellipse
                    cx={80}
                    cy={60}
                    rx={finalRx}
                    ry={finalRy}
                    fill={`rgba(${Math.round(255 * (1 - f * 0.3))}, ${Math.round(100 * f)}, ${Math.round(50 * f)}, ${0.15 + f * 0.2})`}
                    stroke={f === 1 ? '#f59e0b' : '#f59e0b44'}
                    strokeWidth={f === 1 ? 1.5 : 0.5}
                  />
                  <text
                    x={80 + finalRx + 2}
                    y={60}
                    fontSize="9"
                    fill="#94a3b8"
                    dominantBaseline="middle"
                  >
                    {label}
                  </text>
                </g>
              );
            })}
            {/* a/b dimension labels */}
            <text x={80} y={130} fontSize="9" fill="#f59e0b" textAnchor="middle">
              b={b.toFixed(3)} {unitLabel} (circ.)
            </text>
            <text x={15} y={60} fontSize="9" fill="#f59e0b" textAnchor="middle" transform="rotate(-90,15,60)">
              a={a.toFixed(3)} {unitLabel} (merid.)
            </text>
          </svg>
        </div>
      </div>
    </div>
  );
}
