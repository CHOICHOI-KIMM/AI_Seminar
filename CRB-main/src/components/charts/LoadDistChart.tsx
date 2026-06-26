import { useState } from 'react';
import Plot from './PlotWithCopy';
import { useActiveResult } from '../../hooks/useActiveResult';
import { useAppState } from '../../store';
import { darkLayout, plotConfig } from './plotlyDefaults';
import { DetailTable } from '../shared/DetailTable';
import type { BearingResult, RollerResult, SliceContactResult } from '../../types/bearing';

type LoadRaceway = 'outer' | 'inner';

export default function LoadDistChart() {
  const result = useActiveResult();
  const [loadRaceway, setLoadRaceway] = useState<LoadRaceway>('outer');
  if (!result) return null;

  const angDist = result.equilibrium.angular_distribution;
  const rollers = result.equilibrium.roller_results;
  const modeLabel = result.mode === 'Gen1' ? 'Gen1' : 'Gen3';
  const isInner = loadRaceway === 'inner';

  // Helper: get Q for the selected raceway
  const getQ = (r: RollerResult) => isInner ? r.q_normal_inner : r.q_normal;
  const racewayLabel = isInner ? 'Inner' : 'Outer';
  const racewayColor = isInner ? '#3b82f6' : '#f59e0b';

  // ─── Polar chart data ───
  const polarData: Plotly.Data[] = [];

  if (angDist && angDist.length > 0) {
    const psiEnv = angDist.map(p => p.psi_deg);
    const qEnv = angDist.map(p => p.q_total / 1000);
    psiEnv.push(psiEnv[0]);
    qEnv.push(qEnv[0]);

    polarData.push({
      type: 'scatterpolar',
      r: qEnv,
      theta: psiEnv,
      mode: 'lines',
      fill: 'toself',
      fillcolor: isInner ? 'rgba(59,130,246,0.15)' : 'rgba(245,158,11,0.15)',
      line: { color: racewayColor, width: 2 },
      name: `Envelope (${racewayLabel})`,
      hovertemplate: 'ψ=%{theta:.1f}°<br>Q=%{r:.3f} kN<extra></extra>',
    } as Plotly.Data);

    const rollerPsi = rollers.map(r => r.psi_deg);
    const rollerQ = rollers.map(r => getQ(r) / 1000);

    polarData.push({
      type: 'scatterpolar',
      r: rollerQ,
      theta: rollerPsi,
      mode: 'markers',
      marker: {
        size: 8,
        color: rollerQ.map(q => q),
        colorscale: [[0, '#3b82f6'], [0.5, '#f59e0b'], [1, '#ef4444']],
        showscale: false,
        line: { color: '#1e293b', width: 1 },
      },
      name: 'Rollers',
      hovertemplate: `Roller ψ=%{theta:.1f}°<br>Q_${racewayLabel.toLowerCase()}=%{r:.3f} kN<extra></extra>`,
    } as Plotly.Data);
  } else {
    const psi = rollers.map(r => r.psi_deg);
    const loads = rollers.map(r => getQ(r) / 1000);

    polarData.push({
      type: 'barpolar',
      r: loads,
      theta: psi,
      marker: {
        color: loads.map(q => q),
        colorscale: [[0, '#3b82f6'], [0.5, '#f59e0b'], [1, '#ef4444']],
        showscale: false,
      },
      width: 360 / rollers.length * 0.8,
    });
  }

  const polarLayout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: { text: `Load Distribution (${modeLabel})`, font: { size: 14, color: '#e2e8f0' } },
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
    margin: { l: 30, r: 30, t: 40, b: 30 },
    showlegend: angDist && angDist.length > 0,
    legend: {
      font: { size: 11, color: '#94a3b8' },
      bgcolor: 'transparent',
      x: 0.02, y: 0.98,
    },
  };

  // ─── Bar chart data (all rollers by index) ───
  const barData: Plotly.Data[] = [
    {
      type: 'bar',
      x: rollers.map((_, i) => i + 1),
      y: rollers.map(r => getQ(r) / 1000),
      marker: {
        color: rollers.map(r => getQ(r) / 1000),
        colorscale: [[0, '#3b82f6'], [0.5, '#f59e0b'], [1, '#ef4444']],
        showscale: true,
        colorbar: {
          title: { text: 'kN', font: { size: 12, color: '#94a3b8' } },
          tickfont: { size: 11, family: 'JetBrains Mono', color: '#94a3b8' },
          len: 0.8,
          thickness: 12,
        },
      },
      hovertemplate: `#%{x}<br>ψ=%{customdata:.1f}°<br>Q_${racewayLabel.toLowerCase()}=%{y:.3f} kN<extra></extra>`,
      customdata: rollers.map(r => r.psi_deg),
    },
  ];

  const barLayout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: { text: `Roller Loads — ${racewayLabel} (${modeLabel})`, font: { size: 14, color: '#e2e8f0' } },
    xaxis: {
      ...darkLayout.xaxis,
      title: { text: 'Roller #', font: { size: 12, color: '#94a3b8' } },
      dtick: 1,
    },
    yaxis: {
      ...darkLayout.yaxis,
      title: { text: 'Q [kN]', font: { size: 12, color: '#94a3b8' } },
    },
    margin: { l: 50, r: 20, t: 35, b: 40 },
    showlegend: false,
  };

  return (
    <div className="w-full h-full overflow-auto custom-scrollbar">
      {/* Inner/Outer toggle */}
      <div className="flex justify-center gap-1 py-1">
        {(['inner', 'outer'] as const).map(rw => (
          <button
            key={rw}
            onClick={() => setLoadRaceway(rw)}
            className={`px-3 py-1 text-[13px] rounded transition-colors ${
              loadRaceway === rw
                ? rw === 'inner' ? 'bg-blue-600 text-white' : 'bg-amber-600 text-white'
                : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
            }`}
          >
            {rw === 'inner' ? 'Inner Raceway' : 'Outer Raceway'}
          </button>
        ))}
      </div>
      {/* Charts: polar (left) + bar (right) */}
      <div className="flex overflow-hidden" style={{ height: '500px', minHeight: '500px' }}>
        <div className="w-1/2 h-full overflow-hidden">
          <Plot
            data={polarData}
            layout={polarLayout}
            config={plotConfig}
            useResizeHandler
            style={{ width: '100%', height: '100%' }}
          />
        </div>
        <div className="w-1/2 h-full overflow-hidden border-l border-white/5">
          <Plot
            data={barData}
            layout={barLayout}
            config={plotConfig}
            useResizeHandler
            style={{ width: '100%', height: '100%' }}
          />
        </div>
      </div>

      {/* Detail tables */}
      <div className="px-4 pb-4 border-t border-white/5">
        <LoadDetailPanel result={result} />
      </div>
    </div>
  );
}

// ─── Load Detail Panel ────────────────────────────────────────────

function LoadDetailPanel({ result }: { result: BearingResult }) {
  const { state } = useAppState();
  const input = state.input;
  const op = input.operating;
  const geo = result.geometry;
  const eq = result.equilibrium;
  const rollers = eq.roller_results;

  const qMaxOuter = Math.max(...eq.roller_loads);
  const qMaxInner = Math.max(...rollers.map(r => r.q_normal_inner));
  const pMaxInner = Math.max(
    ...rollers.flatMap(r => r.slice_results.map(s => s.p_max_k))
  );
  const pMaxOuter = Math.max(
    ...rollers.flatMap(r => r.slice_results.map(s => s.p_max_k_outer))
  );
  const loadedCount = eq.roller_loads.filter(q => q > 0).length;

  const maxIdx = eq.roller_loads.indexOf(qMaxOuter);
  const maxRoller = rollers[maxIdx];

  const ribResults = rollers
    .filter(r => r.rib_result && r.rib_result.f_rib > 0)
    .map(r => r.rib_result!);
  const ribMaxStress = ribResults.length > 0
    ? Math.max(...ribResults.map(r => r.p_max_rib))
    : 0;
  const ribMaxForce = ribResults.length > 0
    ? Math.max(...ribResults.map(r => r.f_rib))
    : 0;
  const maxRibRoller = rollers.find(
    r => r.rib_result && r.rib_result.p_max_rib === ribMaxStress
  );

  return (
    <div className="space-y-4 pt-3">
      <DetailTable title="Operating Conditions" rows={[
        ['F_x', `${op.f_x.toFixed(2)}`, 'kN'],
        ['F_y', `${op.f_y.toFixed(2)}`, 'kN'],
        ...(geo.f_r_kn > 0.001 ? [
          ['Load angle (φ)', `${(result.load_angle_deg ?? 0).toFixed(1)}`, '°'] as [string, string, string],
        ] : []),
        ['F_r (resultant)', `${geo.f_r_kn.toFixed(3)}`, 'kN'],
        ['F_a (input)', `${op.f_a.toFixed(2)}`, 'kN'],
        ['F_a / F_r', geo.f_r_kn > 0 ? `${(geo.f_a_kn / geo.f_r_kn).toFixed(4)}` : '-', ''],
        ['n inner / outer', `${op.n_inner_rpm.toFixed(0)} / ${op.n_outer_rpm.toFixed(0)}`, 'rpm'],
        ['Misalignment (γ)', `${(geo.gamma_rad * 180 * 60 / Math.PI).toFixed(3)}`, 'arcmin'],
        ...(result.f_a_induced_kn > 0 ? [
          ['F_a,induced', `${result.f_a_induced_kn.toFixed(3)}`, 'kN'] as [string, string, string],
          ['F_a,effective', `${result.f_a_effective_kn.toFixed(3)}`, 'kN'] as [string, string, string],
        ] : []),
        ...(result.preload_mode && result.preload_mode !== 'Force' ? [
          ['Preload mode', `${result.preload_mode === 'DisplacementFromForce' ? 'Disp←Force' : result.preload_mode}`, ''] as [string, string, string],
          ['δ_preload', `${result.delta_preload_um.toFixed(2)}`, 'μm'] as [string, string, string],
          ['F_a reaction', `${result.f_a_reaction_kn.toFixed(3)}`, 'kN'] as [string, string, string],
        ] : []),
      ]} />

      <DetailTable title="Equilibrium Displacement" rows={[
        ['δx (radial X)', `${eq.displacement[0].toFixed(4)}`, 'μm'],
        ['δy (radial Y)', `${eq.displacement[1].toFixed(4)}`, 'μm'],
        ['δz (axial)', `${eq.displacement[2].toFixed(4)}`, 'μm'],
        ['γx (tilt X)', `${(eq.displacement[3] * 1e6).toFixed(2)}`, 'μrad'],
        ['γy (tilt Y)', `${(eq.displacement[4] * 1e6).toFixed(2)}`, 'μrad'],
      ]} />

      <DetailTable title="Bearing Stiffness" rows={[
        ['K_radial', `${result.k_radial > 0 ? result.k_radial.toFixed(1) : '-'}`, 'N/μm'],
        ['K_axial', `${result.k_axial > 0 ? result.k_axial.toFixed(1) : '-'}`, 'N/μm'],
        ...(result.k_radial > 0 && result.k_axial > 0 ? [
          ['K_axial / K_radial', `${(result.k_axial / result.k_radial).toFixed(3)}`, ''] as [string, string, string],
        ] : []),
      ]} />

      <DetailTable title="Load Summary" rows={[
        ['Max roller load — Outer (Q_o)', `${qMaxOuter.toFixed(1)}`, 'N'],
        ['Max roller load — Inner (Q_i)', `${qMaxInner.toFixed(1)}`, 'N'],
        ['Max stress — Inner (p_max,i)', `${pMaxInner.toFixed(0)}`, 'MPa'],
        ['Max stress — Outer (p_max,o)', `${pMaxOuter.toFixed(0)}`, 'MPa'],
        ['Loaded rollers', `${loadedCount} / ${eq.roller_loads.length}`, ''],
        ['Load zone', `${(loadedCount / eq.roller_loads.length * 360).toFixed(0)}`, 'deg'],
        ...(ribResults.length > 0 ? [
          ['Max rib force', `${ribMaxForce.toFixed(1)}`, 'N'] as [string, string, string],
          ['Max rib stress', `${ribMaxStress.toFixed(0)}`, 'MPa'] as [string, string, string],
          ['Rollers with rib contact', `${ribResults.length}`, ''] as [string, string, string],
        ] : []),
      ]} />

      <RollerLoadTable rollers={rollers} />

      {maxRoller && (
        <SliceContactTable
          title={`Slice Detail — Roller #${maxIdx} (ψ=${maxRoller.psi_deg.toFixed(1)}°, Q=${qMaxOuter.toFixed(0)} N)`}
          slices={maxRoller.slice_results}
        />
      )}

      {maxRibRoller && maxRibRoller.rib_result && (
        <RibContactDetail roller={maxRibRoller} />
      )}
    </div>
  );
}

function RollerLoadTable({ rollers }: { rollers: RollerResult[] }) {
  const loaded = rollers.filter(r => r.q_normal > 0);
  if (loaded.length === 0) return null;

  return (
    <div>
      <h4 className="text-sm font-semibold text-text-light mb-2 uppercase tracking-wider">
        Per-Roller Loads ({loaded.length} loaded)
      </h4>
      <div className="overflow-x-auto">
        <table className="w-full text-[13px] font-mono">
          <thead>
            <tr className="text-text-canvas border-b border-white/10">
              <th className="px-2 py-1 text-right">ψ [deg]</th>
              <th className="px-2 py-1 text-right">Q_i [N]</th>
              <th className="px-2 py-1 text-right">Q_o [N]</th>
              <th className="px-2 py-1 text-right">p_max,i [MPa]</th>
              <th className="px-2 py-1 text-right">p_max,o [MPa]</th>
              <th className="px-2 py-1 text-right">Slices in contact</th>
              <th className="px-2 py-1 text-right">Rib F [N]</th>
              <th className="px-2 py-1 text-right">Rib p_max [MPa]</th>
            </tr>
          </thead>
          <tbody>
            {loaded.map((r, i) => {
              const pmaxInner = Math.max(...r.slice_results.map(s => s.p_max_k));
              const pmaxOuter = Math.max(...r.slice_results.map(s => s.p_max_k_outer));
              const contactSlices = r.slice_results.filter(s => s.in_contact).length;
              return (
                <tr key={i} className="border-b border-white/5 hover:bg-white/5">
                  <td className="px-2 py-0.5 text-right text-text-light">{r.psi_deg.toFixed(1)}</td>
                  <td className="px-2 py-0.5 text-right text-text-light">{r.q_normal_inner.toFixed(1)}</td>
                  <td className="px-2 py-0.5 text-right text-text-light">{r.q_normal.toFixed(1)}</td>
                  <td className="px-2 py-0.5 text-right text-text-light">{pmaxInner.toFixed(0)}</td>
                  <td className="px-2 py-0.5 text-right text-text-light">{pmaxOuter.toFixed(0)}</td>
                  <td className="px-2 py-0.5 text-right text-text-light">
                    {contactSlices}/{r.slice_results.length}
                  </td>
                  <td className="px-2 py-0.5 text-right text-text-light">
                    {r.rib_result ? r.rib_result.f_rib.toFixed(1) : '-'}
                  </td>
                  <td className="px-2 py-0.5 text-right text-text-light">
                    {r.rib_result ? r.rib_result.p_max_rib.toFixed(0) : '-'}
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </div>
  );
}

function RibContactDetail({ roller }: { roller: RollerResult }) {
  const rib = roller.rib_result;
  if (!rib || rib.f_rib <= 0) return null;

  return (
    <DetailTable
      title={`Rib Contact — Roller ψ=${roller.psi_deg.toFixed(1)}° (max rib stress)`}
      rows={[
        ['Rib force formula', 'Q·sin(α_o−α_i)/cos(α_i)', ''],
        ['Rib contact force (F_rib)', `${rib.f_rib.toFixed(1)}`, 'N'],
        ['Max contact stress (p_max)', `${rib.p_max_rib.toFixed(0)}`, 'MPa'],
        ['Mean contact stress (p_mean)', `${(rib.f_rib / (Math.PI * rib.a_ellipse * rib.b_ellipse)).toFixed(0)}`, 'MPa'],
        ['Ellipse semi-axis a (meridional)', `${rib.a_ellipse.toFixed(4)}`, 'mm'],
        ['Ellipse semi-axis b (circumferential)', `${rib.b_ellipse.toFixed(4)}`, 'mm'],
        ['Ellipse area (πab)', `${(Math.PI * rib.a_ellipse * rib.b_ellipse).toFixed(4)}`, 'mm²'],
        ['Ellipticity ratio (a/b)', rib.b_ellipse > 0 ? `${(rib.a_ellipse / rib.b_ellipse).toFixed(3)}` : '-', ''],
        ['Approach (δ_rib)', `${rib.delta_rib.toFixed(3)}`, 'μm'],
        ['Tangent stiffness (K_rib)', `${rib.k_rib.toFixed(1)}`, 'N/μm'],
        ['Spin moment', `${rib.spin_moment.toFixed(3)}`, 'N·mm'],
        ['Contact height (h_c)', `${rib.h_c_mm.toFixed(3)}`, 'mm'],
        ['Contact radius (r_c)', `${rib.r_contact_mm.toFixed(3)}`, 'mm'],
        ['R_rib_circ (used)', `${rib.r_rib_circ_mm.toFixed(2)}`, 'mm'],
      ]}
    />
  );
}

function SliceContactTable({ title, slices }: { title: string; slices: SliceContactResult[] }) {
  return (
    <div>
      <h4 className="text-sm font-semibold text-text-light mb-2 uppercase tracking-wider">
        {title}
      </h4>
      <div className="overflow-x-auto">
        <table className="w-full text-[13px] font-mono">
          <thead>
            <tr className="text-text-canvas">
              <th className="px-2 py-0.5 text-left" rowSpan={2}>#</th>
              <th className="px-2 py-0.5 text-right" rowSpan={2}>δ [μm]</th>
              <th className="px-2 py-0.5 text-right" rowSpan={2}>q_i [N/mm]</th>
              <th className="px-2 py-0.5 text-right" rowSpan={2}>q_o [N/mm]</th>
              <th className="px-2 py-0.5 text-center border-b border-blue-400/30" colSpan={4}>
                <span className="text-blue-300">Inner</span>
              </th>
              <th className="px-2 py-0.5 text-center border-b border-amber-400/30" colSpan={4}>
                <span className="text-amber-300">Outer</span>
              </th>
              <th className="px-2 py-0.5 text-center border-b border-emerald-400/30" rowSpan={2}>
                <span className="text-emerald-300">k_comb<br/><span className="text-[10px] opacity-60">[N/mm/μm]</span></span>
              </th>
              <th className="px-2 py-0.5 text-center" rowSpan={2}>Contact</th>
            </tr>
            <tr className="text-text-canvas border-b border-white/10">
              <th className="px-2 py-0.5 text-right">b <span className="text-[10px] opacity-60">[mm]</span></th>
              <th className="px-2 py-0.5 text-right">p_max <span className="text-[10px] opacity-60">[MPa]</span></th>
              <th className="px-2 py-0.5 text-right">h_bulk <span className="text-[10px] opacity-60">[μm]</span></th>
              <th className="px-2 py-0.5 text-right">k_h <span className="text-[10px] opacity-60">[N/mm/μm]</span></th>
              <th className="px-2 py-0.5 text-right">b <span className="text-[10px] opacity-60">[mm]</span></th>
              <th className="px-2 py-0.5 text-right">p_max <span className="text-[10px] opacity-60">[MPa]</span></th>
              <th className="px-2 py-0.5 text-right">h_bulk <span className="text-[10px] opacity-60">[μm]</span></th>
              <th className="px-2 py-0.5 text-right">k_h <span className="text-[10px] opacity-60">[N/mm/μm]</span></th>
            </tr>
          </thead>
          <tbody>
            {slices.map(s => (
              <tr
                key={s.k}
                className={`border-b border-white/5 hover:bg-white/5 ${
                  !s.in_contact ? 'opacity-40' : ''
                }`}
              >
                <td className="px-2 py-0.5 text-text-canvas">{s.k}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{s.delta_k.toFixed(4)}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{s.q_k_inner.toFixed(2)}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{s.q_k_outer.toFixed(2)}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{s.b_k.toFixed(5)}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{s.p_max_k.toFixed(0)}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{s.h_bulk_k.toFixed(4)}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{s.k_hertz_k > 0 ? s.k_hertz_k.toFixed(1) : '-'}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{s.b_k_outer.toFixed(5)}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{s.p_max_k_outer.toFixed(0)}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{s.h_bulk_k_outer.toFixed(4)}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{s.k_hertz_k_outer > 0 ? s.k_hertz_k_outer.toFixed(1) : '-'}</td>
                <td className="px-2 py-0.5 text-right text-emerald-300">{s.k_combined_k > 0 ? s.k_combined_k.toFixed(1) : '-'}</td>
                <td className="px-2 py-0.5 text-center">
                  {s.in_contact ? (
                    <span className="text-emerald-400">●</span>
                  ) : (
                    <span className="text-text-canvas">○</span>
                  )}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
