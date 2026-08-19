// @ts-nocheck
// CRB Phase 1.4 stub: 이 컴포넌트는 TRB 데이터 모델을 참조 중 → Phase 6 (Frontend UI 변경) 에서 CRB 로 정식 재작성 예정
import { useState } from 'react';
import { useAppState } from '../../store';
import { useActiveResult } from '../../hooks/useActiveResult';
import type { BearingResult, DualModeComparison, FatigueLifeResult } from '../../types/bearing';

export default function ResultsCard() {
  const { state, dispatch } = useAppState();
  const { dualResult, resultsPanelOpen } = state;
  const result = useActiveResult();

  const toggle = () => dispatch({ type: 'TOGGLE_RESULTS_PANEL' });

  // Collapsed state: thin strip with toggle button
  if (!resultsPanelOpen) {
    return (
      <aside className="shrink-0 bg-glass-bg border-l border-glass-border flex flex-col items-center py-3">
        <button
          onClick={toggle}
          className="w-7 h-7 flex items-center justify-center rounded hover:bg-white/10 text-text-canvas transition-colors cursor-pointer"
          title="Show Results"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
            <polyline points="15 18 9 12 15 6" />
          </svg>
        </button>
        <span className="text-xs text-text-canvas mt-2 [writing-mode:vertical-rl] rotate-180 tracking-wider">
          Summary
        </span>
      </aside>
    );
  }

  return (
    <aside className="w-72 shrink-0 bg-glass-bg backdrop-blur-xl border-l border-glass-border flex flex-col min-h-0">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-2.5 border-b border-white/5 shrink-0">
        <div className="flex items-center gap-2">
          <h3 className="text-sm font-semibold text-text-light uppercase tracking-wider">Summary</h3>
          {result && (
            <>
              <span className="text-xs font-mono text-text-canvas">{formatTime(result.elapsed_ms)}</span>
              <span className={`text-xs px-1.5 py-0.5 rounded font-mono ${
                result.mode === 'Gen1' ? 'bg-blue-500/20 text-blue-300' : 'bg-emerald-500/20 text-emerald-300'
              }`}>
                {result.mode}
              </span>
            </>
          )}
        </div>
        <button
          onClick={toggle}
          className="w-6 h-6 flex items-center justify-center rounded hover:bg-white/10 text-text-canvas transition-colors cursor-pointer"
          title="Hide Results"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
            <polyline points="9 18 15 12 9 6" />
          </svg>
        </button>
      </div>

      {/* Scrollable content */}
      <div className="flex-1 overflow-y-auto custom-scrollbar">
        {!result ? (
          <div className="flex items-center justify-center h-full">
            <p className="text-text-canvas text-sm">No results yet</p>
          </div>
        ) : (
          <div className="p-4 space-y-3">
            {/* Load */}
            <ResultRow label="Q_max" value={Math.max(...result.equilibrium.roller_loads).toFixed(0)} unit="N" />
            <ResultRow
              label="p_max (inner)"
              value={Math.max(
                ...result.equilibrium.roller_results.flatMap(r => r.slice_results.map(s => s.p_max_k))
              ).toFixed(0)}
              unit="MPa"
            />
            <ResultRow
              label="p_max (outer)"
              value={Math.max(
                ...result.equilibrium.roller_results.flatMap(r => r.slice_results.map(s => s.p_max_k_outer))
              ).toFixed(0)}
              unit="MPa"
            />
            <ResultRow
              label="Loaded rollers"
              value={`${result.equilibrium.roller_loads.filter(q => q > 0).length}/${result.equilibrium.roller_loads.length}`}
            />

            {/* Displacement */}
            <Section title="Displacement">
              <div className="grid grid-cols-3 gap-x-3 gap-y-1">
                <DataCell label="δx" value={result.equilibrium.displacement[0].toFixed(2)} unit="μm" />
                <DataCell label="δy" value={result.equilibrium.displacement[1].toFixed(2)} unit="μm" />
                <DataCell label="δz" value={result.equilibrium.displacement[2].toFixed(2)} unit="μm" />
              </div>
              {result.preload_mode && (
                <div className="mt-1.5 space-y-1">
                  <ResultRow label="Preload mode" value={
                    result.preload_mode === 'DisplacementFromForce' ? 'Disp←Force' :
                    result.preload_mode === 'DisplacementFromForceIterative' ? 'Disp←Force (iter.)' :
                    'Displacement'
                  } />
                  <ResultRow label="δ_preload" value={result.delta_preload_um.toFixed(2)} unit="μm" />
                  <ResultRow label="F_a reaction" value={result.f_a_reaction_kn.toFixed(3)} unit="kN" />
                </div>
              )}
            </Section>

            {/* Stiffness */}
            <Section title="Stiffness">
              <ResultRow label="K_radial" value={formatStiffness(result.k_radial)} unit="N/μm" />
              <ResultRow label="K_axial" value={formatStiffness(result.k_axial)} unit="N/μm" />
            </Section>

            {/* Life — detailed ISO 16281 rating */}
            <LifeDetailSection
              life={result.life}
              designLifeHours={state.input.operating.design_life_hours}
              nRpm={Math.abs(state.input.operating.n_inner_rpm - state.input.operating.n_outer_rpm)}
            />

            {/* Static Rating (ISO 76 + ISO 17956) */}
            <Section title="Static Rating (ISO 76 / 17956)">
              <ResultRow label="C₀ᵣ" value={result.static_rating.c_0r_kn.toFixed(1)} unit="kN" />
              <ResultRow label="P₀ᵣ" value={result.static_rating.p_0r_kn.toFixed(2)} unit="kN" />
              <ResultRow label="S₀ (ISO 76)" value={result.static_rating.s_0.toFixed(2)} />
              <ResultRow label="S₀,eff (17956)" value={result.static_rating.s_0_eff.toFixed(2)} />
              <div className={`mt-1 px-2 py-1 rounded text-xs font-medium ${
                result.static_rating.s_0_adequate
                  ? 'bg-emerald-500/20 text-emerald-300'
                  : 'bg-red-500/20 text-red-300'
              }`}>
                {result.static_rating.s_0_adequate
                  ? `S₀,eff ≥ S₀,min (${state.input.solver.f_s_min})`
                  : `S₀,eff < S₀,min (${state.input.solver.f_s_min}) — Inadequate!`}
              </div>
            </Section>

            {/* Dual comparison */}
            {dualResult && <DualSummary gen1={dualResult.gen1_result} gen3={dualResult.gen3_result} dualResult={dualResult} />}

            {/* Transient summary */}
            {state.transientResult && (
              <Section title="Transient Analysis">
                <ResultRow label="Snapshots" value={String(state.transientResult.snapshots.length)} />
                <ResultRow label="Duration" value={state.transientResult.total_time_s.toFixed(4)} unit="s" />
                <ResultRow label="Slip events" value={String(state.transientResult.damage_summary.total_slip_events)} />
                <ResultRow label="Max SRR" value={(state.transientResult.damage_summary.max_slip_ratio_overall * 100).toFixed(3)} unit="%" />
                <ResultRow label="WEC index" value={state.transientResult.damage_summary.wec_risk_index.toFixed(4)} />
                {state.transientResult.risk_assessment && (
                  <div className={`mt-1 px-2 py-1 rounded text-xs font-medium ${
                    state.transientResult.risk_assessment.overall_risk_level === 'Low'
                      ? 'bg-emerald-500/20 text-emerald-300'
                      : state.transientResult.risk_assessment.overall_risk_level === 'Medium'
                      ? 'bg-amber-500/20 text-amber-300'
                      : state.transientResult.risk_assessment.overall_risk_level === 'High'
                      ? 'bg-orange-500/20 text-orange-300'
                      : 'bg-red-500/20 text-red-300'
                  }`}>
                    Risk: {state.transientResult.risk_assessment.overall_risk_level}
                  </div>
                )}
                <ResultRow label="Elapsed" value={formatTime(state.transientResult.elapsed_ms)} />
              </Section>
            )}
          </div>
        )}
      </div>
    </aside>
  );
}

/** Weibull reliability at time T given L_10 life (both in same units).
 *  R(T) = exp(-(T/η)^e) where η = L_10 / (-ln(0.9))^(1/e), e = 10/9 (line contact). */
function weibullReliability(t: number, l10: number): number {
  if (!isFinite(l10) || l10 <= 0 || t <= 0) return 1;
  const e = 10 / 9; // Weibull slope for roller bearings (Lundberg-Palmgren)
  const eta = l10 / Math.pow(-Math.log(0.9), 1 / e);
  return Math.exp(-Math.pow(t / eta, e));
}

function LifeDetailSection({ life, designLifeHours, nRpm }: {
  life: FatigueLifeResult;
  designLifeHours: number;
  nRpm: number;
}) {
  const [expanded, setExpanded] = useState(false);
  const im = life.intermediates;

  // Basic reference life in hours
  const l10rh = nRpm > 1e-3 ? life.l_10_basic * 1e6 / (60 * nRpm) : Infinity;
  // Modified reference life
  const l10mr = life.a_iso * life.l_10_basic;
  const l10mrh = life.l_nm_hours;

  // Damage %
  const dmgBasic = isFinite(l10rh) ? (designLifeHours / l10rh) * 100 : 0;
  const dmgMod = isFinite(l10mrh) ? (designLifeHours / l10mrh) * 100 : 0;

  // Reliability / Unreliability via Weibull
  const relBasic = weibullReliability(designLifeHours, l10rh) * 100;
  const unrelBasic = 100 - relBasic;
  const relMod = weibullReliability(designLifeHours, l10mrh) * 100;
  const unrelMod = 100 - relMod;

  const methodLabel = 'ISO 16281';

  // Collapsed: show key summary
  if (!expanded) {
    return (
      <div className="border-t border-white/5 pt-3">
        <button
          onClick={() => setExpanded(true)}
          className="w-full flex items-center justify-between cursor-pointer group"
        >
          <p className="text-[13px] text-text-canvas">Fatigue Life ({methodLabel})</p>
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"
            className="text-text-canvas group-hover:text-text-light transition-colors">
            <polyline points="6 9 12 15 18 9" />
          </svg>
        </button>
        <div className="space-y-1 mt-2">
          <ResultRow label="L₁₀ᵣ" value={life.l_10_basic.toFixed(2)} unit="Mrev" />
          <ResultRow label="L₁₀ᵣₕ" value={formatHours(l10rh)} unit="hrs" />
          <ResultRow label="L₁₀ₘᵣ" value={l10mr.toFixed(2)} unit="Mrev" />
          <ResultRow label="L₁₀ₘᵣₕ" value={formatHours(l10mrh)} unit="hrs" />
          <ResultRow label="Damage" value={dmgMod.toFixed(1)} unit="%" />
          <ResultRow label="Reliability" value={relMod.toFixed(1)} unit="%" />
        </div>
      </div>
    );
  }

  return (
    <div className="border-t border-white/5 pt-3">
      <button
        onClick={() => setExpanded(false)}
        className="w-full flex items-center justify-between cursor-pointer group"
      >
        <p className="text-[13px] text-text-canvas font-semibold">{methodLabel} Rating</p>
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"
          className="text-text-canvas group-hover:text-text-light transition-colors">
          <polyline points="6 15 12 9 18 15" />
        </svg>
      </button>

      <div className="space-y-0.5 mt-2 text-[12px]">
        {/* Reference parameters */}
        <SubHeader title="Reference Parameters" />
        <ResultRow label="Duration" value={designLifeHours.toFixed(0)} unit="hr" />
        <ResultRow label="P_ref" value={life.p_ref.toFixed(4)} unit="kN" tooltip="C_r / L₁₀ᵣ(ISO16281)^(3/10)" />
        <ResultRow label="P_ref,dmg" value={life.p_ref_damage.toFixed(4)} unit="kN" tooltip="Damage-weighted avg P_sk — 에지 로딩 진단" />
        <ResultRow label="P_ref ratio" value={life.p_ref > 0 ? (life.p_ref_damage / life.p_ref).toFixed(3) : '-'} tooltip="dmg/bc — >1이면 하중 집중" />
        <ResultRow label="C_r" value={life.c_dyn.toFixed(4)} unit="kN" />
        <ResultRow label="C_u" value={im.c_u_kn.toFixed(4)} unit="kN" />

        {/* Load factors */}
        <SubHeader title="Load Factors" />
        <ResultRow label="F_a/F_r" value={isFinite(im.f_a_over_f_r) ? im.f_a_over_f_r.toFixed(4) : '∞'} />
        <ResultRow label="e (demarcation)" value={im.e_demarcation.toFixed(4)} />
        <ResultRow label="X" value={im.x_factor.toFixed(1)} />
        <ResultRow label="Y" value={im.y_factor.toFixed(4)} />

        {/* Geometry factors */}
        <SubHeader title="Geometry / Material Factors" />
        <ResultRow label="γ" value={im.gamma_bearing.toFixed(4)} />
        <ResultRow label="b_m" value={im.b_m.toFixed(1)} />
        <ResultRow label="f_c" value={im.f_c.toFixed(2)} />

        {/* Lamina capacity */}
        <SubHeader title="Dynamic Load Rating (Lamina)" />
        <ResultRow label="q_ci" value={(im.q_c_lamina_inner / 1000).toFixed(4)} unit="kN" />
        <ResultRow label="q_ce" value={(im.q_c_lamina_outer / 1000).toFixed(4)} unit="kN" />

        {/* Roller capacity */}
        <SubHeader title="Dynamic Load Rating (Roller)" />
        <ResultRow label="Q_ci" value={(im.q_ci / 1000).toFixed(4)} unit="kN" />
        <ResultRow label="Q_ce" value={(im.q_co / 1000).toFixed(4)} unit="kN" />
        <ResultRow label="f_ci" value={im.f_ci.toFixed(4)} />
        <ResultRow label="f_co" value={im.f_co.toFixed(4)} />

        {/* Lubrication */}
        <SubHeader title="Lubrication" />
        <ResultRow label="ν (T_op)" value={im.nu_actual.toFixed(2)} unit="mm²/s" />
        <ResultRow label="ν₁ (ref)" value={im.nu_ref.toFixed(2)} unit="mm²/s" />
        {life.kappa_inner === life.kappa_outer ? (
          <ResultRow label="κ" value={life.kappa_inner.toFixed(4)} unit={im.kappa_method === 'FilmThicknessRatio' ? `(Λ=${im.lambda_inner?.toFixed(2)})` : '(ν/ν₁)'} />
        ) : (
          <>
            <ResultRow label="κ (inner)" value={life.kappa_inner.toFixed(4)} unit={`Λ=${im.lambda_inner?.toFixed(2)}`} />
            <ResultRow label="κ (outer)" value={life.kappa_outer.toFixed(4)} unit={`Λ=${im.lambda_outer?.toFixed(2)}`} />
          </>
        )}
        <ResultRow label="e_c" value={(im.c_u_over_p / (im.c_u_kn / life.p_equiv)).toFixed(2)} />

        {/* Basic Reference Rating Life */}
        <SubHeader title="Basic Reference Rating Life" />
        <ResultRow label="L₁₀ᵣ (×10⁶)" value={life.l_10_basic.toFixed(4)} unit="" />
        <ResultRow label="  inner" value={life.l_10_inner.toFixed(4)} unit="" />
        <ResultRow label="  outer" value={life.l_10_outer.toFixed(4)} unit="" />
        <ResultRow label="L₁₀ᵣₕ" value={l10rh.toFixed(4)} unit="hr" />
        <DamageRow label="Damage" value={dmgBasic} />
        <ReliabilityRow label="Reliability" value={relBasic} />
        <ResultRow label="Unreliability" value={unrelBasic.toFixed(2)} unit="%" />

        {/* Life modification factors */}
        <SubHeader title="Life Modification Factors" />
        <ResultRow label="a₁ (reliability)" value="1" />
        <ResultRow label="a_ISO (systems)" value={life.a_iso.toFixed(4)} />
        <ResultRow label="C_u/P" value={im.c_u_over_p.toFixed(4)} />

        {/* Modified Reference Rating Life */}
        <SubHeader title="Modified Reference Rating Life" />
        <ResultRow label="L₁₀ₘᵣ (×10⁶)" value={l10mr.toFixed(4)} unit="" />
        <ResultRow label="L₁₀ₘᵣₕ" value={l10mrh.toFixed(4)} unit="hr" />
        <DamageRow label="Damage" value={dmgMod} />
        <ReliabilityRow label="Reliability" value={relMod} />
        <ResultRow label="Unreliability" value={unrelMod.toFixed(2)} unit="%" />

        {/* Weakest lamina */}
        <SubHeader title="Weakest Lamina" />
        <ResultRow label="Lamina index" value={String(life.weakest_lamina)} />
      </div>
    </div>
  );
}

function SubHeader({ title }: { title: string }) {
  return (
    <p className="text-[11px] text-text-canvas/60 uppercase tracking-wider pt-2 pb-0.5 border-b border-white/5">
      {title}
    </p>
  );
}

function DamageRow({ label, value }: { label: string; value: number }) {
  const color = value > 100 ? 'text-red-400' : value > 80 ? 'text-amber-300' : 'text-emerald-300';
  return (
    <div className="flex items-center justify-between">
      <span className="text-[13px] text-text-canvas">{label}</span>
      <span className={`text-sm font-mono tabular-nums ${color}`}>
        {value.toFixed(2)}%
      </span>
    </div>
  );
}

function ReliabilityRow({ label, value }: { label: string; value: number }) {
  const color = value >= 90 ? 'text-emerald-300' : value >= 50 ? 'text-amber-300' : 'text-red-400';
  return (
    <div className="flex items-center justify-between">
      <span className="text-[13px] text-text-canvas">{label}</span>
      <span className={`text-sm font-mono tabular-nums ${color}`}>
        {value.toFixed(2)}%
      </span>
    </div>
  );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div className="border-t border-white/5 pt-3">
      <p className="text-[13px] text-text-canvas mb-2">{title}</p>
      <div className="space-y-1">{children}</div>
    </div>
  );
}

function DualSummary({ gen1, gen3, dualResult }: {
  gen1: BearingResult;
  gen3: BearingResult;
  dualResult: DualModeComparison;
}) {
  const g1pMax = Math.max(...gen1.equilibrium.roller_results.flatMap(r => r.slice_results.map(s => s.p_max_k)));
  const g3pMax = Math.max(...gen3.equilibrium.roller_results.flatMap(r => r.slice_results.map(s => s.p_max_k)));
  const g1pMaxOuter = Math.max(...gen1.equilibrium.roller_results.flatMap(r => r.slice_results.map(s => s.p_max_k_outer)));
  const g3pMaxOuter = Math.max(...gen3.equilibrium.roller_results.flatMap(r => r.slice_results.map(s => s.p_max_k_outer)));
  const g1qMax = Math.max(...gen1.equilibrium.roller_loads);
  const g3qMax = Math.max(...gen3.equilibrium.roller_loads);
  const speedRatio = dualResult.gen1_elapsed_ms > 0
    ? (dualResult.gen3_elapsed_ms / dualResult.gen1_elapsed_ms).toFixed(1)
    : '-';

  return (
    <div className="border-t border-white/5 pt-3">
      <p className="text-[13px] text-text-canvas mb-2 font-semibold">Gen1 vs Gen3</p>

      {/* Side-by-side metrics */}
      <div className="grid grid-cols-3 gap-x-2 text-[13px] mb-2">
        <span className="text-text-canvas"></span>
        <span className="text-blue-300 text-center font-mono">Gen1</span>
        <span className="text-emerald-300 text-center font-mono">Gen3</span>

        <span className="text-text-canvas">Q_max</span>
        <span className="text-text-light text-center font-mono">{g1qMax.toFixed(0)}</span>
        <span className="text-text-light text-center font-mono">{g3qMax.toFixed(0)}</span>

        <span className="text-text-canvas">p_max,i</span>
        <span className="text-text-light text-center font-mono">{g1pMax.toFixed(0)}</span>
        <span className="text-text-light text-center font-mono">{g3pMax.toFixed(0)}</span>

        <span className="text-text-canvas">p_max,o</span>
        <span className="text-text-light text-center font-mono">{g1pMaxOuter.toFixed(0)}</span>
        <span className="text-text-light text-center font-mono">{g3pMaxOuter.toFixed(0)}</span>

        <span className="text-text-canvas">L₁₀</span>
        <span className="text-text-light text-center font-mono">{gen1.life.l_10_basic.toFixed(1)}</span>
        <span className="text-text-light text-center font-mono">{gen3.life.l_10_basic.toFixed(1)}</span>

        <span className="text-text-canvas">Time</span>
        <span className="text-text-light text-center font-mono">{formatTime(dualResult.gen1_elapsed_ms)}</span>
        <span className="text-text-light text-center font-mono">{formatTime(dualResult.gen3_elapsed_ms)}</span>
      </div>

      {/* Speed ratio */}
      <div className="flex items-center justify-between text-[13px] mb-2">
        <span className="text-text-canvas">Gen3/Gen1 speed</span>
        <span className="font-mono text-text-light">{speedRatio}x</span>
      </div>

      {/* Deltas */}
      <DeltaRow label="Δp_max" value={dualResult.delta_p_max_pct} />
      <DeltaRow label="ΔQ_max" value={dualResult.delta_q_max_pct} />
      <DeltaRow label="ΔL₁₀" value={dualResult.delta_l10_pct} />
      <div className={`mt-2 px-2.5 py-1.5 rounded text-[13px] ${
        dualResult.gen3_recommended
          ? 'bg-amber-500/20 text-amber-300'
          : 'bg-emerald-500/20 text-emerald-300'
      }`}>
        {dualResult.gen3_recommended ? 'Gen3 recommended' : 'Gen1 sufficient'}
        <span className="ml-1 opacity-60">({formatTime(dualResult.total_elapsed_ms)})</span>
      </div>
    </div>
  );
}

function ResultRow({ label, value, unit, tooltip }: { label: string; value: string; unit?: string; tooltip?: string }) {
  return (
    <div className="flex items-center justify-between" title={tooltip}>
      <span className="text-[13px] text-text-canvas">{label}</span>
      <span className="text-sm text-text-light font-mono tabular-nums">
        {value}
        {unit && <span className="text-[13px] text-text-canvas ml-1">{unit}</span>}
      </span>
    </div>
  );
}

function DataCell({ label, value, unit }: { label: string; value: string; unit: string }) {
  return (
    <div className="text-[13px]">
      <span className="text-text-canvas">{label}=</span>
      <span className="text-text-light font-mono">{value}</span>
      <span className="text-text-canvas ml-0.5">{unit}</span>
    </div>
  );
}

function DeltaRow({ label, value }: { label: string; value: number }) {
  const color = Math.abs(value) > 5 ? 'text-amber-300' : 'text-emerald-300';
  return (
    <div className="flex items-center justify-between">
      <span className="text-[13px] text-text-canvas">{label}</span>
      <span className={`text-sm font-mono tabular-nums ${color}`}>
        {value > 0 ? '+' : ''}{value.toFixed(1)}%
      </span>
    </div>
  );
}

function formatHours(hours: number): string {
  if (!isFinite(hours) || hours > 1e9) return '∞';
  if (hours > 10000) return `${(hours / 1000).toFixed(1)}k`;
  return hours.toFixed(0);
}

function formatStiffness(v: number): string {
  if (!v || !isFinite(v)) return '-';
  if (v >= 1e6) return `${(v / 1e6).toFixed(1)}M`;
  if (v >= 1e3) return `${(v / 1e3).toFixed(1)}k`;
  return v.toFixed(0);
}

function formatTime(ms: number): string {
  if (!ms || !isFinite(ms)) return '-';
  if (ms < 1) return `${(ms * 1000).toFixed(0)}μs`;
  if (ms < 1000) return `${ms.toFixed(0)}ms`;
  return `${(ms / 1000).toFixed(2)}s`;
}
