// @ts-nocheck
// CRB Phase 1.4 stub: 이 컴포넌트는 TRB 데이터 모델을 참조 중 → Phase 6 (Frontend UI 변경) 에서 CRB 로 정식 재작성 예정
import Plot from './PlotWithCopy';
import { useActiveResult } from '../../hooks/useActiveResult';
import { useAppState } from '../../store';
import { darkLayout, plotConfig } from './plotlyDefaults';
import { DetailTable, formatHours } from '../shared/DetailTable';
import type { BearingResult, FatigueLifeResult } from '../../types/bearing';

export default function LifeChart() {
  const result = useActiveResult();
  if (!result) return null;

  const life = result.life;
  const modeLabel = result.mode === 'Gen1' ? 'Gen1' : 'Gen3';
  const hasLamina = !!life.lamina_lives;

  const barData: Plotly.Data[] = [
    {
      type: 'bar',
      x: ['L₁₀ Inner', 'L₁₀ Outer', 'L₁₀ Combined'],
      y: [life.l_10_inner, life.l_10_outer, life.l_10_basic],
      marker: {
        color: ['#3b82f6', '#22c55e', '#f59e0b'],
      },
      text: [
        `${life.l_10_inner.toFixed(1)}`,
        `${life.l_10_outer.toFixed(1)}`,
        `${life.l_10_basic.toFixed(1)}`,
      ],
      textposition: 'outside',
      textfont: { family: 'JetBrains Mono', size: 12, color: '#e2e8f0' },
    },
  ];

  const barLayout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: { text: `Fatigue Life — ISO 16281 (${modeLabel})`, font: { size: 15, color: '#e2e8f0' } },
    yaxis: {
      ...darkLayout.yaxis,
      title: { text: 'L₁₀ [10⁶ rev]', font: { size: 13, color: '#94a3b8' } },
      type: 'log',
    },
    annotations: [
      {
        x: 0.02,
        y: 0.98,
        xref: 'paper',
        yref: 'paper',
        text: `L_nm = ${formatHours(life.l_nm_hours)} hrs | ${life.kappa_inner === life.kappa_outer ? `κ = ${life.kappa_inner.toFixed(2)}` : `κi = ${life.kappa_inner.toFixed(2)}, κo = ${life.kappa_outer.toFixed(2)}`}${life.intermediates.kappa_method === 'FilmThicknessRatio' ? ' (Λ)' : ''} | a_ISO = ${life.a_iso.toFixed(2)}`,
        showarrow: false,
        font: { size: 12, color: '#94a3b8', family: 'JetBrains Mono' },
        xanchor: 'left',
        yanchor: 'top',
      },
    ],
  };

  const laminaChart = life.lamina_lives ? renderLaminaChart(life.lamina_lives) : null;

  return (
    <div className="w-full h-full overflow-auto custom-scrollbar">
      {/* Charts: bar (left) + lamina (right) */}
      <div className="flex overflow-hidden" style={{ height: '500px', minHeight: '500px' }}>
        <div className={`${hasLamina ? 'w-1/2' : 'w-full'} h-full overflow-hidden`}>
          <Plot
            data={barData}
            layout={barLayout}
            config={plotConfig}
            useResizeHandler
            style={{ width: '100%', height: '100%' }}
          />
        </div>
        {laminaChart && (
          <div className="w-1/2 h-full overflow-hidden border-l border-white/5">
            {laminaChart}
          </div>
        )}
      </div>

      {/* Detail tables */}
      <div className="px-4 pb-4 border-t border-white/5">
        <LifeDetailPanel result={result} />
      </div>
    </div>
  );
}

function renderLaminaChart(laminae: { k: number; l_10_inner: number; l_10_outer: number }[]) {
  const x = laminae.map(l => l.k + 1);

  const data: Plotly.Data[] = [
    {
      type: 'scatter',
      x,
      y: laminae.map(l => Math.min(l.l_10_inner, 1e8)),
      name: 'Inner',
      line: { color: '#3b82f6', width: 1.5 },
      mode: 'lines',
    },
    {
      type: 'scatter',
      x,
      y: laminae.map(l => Math.min(l.l_10_outer, 1e8)),
      name: 'Outer',
      line: { color: '#22c55e', width: 1.5 },
      mode: 'lines',
    },
  ];

  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: { text: 'Lamina Life', font: { size: 14, color: '#e2e8f0' } },
    showlegend: true,
    legend: { x: 0.02, y: 0.98, font: { size: 11, color: '#94a3b8' } },
    xaxis: { ...darkLayout.xaxis, title: { text: 'Slice', font: { size: 12, color: '#94a3b8' } } },
    yaxis: {
      ...darkLayout.yaxis,
      title: { text: 'L₁₀ [10⁶ rev]', font: { size: 12, color: '#94a3b8' } },
      type: 'log',
    },
    margin: { l: 50, r: 50, t: 30, b: 40 },
  };

  return (
    <Plot
      data={data}
      layout={layout}
      config={{ displayModeBar: false, responsive: true }}
      useResizeHandler
      style={{ width: '100%', height: '100%' }}
    />
  );
}

// ─── Life + Static Detail Panel ───────────────────────────────────

/** Weibull reliability at time T given L_10 life (same units).
 *  R(T) = exp(-(T/η)^e), η = L_10 / (-ln(0.9))^(1/e), e = 10/9 (line contact). */
function weibullReliability(t: number, l10: number): number {
  if (!isFinite(l10) || l10 <= 0 || t <= 0) return 1;
  const e = 10 / 9;
  const eta = l10 / Math.pow(-Math.log(0.9), 1 / e);
  return Math.exp(-Math.pow(t / eta, e));
}

function LifeDetailPanel({ result }: { result: BearingResult }) {
  const { state } = useAppState();
  const life = result.life;
  const im = life.intermediates;
  const sr = result.static_rating;
  const s0min = state.input.solver.f_s_min;
  const nRpm = Math.abs(state.input.operating.n_inner_rpm - state.input.operating.n_outer_rpm);
  const designLife = state.input.operating.design_life_hours;

  // Basic reference life in hours
  const l10rh = nRpm > 1e-3 ? life.l_10_basic * 1e6 / (60 * nRpm) : Infinity;
  // Modified reference life
  const l10mr = life.a_iso * life.l_10_basic;
  const l10mrh = life.l_nm_hours;

  // Inner/outer ring life in hours
  const l10ih = nRpm > 1e-3 ? life.l_10_inner * 1e6 / (60 * nRpm) : Infinity;
  const l10oh = nRpm > 1e-3 ? life.l_10_outer * 1e6 / (60 * nRpm) : Infinity;

  // Damage %
  const dmgBasic = isFinite(l10rh) ? (designLife / l10rh) * 100 : 0;
  const dmgMod = isFinite(l10mrh) ? (designLife / l10mrh) * 100 : 0;

  // Weibull reliability / unreliability
  const relBasic = weibullReliability(designLife, l10rh) * 100;
  const relMod = weibullReliability(designLife, l10mrh) * 100;

  // C_0r for display
  const c0r = state.input.solver.c_0r_kn ?? sr.c_0r_kn;

  // Per-lamina a_ISO range
  const aIsoRange: [number, number] | null = life.lamina_lives && life.lamina_lives.length > 0
    ? life.lamina_lives.reduce<[number, number]>((acc, l) => [
        Math.min(acc[0], l.a_iso_k_inner, l.a_iso_k_outer),
        Math.max(acc[1], l.a_iso_k_inner, l.a_iso_k_outer),
      ], [Infinity, -Infinity])
    : null;

  return (
    <div className="space-y-4 pt-3">
      {/* ═══ SECTION 1: Static Rating ═══ */}
      <DetailTable title="Static Rating (ISO 76 / ISO 17956)" rows={[
        ['C\u2080\u1D63 (static rating)', sr.c_0r_kn.toFixed(2), 'kN'],
        ['P\u2080\u1D63 (static equiv. load)', sr.p_0r_kn.toFixed(2), 'kN'],
        ['S\u2080 = C\u2080\u1D63 / P\u2080\u1D63', `${sr.s_0.toFixed(2)} ${sr.s_0 >= s0min ? '\u2705' : '\u274C'} (min ${s0min})`, ''],
        ['S\u2080,eff (ISO 17956)', `${sr.s_0_eff.toFixed(2)} ${sr.s_0_adequate ? '\u2705' : '\u274C'} (min ${s0min})`, ''],
        ['X\u2080 / Y\u2080', `${sr.x_0.toFixed(2)} / ${sr.y_0.toFixed(3)}`, ''],
        ['q\u2080 / q_max', `${sr.q_0.toFixed(1)} / ${sr.q_max.toFixed(1)}`, 'N'],
        ['q_max location', `roller #${sr.q_max_roller_idx + 1}, lamina k=${sr.q_max_lamina_idx}`, ''],
      ]} />

      {/* ═══ SECTION 2: Input — Load & Rating ═══ */}
      <DetailTable title="Load & Rating Input" rows={[
        ['F_r = \u221A(F_x\u00B2+F_y\u00B2)', `${Math.sqrt(state.input.operating.f_x**2 + state.input.operating.f_y**2).toFixed(4)}`, 'kN'],
        ['F_a', `${state.input.operating.f_a.toFixed(4)}`, 'kN'],
        ['C_r (dynamic)', `${life.c_dyn.toFixed(2)}`, 'kN'],
        ['C\u2080\u1D63 (static)', `${c0r.toFixed(2)}`, 'kN'],
        ['C_u = C\u2080/8.2 (fatigue limit)', `${im.c_u_kn.toFixed(4)}`, 'kN'],
        ['Speed n', `${nRpm.toFixed(0)}`, 'rpm'],
        ['Duration', `${designLife.toFixed(0)}`, 'hr'],
        ['Slices n_s', `${state.input.solver.n_slices}`, ''],
      ]} />

      {/* ═══ SECTION 3: ISO 281 Equivalent Load ═══ */}
      <DetailTable title="ISO 281 Equivalent Load" rows={[
        ['e = 1.5\u00B7tan(\u03B1)', `${im.e_demarcation.toFixed(4)}`, ''],
        ['F_a / F_r', isFinite(im.f_a_over_f_r) ? `${im.f_a_over_f_r.toFixed(4)}` : '\u221E', ''],
        ['Regime', im.f_a_over_f_r > im.e_demarcation ? 'Combined (F_a/F_r > e)' : 'Radial dominant (F_a/F_r \u2264 e)', ''],
        ['X / Y', `${im.x_factor.toFixed(2)} / ${im.y_factor.toFixed(4)}`, ''],
        ['P = X\u00B7F_r + Y\u00B7F_a', `${life.p_equiv.toFixed(4)}`, 'kN'],
        ['C_r / P', life.p_equiv > 0 ? `${(life.c_dyn / life.p_equiv).toFixed(4)}` : '-', ''],
        ['C_u / P', `${im.c_u_over_p.toFixed(4)}`, ''],
      ]} />

      {/* ═══ SECTION 4: Lubrication & Contamination ═══ */}
      <DetailTable title="Lubrication & Contamination" rows={[
        ['Type', state.input.operating.lubrication_type, ''],
        ['\u03BD\u2084\u2080 / \u03BD\u2081\u2080\u2080', `${state.input.operating.nu_40.toFixed(1)} / ${state.input.operating.nu_100.toFixed(2)}`, 'mm\u00B2/s'],
        ['T_op', `${state.input.operating.t_op.toFixed(1)}`, '\u00B0C'],
        ['\u03BD at T_op', `${im.nu_actual.toFixed(4)}`, 'mm\u00B2/s'],
        ['\u03BD\u2081 (ISO 281 ref)', `${im.nu_ref.toFixed(4)}`, 'mm\u00B2/s'],
        ...(life.kappa_inner === life.kappa_outer
          ? [[im.kappa_method === 'FilmThicknessRatio'
              ? `\u03BA = \u039B\u00B9\u00B7\u00B3 (\u039B=${im.lambda_inner?.toFixed(2) ?? '?'})`
              : '\u03BA = \u03BD/\u03BD\u2081',
             `${life.kappa_inner.toFixed(4)}`, '']]
          : [[`\u03BAi (\u039B=${im.lambda_inner?.toFixed(2)})`, `${life.kappa_inner.toFixed(4)}`, ''],
             [`\u03BAo (\u039B=${im.lambda_outer?.toFixed(2)})`, `${life.kappa_outer.toFixed(4)}`, '']]
        ) as [string, string, string][],
        ['e_C (contamination)', `${im.e_c_used.toFixed(4)}${state.input.solver.e_c <= 0 ? ' (auto)' : ' (manual)'}`, ''],
        ['\u03C6_s (starvation)', `${state.input.operating.starvation_factor.toFixed(2)}`, ''],
      ]} />

      {/* ═══ SECTION 5: Capacity Factors ═══ */}
      <DetailTable title="Bearing Geometry & Capacity" rows={[
        ['\u03B3 = D_we\u00B7cos\u03B1/d_pw', `${im.gamma_bearing.toFixed(6)}`, ''],
        ['b_m (manufacturing)', `${im.b_m.toFixed(1)}`, ''],
        ['f_c (ISO 281 Table 6)', `${im.f_c.toFixed(4)}`, ''],
        ['f_ci / f_co (raceway)', `${im.f_ci.toFixed(4)} / ${im.f_co.toFixed(4)}`, ''],
        ['Q_ci / Q_ce (per-roller)', `${(im.q_ci / 1000).toFixed(4)} / ${(im.q_co / 1000).toFixed(4)}`, 'kN'],
        ['q_ci / q_ce (per-lamina)', `${(im.q_c_lamina_inner / 1000).toFixed(4)} / ${(im.q_c_lamina_outer / 1000).toFixed(4)}`, 'kN'],
      ]} />

      {/* ═══ SECTION 6: Basic Reference Rating Life ═══ */}
      <DetailTable title="Basic Reference Rating Life" rows={[
        ['L_10r (cycles)', `${life.l_10_basic.toFixed(4)}`, '\xD710\u2076 rev'],
        ['L_10r (time)', `${formatHours(l10rh)}`, 'hr'],
        ['  Inner ring', `${life.l_10_inner.toFixed(4)} \xD710\u2076 rev  (${formatHours(l10ih)} hr)`, ''],
        ['  Outer ring', `${life.l_10_outer.toFixed(4)} \xD710\u2076 rev  (${formatHours(l10oh)} hr)`, ''],
        ['  Weakest lamina', `k = ${life.weakest_lamina}`, ''],
        [`Damage (@ ${designLife} hr)`, fmtDamage(dmgBasic), '%'],
        ['Reliability', fmtReliability(relBasic), '%'],
        ['Unreliability', `${(100 - relBasic).toFixed(2)}`, '%'],
      ]} />
      <div className="text-[11px] text-text-canvas/50 -mt-2 mb-2 px-1">
        ISO 16281 Eq.(29): L_10r = {'{'}&Sigma;_k [(q_ci/q_eik)^(-9/2) + (q_ce/q_eek)^(-9/2)]{'}'}^(-8/9) — inner/outer damage가 lamina별로 합산된 후 전체 Weibull 결합. 개별 ring 수명은 보고용 참고값.
      </div>

      {/* ═══ SECTION 7: Life Modification ═══ */}
      <DetailTable title="Life Modification (a_ISO)" rows={[
        ['a_1 (reliability)', '1', ''],
        ['a_ISO (bearing effective)', `${life.a_iso.toFixed(4)}`, ''],
        ...(aIsoRange ? [
          ['  per-lamina range', `${aIsoRange[0].toFixed(4)} ~ ${aIsoRange[1].toFixed(4)}`, ''] as [string, string, string],
        ] : []),
        ['Weibull exponent (e)', `${im.weibull_e.toFixed(4)}`, ''],
      ]} />

      {/* ═══ SECTION 8: Modified Reference Rating Life ═══ */}
      <DetailTable title="Modified Reference Rating Life" rows={[
        ['P_ref (back-calc.)', `${life.p_ref.toFixed(4)}`, 'kN'],
        ['P_ref (damage-wt.)', `${life.p_ref_damage.toFixed(4)}`, 'kN'],
        ['P_ref ratio (dmg/bc)', life.p_ref > 0 ? `${(life.p_ref_damage / life.p_ref).toFixed(4)}` : '-', ''],
        ['L_10mr (cycles)', `${l10mr.toFixed(4)}`, '\xD710\u2076 rev'],
        ['L_10mr (time)', `${formatHours(l10mrh)}`, 'hr'],
        [`Damage (@ ${designLife} hr)`, fmtDamage(dmgMod), '%'],
        ['Reliability', fmtReliability(relMod), '%'],
        ['Unreliability', `${(100 - relMod).toFixed(2)}`, '%'],
      ]} />
      <div className="text-[10px] text-gray-500 mt-0.5 px-1 leading-tight">
        P_ref (back-calc.) = C_r / L₁₀ᵣ<sup>3/10</sup> — ISO 16281 L₁₀ᵣ 기반 역산.{' '}
        P_ref (damage-wt.) = ΣP_sk·D_k / ΣD_k — 손상 기여 가중, 에지 로딩에 민감.{' '}
        Ratio &gt; 1: 하중 집중 경고 (에지 로딩 또는 미스얼라인먼트).
      </div>

      {/* Lamina lives */}
      {life.lamina_lives && life.lamina_lives.length > 0 && (
        <LaminaLifeTable lives={life.lamina_lives} weakest={life.weakest_lamina} />
      )}

    </div>
  );
}

function fmtDamage(v: number): string {
  return v.toFixed(2);
}

function fmtReliability(v: number): string {
  return v.toFixed(2);
}

function LaminaLifeTable({ lives, weakest }: { lives: NonNullable<FatigueLifeResult['lamina_lives']>; weakest: number }) {
  return (
    <div>
      <h4 className="text-sm font-semibold text-text-light mb-2 uppercase tracking-wider">
        Per-Lamina Life (ISO 16281)
      </h4>
      <div className="overflow-x-auto">
        <table className="w-full text-[13px] font-mono">
          <thead>
            <tr className="text-text-canvas border-b border-white/10">
              <th className="px-2 py-1 text-left">#</th>
              <th className="px-2 py-1 text-right">q_eq,i [N/mm]</th>
              <th className="px-2 py-1 text-right">q_eq,o [N/mm]</th>
              <th className="px-2 py-1 text-right">L₁₀,i [Mrev]</th>
              <th className="px-2 py-1 text-right">L₁₀,o [Mrev]</th>
            </tr>
          </thead>
          <tbody>
            {lives.map(l => (
              <tr
                key={l.k}
                className={`border-b border-white/5 hover:bg-white/5 ${
                  l.k === weakest ? 'bg-amber-500/10' : ''
                }`}
              >
                <td className="px-2 py-0.5 text-text-canvas">
                  {l.k}
                  {l.k === weakest && <span className="ml-1 text-amber-400">★</span>}
                </td>
                <td className="px-2 py-0.5 text-right text-text-light">{l.q_equiv_inner.toFixed(2)}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{l.q_equiv_outer.toFixed(2)}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{l.l_10_inner.toFixed(2)}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{l.l_10_outer.toFixed(2)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
