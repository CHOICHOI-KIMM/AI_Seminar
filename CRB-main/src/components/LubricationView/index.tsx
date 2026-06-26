import { useState, useMemo, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import Plot from '../charts/PlotWithCopy';
import { useActiveResult } from '../../hooks/useActiveResult';
import { useAppState } from '../../store';
import { darkLayout, plotConfig, viridisScale } from '../charts/plotlyDefaults';
import type {
  BearingResult,
  FilmThicknessResult,
  TractionSummary,
  RollerTractionResult,
  RollerFilmDistribution,
  LubricationModel,
  OperatingConditions,
  HMEHLResult,
} from '../../types/bearing';

type LubSection = 'health' | 'film' | 'friction' | 'rib' | 'diagnostic';

export default function LubricationView() {
  const result = useActiveResult();
  const { state } = useAppState();
  const lubModel: LubricationModel = state.input.operating.lubrication_model;
  const [section, setSection] = useState<LubSection>('health');

  if (!result) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-text-canvas text-sm">No results yet</p>
      </div>
    );
  }

  const film = result.life.film_thickness;
  const traction = result.traction;

  if (!film) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-text-canvas text-sm">Film thickness data not available</p>
      </div>
    );
  }

  const hasRibEhl = result.equilibrium.roller_results.some(r => r.rib_result?.ehl != null);
  const sections: { key: LubSection; label: string }[] = [
    { key: 'health',     label: 'Health' },
    { key: 'film',       label: 'Film' },
    { key: 'friction',   label: 'Friction' },
    ...(hasRibEhl ? [{ key: 'rib' as const, label: 'Rib EHL' }] : []),
    { key: 'diagnostic', label: 'Diagnostic' },
  ];

  return (
    <div className="flex flex-col h-full">
      {/* Sub-tab bar */}
      <div className="flex items-center gap-1 px-4 py-2 border-b border-white/5 shrink-0">
        {sections.map(s => (
          <button
            key={s.key}
            onClick={() => setSection(s.key)}
            className={`px-3 py-1 text-xs font-medium rounded transition-colors cursor-pointer ${
              section === s.key
                ? 'bg-white/10 text-text-light'
                : 'text-text-canvas hover:text-text-light hover:bg-white/5'
            }`}
          >
            {s.label}
          </button>
        ))}
        <ModelBadge model={lubModel} />
        <RegimeBadge regime={film.regime} lambda={film.lambda_ratio} />
      </div>

      {/* Operating Context Strip — always visible */}
      <OperatingContextStrip operating={state.input.operating} film={film} />

      {/* Content */}
      <div className="flex-1 overflow-auto custom-scrollbar">
        {section === 'health' && <LubOverview film={film} traction={traction} model={lubModel} hasRibEhl={hasRibEhl} result={result} />}
        {section === 'film' && (
          <>
            <FilmThicknessDetail film={film} model={lubModel} traction={traction} />
            <FilmDistributionView film={film} result={result} />
          </>
        )}
        {section === 'friction' && (
          <FrictionTab film={film} traction={traction} model={lubModel} result={result} />
        )}
        {section === 'rib' && <RibEhlSection result={result} />}
        {section === 'diagnostic' && <DiagnosticTab film={film} />}
      </div>
    </div>
  );
}

// ─── Operating Context Strip ──────────────────────────────────────
//
// Always-visible strip at the top of LubricationView showing the operating
// conditions that produced the current results.  Lets users switch between
// sub-tabs without losing the (n, ν, T, F) context.

function OperatingContextStrip({ operating, film }: {
  operating: OperatingConditions; film: FilmThicknessResult;
}) {
  const nRel = Math.abs(operating.n_inner_rpm - operating.n_outer_rpm);
  const f_radial = Math.hypot(operating.f_x, operating.f_y);
  return (
    <div className="flex flex-wrap items-center gap-x-4 gap-y-1 px-4 py-1.5 border-b border-white/5 bg-white/[0.015] shrink-0 text-[11px] font-mono text-text-canvas">
      <span><span className="text-text-muted">n_in/out</span> {operating.n_inner_rpm.toFixed(0)}/{operating.n_outer_rpm.toFixed(0)} <span className="text-text-muted">rpm</span> (Δ{nRel.toFixed(0)})</span>
      <span className="text-white/10">│</span>
      <span><span className="text-text-muted">ν₄₀/T_op</span> {operating.nu_40.toFixed(0)} cSt @ {operating.t_op.toFixed(0)}°C</span>
      <span className="text-white/10">│</span>
      <span><span className="text-text-muted">F_r/F_a</span> {(f_radial / 1000).toFixed(1)}/{(operating.f_a / 1000).toFixed(1)} <span className="text-text-muted">kN</span></span>
      <span className="text-white/10">│</span>
      <span><span className="text-text-muted">u_m</span> {film.u_mean_m_s.toFixed(2)}/{film.u_mean_m_s_outer.toFixed(2)} <span className="text-text-muted">m/s</span></span>
      <span className="text-white/10">│</span>
      <span><span className="text-text-muted">ω_cage/ω_roll</span> {film.cage_speed_rpm.toFixed(0)}/{film.roller_spin_rpm.toFixed(0)} <span className="text-text-muted">rpm</span></span>
    </div>
  );
}

// ─── Model Badge ──────────────────────────────────────────────────

const MODEL_LABELS: Record<LubricationModel, string> = {
  Method1_DH: 'M1 — Dowson-Higginson',
  Method2_MK: 'M2 — Masjedi-Khonsari',
  Method3_NVM: 'M3 — Nijenbanning-Venner-Moes',
};

function ModelBadge({ model }: { model: LubricationModel }) {
  const isM2 = model === 'Method2_MK' || model === 'Method3_NVM';
  return (
    <span className={`px-2 py-0.5 rounded text-xs font-semibold border ${
      isM2
        ? 'bg-violet-500/15 text-violet-300 border-violet-500/20'
        : 'bg-slate-500/15 text-slate-300 border-slate-500/20'
    }`}>
      {MODEL_LABELS[model]}
    </span>
  );
}

// ─── Regime Badge ──────────────────────────────────────────────────

function RegimeBadge({ regime, lambda }: { regime: string; lambda: number }) {
  const config = {
    FullEhl: { label: 'Full EHL', color: 'emerald', desc: 'Λ > 3' },
    Mixed: { label: 'Mixed', color: 'amber', desc: '1 ≤ Λ ≤ 3' },
    Boundary: { label: 'Boundary', color: 'red', desc: 'Λ < 1' },
  }[regime] ?? { label: regime, color: 'slate', desc: '' };

  const colorMap: Record<string, string> = {
    emerald: 'bg-emerald-500/15 text-emerald-300 border-emerald-500/20',
    amber: 'bg-amber-500/15 text-amber-300 border-amber-500/20',
    red: 'bg-red-500/15 text-red-300 border-red-500/20',
    slate: 'bg-slate-500/15 text-slate-300 border-slate-500/20',
  };

  return (
    <div className="ml-auto flex items-center gap-2">
      <span className={`px-2 py-0.5 rounded text-xs font-semibold border ${colorMap[config.color]}`}>
        {config.label}
      </span>
      <span className="text-xs text-text-canvas font-mono">
        Λ = {lambda.toFixed(2)} ({config.desc})
      </span>
    </div>
  );
}

// ─── Film Thickness Detail ─────────────────────────────────────────

// Two-column comparison table (Inner | Outer) — replaces the duplicate
// "Inner Raceway" / "Outer Raceway" tables.
function InnerOuterTable({ title, rows }: {
  title: string;
  rows: { label: string; inner: string; outer: string; unit?: string }[];
}) {
  return (
    <div>
      <h4 className="text-xs font-semibold text-text-light mb-2 uppercase tracking-wider">{title}</h4>
      <table className="w-full text-xs font-mono">
        <thead>
          <tr className="text-text-canvas border-b border-white/5">
            <th className="px-2 py-1 text-left font-medium"></th>
            <th className="px-2 py-1 text-right font-medium">Inner</th>
            <th className="px-2 py-1 text-right font-medium">Outer</th>
            <th className="px-2 py-1 text-left font-medium pl-3 w-20">Unit</th>
          </tr>
        </thead>
        <tbody>
          {rows.map((r, i) => (
            <tr key={i} className="border-b border-white/5">
              <td className="px-2 py-0.5 text-text-canvas">{r.label}</td>
              <td className="px-2 py-0.5 text-right text-text-light">{r.inner}</td>
              <td className="px-2 py-0.5 text-right text-text-light">{r.outer}</td>
              <td className="px-2 py-0.5 text-left text-text-muted/70 pl-3">{r.unit ?? ''}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function FilmThicknessDetail({ film, model, traction }: { film: FilmThicknessResult; model: LubricationModel; traction: TractionSummary | null }) {
  const isAdv = model === 'Method2_MK';

  // Max-loaded roller traction data for sliding/SRR summary
  const maxRoller = traction?.rollers?.reduce((max, r) =>
    (r.inner.f_traction_n + r.outer.f_traction_n) > (max.inner.f_traction_n + max.outer.f_traction_n) ? r : max
  );

  return (
    <div className="p-4 space-y-4">
      {/* EHL film + roughness side-by-side per raceway (replaces 2 split tables) */}
      <InnerOuterTable
        title="EHL Film Thickness"
        rows={[
          { label: 'h_min', inner: film.h_min_um.toFixed(3), outer: film.h_min_um_outer.toFixed(3), unit: 'μm' },
          { label: 'h_c (central)', inner: film.h_central_um.toFixed(3), outer: film.h_central_um_outer.toFixed(3), unit: 'μm' },
          { label: 'Λ = h_min/σ', inner: film.lambda_ratio.toFixed(3), outer: film.lambda_ratio_outer.toFixed(3) },
          { label: 'Regime', inner: film.regime, outer: film.regime_outer },
          { label: 'σ composite', inner: film.sigma_composite_um.toFixed(3), outer: film.sigma_composite_um_outer.toFixed(3), unit: 'μm' },
          ...(maxRoller ? [
            { label: 'V_slide (worst roller)', inner: maxRoller.inner.u_sliding.toFixed(4), outer: maxRoller.outer.u_sliding.toFixed(4), unit: 'm/s' },
            { label: 'SRR (worst roller)', inner: maxRoller.inner.srr.toFixed(4), outer: maxRoller.outer.srr.toFixed(4) },
          ] : []),
        ]}
      />

      {/* Surface Roughness (effective Rq values used in σ calculation) */}
      <DetailTable title="Surface Roughness (Rq effective)" rows={[
        ['Roller Rq', `${film.rq_roller_um.toFixed(3)}`, 'μm'],
        ['Inner raceway Rq', `${film.rq_inner_um.toFixed(3)}`, 'μm'],
        ['Outer raceway Rq', `${film.rq_outer_um.toFixed(3)}`, 'μm'],
      ]} />

      {/* Operating Conditions */}
      <DetailTable title="Operating Parameters" rows={[
        ['Starvation factor (φ_s)', `${film.starvation_factor.toFixed(3)}`, isAdv ? '(physics)' : '(user)'],
        ['Thermal correction factor (φ_T)', `${film.thermal_factor.toFixed(4)}`, isAdv ? '(Murch-Wilson)' : '(Gupta)'],
        ...(isAdv && film.flash_temp_c != null ? [
          ['Flash temperature (ΔT_flash)', `${film.flash_temp_c.toFixed(1)}`, `°C (${film.flash_temp_c < 80 ? 'Low' : film.flash_temp_c < 150 ? 'Medium' : film.flash_temp_c < 300 ? 'High' : 'Critical'})`],
        ] as [string, string, string][] : []),
      ]} />

      {/* Dimensionless Parameters */}
      <DetailTable title={isAdv ? 'Dimensionless Groups (Masjedi-Khonsari)' : 'Dimensionless Groups (Dowson-Higginson)'} rows={[
        ['Speed parameter (U)', `${film.u_param.toExponential(4)}`, ''],
        ['Material parameter (G)', `${film.g_param.toFixed(1)}`, ''],
        ['Load parameter (W)', `${film.w_param.toExponential(4)}`, ''],
      ]} />

      {/* Formulas */}
      <div className="text-xs text-text-canvas space-y-1 mt-4">
        {isAdv ? (
          <>
            <p>Masjedi-Khonsari (2015): H_c = a₁·U^a₂·G^a₃·W^a₄·(1+a₅·σ̄^a₆·V^a₇·W^a₈)</p>
            <p>Roelands (1966): η(p) = η₀·exp[(ln η₀+9.67)·((1+p/p_r)^Z−1)]</p>
            <p>Wilson (1979): φ_T = 1/[1+0.1·(1+14.8·SRR⁰·⁸³)·L⁰·⁶⁴]</p>
            <p>Eyring: τ = τ₀·sinh⁻¹(η_eff·γ̇/τ₀), |τ| ≤ 0.1·p</p>
          </>
        ) : (
          <>
            <p>Dowson-Higginson (1977): H_min = 2.65·U⁰·⁷⁰·G⁰·⁵⁴·W⁻⁰·¹³</p>
            <p>Dowson-Toyoda (1978): H_c = 3.06·U⁰·⁶⁹·G⁰·⁵⁶·W⁻⁰·¹⁰</p>
            <p>Gupta TEHL: φ_T = 1/(1 + 0.1·L_th⁰·⁶⁴)</p>
            <p>h = H·R_eq, h_eff = h·φ_s·φ_T</p>
          </>
        )}
      </div>
    </div>
  );
}

// ─── Friction Tab — unified (Mixed lubrication + Traction + Power) ──
//
// Replaces the previous two-component flow (MixedLubricationDetail +
// TractionDetail) with a single tab that follows a clear top-down narrative:
//   1. Headline summary strip — 4 cards user reads first
//   2. Charts row (50/50) — power breakdown pie + per-roller bar
//   3. Tables row (50/50) — asperity sharing + traction & power
//   4. Per-roller traction table (full width)
//   5. Collapsible — Eyring traction curve + formulas (advanced reference)

function FrictionTab({ film, traction, model }: {
  film: FilmThicknessResult; traction: TractionSummary | null;
  model: LubricationModel; result: BearingResult;
}) {
  const { state } = useAppState();
  const isAdv = model === 'Method2_MK';
  const m = film.mixed;

  if (!traction) {
    return (
      <div className="p-4">
        {/* Asperity-only fallback when traction wasn't computed */}
        <SummaryCard title="Asperity Load Share" value={`${(m.asperity_load_ratio * 100).toFixed(1)} %`} hint={m.asperity_load_ratio > 0.5 ? 'boundary-dominated' : m.asperity_load_ratio > 0.1 ? 'mixed' : 'full EHL'} />
        <p className="text-text-canvas text-sm mt-4">Traction data not available — only mixed-lubrication asperity model results shown.</p>
      </div>
    );
  }

  const loadedRollers = traction.rollers.filter(r =>
    r.inner.f_traction_n > 0 || r.outer.f_traction_n > 0
  );

  // Dominant power-loss regime label
  const total = traction.p_contact_total_w;
  const dominant =
    total < 1e-6 ? '—'
    : traction.p_rolling_w >= traction.p_sliding_w && traction.p_rolling_w >= traction.p_rib_w ? 'rolling'
    : traction.p_sliding_w >= traction.p_rib_w ? 'sliding'
    : 'rib';

  // Asperity hint
  const asperityHint =
    m.asperity_load_ratio > 0.5 ? 'boundary-dominated'
    : m.asperity_load_ratio > 0.1 ? 'mixed regime'
    : 'full EHL (asperity ~0)';

  return (
    <div className="p-4 space-y-4">
      {/* Headline strip — 4 cards */}
      <div className="grid grid-cols-4 gap-3">
        <SummaryCard
          title="Effective friction / μ_eff"
          value={m.mu_effective.toFixed(4)}
          hint={`= (1−f_a)·μ_ehl + f_a·μ_bd, f_a=${(m.asperity_load_ratio * 100).toFixed(1)}%`}
        />
        <SummaryCard
          title="Total Contact Loss / P_total"
          value={`${traction.p_contact_total_w.toFixed(2)} W`}
          hint={`M_friction = ${traction.m_friction_nmm.toFixed(0)} N·mm · dominant: ${dominant}`}
        />
        <SummaryCard
          title="Asperity Load Share / f_a"
          value={`${(m.asperity_load_ratio * 100).toFixed(1)} %`}
          hint={asperityHint}
        />
        <SummaryCard
          title={film.flash_temp_c != null ? 'Flash temperature / ΔT_flash' : 'Friction Model'}
          value={film.flash_temp_c != null
            ? `${film.flash_temp_c.toFixed(0)} °C`
            : (traction.friction_model ?? 'Palmgren')}
          hint={film.flash_temp_c != null
            ? (film.flash_temp_c > 150 ? 'scuffing risk' : film.flash_temp_c > 80 ? 'medium' : 'low')
            : 'rolling resistance model'}
        />
      </div>

      {/* Charts row — 50/50 (Pie | Per-roller bar) */}
      <div className="grid grid-cols-2 gap-3" style={{ height: 320 }}>
        <div className="bg-white/[0.02] rounded-lg overflow-hidden border border-white/5">
          <PowerBreakdownChart traction={traction} />
        </div>
        <div className="bg-white/[0.02] rounded-lg overflow-hidden border border-white/5">
          <RollerPowerChart rollers={loadedRollers} />
        </div>
      </div>

      {/* Tables row — 50/50 (Asperity | Traction & Power) */}
      <div className="grid grid-cols-2 gap-4">
        <DetailTable
          title={isAdv ? 'Asperity Load Sharing (Masjedi-Khonsari)' : 'Asperity Load Sharing (Greenwood-Tripp)'}
          rows={[
            ['Asperity load ratio (F_asp/F_total)', `${(m.asperity_load_ratio * 100).toFixed(2)}`, '%'],
            ['Asperity area ratio (A_real/A_hertz)', `${(m.asperity_area_ratio * 100).toFixed(3)}`, '%'],
            ['Asperity contact pressure', `${m.p_asperity_mpa.toFixed(1)}`, 'MPa'],
            ['Fluid (EHL) pressure', `${m.p_fluid_mpa.toFixed(1)}`, 'MPa'],
            ...(m.f_5_2 > 0 ? [
              ['GT integral F_{5/2}(Λ)', `${m.f_5_2.toExponential(3)}`, ''],
              ['GT integral F_2(Λ)', `${m.f_2.toExponential(3)}`, ''],
            ] as [string, string, string][] : []),
          ]}
        />
        <div className="space-y-4">
          <DetailTable
            title={isAdv ? 'Traction Coefficients (Eyring + Roelands)' : 'Friction Coefficients'}
            rows={[
              ['EHL traction (μ_ehl)', `${m.mu_ehl.toFixed(5)}`, '(fluid)'],
              ['Boundary friction (μ_bd)', `${m.mu_boundary.toFixed(3)}`, '(metal-metal)'],
              ['Effective friction (μ_eff)', `${m.mu_effective.toFixed(5)}`, '(weighted)'],
            ]}
          />
          <DetailTable
            title="Power Loss Summary"
            rows={[
              ['Rolling friction power / P_roll', `${traction.p_rolling_w.toFixed(2)}`, 'W (BH viscous EHL)'],
              ['Sliding friction power / P_slide', `${traction.p_sliding_w.toFixed(2)}`, 'W'],
              ['Rib friction power / P_rib', `${traction.p_rib_w.toFixed(2)}`, 'W'],
              ...(traction.p_hysteresis_w > 0 ? [
                [`Material hysteresis power / P_hys (Johnson, α_v=${state.input.operating.hysteresis_loss_factor.toFixed(4)})`,
                  `${traction.p_hysteresis_w.toFixed(2)}`, 'W'],
              ] as [string, string, string][] : []),
              ['Total contact power loss / P_total', `${traction.p_contact_total_w.toFixed(2)}`, 'W'],
              ['Friction torque / M_friction', `${traction.m_friction_nmm.toFixed(2)}`, 'N·mm'],
            ]}
          />
        </div>
      </div>

      {/* Per-roller table — full width */}
      <RollerTractionTable rollers={loadedRollers} />

      {/* Collapsible — Eyring traction curve (M2 only) + formulas */}
      <CollapsibleSection
        title={isAdv ? 'Eyring traction curve (μ vs SRR)' : 'Friction model formulas'}
        available={true}
        defaultOpen={false}
        badge={isAdv ? 'τ = τ₀·sinh⁻¹(η_eff·γ̇/τ₀)' : undefined}
      >
        <div className="p-3 space-y-3">
          {isAdv && (
            <div style={{ height: 280 }}>
              <EyringTractionCurveChart operating={state.input.operating} film={film} />
            </div>
          )}
          <div className="text-xs text-text-canvas space-y-1">
            {isAdv ? (
              <>
                <p>Masjedi-Khonsari W_asp = f(U, G, W, σ̄, V) — regression-based asperity load fraction</p>
                <p>Eyring: τ = τ₀·sinh⁻¹(η_eff·γ̇/τ₀), |τ| ≤ 0.1·p (shear limit)</p>
                <p>η_eff = Roelands(η₀, p, Z_r): η₀·exp[(ln η₀+9.67)·((1+p/p_r)^Z−1)]</p>
                <p>μ_eff = (1−f_a)·μ_ehl + f_a·μ_boundary</p>
              </>
            ) : (
              <>
                <p>Greenwood-Tripp F_n(Λ) = ∫_Λ^∞ (s−Λ)^n·φ(s)ds, φ(s) = Gaussian PDF</p>
                <p>f_a = F_{'{5/2}'}(Λ) / F_{'{5/2}'}(0) — asperity load fraction</p>
                <p>μ_ehl = τ_0/p_mean (Eyring constant, τ₀=5 MPa)</p>
                <p>μ_eff = (1−f_a)·μ_ehl + f_a·μ_boundary</p>
                <p>P_contact = Σ_rollers (P_inner + P_outer + P_rib),  M_friction = P_contact / ω_cage</p>
              </>
            )}
          </div>
        </div>
      </CollapsibleSection>

      {/* Collapsible — External Reference Comparison (SKF Catalogue 2018) */}
      {traction.skf_reference && (
        <CollapsibleSection
          title="External Reference Comparison (SKF Catalogue 2018)"
          available={true}
          defaultOpen={false}
          badge="head-to-head: Ours vs SKF"
        >
          <ExternalReferenceComparison traction={traction} />
        </CollapsibleSection>
      )}
    </div>
  );
}

// Compact summary card for the Friction headline strip.
function SummaryCard({ title, value, hint }: { title: string; value: string; hint?: string }) {
  return (
    <div className="rounded-lg p-3 border border-white/10 bg-white/[0.03]">
      <h4 className="text-[11px] font-semibold text-text-muted uppercase mb-1">{title}</h4>
      <div className="text-base font-mono text-text-light">{value}</div>
      {hint && <div className="text-[11px] text-text-canvas/70 mt-1">{hint}</div>}
    </div>
  );
}

// ─── Eyring Traction Curve Chart (Advanced only) ──────────────────

function EyringTractionCurveChart({ operating, film }: { operating: OperatingConditions; film: FilmThicknessResult | null }) {
  const curveData = useMemo(() => {
    if (!film) return null;

    const eta_0_pa_s = operating.nu_40 * (operating.rho_oil || 850) / 1e6;
    const tau_0 = (operating.tau_eyring || 5.0) * 1e6; // Pa
    const z_r = operating.z_roelands || 0.67;
    const h_c = (film.h_central_um || 0.5) * 1e-6; // m
    const u_roll = film.u_mean_m_s || 1.0;

    // Estimate mean Hertzian pressure from w_param (rough)
    const p_mean = 500e6; // typical, Pa

    // Roelands viscosity
    const P_R = 196.2e6;
    const logTerm = (Math.log(eta_0_pa_s) + 9.67) * (Math.pow(1 + p_mean / P_R, z_r) - 1);
    const eta_eff = eta_0_pa_s * Math.exp(logTerm);

    // Generate μ vs SRR curve
    const srrValues: number[] = [];
    const muValues: number[] = [];
    for (let i = 0; i <= 100; i++) {
      const srr = i * 0.05; // 0 to 5.0
      const u_slide = Math.abs(srr) * u_roll;
      const gamma_dot = u_slide / Math.max(h_c, 1e-10);
      const x = eta_eff * gamma_dot / tau_0;
      const tau = tau_0 * Math.asinh(x);
      const tau_lim = 0.10 * p_mean;
      const tau_clamped = Math.min(tau, tau_lim);
      const mu = Math.min(tau_clamped / p_mean, 0.15);
      srrValues.push(srr);
      muValues.push(mu);
    }

    // Mark actual operating SRR from roller data
    return { srrValues, muValues, tau_lim: 0.10 };
  }, [operating, film]);

  if (!curveData) {
    return <div className="flex items-center justify-center h-full text-text-canvas text-xs">No film data</div>;
  }

  const data: Plotly.Data[] = [
    {
      type: 'scatter',
      x: curveData.srrValues,
      y: curveData.muValues,
      mode: 'lines',
      name: 'μ_ehl (Eyring)',
      line: { color: '#a78bfa', width: 2 },
    },
    {
      type: 'scatter',
      x: [0, 5],
      y: [curveData.tau_lim, curveData.tau_lim],
      mode: 'lines',
      name: 'τ_lim/p (shear limit)',
      line: { color: '#ef4444', width: 1, dash: 'dash' },
    },
  ];

  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: { text: 'Eyring Traction Curve', font: { size: 12, color: '#e2e8f0' } },
    showlegend: true,
    legend: { x: 0.5, y: 0.98, font: { size: 8, color: '#94a3b8' }, orientation: 'h', xanchor: 'center' },
    xaxis: {
      ...darkLayout.xaxis,
      title: { text: 'SRR', font: { size: 10, color: '#94a3b8' } },
      range: [0, 2],
    },
    yaxis: {
      ...darkLayout.yaxis,
      title: { text: 'μ_ehl', font: { size: 10, color: '#94a3b8' } },
      range: [0, 0.12],
    },
    margin: { l: 45, r: 10, t: 35, b: 40 },
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

// ─── Power Breakdown Pie Chart ─────────────────────────────────────

function PowerBreakdownChart({ traction }: { traction: TractionSummary }) {
  const values = [traction.p_rolling_w, traction.p_sliding_w, traction.p_rib_w];
  const labels = ['Rolling', 'Sliding', 'Rib'];
  const colors = ['#3b82f6', '#f59e0b', '#ef4444'];

  // Filter out zero values
  const filtered = values.map((v, i) => ({ v, l: labels[i], c: colors[i] })).filter(x => x.v > 0.001);

  const data: Plotly.Data[] = [{
    type: 'pie',
    values: filtered.map(x => x.v),
    labels: filtered.map(x => x.l),
    marker: { colors: filtered.map(x => x.c) },
    textinfo: 'label+percent',
    textfont: { size: 10, family: 'JetBrains Mono', color: '#e2e8f0' },
    hovertemplate: '%{label}: %{value:.2f} W<extra></extra>',
    hole: 0.4,
  }];

  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: { text: 'Power Loss Breakdown', font: { size: 12, color: '#e2e8f0' } },
    showlegend: false,
    margin: { l: 10, r: 10, t: 35, b: 10 },
    annotations: [{
      text: `${traction.p_contact_total_w.toFixed(1)}W`,
      showarrow: false,
      font: { size: 12, color: '#e2e8f0', family: 'JetBrains Mono' },
      x: 0.5,
      y: 0.5,
    }],
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

// ─── Per-Roller Power Bar Chart ────────────────────────────────────

function RollerPowerChart({ rollers }: { rollers: RollerTractionResult[] }) {
  const psi = rollers.map(r => r.psi_deg);
  const pInnerRoll = rollers.map(r => r.inner.p_rolling_w);
  const pInnerSlide = rollers.map(r => r.inner.power_loss_w);
  const pOuterRoll = rollers.map(r => r.outer.p_rolling_w);
  const pOuterSlide = rollers.map(r => r.outer.power_loss_w);
  const pRib = rollers.map(r => r.rib?.power_loss_w ?? 0);

  // Stack order: rolling first (largest at typical SRR≈0), then sliding,
  // then rib. Distinct shades for rolling vs sliding within the same raceway
  // so users can see at a glance how much is rolling resistance vs traction.
  const data: Plotly.Data[] = [
    {
      type: 'bar', x: psi, y: pInnerRoll,
      name: 'Inner rolling',
      marker: { color: '#3b82f6' }, // blue
    },
    {
      type: 'bar', x: psi, y: pInnerSlide,
      name: 'Inner sliding',
      marker: { color: '#93c5fd' }, // light blue
    },
    {
      type: 'bar', x: psi, y: pOuterRoll,
      name: 'Outer rolling',
      marker: { color: '#22c55e' }, // green
    },
    {
      type: 'bar', x: psi, y: pOuterSlide,
      name: 'Outer sliding',
      marker: { color: '#86efac' }, // light green
    },
    {
      type: 'bar', x: psi, y: pRib,
      name: 'Rib',
      marker: { color: '#ef4444' }, // red
    },
  ];

  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: { text: 'Per-Roller Power Loss', font: { size: 12, color: '#e2e8f0' } },
    barmode: 'stack',
    showlegend: true,
    legend: { x: 0.02, y: 0.98, font: { size: 9, color: '#94a3b8' } },
    xaxis: {
      ...darkLayout.xaxis,
      title: { text: 'ψ [deg]', font: { size: 10, color: '#94a3b8' } },
    },
    yaxis: {
      ...darkLayout.yaxis,
      title: { text: 'Power [W]', font: { size: 10, color: '#94a3b8' } },
    },
    margin: { l: 50, r: 10, t: 35, b: 40 },
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

// ─── Per-Roller Traction Table ─────────────────────────────────────

function RollerTractionTable({ rollers }: { rollers: RollerTractionResult[] }) {
  if (rollers.length === 0) return null;

  return (
    <div>
      <h4 className="text-xs font-semibold text-text-light mb-2 uppercase tracking-wider">
        Per-Roller Contact Friction ({rollers.length} loaded)
      </h4>
      <div className="overflow-x-auto">
        <table className="w-full text-xs font-mono">
          <thead>
            <tr className="text-text-canvas">
              <th className="px-2 py-0.5 text-right" rowSpan={2}>ψ [deg]</th>
              <th className="px-2 py-0.5 text-center border-b border-blue-400/30" colSpan={5}>
                <span className="text-blue-300">Inner</span>
              </th>
              <th className="px-2 py-0.5 text-center border-b border-emerald-400/30" colSpan={5}>
                <span className="text-emerald-300">Outer</span>
              </th>
              <th className="px-2 py-0.5 text-center border-b border-red-400/30" colSpan={2}>
                <span className="text-red-300">Rib</span>
              </th>
            </tr>
            <tr className="text-text-canvas border-b border-white/10">
              <th className="px-1.5 py-0.5 text-right">U_r [m/s]</th>
              <th className="px-1.5 py-0.5 text-right">SRR</th>
              <th className="px-1.5 py-0.5 text-right">μ</th>
              <th className="px-1.5 py-0.5 text-right">P_slide [W]</th>
              <th className="px-1.5 py-0.5 text-right">P_roll [W]</th>
              <th className="px-1.5 py-0.5 text-right">U_r [m/s]</th>
              <th className="px-1.5 py-0.5 text-right">SRR</th>
              <th className="px-1.5 py-0.5 text-right">μ</th>
              <th className="px-1.5 py-0.5 text-right">P_slide [W]</th>
              <th className="px-1.5 py-0.5 text-right">P_roll [W]</th>
              <th className="px-1.5 py-0.5 text-right">μ</th>
              <th className="px-1.5 py-0.5 text-right">P [W]</th>
            </tr>
          </thead>
          <tbody>
            {rollers.map((r, i) => (
              <tr key={i} className="border-b border-white/5 hover:bg-white/5">
                <td className="px-2 py-0.5 text-right text-text-light">{r.psi_deg.toFixed(1)}</td>
                <td className="px-1.5 py-0.5 text-right text-text-light">{r.inner.u_rolling.toFixed(4)}</td>
                <td className="px-1.5 py-0.5 text-right text-text-light">{r.inner.srr.toFixed(4)}</td>
                <td className="px-1.5 py-0.5 text-right text-text-light">{r.inner.mu.toFixed(5)}</td>
                <td className="px-1.5 py-0.5 text-right text-text-light">{r.inner.power_loss_w.toFixed(3)}</td>
                <td className="px-1.5 py-0.5 text-right text-text-light">{r.inner.p_rolling_w.toFixed(3)}</td>
                <td className="px-1.5 py-0.5 text-right text-text-light">{r.outer.u_rolling.toFixed(4)}</td>
                <td className="px-1.5 py-0.5 text-right text-text-light">{r.outer.srr.toFixed(4)}</td>
                <td className="px-1.5 py-0.5 text-right text-text-light">{r.outer.mu.toFixed(5)}</td>
                <td className="px-1.5 py-0.5 text-right text-text-light">{r.outer.power_loss_w.toFixed(3)}</td>
                <td className="px-1.5 py-0.5 text-right text-text-light">{r.outer.p_rolling_w.toFixed(3)}</td>
                <td className="px-1.5 py-0.5 text-right text-text-light">
                  {r.rib ? r.rib.mu.toFixed(4) : '-'}
                </td>
                <td className="px-1.5 py-0.5 text-right text-text-light">
                  {r.rib ? r.rib.power_loss_w.toFixed(3) : '-'}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

// ─── Film Distribution (Contour & 3D Surface) ────────────────

type FilmMapMode = 'hmin' | 'hc' | 'lambda';
type FilmChartMode = 'contour' | 'surface';
type FilmRaceway = 'inner' | 'outer';

function FilmDistributionView({ film, result }: { film: FilmThicknessResult; result: BearingResult }) {
  const [mapMode, setMapMode] = useState<FilmMapMode>('hmin');
  const [chartMode, setChartMode] = useState<FilmChartMode>('contour');
  const [raceway, setRaceway] = useState<FilmRaceway>('inner');
  const [selectedRollerIdx, setSelectedRollerIdx] = useState<number | null>(null);

  const filmDist = result.film_distribution;
  const rollers = result.equilibrium.roller_results;

  // Build 2D arrays directly from backend film distribution
  // (now computed at angular_distribution resolution, same as stress contour)
  const distData = useMemo(() => {
    if (!filmDist || filmDist.length === 0) return null;

    const nSlices = filmDist[0].slices.length;
    const psiValues: number[] = [];
    const zHmin: number[][] = [];
    const zHc: number[][] = [];
    const zLambda: number[][] = [];

    const isOuter = raceway === 'outer';
    for (const rd of filmDist) {
      psiValues.push(rd.psi_deg);
      // Non-contact zones (h_min=0) → NaN so Plotly renders them as transparent gaps
      // (EHL theory is not applicable outside the loaded zone)
      const hKey = isOuter ? 'h_min_um_outer' : 'h_min_um';
      const hcKey = isOuter ? 'h_central_um_outer' : 'h_central_um';
      const lKey = isOuter ? 'lambda_outer' : 'lambda';
      zHmin.push(rd.slices.map(s => s[hKey] > 0 ? s[hKey] : NaN));
      zHc.push(rd.slices.map(s => s[hKey] > 0 ? s[hcKey] : NaN));
      zLambda.push(rd.slices.map(s => s[hKey] > 0 ? s[lKey] : NaN));
    }

    const sliceLabels = Array.from({ length: nSlices }, (_, k) => k + 1);
    return { psiValues, sliceLabels, zHmin, zHc, zLambda };
  }, [filmDist, raceway]);

  if (!distData) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-text-canvas text-sm">No film distribution data (static or unloaded)</p>
      </div>
    );
  }

  // Selected single roller data — find the film distribution entry closest to the selected roller's ψ
  const selectedRollerFilm = useMemo(() => {
    if (selectedRollerIdx === null || !filmDist || !rollers[selectedRollerIdx]) return null;
    const targetPsi = rollers[selectedRollerIdx].psi_deg;
    let best = filmDist[0];
    let bestDist = Math.abs(best.psi_deg - targetPsi);
    for (const rd of filmDist) {
      const d = Math.abs(rd.psi_deg - targetPsi);
      if (d < bestDist) { best = rd; bestDist = d; }
    }
    return bestDist < 2.0 ? best : null;  // within 2° tolerance
  }, [selectedRollerIdx, filmDist, rollers]);

  const { psiValues, sliceLabels, zHmin, zHc, zLambda } = distData;
  const zData = mapMode === 'hmin' ? zHmin : mapMode === 'hc' ? zHc : zLambda;
  const colorbarTitle = mapMode === 'hmin' ? 'h_min [μm]' : mapMode === 'hc' ? 'h_c [μm]' : 'Λ';
  const rwLabel = raceway === 'outer' ? 'Outer' : 'Inner';
  const titleText = mapMode === 'hmin'
    ? `${rwLabel} — Minimum Film Thickness h_min Distribution`
    : mapMode === 'hc'
    ? `${rwLabel} — Central Film Thickness h_c Distribution`
    : `${rwLabel} — Lambda Ratio Distribution`;

  // Color scale: for hmin/hc, use viridis; for lambda, use regime-colored scale
  const colorScale = mapMode === 'lambda'
    ? ([
        [0, '#ef4444'],     // red: boundary (λ < 1)
        [0.2, '#f59e0b'],   // amber: mixed transition
        [0.6, '#f59e0b'],   // amber: mixed (1-3)
        [0.8, '#22c55e'],   // green: full EHL
        [1, '#06b6d4'],     // cyan: thick film
      ] as [number, string][])
    : viridisScale;

  return (
    <div className="flex flex-col h-full">
      {/* Controls */}
      <div className="flex items-center gap-2 px-4 py-2 border-b border-white/5 shrink-0">
        {/* Roller selector */}
        <label className="text-xs text-slate-400">Roller:</label>
        <select
          value={selectedRollerIdx ?? ''}
          onChange={e => {
            const v = e.target.value;
            setSelectedRollerIdx(v === '' ? null : Number(v));
          }}
          className="bg-slate-800 text-slate-200 text-xs px-2 py-1 rounded border border-white/10 focus:outline-none focus:border-blue-500 cursor-pointer"
        >
          <option value="">All Rollers</option>
          {rollers.map((r, i) => (
            r.q_normal > 0 && (
              <option key={i} value={i}>
                #{i + 1} — ψ={r.psi_deg.toFixed(1)}° — Q={r.q_normal.toFixed(0)} N
              </option>
            )
          ))}
        </select>

        {selectedRollerIdx !== null && (
          <button
            onClick={() => setSelectedRollerIdx(null)}
            className="px-2 py-0.5 text-xs text-slate-400 hover:text-white transition-colors cursor-pointer"
          >
            ✕
          </button>
        )}

        {/* Raceway toggle (always visible) */}
        <div className="h-4 w-px bg-white/10" />
        <div className="flex gap-0.5">
          <button
            onClick={() => setRaceway('inner')}
            className={`px-2 py-0.5 text-xs rounded-l transition-colors cursor-pointer ${
              raceway === 'inner' ? 'bg-amber-600 text-white' : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
            }`}
          >
            Inner
          </button>
          <button
            onClick={() => setRaceway('outer')}
            className={`px-2 py-0.5 text-xs rounded-r transition-colors cursor-pointer ${
              raceway === 'outer' ? 'bg-amber-600 text-white' : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
            }`}
          >
            Outer
          </button>
        </div>

        {/* Data mode & chart type toggles (bearing overview only) */}
        {selectedRollerIdx === null && (
          <>
            <div className="h-4 w-px bg-white/10" />

            <div className="flex gap-0.5">
              <button
                onClick={() => setMapMode('hmin')}
                className={`px-2 py-0.5 text-xs rounded-l transition-colors cursor-pointer ${
                  mapMode === 'hmin' ? 'bg-blue-600 text-white' : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
                }`}
              >
                h_min [μm]
              </button>
              <button
                onClick={() => setMapMode('hc')}
                className={`px-2 py-0.5 text-xs transition-colors cursor-pointer ${
                  mapMode === 'hc' ? 'bg-blue-600 text-white' : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
                }`}
              >
                h_c [μm]
              </button>
              <button
                onClick={() => setMapMode('lambda')}
                className={`px-2 py-0.5 text-xs rounded-r transition-colors cursor-pointer ${
                  mapMode === 'lambda' ? 'bg-blue-600 text-white' : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
                }`}
              >
                Lambda (Λ)
              </button>
            </div>

            <div className="h-4 w-px bg-white/10" />

            <div className="flex gap-0.5">
              <button
                onClick={() => setChartMode('contour')}
                className={`px-2 py-0.5 text-xs rounded-l transition-colors cursor-pointer ${
                  chartMode === 'contour' ? 'bg-emerald-600 text-white' : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
                }`}
              >
                Contour
              </button>
              <button
                onClick={() => setChartMode('surface')}
                className={`px-2 py-0.5 text-xs rounded-r transition-colors cursor-pointer ${
                  chartMode === 'surface' ? 'bg-emerald-600 text-white' : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
                }`}
              >
                3D Surface
              </button>
            </div>
          </>
        )}

        <span className="ml-auto text-xs text-text-canvas font-mono">
          h_min(i) = {film.h_min_um.toFixed(3)} | h_min(o) = {film.h_min_um_outer.toFixed(3)} μm | σ_i = {film.sigma_composite_um.toFixed(3)} | σ_o = {film.sigma_composite_um_outer.toFixed(3)} μm
        </span>
      </div>

      {/* Chart area */}
      <div className="flex-1 min-h-0">
        {selectedRollerIdx !== null && selectedRollerFilm ? (
          <RollerFilmDetailChart
            rollerFilm={selectedRollerFilm}
            sliceLabels={sliceLabels}
            sigma={raceway === 'outer' ? film.sigma_composite_um_outer : film.sigma_composite_um}
            raceway={raceway}
          />
        ) : chartMode === 'contour' ? (
          <FilmContourPlot
            z={zData}
            x={sliceLabels}
            y={psiValues}
            title={titleText}
            colorbarTitle={colorbarTitle}
            colorScale={colorScale}
            mapMode={mapMode}
          />
        ) : (
          <FilmSurfacePlot
            z={zData}
            x={sliceLabels}
            y={psiValues}
            title={titleText}
            colorbarTitle={colorbarTitle}
            colorScale={colorScale}
            mapMode={mapMode}
          />
        )}
      </div>
    </div>
  );
}

function RollerFilmDetailChart({
  rollerFilm, sliceLabels, sigma, raceway,
}: {
  rollerFilm: RollerFilmDistribution;
  sliceLabels: number[];
  sigma: number;
  raceway: FilmRaceway;
}) {
  const slices = rollerFilm.slices;
  const xLabels = sliceLabels.map(String);
  const isOuter = raceway === 'outer';
  const rwLabel = isOuter ? 'Outer' : 'Inner';

  // Select inner or outer data
  const hMin = (s: typeof slices[0]) => isOuter ? s.h_min_um_outer : s.h_min_um;
  const hCen = (s: typeof slices[0]) => isOuter ? s.h_central_um_outer : s.h_central_um;
  const lam  = (s: typeof slices[0]) => isOuter ? s.lambda_outer : s.lambda;
  const reg  = (s: typeof slices[0]) => isOuter ? s.regime_outer : s.regime;

  // Regime color helper
  const regimeColor = (regime: string) => {
    switch (regime) {
      case 'FullEhl': return '#22c55e';
      case 'Mixed': return '#f59e0b';
      case 'Boundary': return '#ef4444';
      default: return '#64748b';
    }
  };

  const subplots: { title: string; yLabel: string; traces: Plotly.Data[] }[] = [
    {
      title: `${rwLabel} — Minimum Film Thickness h_min`,
      yLabel: 'h_min [μm]',
      traces: [
        {
          type: 'scatter',
          x: xLabels,
          y: slices.map(s => hMin(s) > 0 ? hMin(s) : null),
          mode: 'lines+markers',
          name: 'h_min',
          line: { color: '#60a5fa', width: 2 },
          marker: { size: 4 },
          fill: 'tozeroy',
          fillcolor: 'rgba(96,165,250,0.08)',
        },
        {
          type: 'scatter',
          x: xLabels,
          y: slices.map(() => sigma),
          mode: 'lines',
          name: `σ = ${sigma.toFixed(3)} μm`,
          line: { color: '#ef4444', width: 1, dash: 'dash' },
        },
      ],
    },
    {
      title: `${rwLabel} — Central Film Thickness h_c`,
      yLabel: 'h_central [μm]',
      traces: [
        {
          type: 'scatter',
          x: xLabels,
          y: slices.map(s => hCen(s) > 0 ? hCen(s) : null),
          mode: 'lines+markers',
          name: 'h_central',
          line: { color: '#a78bfa', width: 2 },
          marker: { size: 4 },
          fill: 'tozeroy',
          fillcolor: 'rgba(167,139,250,0.08)',
        },
        {
          type: 'scatter',
          x: xLabels,
          y: slices.map(() => sigma),
          mode: 'lines',
          name: `σ = ${sigma.toFixed(3)} μm`,
          line: { color: '#ef4444', width: 1, dash: 'dash' },
        },
      ],
    },
    {
      title: `${rwLabel} — Lambda Ratio (Λ = h_min / σ)`,
      yLabel: 'Λ [-]',
      traces: [
        {
          type: 'bar',
          x: xLabels,
          y: slices.map(s => lam(s)),
          marker: {
            color: slices.map(s => regimeColor(reg(s))),
          },
          name: 'Λ',
          hovertemplate: 'Slice %{x}<br>Λ = %{y:.2f}<extra></extra>',
        },
        {
          type: 'scatter',
          x: [xLabels[0], xLabels[xLabels.length - 1]],
          y: [1, 1],
          mode: 'lines',
          name: 'Λ=1 (Boundary)',
          line: { color: '#ef4444', width: 1, dash: 'dot' },
        },
        {
          type: 'scatter',
          x: [xLabels[0], xLabels[xLabels.length - 1]],
          y: [3, 3],
          mode: 'lines',
          name: 'Λ=3 (Full EHL)',
          line: { color: '#22c55e', width: 1, dash: 'dot' },
        },
      ],
    },
    {
      title: `${rwLabel} — Lubrication Regime`,
      yLabel: '',
      traces: [
        {
          type: 'scatter',
          x: xLabels,
          y: slices.map(s => reg(s) === 'FullEhl' ? 3 : reg(s) === 'Mixed' ? 2 : 1),
          mode: 'markers',
          marker: {
            size: 12,
            color: slices.map(s => regimeColor(reg(s))),
            symbol: 'circle',
          },
          text: slices.map(s => reg(s)),
          hovertemplate: 'Slice %{x}<br>%{text}<br>Λ = %{y:.2f}<extra></extra>',
          name: 'Regime',
        },
      ],
    },
  ];

  // Override yaxis for regime plot
  const regimeYaxis = {
    tickvals: [1, 2, 3],
    ticktext: ['Boundary', 'Mixed', 'Full EHL'],
    range: [0.5, 3.5],
  };

  return (
    <div className="grid grid-cols-2 gap-1 w-full h-full p-1">
      {subplots.map((sp, i) => (
        <div key={i} className="min-h-0">
          <Plot
            data={sp.traces}
            layout={{
              ...darkLayout,
              title: {
                text: `${sp.title} — Roller #${rollerFilm.roller_idx + 1} (ψ=${rollerFilm.psi_deg.toFixed(1)}°)`,
                font: { size: 11, color: '#e2e8f0' },
              },
              xaxis: {
                ...darkLayout.xaxis,
                title: { text: 'Slice', font: { size: 9, color: '#94a3b8' } },
                tickfont: { size: 8, family: 'JetBrains Mono' },
              },
              yaxis: {
                ...darkLayout.yaxis,
                title: { text: sp.yLabel, font: { size: 9, color: '#94a3b8' } },
                tickfont: { size: 8, family: 'JetBrains Mono' },
                ...(i === 3 ? regimeYaxis : {}),
              },
              margin: { l: 55, r: 10, t: 28, b: 35 },
              showlegend: sp.traces.length > 1,
              legend: {
                x: 1, y: 1, xanchor: 'right',
                font: { size: 9, color: '#94a3b8' },
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

function FilmContourPlot({
  z, x, y, title, colorbarTitle, colorScale, mapMode,
}: {
  z: number[][]; x: number[]; y: number[];
  title: string; colorbarTitle: string;
  colorScale: [number, string][];
  mapMode: FilmMapMode;
}) {
  const data: Plotly.Data[] = [{
    type: 'heatmap',
    z,
    x: x.map(String),
    y: y.map(v => `${v.toFixed(1)}°`),
    colorscale: colorScale,
    colorbar: {
      title: { text: colorbarTitle, font: { size: 10, color: '#94a3b8' } },
      tickfont: { size: 9, family: 'JetBrains Mono', color: '#94a3b8' },
      len: 0.8,
      thickness: 12,
    },
    zsmooth: 'best',
    hovertemplate: mapMode === 'hmin'
      ? 'Slice %{x}<br>ψ = %{y}<br>h_min = %{z:.3f} μm<extra></extra>'
      : mapMode === 'hc'
      ? 'Slice %{x}<br>ψ = %{y}<br>h_c = %{z:.3f} μm<extra></extra>'
      : 'Slice %{x}<br>ψ = %{y}<br>Λ = %{z:.2f}<extra></extra>',
  }];

  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: { text: title, font: { size: 13, color: '#e2e8f0' } },
    xaxis: {
      ...darkLayout.xaxis,
      title: { text: 'Slice', font: { size: 11, color: '#94a3b8' } },
    },
    yaxis: {
      ...darkLayout.yaxis,
      title: { text: 'ψ [deg]', font: { size: 11, color: '#94a3b8' } },
    },
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

function FilmSurfacePlot({
  z, x, y, title, colorbarTitle, colorScale, mapMode,
}: {
  z: number[][]; x: number[]; y: number[];
  title: string; colorbarTitle: string;
  colorScale: [number, string][];
  mapMode: FilmMapMode;
}) {
  const data: Plotly.Data[] = [{
    type: 'surface' as any,
    z,
    x,
    y,
    colorscale: colorScale,
    colorbar: {
      title: { text: colorbarTitle, font: { size: 10, color: '#94a3b8' } },
      tickfont: { size: 9, family: 'JetBrains Mono', color: '#94a3b8' },
      len: 0.8,
      thickness: 12,
    },
    hovertemplate: mapMode === 'hmin'
      ? 'Slice %{x}<br>ψ = %{y:.1f}°<br>h_min = %{z:.3f} μm<extra></extra>'
      : mapMode === 'hc'
      ? 'Slice %{x}<br>ψ = %{y:.1f}°<br>h_c = %{z:.3f} μm<extra></extra>'
      : 'Slice %{x}<br>ψ = %{y:.1f}°<br>Λ = %{z:.2f}<extra></extra>',
  }];

  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: { text: title, font: { size: 13, color: '#e2e8f0' } },
    margin: { l: 0, r: 0, t: 35, b: 0 },
    scene: {
      xaxis: {
        title: { text: 'Slice', font: { size: 10, color: '#94a3b8' } },
        gridcolor: '#1e293b',
        tickfont: { size: 9, family: 'JetBrains Mono', color: '#94a3b8' },
      },
      yaxis: {
        title: { text: 'ψ [deg]', font: { size: 10, color: '#94a3b8' } },
        gridcolor: '#1e293b',
        tickfont: { size: 9, family: 'JetBrains Mono', color: '#94a3b8' },
      },
      zaxis: {
        title: { text: colorbarTitle, font: { size: 10, color: '#94a3b8' } },
        gridcolor: '#1e293b',
        tickfont: { size: 9, family: 'JetBrains Mono', color: '#94a3b8' },
      },
      bgcolor: 'transparent',
      camera: { eye: { x: 1.5, y: -1.8, z: 1.2 } },
    },
  };

  return (
    <Plot
      data={data}
      layout={layout}
      config={{ ...plotConfig, displayModeBar: true }}
      useResizeHandler
      style={{ width: '100%', height: '100%' }}
    />
  );
}

// ─── Shared Components ─────────────────────────────────────────────

function DetailTable({ title, rows }: { title: string; rows: [string, string, string][] }) {
  return (
    <div>
      <h4 className="text-xs font-semibold text-text-light mb-2 uppercase tracking-wider">
        {title}
      </h4>
      <table className="text-xs">
        <tbody>
          {rows.map(([label, value, unit], i) => (
            <tr key={i} className="border-b border-white/[0.03]">
              <td className="py-0.5 pr-6 text-text-canvas whitespace-nowrap">{label}</td>
              <td className="py-0.5 text-right font-mono text-text-light tabular-nums">{value}</td>
              <td className="py-0.5 pl-1.5 text-text-canvas font-mono whitespace-nowrap">{unit}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

// ─── Film Decay Detail (Van Zoelen) ──────────────────────────────

// ─── Overview (핵심 요약 대시보드) ──────────────────────────────

// Traffic-light card: red = bad, yellow = marginal, green = OK, gray = N/A.
type Status = 'good' | 'warn' | 'bad' | 'na';
const STATUS_COLOR: Record<Status, string> = {
  good: 'border-emerald-500/40 bg-emerald-500/5',
  warn: 'border-amber-500/40 bg-amber-500/5',
  bad:  'border-red-500/40 bg-red-500/5',
  na:   'border-white/10 bg-white/[0.02]',
};
const STATUS_DOT: Record<Status, string> = {
  good: 'bg-emerald-400', warn: 'bg-amber-400', bad: 'bg-red-400', na: 'bg-white/20',
};

function HealthCard({ title, status, headline, rows }: {
  title: string; status: Status; headline: string;
  rows: { k: string; v: string; vClass?: string }[];
}) {
  return (
    <div className={`rounded-lg p-3 border ${STATUS_COLOR[status]}`}>
      <div className="flex items-center justify-between mb-2">
        <h4 className="text-[11px] font-semibold text-text-muted uppercase">{title}</h4>
        <span className={`w-2 h-2 rounded-full ${STATUS_DOT[status]}`} />
      </div>
      <div className="text-sm font-mono text-text-light mb-2">{headline}</div>
      <div className="space-y-0.5 text-xs">
        {rows.map((r, i) => (
          <div key={i} className="flex justify-between">
            <span className="text-text-canvas">{r.k}</span>
            <span className={`font-mono ${r.vClass ?? 'text-text-light/80'}`}>{r.v}</span>
          </div>
        ))}
      </div>
    </div>
  );
}

// Shared ratio band → tailwind text color, used by Health card hint and the
// External Reference Comparison row.  A single source of truth so the band
// thresholds stay consistent across Friction surfaces.
function ratioBandColor(r: number): string {
  if (!isFinite(r) || r <= 0) return 'text-text-muted';
  if (r >= 0.8 && r <= 1.25) return 'text-emerald-300';
  if (r >= 0.5 && r <= 2.0)  return 'text-amber-300';
  if (r >= 0.2 && r <= 5.0)  return 'text-orange-300';
  return 'text-red-400';
}

function LubOverview({ film, traction, hasRibEhl, result }: {
  film: FilmThicknessResult; traction: TractionSummary | null;
  model: LubricationModel; hasRibEhl: boolean; result: BearingResult;
}) {
  const { state } = useAppState();
  const decay = film.film_decay;
  const mp = film.micropitting;

  // Film card status: based on minimum Λ across both raceways
  const lambdaMin = Math.min(film.lambda_ratio, film.lambda_ratio_outer);
  const filmStatus: Status =
    lambdaMin >= 3.0 ? 'good' : lambdaMin >= 1.0 ? 'warn' : 'bad';
  const filmRegime =
    lambdaMin >= 3.0 ? 'FullEhl' : lambdaMin >= 1.0 ? 'Mixed' : 'Boundary';

  // Surface card combines micropitting risk + film decay starvation
  const surfaceStatus: Status = (() => {
    if (!mp && !decay) return 'na';
    const mpRisk = mp ? (mp.risk_inner === 'AtRisk' || mp.risk_outer === 'AtRisk' ? 'bad'
                       : mp.risk_inner === 'Marginal' || mp.risk_outer === 'Marginal' ? 'warn'
                       : 'good') : 'good';
    const decayRisk = decay ? (decay.starvation_ratio_inner < 0.3 ? 'bad'
                             : decay.starvation_ratio_inner < 0.7 ? 'warn'
                             : 'good') : 'good';
    if (mpRisk === 'bad' || decayRisk === 'bad') return 'bad';
    if (mpRisk === 'warn' || decayRisk === 'warn') return 'warn';
    return 'good';
  })();

  // Friction card status: based on flash temperature (if available) — flash > 150°C ≈ scuffing risk
  const flash = film.flash_temp_c;
  const frictionStatus: Status =
    !traction ? 'na'
    : flash != null && flash > 150 ? 'bad'
    : flash != null && flash > 80 ? 'warn'
    : 'good';

  // Rib card status: worst-case rib Λ (if rib EHL is active)
  const ribCard = (() => {
    if (!hasRibEhl) return null;
    const ribRollers = result.equilibrium.roller_results.filter(r => r.rib_result?.ehl != null);
    if (ribRollers.length === 0) return null;
    const ribLambdas = ribRollers.map(r => r.rib_result!.ehl!.lambda_ratio);
    const ribMuEffs = ribRollers.map(r => r.rib_result!.ehl!.mu_eff);
    const ribFlashes = ribRollers.map(r => r.rib_result!.ehl!.flash_temp_c);
    const lMin = Math.min(...ribLambdas);
    const muMax = Math.max(...ribMuEffs);
    const flashMax = Math.max(...ribFlashes);
    const status: Status = lMin < 1 || flashMax > 50 ? 'bad' : lMin < 3 ? 'warn' : 'good';
    const headline = `Λ ${lMin.toFixed(2)} · μ ${muMax.toFixed(3)} · ΔT ${flashMax.toFixed(0)}°C`;
    return { status, headline, lMin, muMax, flashMax };
  })();

  return (
    <div className="p-4 space-y-3">
      {/* 4-card traffic-light dashboard */}
      <div className={`grid gap-3 ${ribCard ? 'grid-cols-4' : 'grid-cols-3'}`}>
        <HealthCard
          title="Film (EHL)"
          status={filmStatus}
          headline={`Λ_min ${lambdaMin.toFixed(2)} — ${filmRegime}`}
          rows={[
            { k: 'Min film / h_min', v: `${film.h_min_um.toFixed(3)} μm` },
            { k: 'Central film / h_c (in/out)', v: `${film.h_central_um.toFixed(2)}/${film.h_central_um_outer.toFixed(2)} μm` },
            { k: 'Starvation · Thermal / φ_s · φ_T', v: `${film.starvation_factor.toFixed(2)} · ${film.thermal_factor.toFixed(2)}` },
          ]}
        />
        <HealthCard
          title="Friction & Loss"
          status={frictionStatus}
          headline={traction
            ? `${traction.p_contact_total_w.toFixed(1)} W · ${traction.m_friction_nmm.toFixed(0)} N·mm`
            : 'N/A'}
          rows={traction ? (() => {
            const skf = traction.skf_reference;
            const skfRatio = skf && skf.m_total_nmm > 1e-9
              ? traction.m_friction_nmm / skf.m_total_nmm
              : null;
            const isSkfActive = traction.friction_model === 'SkfAdvanced';
            return [
              { k: 'Rolling / P_roll', v: `${traction.p_rolling_w.toFixed(2)} W` },
              { k: 'Sliding / P_slide', v: `${traction.p_sliding_w.toFixed(2)} W` },
              { k: 'Rib / P_rib', v: `${traction.p_rib_w.toFixed(2)} W` },
              ...(traction.p_hysteresis_w > 0 ? [{
                k: `Hysteresis / P_hys (α_v=${state.input.operating.hysteresis_loss_factor.toFixed(4)})`,
                v: `${traction.p_hysteresis_w.toFixed(2)} W`,
              }] : []),
              ...(flash != null ? [{ k: 'Flash temperature / ΔT_flash', v: `${flash.toFixed(0)} °C` }] : []),
              ...(skfRatio != null ? [{
                k: 'vs SKF (M_total ratio)',
                v: isSkfActive ? '1.00× (active)' : `${skfRatio.toFixed(2)}×`,
                vClass: isSkfActive ? 'text-text-muted' : ratioBandColor(skfRatio),
              }] : []),
            ];
          })() : [{ k: '—', v: 'no traction data' }]}
        />
        <HealthCard
          title="Surface Risk"
          status={surfaceStatus}
          headline={
            mp && decay ? `S_λ ${Math.min(mp.s_lambda_inner, mp.s_lambda_outer).toFixed(2)} · h/h_ff ${decay.starvation_ratio_inner.toFixed(2)}`
            : mp ? `S_λ ${Math.min(mp.s_lambda_inner, mp.s_lambda_outer).toFixed(2)} (${mp.risk_inner === 'AtRisk' || mp.risk_outer === 'AtRisk' ? 'AtRisk' : mp.risk_inner === 'Marginal' || mp.risk_outer === 'Marginal' ? 'Marginal' : 'Safe'})`
            : decay ? `Decay ${decay.starvation_ratio_inner.toFixed(2)} @ ${decay.t_hours}hr`
            : '—'
          }
          rows={[
            ...(mp ? [
              { k: 'Safety factor / S_λ (in/out)', v: `${mp.s_lambda_inner.toFixed(2)}/${mp.s_lambda_outer.toFixed(2)}` },
              { k: 'Permissible Λ / Λ_perm', v: mp.lambda_perm.toFixed(2) },
            ] : []),
            ...(decay ? [
              { k: `Decayed film ratio / h/h_ff @${decay.t_hours}hr`, v: decay.starvation_ratio_inner.toFixed(2) },
              { k: 'Regime (decayed)', v: decay.regime_decayed_inner },
            ] : []),
            ...((!mp && !decay) ? [{ k: '—', v: 'not evaluated' }] : []),
          ]}
        />
        {ribCard && (
          <HealthCard
            title="Rib EHL"
            status={ribCard.status}
            headline={ribCard.headline}
            rows={[
              { k: 'Lambda ratio / Λ_rib (worst)', v: ribCard.lMin.toFixed(2) },
              { k: 'Friction coef / μ_rib (worst)', v: ribCard.muMax.toFixed(4) },
              { k: 'Flash temp / ΔT_flash (max)', v: `${ribCard.flashMax.toFixed(1)} °C` },
            ]}
          />
        )}
      </div>

      {/* Recommendations */}
      <div className="bg-white/[0.03] rounded-lg p-3">
        <h4 className="text-[11px] font-semibold text-text-muted uppercase mb-2">Recommendations</h4>
        <div className="space-y-1 text-xs">
          {filmStatus === 'good' && <div className="text-emerald-400">✓ Full EHL: adequate lubrication (Λ_min ≥ 3)</div>}
          {filmStatus === 'warn' && <div className="text-amber-400">⚠ Mixed lubrication: monitor surface condition (Λ_min {lambdaMin.toFixed(2)})</div>}
          {filmStatus === 'bad' && <div className="text-red-400">✗ Boundary: ISO 281 life unreliable, surface failure risk (Λ_min {lambdaMin.toFixed(2)})</div>}
          {mp && (mp.risk_inner === 'AtRisk' || mp.risk_outer === 'AtRisk') && <div className="text-red-400">✗ Micropitting risk: upgrade to Superfinish or EP additive</div>}
          {mp && (mp.risk_inner === 'Marginal' || mp.risk_outer === 'Marginal') && <div className="text-amber-400">⚠ Micropitting marginal: consider surface treatment</div>}
          {decay && decay.starvation_ratio_inner < 0.3 && <div className="text-red-400">✗ Severe starvation at {decay.t_hours}hr: increase R or reduce interval</div>}
          {flash != null && flash > 150 && <div className="text-red-400">✗ Flash temperature {flash.toFixed(0)}°C — scuffing risk</div>}
          {ribCard && ribCard.lMin < 1 && <div className="text-red-400">✗ Rib boundary lubrication (Λ_rib {ribCard.lMin.toFixed(2)}) — increase axial film or improve geometry</div>}
        </div>
      </div>
    </div>
  );
}

// ─── External Reference Comparison (SKF Catalogue 2018) ────────────
//
// Embedded as a collapsible at the end of the Friction tab.  Renders a
// head-to-head bar chart + table comparing the solver's per-component
// (rolling/sliding/total) torque & power against the SKF Catalogue 2018
// reference values, plus a deeper-collapsed SKF Intermediates table for
// debug.  Replaces the previous standalone SKF Reference sub-tab.

function ExternalReferenceComparison({ traction }: { traction: TractionSummary }) {
  const skf = traction.skf_reference;
  if (!skf) {
    return <div className="p-3 text-xs text-text-canvas">SKF reference not computed.</div>;
  }

  // Split our M_friction into rolling/sliding/rib/hysteresis by power proportion so
  // M_components sum to M_total (sums coherently to bearing torque).
  // SKF Catalogue 2018 has no rib or hysteresis counterpart; those rows show
  // "—" in SKF/Δ/Ratio cells.  Hysteresis row only shown when active (BH mode).
  const totalP = traction.p_contact_total_w;
  const m_total = traction.m_friction_nmm;
  const m_roll = totalP > 1e-9 ? m_total * (traction.p_rolling_w / totalP) : 0;
  const m_slide = totalP > 1e-9 ? m_total * (traction.p_sliding_w / totalP) : 0;
  const m_rib = totalP > 1e-9 ? m_total * (traction.p_rib_w / totalP) : 0;
  const m_hys = totalP > 1e-9 ? m_total * (traction.p_hysteresis_w / totalP) : 0;
  const hasHys = traction.p_hysteresis_w > 1e-6;

  const rows: { label: string; ours: number; ref: number | null; unit: string; bold?: boolean }[] = [
    { label: 'M_rolling (BH viscous)', ours: m_roll, ref: skf.m_rr_nmm, unit: 'N·mm' },
    { label: 'M_sliding', ours: m_slide, ref: skf.m_sl_nmm, unit: 'N·mm' },
    ...(hasHys ? [
      { label: 'M_hys (Johnson, no SKF eq.)', ours: m_hys, ref: null, unit: 'N·mm' },
    ] : []),
    { label: 'M_rib (no SKF eq.)', ours: m_rib, ref: null, unit: 'N·mm' },
    { label: 'M_total', ours: m_total, ref: skf.m_total_nmm, unit: 'N·mm', bold: true },
    { label: 'P_rolling (BH viscous)', ours: traction.p_rolling_w, ref: skf.p_rolling_w, unit: 'W' },
    { label: 'P_sliding', ours: traction.p_sliding_w, ref: skf.p_sliding_w, unit: 'W' },
    ...(hasHys ? [
      { label: 'P_hys (Johnson, no SKF eq.)', ours: traction.p_hysteresis_w, ref: null, unit: 'W' },
    ] : []),
    { label: 'P_rib (no SKF eq.)', ours: traction.p_rib_w, ref: null, unit: 'W' },
    { label: 'P_total', ours: traction.p_contact_total_w, ref: skf.p_total_w, unit: 'W', bold: true },
  ];

  const activeIsSkf = traction.friction_model === 'SkfAdvanced';
  const activeModelLabel: Record<string, string> = {
    PalmgrenLike: 'Palmgren-like (μ_rr·Q·u, default)',
    BibouletHoupert: 'Biboulet-Houpert 2010 per-contact (analytical)',
    SkfAdvanced: 'SKF Catalogue 2018 (industry-calibrated)',
  };

  // Side-by-side bar chart — Ours vs SKF for each component.  M_rib has no
  // SKF counterpart so its SKF bar is null (Plotly skips), making the gap
  // visible at a glance.
  const xCats = ['M_rolling', 'M_sliding', 'M_rib', 'M_total'];
  const torqueBar: Plotly.Data[] = [
    {
      type: 'bar', name: 'Ours',
      x: xCats,
      y: [m_roll, m_slide, m_rib, m_total],
      marker: { color: '#3b82f6' },
      hovertemplate: '%{x}: %{y:.1f} N·mm<extra>Ours</extra>',
    },
    {
      type: 'bar', name: 'SKF Ref',
      x: xCats,
      y: [skf.m_rr_nmm, skf.m_sl_nmm, null as unknown as number, skf.m_total_nmm],
      marker: { color: '#94a3b8' },
      hovertemplate: '%{x}: %{y:.1f} N·mm<extra>SKF</extra>',
    },
  ];

  return (
    <div className="p-3 space-y-4">
      {/* Active-model context */}
      <div className="text-xs text-text-canvas">
        <span className="text-text-muted">Active friction model:</span>{' '}
        <span className="text-text-light font-mono">{activeModelLabel[traction.friction_model] ?? traction.friction_model}</span>
        {activeIsSkf && (
          <div className="text-amber-300 mt-1">
            ⓘ Active model is SKF — bearing-level totals are dispatched directly from SKF, so the Ours/SKF ratio is exactly 1.00 by construction.  Switch to Palmgren or BH to see a meaningful comparison.
          </div>
        )}
      </div>

      {/* Side-by-side bar chart (torque components) */}
      <div className="bg-white/[0.02] rounded-lg overflow-hidden border border-white/5" style={{ height: 240 }}>
        <Plot
          data={torqueBar}
          layout={{
            ...darkLayout,
            barmode: 'group',
            title: { text: 'Friction Torque — Head-to-Head', font: { size: 12, color: '#e2e8f0' } },
            margin: { l: 55, r: 10, t: 35, b: 30 },
            xaxis: { ...darkLayout.xaxis, color: '#94a3b8' },
            yaxis: {
              ...darkLayout.yaxis,
              title: { text: 'Friction Torque [N·mm]', font: { size: 10, color: '#94a3b8' } },
            },
            showlegend: true,
            legend: { x: 0.02, y: 0.98, font: { size: 10, color: '#94a3b8' } },
          }}
          config={plotConfig}
          useResizeHandler
          style={{ width: '100%', height: '100%' }}
        />
      </div>

      {/* Head-to-head numerical table */}
      <div>
        <h4 className="text-xs font-semibold text-text-light mb-2 uppercase tracking-wider">
          Head-to-Head Comparison (Ours vs SKF Catalogue 2018)
        </h4>
        <table className="w-full text-xs font-mono">
          <thead>
            <tr className="text-text-canvas border-b border-white/10">
              <th className="px-2 py-1 text-left font-medium">Component</th>
              <th className="px-2 py-1 text-right font-medium">Ours</th>
              <th className="px-2 py-1 text-right font-medium">SKF</th>
              <th className="px-2 py-1 text-right font-medium">Δ</th>
              <th className="px-2 py-1 text-right font-medium">Ratio</th>
              <th className="px-2 py-1 text-left font-medium pl-3 w-16">Unit</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((r, i) => {
              const fmt = (v: number) => r.unit === 'N·mm' ? v.toFixed(1) : v.toFixed(2);
              const hasRef = r.ref != null;
              const delta = hasRef ? r.ours - (r.ref as number) : 0;
              const ratio = hasRef && (r.ref as number) > 1e-9 ? r.ours / (r.ref as number) : 0;
              const labelClass = r.bold ? 'text-text-light font-semibold' : 'text-text-canvas';
              const valClass   = r.bold ? 'text-text-light font-semibold' : 'text-text-light';
              return (
                <tr key={i} className={`border-b border-white/5 ${r.bold ? 'bg-white/[0.02]' : ''}`}>
                  <td className={`px-2 py-1 ${labelClass}`}>{r.label}</td>
                  <td className={`px-2 py-1 text-right ${valClass}`}>{fmt(r.ours)}</td>
                  <td className={`px-2 py-1 text-right ${hasRef ? valClass : 'text-text-muted/60'}`}>
                    {hasRef ? fmt(r.ref as number) : '—'}
                  </td>
                  <td className={`px-2 py-1 text-right ${!hasRef ? 'text-text-muted/60' : delta > 0 ? 'text-orange-300' : delta < 0 ? 'text-blue-300' : 'text-text-muted'}`}>
                    {hasRef ? `${delta > 0 ? '+' : ''}${fmt(delta)}` : '—'}
                  </td>
                  <td className={`px-2 py-1 text-right ${hasRef ? ratioBandColor(ratio) : 'text-text-muted/60'} ${r.bold ? 'font-semibold' : ''}`}>
                    {hasRef && ratio > 0 ? `${ratio.toFixed(2)}×` : '—'}
                  </td>
                  <td className="px-2 py-1 text-left text-text-muted/70 pl-3">{r.unit}</td>
                </tr>
              );
            })}
          </tbody>
        </table>
        <div className="text-[10px] text-text-muted/70 mt-2 leading-relaxed">
          Ratio color guide:
          <span className="text-emerald-300 ml-2">●</span> 0.8–1.25 (good)
          <span className="text-amber-300 ml-2">●</span> 0.5–2.0 (acceptable)
          <span className="text-orange-300 ml-2">●</span> 0.2–5 (calibration-level)
          <span className="text-red-400 ml-2">●</span> outside (model disagreement).
          <br />Rib component has no SKF Catalogue 2018 counterpart, but is included in <span className="text-text-light">M_total / P_total</span> (rolling + sliding + rib = total).
        </div>
      </div>

      {/* Inner collapsible: SKF Intermediates (debug) */}
      <CollapsibleSection
        title="SKF Intermediate Quantities (G_rr, G_sl, φ_ish, φ_rs, μ_sl, φ_bl)"
        available={true}
        defaultOpen={false}
        badge="debug"
      >
        <div className="p-3">
          <table className="w-full text-xs font-mono">
            <tbody>
              <tr className="border-b border-white/5">
                <td className="px-2 py-0.5 text-text-canvas">G_rr (rolling variable)</td>
                <td className="px-2 py-0.5 text-right text-text-light">{skf.g_rr.toExponential(3)}</td>
                <td className="px-2 py-0.5 text-text-canvas pl-4">G_sl (sliding variable)</td>
                <td className="px-2 py-0.5 text-right text-text-light">{skf.g_sl.toFixed(2)} N·mm</td>
              </tr>
              <tr className="border-b border-white/5">
                <td className="px-2 py-0.5 text-text-canvas">φ_ish (inlet shear heating)</td>
                <td className="px-2 py-0.5 text-right text-text-light">{skf.phi_ish.toFixed(4)}</td>
                <td className="px-2 py-0.5 text-text-canvas pl-4">φ_rs (kinematic starvation)</td>
                <td className="px-2 py-0.5 text-right text-text-light">{skf.phi_rs.toFixed(4)}</td>
              </tr>
              <tr className="border-b border-white/5">
                <td className="px-2 py-0.5 text-text-canvas">μ_sl (sliding coefficient)</td>
                <td className="px-2 py-0.5 text-right text-text-light">{skf.mu_sl.toFixed(5)}</td>
                <td className="px-2 py-0.5 text-text-canvas pl-4">φ_bl (boundary weight)</td>
                <td className="px-2 py-0.5 text-right text-text-light">{skf.phi_bl.toFixed(4)}</td>
              </tr>
              <tr className="border-b border-white/5">
                <td className="px-2 py-0.5 text-text-canvas">Series</td>
                <td className="px-2 py-0.5 text-right text-text-light">{skf.series}</td>
                <td className="px-2 py-0.5 text-text-canvas pl-4">Lubrication</td>
                <td className="px-2 py-0.5 text-right text-text-light">{skf.lubrication}</td>
              </tr>
              <tr>
                <td className="px-2 py-0.5 text-text-canvas">Y factor</td>
                <td className="px-2 py-0.5 text-right text-text-light">{skf.y_factor.toFixed(2)}</td>
                <td className="px-2 py-0.5 text-text-canvas pl-4">d_m / n / ν_op</td>
                <td className="px-2 py-0.5 text-right text-text-light">
                  {skf.d_m_mm.toFixed(1)} mm / {skf.n_rpm.toFixed(0)} rpm / {skf.nu_op_cst.toFixed(1)} cSt
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </CollapsibleSection>

      <div className="text-[11px] text-text-canvas/70 leading-relaxed">
        ⓘ Independent verification via SKF Bearing Calculator: <span className="font-mono text-text-light">https://www.skfbearingselect.com/</span>
        {' '}— SKF Catalogue 2018 model has documented ±20–30 % accuracy.  Larger ratios in the rolling component typically indicate that Palmgren μ_rr·Q·u over-predicts viscous rolling resistance vs SKF's Biboulet-Houpert calibration.
      </div>
    </div>
  );
}

// ─── Rib EHL Section (Lubrication tab → Rib EHL sub-tab) ─────────

function RibEhlSection({ result }: { result: BearingResult }) {
  const ribRollers = result.equilibrium.roller_results.filter(r => r.rib_result?.ehl != null);
  if (ribRollers.length === 0) {
    return (
      <div className="p-4 text-sm text-text-canvas">
        Rib EHL not evaluated — needs rotating bearing with axial load and α_i ≠ α_o.
      </div>
    );
  }

  // Worst case: lowest Λ (most severe lubrication regime)
  const worst = ribRollers.reduce((acc, r) =>
    r.rib_result!.ehl!.lambda_ratio < acc.rib_result!.ehl!.lambda_ratio ? r : acc
  );
  const worstEhl = worst.rib_result!.ehl!;

  // Aggregate stats
  const lambdas = ribRollers.map(r => r.rib_result!.ehl!.lambda_ratio);
  const muEffs = ribRollers.map(r => r.rib_result!.ehl!.mu_eff);
  const flashTemps = ribRollers.map(r => r.rib_result!.ehl!.flash_temp_c);
  const lambdaMin = Math.min(...lambdas);
  const lambdaMean = lambdas.reduce((a, b) => a + b, 0) / lambdas.length;
  const muMax = Math.max(...muEffs);
  const muMean = muEffs.reduce((a, b) => a + b, 0) / muEffs.length;
  const flashMax = Math.max(...flashTemps);

  const regimeColor = (r: string) =>
    r === 'FullEhl' ? 'text-green-400' : r === 'Mixed' ? 'text-yellow-400' : 'text-red-400';
  const regimeBg = (r: string) =>
    r === 'FullEhl' ? 'bg-emerald-500/20 text-emerald-300'
    : r === 'Mixed' ? 'bg-amber-500/20 text-amber-300'
    : 'bg-red-500/20 text-red-300';
  const flashColor = (t: number) =>
    t > 50 ? 'text-red-400' : t > 20 ? 'text-yellow-400' : 'text-text-light';

  return (
    <div className="p-4 space-y-3">
      {/* Top summary cards (worst-case + aggregates) */}
      <div className="grid grid-cols-2 gap-3">
        <div className="bg-white/[0.03] rounded-lg p-3">
          <h4 className="text-[11px] font-semibold text-text-muted uppercase mb-2">
            Worst-case Rib (lowest Λ)
          </h4>
          <div className="space-y-1 text-xs">
            <div className="flex justify-between">
              <span className="text-text-canvas">Roller angle / ψ</span>
              <span className="text-text-light font-mono">{worst.psi_deg.toFixed(1)}°</span>
            </div>
            <div className="flex justify-between">
              <span className="text-text-canvas">Rib force / F_rib</span>
              <span className="text-text-light font-mono">{worst.rib_result!.f_rib.toFixed(0)} N</span>
            </div>
            <div className="flex justify-between">
              <span className="text-text-canvas">Min film thickness / h_min</span>
              <span className="text-text-light font-mono">{worstEhl.h_min_um.toFixed(3)} μm</span>
            </div>
            <div className="flex justify-between">
              <span className="text-text-canvas">Lambda ratio / Λ</span>
              <span className={`font-mono ${regimeColor(worstEhl.regime)}`}>
                {worstEhl.lambda_ratio.toFixed(2)} ({worstEhl.regime})
              </span>
            </div>
          </div>
        </div>

        <div className="bg-white/[0.03] rounded-lg p-3">
          <h4 className="text-[11px] font-semibold text-text-muted uppercase mb-2">
            Friction & Heat
          </h4>
          <div className="space-y-1 text-xs">
            <div className="flex justify-between">
              <span className="text-text-canvas">Effective friction / μ_eff (worst)</span>
              <span className="text-text-light font-mono">{muMax.toFixed(4)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-text-canvas">Effective friction / μ_eff (mean)</span>
              <span className="text-text-light font-mono">{muMean.toFixed(4)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-text-canvas">Slide-to-roll ratio / SRR</span>
              <span className="text-text-light font-mono">{worstEhl.srr.toFixed(2)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-text-canvas">Flash temperature / ΔT_flash (max)</span>
              <span className={`font-mono ${flashColor(flashMax)}`}>{flashMax.toFixed(1)} °C</span>
            </div>
          </div>
        </div>

        <div className="bg-white/[0.03] rounded-lg p-3">
          <h4 className="text-[11px] font-semibold text-text-muted uppercase mb-2">
            Λ Distribution
          </h4>
          <div className="space-y-1 text-xs">
            <div className="flex justify-between">
              <span className="text-text-canvas">Lambda ratio / Λ min</span>
              <span className={`font-mono ${regimeColor(worstEhl.regime)}`}>{lambdaMin.toFixed(2)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-text-canvas">Lambda ratio / Λ mean</span>
              <span className="text-text-light font-mono">{lambdaMean.toFixed(2)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-text-canvas">Loaded rollers</span>
              <span className="text-text-light font-mono">{ribRollers.length}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-text-canvas">Composite roughness / σ_c</span>
              <span className="text-text-light font-mono">{worstEhl.sigma_composite_um.toFixed(3)} μm</span>
            </div>
          </div>
        </div>

        <div className="bg-white/[0.03] rounded-lg p-3">
          <h4 className="text-[11px] font-semibold text-text-muted uppercase mb-2">
            EHL Inputs (worst-case)
          </h4>
          <div className="space-y-1 text-xs">
            <div className="flex justify-between">
              <span className="text-text-canvas">Entrainment velocity / u_entrain</span>
              <span className="text-text-light font-mono">{worstEhl.u_entrain_m_s.toFixed(3)} m/s</span>
            </div>
            <div className="flex justify-between">
              <span className="text-text-canvas">Sliding velocity / u_slide</span>
              <span className="text-text-light font-mono">{worstEhl.u_slide_m_s.toFixed(3)} m/s</span>
            </div>
            <div className="flex justify-between">
              <span className="text-text-canvas">Ellipticity / k = a/b</span>
              <span className="text-text-light font-mono">{worstEhl.k_ellipse.toFixed(2)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-text-canvas">Thermal factor / φ_T</span>
              <span className="text-text-light font-mono">{worstEhl.thermal_factor.toFixed(3)}</span>
            </div>
          </div>
        </div>
      </div>

      {/* Recommendations */}
      <div className="bg-white/[0.03] rounded-lg p-3">
        <h4 className="text-[11px] font-semibold text-text-muted uppercase mb-2">Recommendations</h4>
        <div className="space-y-1 text-xs">
          {lambdaMin >= 3.0 && <div className="text-green-400">✓ All rollers in Full-EHL regime — adequate rib lubrication</div>}
          {lambdaMin >= 1.0 && lambdaMin < 3.0 && <div className="text-yellow-400">⚠ Mixed regime present (Λ_min={lambdaMin.toFixed(2)}): monitor rib wear</div>}
          {lambdaMin < 1.0 && <div className="text-red-400">✗ Boundary regime (Λ_min={lambdaMin.toFixed(2)}): high rib wear and scuffing risk</div>}
          {flashMax > 50 && <div className="text-red-400">✗ ΔT_flash {flashMax.toFixed(0)}°C: consider higher-viscosity oil or EP additive</div>}
          {flashMax > 20 && flashMax <= 50 && <div className="text-yellow-400">⚠ ΔT_flash elevated ({flashMax.toFixed(0)}°C): verify thermal margin</div>}
          {worstEhl.srr > 1.5 && <div className="text-text-canvas">ℹ Pure-sliding (SRR≈2): Carreau-Yasuda traction model recommended for rib (research §4.3)</div>}
        </div>
      </div>

      {/* Per-roller table */}
      <div>
        <h4 className="text-xs font-semibold text-text-light mb-2 uppercase tracking-wider">
          Per-Roller Rib EHL ({ribRollers.length} loaded)
        </h4>
        <div className="overflow-x-auto">
          <table className="w-full text-xs font-mono">
            <thead>
              <tr className="text-text-canvas border-b border-white/10">
                <th className="px-2 py-1 text-right">ψ [deg]</th>
                <th className="px-2 py-1 text-right">F_rib [N]</th>
                <th className="px-2 py-1 text-right">p_max [MPa]</th>
                <th className="px-2 py-1 text-right">h_c [μm]</th>
                <th className="px-2 py-1 text-right">h_min [μm]</th>
                <th className="px-2 py-1 text-right">Λ</th>
                <th className="px-2 py-1 text-center">Regime</th>
                <th className="px-2 py-1 text-right">μ_eff</th>
                <th className="px-2 py-1 text-right text-text-canvas">μ_ehl</th>
                <th className="px-2 py-1 text-right text-text-canvas">f_a [%]</th>
                <th className="px-2 py-1 text-right">ΔT_flash [°C]</th>
                <th className="px-2 py-1 text-right text-text-canvas">φ_T</th>
              </tr>
            </thead>
            <tbody>
              {ribRollers.map((r, i) => {
                const e = r.rib_result!.ehl!;
                const isWorst = r === worst;
                return (
                  <tr
                    key={i}
                    className={`border-b border-white/5 hover:bg-white/5 ${
                      isWorst ? 'bg-red-500/5' : ''
                    }`}
                  >
                    <td className="px-2 py-0.5 text-right text-text-light">{r.psi_deg.toFixed(1)}</td>
                    <td className="px-2 py-0.5 text-right text-text-light">{r.rib_result!.f_rib.toFixed(0)}</td>
                    <td className="px-2 py-0.5 text-right text-text-light">{r.rib_result!.p_max_rib.toFixed(0)}</td>
                    <td className="px-2 py-0.5 text-right text-text-light">{e.h_c_um.toFixed(3)}</td>
                    <td className="px-2 py-0.5 text-right text-text-light">{e.h_min_um.toFixed(3)}</td>
                    <td className={`px-2 py-0.5 text-right ${regimeColor(e.regime)}`}>{e.lambda_ratio.toFixed(2)}</td>
                    <td className="px-2 py-0.5 text-center">
                      <span className={`inline-block px-1.5 rounded text-[10px] font-medium ${regimeBg(e.regime)}`}>
                        {e.regime === 'FullEhl' ? 'Full' : e.regime}
                      </span>
                    </td>
                    <td className="px-2 py-0.5 text-right text-text-light">{e.mu_eff.toFixed(4)}</td>
                    <td className="px-2 py-0.5 text-right text-text-canvas">{e.mu_ehl.toFixed(4)}</td>
                    <td className="px-2 py-0.5 text-right text-text-canvas">{(e.asperity_load_ratio * 100).toFixed(1)}</td>
                    <td className={`px-2 py-0.5 text-right ${flashColor(e.flash_temp_c)}`}>{e.flash_temp_c.toFixed(1)}</td>
                    <td className="px-2 py-0.5 text-right text-text-canvas">{e.thermal_factor.toFixed(3)}</td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
        <div className="mt-1 text-[11px] text-text-canvas">
          Worst-case row highlighted in red. SRR for all rib contacts ≈ {worstEhl.srr.toFixed(2)} (pure-sliding limit).
        </div>
      </div>
    </div>
  );
}

// ─── Diagnostic Tab — Decay + Micropitting + HMEHL collapsibles ──
//
// Consolidates the previously-conditional "Decay", "Micropitting" and
// "HMEHL" sub-tabs into a single Diagnostic surface.  Each section is a
// collapsible disclosure: only available data renders, and the user can
// open / close any section without leaving the tab.

function CollapsibleSection({
  title, available, defaultOpen, badge, children,
}: {
  title: string;
  available: boolean;
  defaultOpen?: boolean;
  badge?: string;
  children: React.ReactNode;
}) {
  const [open, setOpen] = useState(!!defaultOpen && available);
  if (!available) {
    return (
      <div className="rounded-lg border border-white/5 bg-white/[0.015] px-4 py-2">
        <div className="flex items-center justify-between text-xs">
          <span className="text-text-muted">{title}</span>
          <span className="text-text-muted/60 italic">{badge ?? 'not evaluated'}</span>
        </div>
      </div>
    );
  }
  return (
    <div className="rounded-lg border border-white/5 bg-white/[0.02]">
      <button
        onClick={() => setOpen(o => !o)}
        className="w-full flex items-center justify-between px-4 py-2 text-xs font-medium text-text-light hover:bg-white/[0.03] transition-colors cursor-pointer"
      >
        <span className="flex items-center gap-2">
          <span className="text-text-muted">{open ? '▼' : '▶'}</span>
          {title}
          {badge && <span className="text-[10px] text-text-muted/70 font-normal">{badge}</span>}
        </span>
      </button>
      {open && <div className="border-t border-white/5">{children}</div>}
    </div>
  );
}

function DiagnosticTab({ film }: { film: FilmThicknessResult }) {
  const hasDecay = !!film.film_decay;
  const hasMp = !!film.micropitting;
  const decay = film.film_decay;
  return (
    <div className="p-4 space-y-3">
      <div className="text-[11px] text-text-muted/70 mb-1">
        ⓘ Diagnostic surfaces — open each section to inspect detailed sub-models.  Only sections whose backing data is available will expand.
      </div>
      <CollapsibleSection
        title="Micropitting (S_λ, ISO/TS 6336-22 adapted)"
        available={hasMp}
        defaultOpen={hasMp}
        badge={hasMp ? `S_λ inner=${film.micropitting!.s_lambda_inner.toFixed(2)} · outer=${film.micropitting!.s_lambda_outer.toFixed(2)}` : undefined}
      >
        {hasMp && <MicropittingDetail film={film} />}
      </CollapsibleSection>

      <CollapsibleSection
        title="Film Decay (Van Zoelen, grease starvation)"
        available={hasDecay}
        defaultOpen={hasDecay}
        badge={hasDecay ? `t=${decay!.t_hours} hr · h/h_ff=${decay!.starvation_ratio_inner.toFixed(2)}` : undefined}
      >
        {hasDecay && <FilmDecayDetail film={film} />}
      </CollapsibleSection>

      <CollapsibleSection
        title="HMEHL (Tier-3 micro-EHL — manual trigger)"
        available={true}
        defaultOpen={false}
        badge="full Reynolds + FFT elastic"
      >
        <HMEHLTab />
      </CollapsibleSection>
    </div>
  );
}

// ─── Micropitting Detail ──────────────────────────────────────────

function MicropittingDetail({ film }: { film: FilmThicknessResult }) {
  const mp = film.micropitting!;
  const riskColor = (r: string) =>
    r === 'Safe' ? 'text-green-400' : r === 'Marginal' ? 'text-yellow-400' : 'text-red-400';

  return (
    <div className="p-4 space-y-4">
      <DetailTable title="Micropitting Safety (ISO/TS 6336-22 adapted)" rows={[
        ['Λ_perm base', mp.lambda_perm_base.toFixed(1), ''],
        ['Additive factor', mp.additive_factor.toFixed(1), ''],
        ['Λ_perm (effective)', mp.lambda_perm.toFixed(2), `= ${mp.lambda_perm_base.toFixed(1)} × ${mp.additive_factor.toFixed(1)}`],
        ['', '', ''],
        ['Λ_min inner', film.lambda_ratio.toFixed(2), ''],
        ['S_λ inner', mp.s_lambda_inner.toFixed(2), mp.risk_inner],
        ['', '', ''],
        ['Λ_min outer', film.lambda_ratio_outer.toFixed(2), ''],
        ['S_λ outer', mp.s_lambda_outer.toFixed(2), mp.risk_outer],
      ]} />

      <div className="bg-white/[0.03] rounded-lg p-3">
        <h4 className="text-[11px] font-semibold text-text-muted uppercase mb-2">Risk Assessment</h4>
        <div className="space-y-2 text-xs">
          <div className="flex items-center gap-2">
            <span className="text-text-canvas w-16">Inner:</span>
            <div className="flex-1 bg-white/5 rounded-full h-4 overflow-hidden">
              <div className={`h-full rounded-full ${
                mp.risk_inner === 'Safe' ? 'bg-green-500' : mp.risk_inner === 'Marginal' ? 'bg-yellow-500' : 'bg-red-500'
              }`} style={{ width: `${Math.min(mp.s_lambda_inner / 3 * 100, 100)}%` }} />
            </div>
            <span className={`font-mono w-10 text-right ${riskColor(mp.risk_inner)}`}>{mp.s_lambda_inner.toFixed(2)}</span>
          </div>
          <div className="flex items-center gap-2">
            <span className="text-text-canvas w-16">Outer:</span>
            <div className="flex-1 bg-white/5 rounded-full h-4 overflow-hidden">
              <div className={`h-full rounded-full ${
                mp.risk_outer === 'Safe' ? 'bg-green-500' : mp.risk_outer === 'Marginal' ? 'bg-yellow-500' : 'bg-red-500'
              }`} style={{ width: `${Math.min(mp.s_lambda_outer / 3 * 100, 100)}%` }} />
            </div>
            <span className={`font-mono w-10 text-right ${riskColor(mp.risk_outer)}`}>{mp.s_lambda_outer.toFixed(2)}</span>
          </div>
          <div className="text-[11px] text-text-muted/60 mt-2">
            S_λ ≥ 2.0: Safe | 1.0~2.0: Marginal | &lt;1.0: At Risk
          </div>
        </div>
      </div>

      <div className="text-[11px] text-text-muted/50 p-2">
        ⓘ ISO/TS 6336-22 (gear standard) adapted for bearings. No equivalent bearing standard exists.
        Values are conservative engineering estimates.
      </div>
    </div>
  );
}

// ─── HMEHL Tab (Tier 3 — manual trigger) ─────────────────────────

function HMEHLTab() {
  const { state } = useAppState();
  const activeResult = useActiveResult();
  const [result, setResult] = useState<HMEHLResult | null>(null);
  const [chartSplit, setChartSplit] = useState(50); // % for 2D panel width
  const [chartHeight, setChartHeight] = useState(480); // px for chart frame height
  const [running, setRunning] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const runHmehl = useCallback(async () => {
    setRunning(true);
    setError(null);
    try {
      // Extract Load Distribution results for worst-loaded slice
      let worstQ = 50.0;       // line load [N/mm]
      let worstPmax = 1000.0;  // Hertz pressure [MPa]
      let worstSliceIdx = 0;
      const isInner = true;
      if (activeResult?.equilibrium.angular_distribution) {
        // Find worst roller (max q_total)
        let maxQ = 0;
        let worstRoller = activeResult.equilibrium.angular_distribution[0];
        for (const pt of activeResult.equilibrium.angular_distribution) {
          if (pt.q_total > maxQ) { maxQ = pt.q_total; worstRoller = pt; }
        }
        // Find worst slice (max p_max for inner)
        if (worstRoller) {
          const pArr = isInner ? worstRoller.slice_p_max : worstRoller.slice_p_max_outer;
          for (let k = 0; k < pArr.length; k++) {
            if (pArr[k] > worstPmax) {
              worstPmax = pArr[k];
              worstSliceIdx = k;
            }
          }
          worstQ = worstRoller.slice_q_k[worstSliceIdx] ?? 50.0;
        }
      }
      // Get R_eq from slice geometry (computed in Gen1 solver)
      // R_eq back-computed from Load Distribution's q_k and p_hertz
      const rEqMm = (() => {
        // Back-compute R_eq from hertz.rs formula: p = 2q/(πb), b² = 4qR/(πE*)
        // → R = π²E*p² / (4q × E*²)... simpler: R = q * E* / (π * p²)
        const eStar = 2.0 / ((1 - 0.3**2) / (state.input.material.e_roller * 1e3) +
                              (1 - 0.3**2) / (state.input.material.e_ring * 1e3)); // MPa
        return worstQ > 0 && worstPmax > 0
          ? worstQ * eStar / (Math.PI * worstPmax * worstPmax) : 5.0;
      })();
      const res = await invoke<HMEHLResult>('run_hmehl', {
        input: state.input, qKNmm: worstQ, pHertzMpa: worstPmax, rEqMm: rEqMm, isInner,
      });
      setResult(res);
    } catch (e: any) {
      setError(String(e));
    } finally {
      setRunning(false);
    }
  }, [state.input]);

  // Grid x-coordinates for plotting (normalized to contact half-width)
  const nx = result?.pressure.length ?? 0;
  const xNorm = result ? Array.from({ length: nx }, (_, i) => (i / (nx - 1) - 0.5) * 2) : [];

  return (
    <div className="p-4 space-y-4">
      <div className="bg-white/[0.03] rounded-lg p-4">
        <h4 className="text-xs font-semibold text-text-light uppercase mb-2">HMEHL Micro-EHL Analysis</h4>
        <p className="text-[11px] text-text-muted/60 mb-3">
          Full Reynolds PDE + FFT elastic deformation + Roelands viscosity + roughness homogenization.
          More accurate than analytical EHL formulas (M1/M2/M3) but computationally heavier (~1-3s).
        </p>
        <button
          onClick={runHmehl}
          disabled={running}
          className={`px-4 py-1.5 text-sm font-medium rounded cursor-pointer transition-colors ${
            running
              ? 'bg-slate-600 text-slate-400 cursor-wait'
              : 'bg-blue-600 text-white hover:bg-blue-500'
          }`}
        >
          {running ? 'Running...' : '▶ Run HMEHL Analysis'}
        </button>
        {error && <div className="mt-2 text-xs text-red-400">{error}</div>}
      </div>

      {result && (
        <>
          {/* Summary table */}
          <DetailTable title={`HMEHL Result (${result.converged ? '✓ converged' : '⚠ not converged'}, ${result.iterations} iter)`} rows={[
            ['h_central', `${(result.h_central * 1e6).toFixed(3)}`, 'μm'],
            ['h_min', `${(result.h_min * 1e6).toFixed(3)}`, 'μm'],
            ['h_min / h_c', `${(result.h_min / Math.max(result.h_central, 1e-15)).toFixed(3)}`, ''],
            ['p_max (EHL)', `${(result.p_max / 1e6).toFixed(1)}`, 'MPa'],
            ['p_hertz', `${(Math.max(...result.hertz_pressure_ref) / 1e6).toFixed(1)}`, 'MPa'],
            ['p_max / p_hertz', `${(result.p_max / Math.max(...result.hertz_pressure_ref, 1)).toFixed(2)}`, ''],
            ['τ_surf_max', `${(result.tau_surf_max / 1e6).toFixed(1)}`, 'MPa'],
            ['T_max', `${result.t_max.toFixed(1)}`, '°C'],
            ['T_mean (contact)', `${result.t_mean_contact.toFixed(1)}`, '°C'],
            ['μ (total)', `${result.mu.toFixed(5)}`, ''],
            ['μ_fluid', `${result.mu_fluid.toFixed(5)}`, ''],
            ['μ_asperity', `${result.mu_asperity.toFixed(5)}`, ''],
            ['p_asp_mean', `${(result.p_asp_mean / 1e6).toFixed(1)}`, 'MPa'],
          ]} />

          {/* 2D + 3D: solid frame, resizable width + height */}
          <div
            className="border border-slate-700 rounded overflow-hidden"
            style={{ display: 'flex', height: chartHeight }}
          >
            {/* 2D Panel */}
            <div
              className="relative"
              style={{ width: `${chartSplit}%`, minWidth: 180, borderRight: 'none' }}
            >
              <div className="absolute inset-0 flex flex-col">
                <div className="px-2 py-1 text-[10px] font-semibold text-slate-400 uppercase tracking-wider bg-slate-800/50 border-b border-slate-700 shrink-0">
                  Pressure & Film (2D)
                </div>
                <div className="flex-1 min-h-0">
                  <Plot
                    data={[
                      ...(result.hertz_pressure_ref ? [{
                        x: xNorm,
                        y: result.hertz_pressure_ref.map((p: number) => p / 1e6),
                        type: 'scatter' as const, mode: 'lines' as const,
                        name: 'Hertz', line: { color: '#94a3b8', width: 1.5, dash: 'dash' as const },
                        yaxis: 'y' as const,
                      }] : []),
                      { x: xNorm, y: result.pressure.map(p => p / 1e6),
                        type: 'scatter', mode: 'lines', name: 'EHL Pressure',
                        line: { color: '#f97316', width: 2.5 }, yaxis: 'y' },
                      { x: xNorm, y: result.film.map(h => h * 1e6),
                        type: 'scatter', mode: 'lines', name: 'Film [μm]',
                        line: { color: '#60a5fa', width: 2 }, yaxis: 'y2' },
                      ...(result.temperature && result.t_max > result.temperature[0] + 0.1 ? [{
                        x: xNorm, y: result.temperature,
                        type: 'scatter' as const, mode: 'lines' as const, name: 'T [°C]',
                        line: { color: '#ef4444', width: 1.5, dash: 'dot' as const },
                        yaxis: 'y3' as const,
                      }] : []),
                    ]}
                    layout={{
                      ...darkLayout, autosize: true,
                      margin: { l: 50, r: 65, t: 8, b: 36 },
                      xaxis: { ...darkLayout.xaxis, title: { text: 'x / b_Hertz', font: { size: 10, color: '#94a3b8' } } },
                      yaxis: { ...darkLayout.yaxis, title: { text: 'P [MPa]', font: { size: 10, color: '#f97316' } }, side: 'left' },
                      yaxis2: { title: { text: 'h [μm]', font: { size: 10, color: '#60a5fa' } },
                        overlaying: 'y', side: 'right', showgrid: false, color: '#60a5fa' },
                      yaxis3: { title: { text: 'T', font: { size: 9, color: '#ef4444' } },
                        overlaying: 'y', side: 'right', showgrid: false, color: '#ef4444',
                        position: 0.93, anchor: 'free' },
                      legend: { x: 0.02, y: 0.98, font: { size: 9, color: '#94a3b8' } },
                      showlegend: true,
                    }}
                    config={plotConfig} useResizeHandler
                    style={{ width: '100%', height: '100%' }}
                  />
                </div>
              </div>
            </div>

            {/* Resize handle */}
            <div
              className="cursor-col-resize flex items-center justify-center shrink-0 hover:bg-blue-500/20 active:bg-blue-500/40 transition-colors"
              style={{ width: 5, background: '#1e293b', borderLeft: '1px solid #334155', borderRight: '1px solid #334155' }}
              onMouseDown={(e) => {
                e.preventDefault();
                const startX = e.clientX;
                const startSplit = chartSplit;
                const container = (e.target as HTMLElement).parentElement;
                const onMove = (ev: MouseEvent) => {
                  if (!container) return;
                  const pct = startSplit + ((ev.clientX - startX) / container.offsetWidth) * 100;
                  setChartSplit(Math.max(20, Math.min(80, pct)));
                };
                const onUp = () => { document.removeEventListener('mousemove', onMove); document.removeEventListener('mouseup', onUp); };
                document.addEventListener('mousemove', onMove);
                document.addEventListener('mouseup', onUp);
              }}
            >
              <div className="w-px h-10 bg-slate-600 rounded" />
            </div>

            {/* 3D Panel */}
            <div className="relative" style={{ flex: 1, minWidth: 180 }}>
              <div className="absolute inset-0 flex flex-col">
                <div className="px-2 py-1 text-[10px] font-semibold text-slate-400 uppercase tracking-wider bg-slate-800/50 border-b border-slate-700 shrink-0">
                  EHL Stress (3D)
                </div>
                <div className="flex-1 min-h-0">
                  {(() => {
                    const nY = 30;
                    const yRange = Array.from({ length: nY }, (_, j) => -1 + 2 * j / (nY - 1));
                    const pSurface = yRange.map(y => {
                      const yf = Math.max(0, 1 - y * y);
                      return xNorm.map((_, i) => (result.pressure[i] / 1e6) * Math.sqrt(yf));
                    });
                    return (
                      <Plot
                        data={[{
                          type: 'surface',
                          x: xNorm, y: yRange, z: pSurface,
                          colorscale: 'Jet',
                          colorbar: {
                            title: { text: 'P [MPa]', font: { size: 10, color: '#94a3b8' } },
                            thickness: 15, len: 0.75, xpad: 4,
                          },
                          opacity: 1.0, showscale: true,
                        } as any]}
                        layout={{
                          ...darkLayout, autosize: true,
                          margin: { l: 0, r: 0, t: 0, b: 0 },
                          scene: {
                            xaxis: { title: { text: 'x / b' }, color: '#94a3b8', gridcolor: '#334155' },
                            yaxis: { title: { text: 'width' }, color: '#94a3b8', gridcolor: '#334155' },
                            zaxis: { title: { text: 'P [MPa]' }, color: '#94a3b8', gridcolor: '#334155' },
                            camera: { eye: { x: 1.4, y: -1.6, z: 0.7 } },
                            bgcolor: '#0f172a',
                            aspectmode: 'manual' as any,
                            aspectratio: { x: 1.5, y: 1, z: 0.8 },
                          },
                          showlegend: false,
                        }}
                        config={plotConfig} useResizeHandler
                        style={{ width: '100%', height: '100%' }}
                      />
                    );
                  })()}
                </div>
              </div>
            </div>
          </div>
          {/* Bottom resize handle for chart height */}
          <div
            className="cursor-row-resize flex justify-center hover:bg-blue-500/20 active:bg-blue-500/40 transition-colors border-x border-b border-slate-700 rounded-b"
            style={{ height: 5, background: '#1e293b' }}
            onMouseDown={(e) => {
              e.preventDefault();
              const startY = e.clientY;
              const startH = chartHeight;
              const onMove = (ev: MouseEvent) => {
                setChartHeight(Math.max(250, Math.min(900, startH + ev.clientY - startY)));
              };
              const onUp = () => { document.removeEventListener('mousemove', onMove); document.removeEventListener('mouseup', onUp); };
              document.addEventListener('mousemove', onMove);
              document.addEventListener('mouseup', onUp);
            }}
          >
            <div className="h-px w-10 bg-slate-600 rounded mt-0.5" />
          </div>
        </>
      )}
    </div>
  );
}

// ─── Film Decay Detail (Van Zoelen) ──────────────────────────────

function FilmDecayDetail({ film }: { film: FilmThicknessResult }) {
  const decay = film.film_decay!;

  // Decay curve chart data
  const tHours = decay.decay_curve.map(d => d[0]);
  const hInner = decay.decay_curve.map(d => d[1]);
  const hOuter = decay.decay_curve.map(d => d[2]);

  return (
    <div className="p-4 space-y-4">
      <DetailTable title="Van Zoelen Film Decay — Side Flow Model" rows={[
        ['Operating time',        decay.t_hours.toFixed(0),              'hr'],
        ['─── Inner raceway ───', '',                                   ''],
        ['h_c (fully flooded)',   film.h_central_um.toFixed(3),         'μm'],
        ['h_c (decayed)',         decay.h_c_decayed_inner_um.toFixed(3),'μm'],
        ['Starvation ratio',     decay.starvation_ratio_inner.toFixed(3), 'h/h_ff'],
        ['Λ (decayed)',          decay.lambda_decayed_inner.toFixed(2), ''],
        ['Regime',               decay.regime_decayed_inner,            ''],
        ['F(0)',                  decay.f0_inner.toExponential(2),      'm⁻²s⁻¹'],
        ...(decay.h_c_equilibrium_inner_um != null
          ? [['h_c (equilibrium)' as string, decay.h_c_equilibrium_inner_um.toFixed(3), 'μm']] as [string, string, string][]
          : []),
        ['─── Outer raceway ───', '',                                   ''],
        ['h_c (fully flooded)',   film.h_central_um_outer.toFixed(3),   'μm'],
        ['h_c (decayed)',         decay.h_c_decayed_outer_um.toFixed(3),'μm'],
        ['Starvation ratio',     decay.starvation_ratio_outer.toFixed(3), 'h/h_ff'],
        ['Λ (decayed)',          decay.lambda_decayed_outer.toFixed(2), ''],
        ['Regime',               decay.regime_decayed_outer,            ''],
        ['F(0)',                  decay.f0_outer.toExponential(2),      'm⁻²s⁻¹'],
        ...(decay.h_c_equilibrium_outer_um != null
          ? [['h_c (equilibrium)' as string, decay.h_c_equilibrium_outer_um.toFixed(3), 'μm']] as [string, string, string][]
          : []),
        ...(decay.replenishment_rate_nm_s > 0
          ? [['Replenishment R' as string, decay.replenishment_rate_nm_s.toFixed(3), 'nm/s']] as [string, string, string][]
          : []),
      ]} />

      {/* Decay curve chart */}
      <div>
        <h4 className="text-xs font-semibold text-text-light mb-2 uppercase tracking-wider">
          Film Thickness Decay Curve
        </h4>
        <Plot
          data={[
            {
              x: tHours,
              y: hInner,
              type: 'scatter',
              mode: 'lines',
              name: 'Inner raceway',
              line: { color: '#60a5fa', width: 2 },
            },
            {
              x: tHours,
              y: hOuter,
              type: 'scatter',
              mode: 'lines',
              name: 'Outer raceway',
              line: { color: '#f97316', width: 2 },
            },
            // Equilibrium lines (if R > 0)
            ...(decay.h_c_equilibrium_inner_um != null ? [{
              x: [tHours[0], tHours[tHours.length - 1]],
              y: [decay.h_c_equilibrium_inner_um * 1e3, decay.h_c_equilibrium_inner_um * 1e3],
              type: 'scatter' as const,
              mode: 'lines' as const,
              name: 'Equilibrium (inner)',
              line: { color: '#60a5fa', width: 1, dash: 'dash' as const },
            }] : []),
            ...(decay.h_c_equilibrium_outer_um != null ? [{
              x: [tHours[0], tHours[tHours.length - 1]],
              y: [decay.h_c_equilibrium_outer_um * 1e3, decay.h_c_equilibrium_outer_um * 1e3],
              type: 'scatter' as const,
              mode: 'lines' as const,
              name: 'Equilibrium (outer)',
              line: { color: '#f97316', width: 1, dash: 'dash' as const },
            }] : []),
            {
              x: [decay.t_hours],
              y: [decay.h_c_decayed_inner_um * 1e3],
              type: 'scatter',
              mode: 'markers',
              name: `t = ${decay.t_hours} hr`,
              marker: { color: '#60a5fa', size: 10, symbol: 'diamond' },
              showlegend: false,
            },
            {
              x: [decay.t_hours],
              y: [decay.h_c_decayed_outer_um * 1e3],
              type: 'scatter',
              mode: 'markers',
              marker: { color: '#f97316', size: 10, symbol: 'diamond' },
              showlegend: false,
            },
          ]}
          layout={{
            ...darkLayout,
            height: 300,
            margin: { l: 60, r: 20, t: 30, b: 50 },
            xaxis: { ...darkLayout.xaxis, title: { text: 'Time [hours]', font: { size: 12, color: '#94a3b8' } } },
            yaxis: { ...darkLayout.yaxis, title: { text: 'h_c [nm]', font: { size: 12, color: '#94a3b8' } } },
            legend: { x: 0.7, y: 0.95, font: { size: 11, color: '#94a3b8' } },
            showlegend: true,
          }}
          config={plotConfig}
          className="w-full"
        />
      </div>
    </div>
  );
}
