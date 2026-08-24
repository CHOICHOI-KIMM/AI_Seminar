// BB 결과 요약 카드 (Plan §3.6.5.2 S3 · §3.6.4.3 처분표 「🔴 ResultsCard — 신규」)
//
// ─────────────────────────────────────────────────────────────────────
//  이 파일이 존재하는 이유
// ─────────────────────────────────────────────────────────────────────
//  기존 `components/ResultsCard` 는 우측 **상시 렌더**이면서 `result.life` ·
//  `result.static_rating` · `result.k_radial` · `result.mode` 를 읽는다.
//  BB `BbResult` 에는 **그 필드가 하나도 없다.** 즉 사용자가 Solve 를 누르는
//  순간 렌더 중 throw 하고, 에러 바운더리가 없는 React 19 는 트리 전체를
//  언마운트해 **화면이 통째로 빈다** (S1 에서 `InputPanel` 이 정확히 그랬다).
//  그것을 없애는 것이 S3 의 실질 목표다.
//  ⚠ 기존 파일은 **지우지 않는다** — 최소 변경(§3.6.4.3), 일괄 정리는 §3.6.4.6.
//
// ─────────────────────────────────────────────────────────────────────
//  이 화면의 검증 임무 — §3.6.4.2
// ─────────────────────────────────────────────────────────────────────
//  대응 Level: **D-2a**(축퇴 항등성) · **C-8**(수렴 보고).
//  「대칭 하중에서 `δ_z`·`γ_y` 가 0 인가」를 육안으로 확인하는 **유일한 곳**이다.
//
//  ⭐ 그래서 5-DOF 5성분을 **한 줄로 뭉뚱그리지 않고 성분마다 한 줄**로 내며,
//     `δ_z`·`γ_y` 는 **눈에 띄게 표시**한다. 뭉뚱그리면 검증 임무를 못 한다.
//
// ─────────────────────────────────────────────────────────────────────
//  단위 정책 (S2 확정 — 바꾸지 말 것)
// ─────────────────────────────────────────────────────────────────────
//  변위 **mm** · 하중 **N** · 응력 **MPa** · **접촉각만 °**(내부 rad) ·
//  **틸트 γ 는 rad 유지**.
//
//  ⚠ **틸트만 rad 인 이유** — Level **D-2a·D-2d 판정이 rad 로 출력**된다.
//    °로 바꿔 보여주면 화면 숫자와 검증 숫자가 달라져 대조가 한 단계 꼬인다.
//    접촉각은 반대다 — 0,698 rad 는 사람이 못 읽으므로 °로 낸다 (rad 도 병기).
//
//  ⚠ **유효숫자 9자리.** 0 이어야 할 성분이 1e−12 인지 1e−3 인지는 자릿수를
//    줄이면 구분되지 않는다. D-2a 판정은 `rel. err < 1e-8` 이다.

import { useAppState } from '../store';
import type { BbResult } from './generated/BbResult';

/** 유효숫자 9자리 (또는 지수표기) — 사유는 파일 헤더 참조. */
function num(v: number): string {
  if (!Number.isFinite(v)) return String(v);
  if (v === 0) return '0';
  const a = Math.abs(v);
  if (a < 1e-4 || a >= 1e9) return v.toExponential(8);
  return v.toPrecision(9);
}

const toDeg = (rad: number) => (rad * 180) / Math.PI;

export default function BbResultsCard() {
  const { state, dispatch } = useAppState();
  const { result, resultsPanelOpen } = state;

  const toggle = () => dispatch({ type: 'TOGGLE_RESULTS_PANEL' });

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
    <aside className="w-80 shrink-0 bg-glass-bg backdrop-blur-xl border-l border-glass-border flex flex-col min-h-0">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-2.5 border-b border-white/5 shrink-0">
        <div className="flex items-center gap-2">
          <h3 className="text-sm font-semibold text-text-light uppercase tracking-wider">Summary</h3>
          {result && (
            <span className="text-xs px-1.5 py-0.5 rounded font-mono bg-blue-500/20 text-blue-300">
              {result.kind}
            </span>
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

      <div className="flex-1 overflow-y-auto custom-scrollbar">
        {!result ? (
          <div className="flex items-center justify-center h-full">
            <p className="text-text-canvas text-sm">No results yet</p>
          </div>
        ) : (
          <Body result={result} />
        )}
      </div>
    </aside>
  );
}

function Body({ result }: { result: BbResult }) {
  const eq = result.equilibrium;
  const d = eq.displacement;
  const balls = eq.ball_results;

  // 접촉각 범위 — §3.6.4.2 D-1/C-2 「α_j 가 볼마다 다르고 하중에 따라 변한다」.
  // 하중을 받지 않는 볼의 α_j 는 물리적 의미가 없으므로 접촉 볼만 본다.
  //   ⚠ 「접촉 볼이 하나도 없다」와 「전부 같다」는 다른 상태다 — 구분해 표시한다.
  const loaded = balls.filter(b => b.loaded);
  const alphas = loaded.map(b => b.alpha_rad);
  const alphaMin = alphas.length > 0 ? Math.min(...alphas) : null;
  const alphaMax = alphas.length > 0 ? Math.max(...alphas) : null;

  // 접촉응력 — 내/외륜 각각의 **최대값** (전 볼 중)
  const pInner = balls.length > 0 ? Math.max(...balls.map(b => b.p_max_inner_mpa)) : 0;
  const pOuter = balls.length > 0 ? Math.max(...balls.map(b => b.p_max_outer_mpa)) : 0;

  return (
    <div className="p-4 space-y-3">
      {/* ── 5-DOF 변위 — D-2a 육안 확인 지점 ───────────────────────── */}
      <Section title="Displacement (5-DOF)">
        <p className="text-[11px] text-text-canvas/60 mb-1.5 leading-snug">
          D-2a 축퇴 항등성 — 대칭 하중에서 <span className="font-mono">δ_z</span> ·{' '}
          <span className="font-mono">γ_y</span> 가 0 이어야 한다 (강조 표시).
        </p>
        <DofRow label="δ_x (axial)" value={d.dx_mm} unit="mm" />
        <DofRow label="δ_y" value={d.dy_mm} unit="mm" />
        <DofRow label="δ_z" value={d.dz_mm} unit="mm" watch />
        <DofRow label="γ_y" value={d.ry_rad} unit="rad" watch />
        <DofRow label="γ_z" value={d.rz_rad} unit="rad" />
      </Section>

      {/* ── 볼 하중 ──────────────────────────────────────────────── */}
      <Section title="Ball Load">
        <Row label="Q_max" value={num(eq.q_max_n)} unit="N" />
        <Row label="Loaded balls" value={`${eq.loaded_count} / ${balls.length}`} unit="" />
      </Section>

      {/* ── 접촉각 ───────────────────────────────────────────────── */}
      <Section title="Contact Angle α_j (loaded balls)">
        {alphaMin === null || alphaMax === null ? (
          <p className="text-[13px] text-amber-300">접촉 중인 볼이 없습니다</p>
        ) : (
          <>
            <Row label="α_j min" value={num(toDeg(alphaMin))} unit="°" />
            <Row label="α_j max" value={num(toDeg(alphaMax))} unit="°" />
            <Row label="α_j min" value={num(alphaMin)} unit="rad" />
            <Row label="α_j max" value={num(alphaMax)} unit="rad" />
          </>
        )}
      </Section>

      {/* ── 접촉응력 ─────────────────────────────────────────────── */}
      <Section title="Max Contact Stress">
        <Row label="p_max (inner)" value={num(pInner)} unit="MPa" />
        <Row label="p_max (outer)" value={num(pOuter)} unit="MPa" />
      </Section>

      {/* ── 수렴 — C-8 ──────────────────────────────────────────── */}
      <Section title="Convergence">
        <div className="flex items-center justify-between py-0.5">
          <span className="text-[13px] text-text-canvas">converged</span>
          <span
            className={`text-[13px] font-mono px-1.5 py-0.5 rounded ${
              eq.converged ? 'bg-emerald-500/20 text-emerald-300' : 'bg-red-500/20 text-red-300'
            }`}
          >
            {String(eq.converged)}
          </span>
        </div>
        <Row label="iterations" value={String(eq.iterations)} unit="" />
        <Row label="residual_norm" value={num(eq.residual_norm)} unit="" />
      </Section>

      {/* ── 기타 ─────────────────────────────────────────────────── */}
      <Section title="Run">
        <Row label="kind" value={result.kind} unit="" />
        <Row label="elapsed" value={num(result.elapsed_ms)} unit="ms" />
        <Row
          label="phase sweep"
          value={result.phase_sweep ? 'present' : 'none'}
          unit=""
        />
      </Section>
    </div>
  );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div>
      <h4 className="text-xs font-semibold text-text-light mb-1 uppercase tracking-wider">
        {title}
      </h4>
      <div className="pl-0.5">{children}</div>
    </div>
  );
}

function Row({ label, value, unit }: { label: string; value: string; unit: string }) {
  return (
    <div className="flex items-baseline justify-between gap-2 py-0.5">
      <span className="text-[13px] text-text-canvas whitespace-nowrap">{label}</span>
      <span className="text-[13px] font-mono tabular-nums text-text-light text-right break-all">
        {value}
        {unit && <span className="text-text-canvas ml-1">{unit}</span>}
      </span>
    </div>
  );
}

/**
 * 5-DOF 성분 한 줄.
 *
 * `watch` = **D-2a 판정 대상**(`δ_z`·`γ_y`). 대칭 하중에서 0 이어야 하는 성분이므로
 * 다른 성분과 시각적으로 구분한다 — 5성분을 뭉뚱그리면 검증 임무를 못 한다.
 */
function DofRow({
  label,
  value,
  unit,
  watch = false,
}: {
  label: string;
  value: number;
  unit: string;
  watch?: boolean;
}) {
  return (
    <div
      className={`flex items-baseline justify-between gap-2 py-0.5 px-1 rounded ${
        watch ? 'bg-amber-500/10 border border-amber-400/25' : ''
      }`}
    >
      <span
        className={`text-[13px] whitespace-nowrap ${
          watch ? 'text-amber-200 font-semibold' : 'text-text-canvas'
        }`}
      >
        {label}
        {watch && <span className="ml-1 text-[10px] opacity-70">D-2a</span>}
      </span>
      <span
        className={`text-[13px] font-mono tabular-nums text-right break-all ${
          watch ? 'text-amber-100' : 'text-text-light'
        }`}
      >
        {num(value)}
        <span className={watch ? 'text-amber-200/70 ml-1' : 'text-text-canvas ml-1'}>{unit}</span>
      </span>
    </div>
  );
}
