// BB 수명·정정격 뷰 (Plan §4 Phase 5-2 · Theory §7)
//
// ─────────────────────────────────────────────────────────────────────
//  이 화면의 검증 임무 — Level E
// ─────────────────────────────────────────────────────────────────────
//  | 구성 | 확인하는 것 | 근거 |
//  |---|---|---|
//  | `f_c`·`b_m` → `C_r` 사슬 | 카탈로그 `C_r` 과 대조 가능한가 | ISO 281 §5.1 |
//  | `Q_ci`·`Q_ce` ↔ `Q_ei`·`Q_ee` | 공칭하중 대비 실하중이 어디쯤인가 | ISO 16281 (1)(2)(5)~(8) |
//  | `L_10r = (C_r / P_ref r)³` | **내부 항등**이 화면에서도 성립하는가 | ISO 16281 (9)(11) |
//  | `γ` → `f_0` → `C_0r` → `S_0` | 정정격 사슬 | ISO 76 (76-1)(76-2)(76-14) |
//
// ─────────────────────────────────────────────────────────────────────
//  🔴 한 탭에 2단인 이유 (Plan §5-2.1 확정 2026-08-24)
// ─────────────────────────────────────────────────────────────────────
//  `BbStaticRatingResult` 는 **`BbLifeResult.static_rating` 으로 이미 같은 페이로드**다.
//  커맨드 왕복 **한 번**에 동적수명과 정정격이 함께 온다. 탭을 쪼개면 한 번의
//  결과를 두 화면으로 흩는 셈이라 비교가 어려워진다.
//
// ─────────────────────────────────────────────────────────────────────
//  🔴 「지어내지 않는다」를 화면에서 지킨다 — 이 탭의 핵심 규율
// ─────────────────────────────────────────────────────────────────────
//  `BbLifeResult` 의 `null` 필드는 **화면에서도 비어 있어야** 한다. 0 이나 대시로
//  채우면 「계산됐는데 0」과 「계산 안 됨」이 구분되지 않는다.
//
//  ⚠ **`nu_1_mm2_s` 를 `kappa`·`a_iso` 와 같은 그룹으로 묶지 마라.**
//     ISO 281 식 (28)(29) 의 `ν₁ = 45 000 n^(−0,83) D_pw^(−0,5)` /
//     `4 500 n^(−0,5) D_pw^(−0,5)` 는 **실동점도 ν 와 무관**하다 —
//     `n > 0` · `D_pw > 0` 이면 ν 없이도 **항상 산출된다**.
//     `ν` 를 요구하는 것은 `κ = ν/ν₁` 과 그 하류(`a_ISO`·`L_nmr`)뿐이다.
//     헬스체크를 확장할 때 이 셋을 한 묶음으로 판정해 **솔버가 옳은데 오판**한
//     전례가 있다. 화면에서도 같은 실수를 하지 않도록 단을 갈라 둔다.
//
// ─────────────────────────────────────────────────────────────────────
//  🔴 단위 경계 (D-10)
// ─────────────────────────────────────────────────────────────────────
//  솔버는 수명을 **`10⁶ rev`(`_mrev`) 로만** 낸다 — 시간 환산은 **이 뷰의 몫**이다:
//    `L_h = L_mrev × 10⁶ / (60 n)`.
//  `n` 은 **`resultInput.operating`**(Solve 시점 스냅샷)에서 읽는다. `bbInput` 은
//  편집 중이라 시점이 어긋난다. `n = 0` 이면 시간 환산을 **하지 않는다**(∞ 로 쓰지 않는다).
//
// ─────────────────────────────────────────────────────────────────────
//  ♻ 재사용
// ─────────────────────────────────────────────────────────────────────
//  TRB `charts/LifeChart` 의 **「요약 카드 → 중간값 사슬 표 → 차트」 3층 배치**를 옮겼다.
//  원본 파일은 손대지 않는다(최소 변경 방침). 라미나(슬라이스)별 수명은 **볼에 대응물이
//  없으므로 폐기**하고 그 자리에 **볼별 `Q_j` 막대**를 둔다.
//  커맨드 호출은 `BbGeometryView`(150 ms 디바운스 + `cancelled` 플래그) 와 같은 패턴이다.

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useAppState } from '../store';
import Plot from '../components/charts/PlotWithCopy';
import { darkLayout, plotConfig } from '../components/charts/plotlyDefaults';
import { DetailTable } from '../components/shared/DetailTable';
import type { BbLifeResult } from './generated/BbLifeResult';
import type { BbLifeConditions } from './generated/BbLifeConditions';
import type { BbFatigueLimitMethod } from './generated/BbFatigueLimitMethod';
import type { AlertLevel } from './generated/AlertLevel';

/** 유효숫자 9자리 (또는 지수표기) — 검증 대조가 목적이라 자릿수를 줄이면 안 된다. */
function num(v: number): string {
  if (!Number.isFinite(v)) return String(v);
  if (v === 0) return '0';
  const a = Math.abs(v);
  if (a < 1e-4 || a >= 1e9) return v.toExponential(8);
  return v.toPrecision(9);
}

/** `null` 은 **빈칸**으로 — 0 이나 대시로 채우지 않는다 (파일 헤더의 규율). */
function opt(v: number | null | undefined): string {
  return v === null || v === undefined ? '' : num(v);
}

// ─── 색 팔레트 — `BbLoadDistView`·`BbAxialSectionView` 와 동일 ─────────
const C_LOADED = '#f59e0b';
const C_UNLOADED = '#64748b';
const C_INNER = '#3b82f6';
const C_OUTER = '#f97316';
const C_REF = '#a3e635';

const ALERT_COLORS: Record<AlertLevel, string> = {
  Info: 'bg-blue-500/10 border-blue-400/30 text-blue-200',
  Warning: 'bg-amber-500/10 border-amber-400/30 text-amber-200',
  Critical: 'bg-red-500/10 border-red-400/30 text-red-200',
};
const ALERT_ICONS: Record<AlertLevel, string> = { Info: 'ℹ', Warning: '⚠', Critical: '⛔' };

/** 솔버 기본값과 같다 — `BbLifeConditions` 의 주석 참조(아무 값도 지어내지 않는 쪽). */
const DEFAULT_CONDITIONS: BbLifeConditions = {
  nu_mm2_s: 0,
  e_c: 1,
  a_1: 1,
  ep_additive: false,
  inner_ring_rotating: true,
  c_u_method: 'Precise',
};

// ─── 요약 카드 ────────────────────────────────────────────────────────

function Card({
  label,
  value,
  unit,
  sub,
  color = '#e2e8f0',
}: {
  label: string;
  value: string;
  unit: string;
  sub?: string;
  color?: string;
}) {
  return (
    <div className="rounded border border-white/10 bg-black/20 px-3 py-2 min-w-[10.5rem]">
      <p className="text-[11px] text-text-canvas/60 uppercase tracking-wider">{label}</p>
      <p className="font-mono tabular-nums text-[15px] leading-tight" style={{ color }}>
        {value === '' ? <span className="text-text-canvas/30">—</span> : value}
        {value !== '' && unit && <span className="text-[11px] text-text-canvas/60 ml-1">{unit}</span>}
      </p>
      {sub && <p className="text-[11px] text-text-canvas/50 mt-0.5">{sub}</p>}
    </div>
  );
}

// ─── 조건 바 ──────────────────────────────────────────────────────────

function NumField({
  label,
  value,
  onChange,
  hint,
}: {
  label: string;
  value: number;
  onChange: (v: number) => void;
  hint?: string;
}) {
  return (
    <label className="text-[12px] text-text-canvas flex items-center gap-1.5" title={hint}>
      {label}
      <input
        type="number"
        value={value}
        step="any"
        onChange={e => onChange(Number(e.target.value))}
        className="w-24 bg-canvas-subtle border border-white/15 rounded px-2 py-1 text-[12px] font-mono text-text-light"
      />
    </label>
  );
}

// ─── 본체 ─────────────────────────────────────────────────────────────

export default function BbLifeView() {
  const { state } = useAppState();
  const { bbInput, result, resultInput } = state;

  const [cond, setCond] = useState<BbLifeConditions>(DEFAULT_CONDITIONS);
  const [life, setLife] = useState<BbLifeResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  // `BbGeometryView` 와 **같은 패턴** — 150 ms 디바운스 + `cancelled` 플래그.
  // 구독 대상은 `bbInput` **전체**와 `cond` 다.
  useEffect(() => {
    if (!bbInput) return;
    let cancelled = false;
    const timer = setTimeout(() => {
      invoke<BbLifeResult>('bb_compute_life', { input: bbInput, conditions: cond })
        .then(r => {
          if (cancelled) return;
          setLife(r);
          setError(null);
        })
        .catch((e: unknown) => {
          if (cancelled) return;
          // Rust 메시지를 **그대로** 낸다 (§3.6.5.3).
          setError(String(e));
          setLife(null);
        });
    }, 150);
    return () => {
      cancelled = true;
      clearTimeout(timer);
    };
  }, [bbInput, cond]);

  if (!bbInput) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-text-canvas text-sm">프리셋을 불러오는 중…</p>
      </div>
    );
  }

  // ── 시간 환산 (D-10 경계) ──────────────────────────────────────
  //  ⚠ Solve 시점 스냅샷에서 읽는다. Solve 전이면 회전속도를 모르므로 **환산하지 않는다**.
  const nRpm = resultInput?.operating.n_inner_rpm ?? null;
  const toHours = (mrev: number | null): number | null =>
    mrev === null || nRpm === null || nRpm <= 0 ? null : (mrev * 1e6) / (60 * nRpm);

  const balls = result?.equilibrium.ball_results ?? [];

  const stale = result !== null && resultInput !== null && resultInput !== bbInput;
  const st = life?.static_rating ?? null;

  // ── 내부 항등 검산: `L_10r = (C_r / P_ref r)³` (ISO 16281 (9)(11)) ──
  //  화면에서도 이 항등이 성립해야 한다. 어긋나면 솔버나 직렬화가 깨진 것이다.
  const identity =
    life && life.p_ref_r_n > 0 ? Math.pow(life.c_r_n / life.p_ref_r_n, 3) : null;
  const identityRel =
    identity !== null && life && life.l_10r_mrev !== 0
      ? (identity - life.l_10r_mrev) / life.l_10r_mrev
      : null;

  if (error) {
    return (
      <div className="h-full overflow-auto custom-scrollbar p-4">
        <div className="p-2 rounded border bg-red-500/10 border-red-400/30 text-red-200">
          <p className="text-[13px] font-medium">수명 계산 실패</p>
          <p className="text-xs opacity-80 whitespace-pre-wrap break-words">{error}</p>
        </div>
      </div>
    );
  }

  return (
    <div className="h-full overflow-auto custom-scrollbar p-4 space-y-5">
      {/* ── 검증 임무 명시 ─────────────────────────────────────────── */}
      <div className="p-2.5 rounded border bg-blue-500/10 border-blue-400/30 text-blue-100 text-[12px] leading-relaxed">
        <p className="font-semibold text-[13px] mb-1">
          수명 · 정정격 — ISO 281 / 16281 (동적) + ISO 76 (정적) · Theory §7
        </p>
        <p>
          두 결과는 <b>커맨드 왕복 한 번</b>에 함께 온다(<span className="font-mono">static_rating</span> 이{' '}
          <span className="font-mono">BbLifeResult</span> 안에 들어 있다). 그래서 한 탭에 2단으로 둔다.
        </p>
        <p className="mt-1 text-blue-200/80">
          <b>값이 없으면 빈칸으로 둔다</b> — 0 이나 대시로 채우면 「계산됐는데 0」과 「계산 안 됨」이 구분되지
          않는다. <span className="font-mono">κ</span>·<span className="font-mono">a_ISO</span>·
          <span className="font-mono">L_nmr</span> 은 <b>실동점도 ν 를 입력해야</b> 나온다.{' '}
          <span className="font-mono">ν₁</span> 은 <b>ν 와 무관</b>하게 항상 나온다 — 식 (28)(29) 가{' '}
          <span className="font-mono">n</span>·<span className="font-mono">D_pw</span> 만의 함수이기 때문이다.
        </p>
      </div>

      {stale && (
        <div className="p-2 rounded border bg-amber-500/10 border-amber-400/30 text-amber-200 text-[12px]">
          입력이 Solve 이후 변경되었다. <b>수명은 현재 입력</b>(<span className="font-mono">bbInput</span>) 기준으로
          다시 계산되지만 <b>회전속도 n 은 Solve 시점 스냅샷</b>(<span className="font-mono">resultInput</span>)에서
          읽으므로 시간[h] 환산의 시점이 어긋나 있다. 다시 Solve 할 것.
        </div>
      )}

      {/* ── 조건 바 ────────────────────────────────────────────────── */}
      <div className="rounded border border-white/10 bg-black/20 p-3">
        <p className="text-[13px] font-semibold text-text-light mb-2">
          수명 조건 (ISO 281 §9 · Theory §7.8~7.10)
        </p>
        <div className="flex flex-wrap items-center gap-x-5 gap-y-2">
          <NumField
            label="ν [mm²/s]"
            value={cond.nu_mm2_s}
            onChange={v => setCond(c => ({ ...c, nu_mm2_s: v }))}
            hint="운전 온도에서의 실동점도. 0 이면 κ·a_ISO·L_nmr 을 산출하지 않는다 (지어내지 않는다)"
          />
          <NumField
            label="e_C"
            value={cond.e_c}
            onChange={v => setCond(c => ({ ...c, e_c: v }))}
            hint="오염계수 (ISO 281 Table 13). 1 = 극청정"
          />
          <NumField
            label="a_1"
            value={cond.a_1}
            onChange={v => setCond(c => ({ ...c, a_1: v }))}
            hint="신뢰도 계수. L_10 (신뢰도 90 %) 기준이 1"
          />
          <label className="text-[12px] text-text-canvas flex items-center gap-1.5">
            <input
              type="checkbox"
              checked={cond.ep_additive}
              onChange={e => setCond(c => ({ ...c, ep_additive: e.target.checked }))}
            />
            EP 첨가제
          </label>
          <label className="text-[12px] text-text-canvas flex items-center gap-1.5">
            <input
              type="checkbox"
              checked={cond.inner_ring_rotating}
              onChange={e => setCond(c => ({ ...c, inner_ring_rotating: e.target.checked }))}
            />
            내륜 회전
          </label>
          <label className="text-[12px] text-text-canvas flex items-center gap-1.5">
            C_u 법
            <select
              value={cond.c_u_method}
              onChange={e => setCond(c => ({ ...c, c_u_method: e.target.value as BbFatigueLimitMethod }))}
              className="bg-canvas-subtle border border-white/15 rounded px-2 py-1 text-[12px] font-mono text-text-light"
            >
              <option value="Precise">Precise (B.10)(B.11)</option>
              <option value="Simplified">Simplified (B.18)(B.19)</option>
            </select>
          </label>
          <span className="text-[12px] text-text-canvas/60 font-mono">
            n = {nRpm === null ? '— (Solve 전)' : `${num(nRpm)} r/min`}
          </span>
        </div>
      </div>

      {!life ? (
        <p className="text-[13px] text-text-canvas/60">계산 중…</p>
      ) : (
        <>
          {/* ═══ 1단 — 동적 수명 (ISO 281 / 16281) ═══════════════════ */}
          <div className="rounded border border-white/10 bg-black/20 p-3 space-y-3">
            <p className="text-[13px] font-semibold text-text-light">
              ① 동적 수명 — ISO 281 / 16281
            </p>

            <div className="flex flex-wrap gap-2">
              <Card
                label="L_10r"
                value={num(life.l_10r_mrev)}
                unit="10⁶ rev"
                sub={
                  toHours(life.l_10r_mrev) === null
                    ? 'Solve 전 — 시간 환산 불가'
                    : `${num(toHours(life.l_10r_mrev)!)} h`
                }
                color={C_REF}
              />
              <Card
                label="L_nmr"
                value={opt(life.l_nmr_mrev)}
                unit="10⁶ rev"
                sub={
                  life.l_nmr_mrev === null
                    ? 'ν 를 입력하면 산출된다'
                    : toHours(life.l_nmr_mrev) === null
                      ? 'Solve 전 — 시간 환산 불가'
                      : `${num(toHours(life.l_nmr_mrev)!)} h`
                }
                color={C_LOADED}
              />
              <Card label="C_r" value={num(life.c_r_n)} unit="N" sub="기본 동정격" color={C_INNER} />
              <Card label="P_ref r" value={num(life.p_ref_r_n)} unit="N" sub="동등가 기준하중 (11)" />
              <Card label="C_u" value={num(life.c_u_n)} unit="N" sub={`피로한계 · ${cond.c_u_method}`} />
            </div>

            <DetailTable
              title="사슬 — 계수 → 정격 → 전동체하중 → 수명 (Theory §7.1~7.9)"
              rows={[
                ['f_c (ISO 281 Table 2)', num(life.f_c), '—'],
                ['b_m (ISO 281 Table 1)', num(life.b_m), '—'],
                ['C_r (채택)', num(life.c_r_n), 'N'],
                ['C_r (식 (1)/(2) 자체산출)', num(life.c_r_computed_n), 'N'],
                ['Q_ci — 내륜 등가 공칭 (1)', num(life.q_ci_n), 'N'],
                ['Q_ce — 외륜 등가 공칭 (2)', num(life.q_ce_n), 'N'],
                ['Q_ei — 내륜 동등가 (5)/(6)', num(life.q_ei_n), 'N'],
                ['Q_ee — 외륜 동등가 (7)/(8)', num(life.q_ee_n), 'N'],
                ['L_10r — 기본 기준정격수명 (9)', num(life.l_10r_mrev), '10⁶ rev'],
                ['P_ref r — 동등가 기준하중 (11)', num(life.p_ref_r_n), 'N'],
                ['P_r — ISO 281 카탈로그 동등가하중', opt(life.p_r_n), 'N'],
                ['Q_u = min(Q_ui, Q_ue) (B.1)(B.9)', num(life.q_u_n), 'N'],
                ['C_u (채택)', num(life.c_u_n), 'N'],
                ['C_u — 정밀법 (B.10)/(B.11)', num(life.c_u_precise_n), 'N'],
                ['C_u — 간이법 (B.18)/(B.19)', num(life.c_u_simplified_n), 'N'],
              ]}
            />

            {/* ⚠ ν₁ 은 위 사슬과 **다른 단**이다 — ν 없이도 항상 나온다 (파일 헤더). */}
            <DetailTable
              title="윤활 보정 — ν₁ 은 ν 와 무관, κ 부터가 ν 를 요구한다 (§7.8~7.10)"
              rows={[
                ['ν₁ — 기준 동점도 (28)/(29)', opt(life.nu_1_mm2_s), 'mm²/s'],
                ['ν — 입력 실동점도', cond.nu_mm2_s === 0 ? '' : num(cond.nu_mm2_s), 'mm²/s'],
                ['κ = ν/ν₁ (27)', opt(life.kappa), '—'],
                ['e_C — 오염계수', num(cond.e_c), '—'],
                ['a_ISO (31)~(33)', opt(life.a_iso), '—'],
                ['a_1 — 신뢰도 계수', num(life.a_1), '—'],
                ['L_nmr — 수정 기준정격수명 (13)', opt(life.l_nmr_mrev), '10⁶ rev'],
              ]}
            />

            <DetailTable
              title="내부 항등 — L_10r = (C_r / P_ref r)³ 가 화면에서도 성립하는가"
              rows={[
                ['(C_r / P_ref r)³', identity === null ? '' : num(identity), '10⁶ rev'],
                ['L_10r (솔버)', num(life.l_10r_mrev), '10⁶ rev'],
                ['상대차', identityRel === null ? '' : num(identityRel), '—'],
              ]}
            />

            {/* 볼별 Q_j 막대 — 라미나별 수명(TRB)의 자리를 대신한다 */}
            {balls.length > 0 ? (
              <div className="h-[300px]">
                <Plot
                  data={[
                    {
                      type: 'bar',
                      x: balls.map((_, i) => i + 1),
                      y: balls.map(b => b.q_n),
                      marker: { color: balls.map(b => (b.loaded ? C_LOADED : C_UNLOADED)) },
                      name: 'Q_j',
                      hovertemplate: '볼 #%{x}<br>Q = %{y:.6g} N<extra></extra>',
                    },
                  ]}
                  layout={{
                    ...darkLayout,
                    height: 300,
                    margin: { l: 60, r: 20, t: 30, b: 40 },
                    title: { text: '볼별 Q_j 와 등가 공칭하중 기준선', font: { size: 12 } },
                    xaxis: { title: { text: '볼 번호' }, dtick: 1 },
                    yaxis: { title: { text: 'Q [N]' } },
                    shapes: [
                      // Q_ci·Q_ce 는 볼 하나가 견디는 **공칭** 하중이다. 실하중이
                      // 여기 얼마나 못 미치는지가 수명 여유의 직관적 척도다.
                      {
                        type: 'line',
                        xref: 'paper',
                        x0: 0,
                        x1: 1,
                        y0: life.q_ei_n,
                        y1: life.q_ei_n,
                        line: { color: C_INNER, width: 1.5, dash: 'dash' },
                      },
                      {
                        type: 'line',
                        xref: 'paper',
                        x0: 0,
                        x1: 1,
                        y0: life.q_ee_n,
                        y1: life.q_ee_n,
                        line: { color: C_OUTER, width: 1.5, dash: 'dot' },
                      },
                    ],
                    annotations: [
                      {
                        xref: 'paper',
                        x: 0.01,
                        y: life.q_ei_n,
                        text: `Q_ei = ${num(life.q_ei_n)} N`,
                        showarrow: false,
                        font: { size: 10, color: C_INNER },
                        yanchor: 'bottom',
                      },
                      {
                        xref: 'paper',
                        x: 0.99,
                        y: life.q_ee_n,
                        text: `Q_ee = ${num(life.q_ee_n)} N`,
                        showarrow: false,
                        font: { size: 10, color: C_OUTER },
                        yanchor: 'top',
                        xanchor: 'right',
                      },
                    ],
                  }}
                  config={plotConfig}
                  style={{ width: '100%', height: '100%' }}
                  useResizeHandler
                />
              </div>
            ) : (
              <p className="text-[12px] text-amber-300/80">
                결과 없음 — 볼별 <span className="font-mono">Q_j</span> 막대는 Solve 후에 그린다. 위 정격·수명은{' '}
                <b>기하와 하중만으로</b> 계산되므로 Solve 없이도 나온다.
              </p>
            )}
          </div>

          {/* ═══ 2단 — 정정격 (ISO 76) ═══════════════════════════════ */}
          {st && (
            <div className="rounded border border-white/10 bg-black/20 p-3 space-y-3">
              <p className="text-[13px] font-semibold text-text-light">
                ② 정정격 — ISO 76 (Theory §7.11~7.13)
              </p>

              <div className="flex flex-wrap gap-2">
                <Card
                  label="S_0"
                  value={num(st.s_0)}
                  unit="—"
                  sub="정적 안전계수 (76-14)"
                  color={st.s_0 < 1 ? '#ef4444' : C_REF}
                />
                <Card label="C_0r" value={num(st.c_0r_n)} unit="N" sub="기본 정정격 (76-1)" color={C_INNER} />
                <Card label="P_0r" value={num(st.p_0r_n)} unit="N" sub="정적 등가 반경하중" color={C_OUTER} />
              </div>

              <DetailTable
                title="사슬 — γ → f_0 → C_0r · X_0 Y_0 → P_0r → S_0"
                rows={[
                  ['γ = D_w cos α / D_pw (표 진입값)', num(st.gamma), '—'],
                  ['f_0 (ISO 76 Table 1)', num(st.f_0), '—'],
                  ['C_0r (채택)', num(st.c_0r_n), 'N'],
                  ['C_0r (식 (76-1) 자체산출)', num(st.c_0r_computed_n), 'N'],
                  ['X_0 (ISO 76 Table 2)', num(st.x_0), '—'],
                  ['Y_0 (ISO 76 Table 2)', num(st.y_0), '—'],
                  ['F_r = √(F_y² + F_z²)', num(st.f_r_n), 'N'],
                  ['F_a = |F_x|', num(st.f_a_n), 'N'],
                  ['P_0r = max{X_0 F_r + Y_0 F_a, F_r}', num(st.p_0r_n), 'N'],
                  ['S_0 = C_0r / P_0r (76-14)', num(st.s_0), '—'],
                ]}
              />

              <div>
                <h4 className="text-sm font-semibold text-text-light mb-2 uppercase tracking-wider">
                  Static Rating Alerts
                </h4>
                {st.alerts.length === 0 ? (
                  <p className="text-[13px] text-text-canvas/60">경고 없음</p>
                ) : (
                  st.alerts.map((a, i) => (
                    <div key={i} className={`flex items-start gap-2 p-2 rounded border mb-1 ${ALERT_COLORS[a.level]}`}>
                      <span className="text-[13px] font-bold mt-0.5">{ALERT_ICONS[a.level]}</span>
                      <div className="min-w-0">
                        <p className="text-[13px] font-medium">{a.code}</p>
                        <p className="text-xs opacity-80">{a.message}</p>
                      </div>
                    </div>
                  ))
                )}
              </div>
            </div>
          )}

          {/* ── 수명 alerts ──────────────────────────────────────────── */}
          <div>
            <h4 className="text-sm font-semibold text-text-light mb-2 uppercase tracking-wider">
              Life Alerts
            </h4>
            {life.alerts.length === 0 ? (
              <p className="text-[13px] text-text-canvas/60">경고 없음</p>
            ) : (
              life.alerts.map((a, i) => (
                <div key={i} className={`flex items-start gap-2 p-2 rounded border mb-1 ${ALERT_COLORS[a.level]}`}>
                  <span className="text-[13px] font-bold mt-0.5">{ALERT_ICONS[a.level]}</span>
                  <div className="min-w-0">
                    <p className="text-[13px] font-medium">{a.code}</p>
                    <p className="text-xs opacity-80">{a.message}</p>
                  </div>
                </div>
              ))
            )}
          </div>

          <p className="text-[11px] text-text-canvas/50 leading-relaxed">
            재사용: TRB <span className="font-mono">charts/LifeChart</span> 의 「요약 카드 → 사슬 표 → 차트」 3층
            배치를 옮겼다(원본 미수정). 라미나별 수명은 볼에 대응물이 없어 폐기하고 볼별{' '}
            <span className="font-mono">Q_j</span> 막대로 바꿨다. 커맨드 호출은{' '}
            <span className="font-mono">BbGeometryView</span> 의 150 ms 디바운스 + <span className="font-mono">cancelled</span>{' '}
            패턴과 같다.
          </p>
        </>
      )}
    </div>
  );
}
