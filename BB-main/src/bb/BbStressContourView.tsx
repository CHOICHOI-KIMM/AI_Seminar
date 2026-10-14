// BB 접촉응력 뷰 (Plan §3.6.5.2 S5 · §3.6.4.3 처분표 「Stress Contour — 개조」)
//
// ─────────────────────────────────────────────────────────────────────
//  이 화면의 검증 임무 — §3.6.4.2
// ─────────────────────────────────────────────────────────────────────
//  대응 Level: **B**(Harris Table 6.1) · **B-3**(1973 Fig. 15 실기 대조).
//  깨졌을 때의 징후는 「**타원비가 1 에 가까움**(χ 해 오류)」 · 「**내·외륜 타원이 뒤바뀜**」이다.
//
//  → 그래서 이 화면은 두 가지를 **동시에** 보여야 한다.
//    ① `a`·`b` 를 **축척 1:1** 로 그려 타원비를 **눈으로** 판정 (`scaleanchor`)
//    ② `χ`·`a*`·`b*`·`δ*` 를 **숫자로** 내어 Harris Table 6.1 과 직접 대조 (Level B 원본 골든값)
//    ①만 있으면 「비슷해 보인다」에서 끝나고, ②만 있으면 뒤바뀜을 못 본다.
//
// ─────────────────────────────────────────────────────────────────────
//  🔴 역할 분담 — §3.6.4.4 (이 경계를 넘지 말 것)
// ─────────────────────────────────────────────────────────────────────
//  | | `BbLoadDistView` (S4) | **이 뷰** (S5) |
//  |---|---|---|
//  | 성격 | **전 볼을 한눈에** (분포) | **볼 하나를 자세히** (상세) |
//  | 내용 | `Q_j(φ)` · `α_j(φ)` · 위상 스윕 · 접촉타원 **형상**(볼별 비교 막대) | 선택한 볼의 타원 **내부** `p(x, y)` 히트맵, 내/외륜 |
//
//  ⚠ **형상 비교 막대·볼별 비교를 여기서 다시 그리지 않는다.** S4 가 이미 그린다.
//    같은 것을 두 번 그리면 어느 쪽이 최신인지 알 수 없게 된다.
//
// ─────────────────────────────────────────────────────────────────────
//  압력분포 식 — Hertz 반타원체 (지어낸 식이 아니다)
// ─────────────────────────────────────────────────────────────────────
//      p(x, y) = p_max · √( 1 − (x/a)² − (y/b)² ),   (x/a)² + (y/b)² ≤ 1
//      p(x, y) = 접촉 없음 (NaN),                     그 밖
//
//  · `a`·`b`·`p_max` 의 정의는 **Theory §6.1~§6.3** — Harris (6.38)(6.40)(6.25).
//    `p_max = 3Q / (2π a b)` (Theory 본문 §6.3, 파일 라인 607).
//  · 위 반타원체를 접촉면 위에서 적분하면 `∫p dA = (2/3)·p_max·πab = Q` 다.
//    즉 **식과 `p_max` 정의가 서로를 함축**한다 — 이 항등을 화면에서 재계산해 표로 낸다
//    (아래 「p_max 재계산 대조」). 솔버(`hertz.rs::contact_ellipse`)와 화면이 같은
//    물리를 쓰는지 **숫자로** 확인되는 지점이다.
//  · ⚠ 임의의 다른 압력식(포물선 근사 등)을 쓰지 않는다. 쓰는 순간 `p_max` 와 어긋난다.
//
// ─────────────────────────────────────────────────────────────────────
//  σ_Hu = 1 500 MPa 등고선
// ─────────────────────────────────────────────────────────────────────
//  `hertz.rs` 의 `pub const SIGMA_HU_MPA: f64 = 1500.0` (라인 337) 를 **소스에서 확인**해 옮겼다.
//  근거는 ISO 281 Annex B.3.1 의 피로한계 접촉응력 권장값이며(Theory §6.3),
//  `bearing.rs`·`commands.rs` 가 이 상수로 `CONTACT_STRESS_OVER_FATIGUE_LIMIT` 경고를 낸다.
//  → 화면의 등고선과 `AlertPanel` 의 경고가 **같은 기준**임이 이 뷰에서 눈으로 이어진다
//    (§3.6.4.2 의 `AlertPanel` 항목).
//
//  등고선은 격자에서 찾지 않고 **해석적으로** 그린다. `p = σ_Hu` 인 자취는
//      (x/a)² + (y/b)² = 1 − (σ_Hu/p_max)²
//  이므로 반경 `a·√(1−(σ_Hu/p_max)²)` · `b·√(…)` 인 **동심 타원**이다. 격자 해상도에
//  좌우되지 않으므로 초과 면적비도 정확히 `1 − (σ_Hu/p_max)²` 로 낼 수 있다.
//
// ─────────────────────────────────────────────────────────────────────
//  🔴 데이터 출처 — §3.6.4.7 의 경고
// ─────────────────────────────────────────────────────────────────────
//  「`bb_compute_contact` 를 접촉타원 뷰의 **주 데이터원으로 쓰면 안 된다.**
//    평형 해의 `Q_j` 는 이미 `BbResult.ball_results[]` 에 있고, 거기서 나온
//    `a`·`b`·`p_max` 를 그려야 **화면과 검증 결과가 같은 숫자**가 된다.」
//
//  → 히트맵·수치표의 **주 데이터는 `result.equilibrium.ball_results[]` 뿐**이다.
//  → `bb_compute_contact` 는 두 곳에서만 쓰고, 둘 다 **평형 해가 아님을 화면에 명시**한다:
//     ⓐ `q_n = 0` 호출 — 하중을 넣지 않으므로 타원이 나오지 않는다. **하중 무관 전처리**
//        (`χ`·`K`·`E`·`a*`·`b*`·`δ*`·`E*`·`c_P`)만 받아 **Level B 대조표**로 쓴다.
//        이 값들은 `BbResult` 어디에도 없어 다른 경로가 없다.
//     ⓑ 사용자가 임의 `Q` 를 넣는 **what-if** 패널 — 호박색 점선 테두리 + 「평형 해 아님」 표기로 분리.
//        Harris Table 6.1 처럼 특정 Q 가 지정된 문헌값을 그대로 넣어 대조할 때 쓴다 (B · B-3).
//  → 하중 입력이 필요하면 `state.bbInput`(편집 중)이 아니라
//    **`state.resultInput`**(Solve 시점 스냅샷)을 읽는다 (S4-3 에서 신설).
//
// ─────────────────────────────────────────────────────────────────────
//  단위·자릿수 (Plan Phase 4-5 §4-5.3)
// ─────────────────────────────────────────────────────────────────────
//  표시는 `shared.ts` 의 단위별 함수만 쓴다 — 응력 MPa 정수 · 하중 N 소수 1 · 각도 ° 소수 2 ·
//  반축 a·b mm 소수 4 · 변위 δ μm 소수 2 · 무차원 소수 4 · 재계산 상대차는 지수.
//  Plotly hover·축도 같은 자릿수다. 정밀 대조(B·B-3)는 Rust 테스트와 헬스체크 몫이다 (§4-5.1 결정 4).

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useAppState } from '../store';
import Plot from '../components/charts/PlotWithCopy';
import { darkLayout, plotConfig, viridisScale } from '../components/charts/plotlyDefaults';
import { DetailTable, ErrorBox, WarnBox } from './ui';
import type { BallResult } from './generated/BallResult';
import type { BbContactDerived } from './generated/BbContactDerived';
import type { Alert } from './generated/Alert';
import {
  toDeg,
  fmtMPa,
  fmtGPa,
  fmtN,
  fmtDeg,
  fmtUm,
  fmtMmFine,
  fmtMm2,
  fmtDimless,
  fmtStiff,
  fmtExp,
  C_INNER,
  C_OUTER,
  C_HU,
} from './shared';

/**
 * ISO 281 Annex B.3.1 피로한계 접촉응력 [MPa].
 *
 * ⚠ **`src-tauri/src/solver/bb/hertz.rs` 의 `SIGMA_HU_MPA` (라인 337) 와 같은 값이어야 한다.**
 * 여기서 다른 값을 쓰면 화면의 등고선과 `AlertPanel` 의
 * `CONTACT_STRESS_OVER_FATIGUE_LIMIT` 경고가 **다른 기준**을 말하게 된다.
 */
const SIGMA_HU_MPA = 1500;


type Race = 'inner' | 'outer';

/** `commands::ContactResponse` 대응 (Rust 쪽은 ts-rs 대상이 아니라 커맨드 전용 래퍼다). */
interface ContactResponse {
  derived: BbContactDerived;
  q_n: number;
  delta_mm: number;
  a_inner_mm: number;
  b_inner_mm: number;
  p_max_inner_mpa: number;
  a_outer_mm: number;
  b_outer_mm: number;
  p_max_outer_mpa: number;
  alerts: Alert[];
}

/** 선택한 궤도의 타원 3인방을 `BallResult` 에서 꺼낸다 (필드명은 `generated/BallResult.ts` 확인). */
function ellipseOf(b: BallResult, race: Race) {
  return race === 'inner'
    ? { a: b.a_inner_mm, b: b.b_inner_mm, pMax: b.p_max_inner_mpa }
    : { a: b.a_outer_mm, b: b.b_outer_mm, pMax: b.p_max_outer_mpa };
}

/**
 * 타원 **내부** 압력분포 히트맵 — 이 뷰의 본체.
 *
 * ⚠ **축척 1:1**(`scaleanchor`/`scaleratio`)이 이 그림의 존재 이유다. 축척을 놓으면
 *   「타원비가 1 에 가까운가」(§3.6.4.2 의 첫 징후)를 눈으로 판정할 수 없다.
 * ⚠ 타원 **밖은 `null`(NaN)** 로 둔다 — 0 으로 채우면 「압력이 0 인 접촉면」처럼 보인다.
 *   접촉이 **없는** 것과 압력이 **0 인** 것은 다르다.
 * ⚠ `zsmooth` 를 끈다. 타원 경계는 압력 기울기가 무한대인 실제 불연속이라,
 *   보간해 부드럽게 만들면 없는 값을 그려 넣는 셈이 된다.
 */
function PressureHeatmap({
  a,
  b,
  pMax,
  race,
  ballNo,
  phiDeg,
}: {
  a: number;
  b: number;
  pMax: number;
  race: Race;
  ballNo: number;
  phiDeg: number;
}) {
  const N = 181;
  const pad = 1.12;
  const xs = Array.from({ length: N }, (_, i) => -a * pad + (2 * a * pad * i) / (N - 1));
  const ys = Array.from({ length: N }, (_, j) => -b * pad + (2 * b * pad * j) / (N - 1));

  // p(x, y) = p_max · √(1 − (x/a)² − (y/b)²)   ← Hertz 반타원체 (파일 헤더 참조)
  const z: (number | null)[][] = ys.map(y =>
    xs.map(x => {
      const r2 = (x / a) ** 2 + (y / b) ** 2;
      return r2 <= 1 ? pMax * Math.sqrt(1 - r2) : null;
    })
  );

  // 해석적 타원 자취. `p = level` 인 곳은 (x/a)²+(y/b)² = 1 − (level/p_max)².
  const ringAt = (level: number, color: string, dash: 'dot' | 'dash', name: string): Plotly.Data | null => {
    const s2 = 1 - (level / pMax) ** 2;
    if (!(s2 > 0)) return null;
    const s = Math.sqrt(s2);
    const t = Array.from({ length: 241 }, (_, k) => (k * 2 * Math.PI) / 240);
    return {
      type: 'scatter',
      x: t.map(v => a * s * Math.cos(v)),
      y: t.map(v => b * s * Math.sin(v)),
      mode: 'lines',
      line: { color, width: 2, dash },
      name,
      hovertemplate: `${name}<br>x = %{x:.4f} mm<br>y = %{y:.4f} mm<extra></extra>`,
    } as Plotly.Data;
  };

  const data: Plotly.Data[] = [
    {
      type: 'heatmap',
      z,
      x: xs,
      y: ys,
      zmin: 0,
      zmax: pMax,
      colorscale: viridisScale,
      zsmooth: false,
      colorbar: {
        title: { text: 'p [MPa]', font: { size: 12, color: '#94a3b8' } },
        tickfont: { size: 11, family: 'JetBrains Mono', color: '#94a3b8' },
        len: 0.85,
        thickness: 12,
      },
      hovertemplate: 'x = %{x:.4f} mm<br>y = %{y:.4f} mm<br>p = %{z:.0f} MPa<extra></extra>',
    } as Plotly.Data,
  ];

  // 접촉 경계 (p = 0) — 타원 그 자체
  const edge = ringAt(0, '#ffffff', 'dot', `접촉 경계 (a = ${fmtMmFine(a)}, b = ${fmtMmFine(b)} mm)`);
  if (edge) data.push(edge);

  // 🔴 σ_Hu 등고선 — `AlertPanel` 의 CONTACT_STRESS_OVER_FATIGUE_LIMIT 과 같은 기준
  const hu = ringAt(SIGMA_HU_MPA, C_HU, 'dash', `σ_Hu = ${SIGMA_HU_MPA} MPa`);
  if (hu) data.push(hu);

  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: {
      text: `볼 #${ballNo} (φ = ${fmtDeg(phiDeg)}°) — ${race === 'inner' ? '내륜' : '외륜'} 압력분포`,
      font: { size: 14, color: '#e2e8f0' },
    },
    xaxis: {
      ...darkLayout.xaxis,
      title: { text: 'a 방향 (장반경) [mm]' },
      constrain: 'domain',
      tickformat: '.4f',
    },
    yaxis: {
      ...darkLayout.yaxis,
      title: { text: 'b 방향 (단반경) [mm]' },
      // ⚠ 이것을 빼면 이 그림의 검증 가치가 사라진다 (파일 헤더 참조).
      scaleanchor: 'x',
      scaleratio: 1,
      constrain: 'domain',
      tickformat: '.4f',
    },
    margin: { l: 70, r: 20, t: 40, b: 50 },
    showlegend: true,
    legend: { font: { size: 11, color: '#94a3b8' }, bgcolor: 'rgba(15,23,42,0.6)', x: 0.01, y: 0.99 },
  };

  return (
    <div className="h-[420px]">
      <Plot
        data={data}
        layout={layout}
        config={plotConfig}
        style={{ width: '100%', height: '100%' }}
        useResizeHandler
      />
    </div>
  );
}

/**
 * 장·단축 단면 `p(x, 0)` · `p(0, y)`.
 *
 * 히트맵은 색으로 보므로 「반타원체인가 포물선인가」가 구분되지 않는다. 단면을 함께 내면
 * 곡선 모양과 `p_max` 도달점이 숫자로 읽힌다. 축척 1:1 을 쓰지 않는 유일한 그림이다
 * (여기서는 형상이 아니라 **값**을 본다).
 */
function PressureProfiles({ a, b, pMax }: { a: number; b: number; pMax: number }) {
  const M = 241;
  const u = Array.from({ length: M }, (_, i) => -1 + (2 * i) / (M - 1));
  const p = u.map(t => pMax * Math.sqrt(Math.max(0, 1 - t * t)));

  const data: Plotly.Data[] = [
    {
      type: 'scatter',
      x: u.map(t => t * a),
      y: p,
      mode: 'lines',
      line: { color: C_INNER, width: 2 },
      name: 'p(x, 0) — 장축 단면',
      hovertemplate: 'x = %{x:.4f} mm<br>p = %{y:.0f} MPa<extra></extra>',
    } as Plotly.Data,
    {
      type: 'scatter',
      x: u.map(t => t * b),
      y: p,
      mode: 'lines',
      line: { color: C_OUTER, width: 2 },
      name: 'p(0, y) — 단축 단면',
      hovertemplate: 'y = %{x:.4f} mm<br>p = %{y:.0f} MPa<extra></extra>',
    } as Plotly.Data,
  ];

  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: { text: '접촉면 단면 압력', font: { size: 14, color: '#e2e8f0' } },
    xaxis: { ...darkLayout.xaxis, title: { text: '접촉면 좌표 [mm]' } },
    yaxis: { ...darkLayout.yaxis, title: { text: 'p [MPa]' }, rangemode: 'tozero' },
    shapes: [
      {
        type: 'line',
        xref: 'paper',
        x0: 0,
        x1: 1,
        y0: SIGMA_HU_MPA,
        y1: SIGMA_HU_MPA,
        line: { color: C_HU, width: 2, dash: 'dash' },
      },
    ],
    annotations: [
      {
        xref: 'paper',
        x: 0.99,
        y: SIGMA_HU_MPA,
        text: `σ_Hu = ${SIGMA_HU_MPA} MPa`,
        showarrow: false,
        xanchor: 'right',
        yanchor: 'bottom',
        font: { size: 11, color: C_HU },
      },
    ],
    showlegend: true,
    legend: { font: { size: 11, color: '#94a3b8' }, bgcolor: 'transparent' },
  };

  return (
    <div className="h-[280px]">
      <Plot
        data={data}
        layout={layout}
        config={plotConfig}
        style={{ width: '100%', height: '100%' }}
        useResizeHandler
      />
    </div>
  );
}

/**
 * 하중 무관 전처리 (`bb_compute_contact` 를 `q_n = 0` 으로 호출해 받는다).
 *
 * **Level B 의 골든값이 Harris Table 6.1 의 `a*`·`b*`·`δ*`** 인데, 그 셋은 `BbResult`
 * 어디에도 실려 오지 않는다 (`BallResult` 는 결과 `a`·`b`·`p_max` 만 갖는다).
 * 화면에서 B 를 대조하려면 이 경로가 유일하다.
 *
 * ⚠ `q_n = 0` 이므로 **타원·응력이 나오지 않는다** — 하중 의존 값은 여기서 쓰지 않는다.
 *   히트맵과 수치표의 주 데이터는 어디까지나 평형 해다 (§3.6.4.7).
 *
 * 대조 방법 (Phase 4-5 에서 화면 설명문을 지우며 옮김): Harris Table 6.1 은 `F(ρ) → a*, b*, δ*`
 * 표이므로, Geometry 탭의 `f_rho_i · f_rho_e` 로 표를 찾아 이 표의 `a* · b* · δ*` 와 대조하면
 * Level B 가 화면에서 확인된다. `χ` 는 평형 해의 `a/b` 와 같은 값이어야 하며, 1 에 가까우면
 * χ 해 오류다 (§3.6.4.2 징후).
 */
function DerivedTable({ derived }: { derived: BbContactDerived }) {
  return (
    <DetailTable
      title="하중 무관 전처리"
      rows={[
        ['χ_i = a/b (내륜)', fmtDimless(derived.chi_inner), ''],
        ['χ_e = a/b (외륜)', fmtDimless(derived.chi_outer), ''],
        ['K(χ_i)', fmtDimless(derived.k_ellip_inner), ''],
        ['E(χ_i)', fmtDimless(derived.e_ellip_inner), ''],
        ['K(χ_e)', fmtDimless(derived.k_ellip_outer), ''],
        ['E(χ_e)', fmtDimless(derived.e_ellip_outer), ''],
        ['a*_i (Harris 6.44)', fmtDimless(derived.a_star_inner), ''],
        ['b*_i (Harris 6.45)', fmtDimless(derived.b_star_inner), ''],
        ['δ*_i (Harris 6.46)', fmtDimless(derived.delta_star_inner), ''],
        ['a*_e', fmtDimless(derived.a_star_outer), ''],
        ['b*_e', fmtDimless(derived.b_star_outer), ''],
        ['δ*_e', fmtDimless(derived.delta_star_outer), ''],
        ['E* 등가 탄성계수', fmtGPa(derived.e_star_mpa), 'GPa'],
        ['c_P 점접촉 스프링상수', fmtStiff(derived.c_p_n_per_mm15), 'N/mm^(3/2)'],
      ]}
    />
  );
}

export default function BbStressContourView() {
  const { state } = useAppState();
  const { result, resultInput } = state;

  const [selected, setSelected] = useState<number | null>(null);
  const [race, setRace] = useState<Race>('inner');

  // ⓐ 하중 무관 전처리 (q_n = 0)
  const [derived, setDerived] = useState<BbContactDerived | null>(null);
  const [derivedErr, setDerivedErr] = useState<string | null>(null);

  // ⓑ what-if — **평형 해가 아니다.** 호박색으로 분리 표시한다.
  const [whatIfQ, setWhatIfQ] = useState('1000');
  const [whatIf, setWhatIf] = useState<ContactResponse | null>(null);
  const [whatIfErr, setWhatIfErr] = useState<string | null>(null);

  useEffect(() => {
    if (!resultInput) return;
    let cancelled = false;
    // ⚠ Tauri v2 는 커맨드 인자를 **camelCase** 로 받는다 (`tauri-macros` 기본 `ArgumentCase::Camel`).
    //   Rust 시그니처는 `bb_compute_contact(input: BbInput, q_n: f64)` 이므로 JS 키는 `qN` 이다.
    invoke<ContactResponse>('bb_compute_contact', { input: resultInput, qN: 0 })
      .then(r => {
        if (cancelled) return;
        setDerived(r.derived);
        setDerivedErr(null);
      })
      .catch((e: unknown) => {
        if (cancelled) return;
        // Rust 쪽 메시지를 **그대로** 낸다 — 프론트가 고쳐 쓰면 어느 쪽이 거부했는지 모른다 (§3.6.5.3).
        setDerivedErr(String(e));
        setDerived(null);
      });
    return () => {
      cancelled = true;
    };
  }, [resultInput]);

  const runWhatIf = () => {
    if (!resultInput) return;
    const q = Number(whatIfQ);
    if (!Number.isFinite(q) || q < 0) {
      setWhatIfErr('Q 는 0 이상의 유한한 수여야 한다');
      setWhatIf(null);
      return;
    }
    invoke<ContactResponse>('bb_compute_contact', { input: resultInput, qN: q })
      .then(r => {
        setWhatIf(r);
        setWhatIfErr(null);
      })
      .catch((e: unknown) => {
        setWhatIfErr(String(e));
        setWhatIf(null);
      });
  };

  if (!result) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-text-canvas text-sm">Solve 를 눌러 결과를 만드세요</p>
      </div>
    );
  }

  const balls = result.equilibrium.ball_results;

  // 기본 선택 = **최대하중 볼**. 그 볼이 σ_Hu 를 넘는지가 이 화면의 첫 질문이다.
  let top = -1;
  for (let i = 0; i < balls.length; i++) {
    if (top < 0 || balls[i].q_n > balls[top].q_n) top = i;
  }
  // 결과가 바뀌어 Z 가 줄면 예전 선택이 범위를 벗어난다 — 그때는 기본값으로 되돌린다.
  const sel = selected !== null && selected >= 0 && selected < balls.length ? selected : top;
  const ball = sel >= 0 ? balls[sel] : null;

  const el = ball ? ellipseOf(ball, race) : null;
  // 반타원체 항등 `∫p dA = (2/3)p_max·πab = Q` 의 역: p_max = 3Q/(2πab).
  // 솔버(`hertz.rs::contact_ellipse`)가 낸 `p_max` 와 대조한다.
  const pMaxRecomputed =
    ball && el && el.a > 0 && el.b > 0 ? (3 * ball.q_n) / (2 * Math.PI * el.a * el.b) : null;
  const pMaxRelDiff =
    pMaxRecomputed !== null && el && el.pMax > 0 ? (pMaxRecomputed - el.pMax) / el.pMax : null;

  const overHu = el !== null && el.pMax > SIGMA_HU_MPA;
  // p = σ_Hu 등고선 안쪽의 면적비 (해석적) = 1 − (σ_Hu/p_max)²
  const overHuAreaFrac = overHu && el ? 1 - (SIGMA_HU_MPA / el.pMax) ** 2 : 0;

  return (
    <div className="h-full overflow-auto custom-scrollbar p-4 space-y-5">
      {/* ── 볼 선택 + 내/외륜 탭 ───────────────────────────────────── */}
      <div className="flex flex-wrap items-center gap-3">
        <label className="text-[12px] text-text-canvas">
          볼 선택
          <select
            value={sel}
            onChange={e => setSelected(Number(e.target.value))}
            className="ml-2 bg-canvas-subtle border border-white/15 rounded px-2 py-1 text-[12px] font-mono text-text-light"
          >
            {balls.map((b, i) => (
              <option key={i} value={i}>
                #{i + 1} · φ = {fmtDeg(toDeg(b.phi_rad))}° · Q = {fmtN(b.q_n)} N
                {b.loaded ? '' : ' · 비접촉'}
                {i === top ? '  ← 최대하중' : ''}
              </option>
            ))}
          </select>
        </label>

        <button
          onClick={() => setSelected(top)}
          className="px-2 py-1 rounded text-[12px] border border-white/15 text-text-canvas hover:text-text-light hover:bg-white/5 cursor-pointer"
          title="기본값 — 최대하중 볼"
        >
          최대하중 볼로
        </button>

        <div className="flex items-center gap-0.5 ml-auto">
          {(['inner', 'outer'] as const).map(r => (
            <button
              key={r}
              onClick={() => setRace(r)}
              className={`px-3 py-1 text-[12px] font-medium rounded border cursor-pointer transition-colors ${
                race === r
                  ? 'bg-canvas-subtle text-text-light border-white/25'
                  : 'text-text-canvas border-white/10 hover:text-text-light hover:bg-white/5'
              }`}
            >
              {r === 'inner' ? '내륜 (inner)' : '외륜 (outer)'}
            </button>
          ))}
        </div>
      </div>

      {/* ── 본체 ───────────────────────────────────────────────────── */}
      {!ball || !el ? (
        <p className="text-[13px] text-text-canvas">볼 결과가 없다.</p>
      ) : !ball.loaded || el.a <= 0 || el.b <= 0 || el.pMax <= 0 ? (
        // 「비접촉」 — 하중을 받지 않는 볼에는 접촉타원도 압력분포도 없다.
        // 0 을 그려 「압력 0 인 접촉면」처럼 보이게 하지 않는다.
        // 하중구간 밖이라는 뜻이며, 몇 번 볼이 하중구간에 드는지는 Load Distribution 극좌표가 보인다 (C-7).
        <p className="text-[13px] text-text-canvas">
          볼 #{sel + 1} 비접촉 — 압력분포 없음 (Q_j = {fmtN(ball.q_n)} N)
        </p>
      ) : (
        <>
          <PressureHeatmap
            a={el.a}
            b={el.b}
            pMax={el.pMax}
            race={race}
            ballNo={sel + 1}
            phiDeg={toDeg(ball.phi_rad)}
          />

          {/* σ_Hu 초과 여부. 같은 상수로 AlertPanel 의 CONTACT_STRESS_OVER_FATIGUE_LIMIT 이 난다 (파일 헤더). */}
          {overHu ? (
            <WarnBox>
              σ_Hu 초과 — {race === 'inner' ? '내륜' : '외륜'} p_max ={' '}
              <span className="font-mono">{fmtMPa(el.pMax)}</span> MPa &gt; {SIGMA_HU_MPA} MPa
            </WarnBox>
          ) : (
            <p className="text-[12px] text-text-canvas">
              p_max = <span className="font-mono">{fmtMPa(el.pMax)}</span> MPa ≤ σ_Hu = {SIGMA_HU_MPA} MPa —
              초과 영역 없음
            </p>
          )}

          <PressureProfiles a={el.a} b={el.b} pMax={el.pMax} />

          <DetailTable
            title={`선택 볼 #${sel + 1}`}
            rows={[
              ['φ_j 각위치', fmtDeg(toDeg(ball.phi_rad)), '°'],
              ['δ_j 총 탄성변형', fmtUm(ball.delta_mm), 'μm'],
              ['α_j 운전 접촉각', fmtDeg(toDeg(ball.alpha_rad)), '°'],
              ['Q_j 볼 하중', fmtN(ball.q_n), 'N'],
              ['— 내륜 —', '', ''],
              ['a_i 장반경', fmtMmFine(ball.a_inner_mm), 'mm'],
              ['b_i 단반경', fmtMmFine(ball.b_inner_mm), 'mm'],
              ['a/b_i 타원비', ball.b_inner_mm > 0 ? fmtDimless(ball.a_inner_mm / ball.b_inner_mm) : '—', ''],
              ['p_max,i', fmtMPa(ball.p_max_inner_mpa), 'MPa'],
              ['— 외륜 —', '', ''],
              ['a_e 장반경', fmtMmFine(ball.a_outer_mm), 'mm'],
              ['b_e 단반경', fmtMmFine(ball.b_outer_mm), 'mm'],
              ['a/b_e 타원비', ball.b_outer_mm > 0 ? fmtDimless(ball.a_outer_mm / ball.b_outer_mm) : '—', ''],
              ['p_max,e', fmtMPa(ball.p_max_outer_mpa), 'MPa'],
              ['— 표시 중인 궤도 대조 —', race === 'inner' ? '내륜' : '외륜', ''],
              ['접촉면적 π·a·b', fmtMm2(Math.PI * el.a * el.b), 'mm²'],
              ['평균압력 Q/(π a b)', fmtMPa(ball.q_n / (Math.PI * el.a * el.b)), 'MPa'],
              ['p_max 재계산 3Q/(2π a b)', pMaxRecomputed === null ? '—' : fmtMPa(pMaxRecomputed), 'MPa'],
              ['재계산 상대차 (솔버 대비)', pMaxRelDiff === null ? '—' : fmtExp(pMaxRelDiff), ''],
              ['σ_Hu (ISO 281 Annex B.3.1)', fmtMPa(SIGMA_HU_MPA), 'MPa'],
              ['p_max / σ_Hu', fmtDimless(el.pMax / SIGMA_HU_MPA), ''],
              ['σ_Hu 초과 면적비', overHu ? fmtDimless(overHuAreaFrac) : '0', ''],
            ]}
          />
        </>
      )}

      {/* ── Level B 대조표 (하중 무관 전처리) ──────────────────────── */}
      {derivedErr && <ErrorBox title="bb_compute_contact 실패" message={derivedErr} />}
      {derived && <DerivedTable derived={derived} />}

      {/* ── what-if — 🔶 평형 해가 아니다 (파일 헤더 「데이터 출처」 ⓑ) ── */}
      <div className="p-3 rounded border-2 border-dashed border-amber-400/50 bg-amber-500/[0.06] space-y-2">
        <p className="text-[12px] text-amber-200">what-if — 평형 해 아님</p>
        <div className="flex flex-wrap items-center gap-2">
          <label className="text-[12px] text-amber-100">
            Q =
            <input
              value={whatIfQ}
              onChange={e => setWhatIfQ(e.target.value)}
              className="ml-2 w-32 bg-canvas-subtle border border-amber-400/30 rounded px-2 py-1 text-[12px] font-mono text-text-light"
            />
            <span className="ml-1 font-mono">N</span>
          </label>
          <button
            onClick={runWhatIf}
            disabled={!resultInput}
            className="px-3 py-1 rounded text-[12px] border border-amber-400/40 text-amber-100 hover:bg-amber-400/10 cursor-pointer disabled:opacity-40 disabled:cursor-not-allowed"
          >
            계산
          </button>
          {ball && ball.loaded && (
            <button
              onClick={() => setWhatIfQ(String(ball.q_n))}
              className="px-2 py-1 rounded text-[11px] border border-white/15 text-text-canvas hover:text-text-light hover:bg-white/5 cursor-pointer"
              title="선택 볼의 Q_j 를 넣어 본다 — 평형 해와 같은 값이 나와야 한다 (교차검증)"
            >
              선택 볼 Q_j 넣기
            </button>
          )}
        </div>
        {whatIfErr && <ErrorBox title="bb_compute_contact 실패" message={whatIfErr} />}
        {whatIf && (
          <DetailTable
            title={`what-if — q_n = ${fmtN(whatIf.q_n)} N`}
            rows={[
              ['δ 총 탄성변형', fmtUm(whatIf.delta_mm), 'μm'],
              ['a_i', fmtMmFine(whatIf.a_inner_mm), 'mm'],
              ['b_i', fmtMmFine(whatIf.b_inner_mm), 'mm'],
              [
                'a/b_i',
                whatIf.b_inner_mm > 0 ? fmtDimless(whatIf.a_inner_mm / whatIf.b_inner_mm) : '—',
                '',
              ],
              ['p_max,i', fmtMPa(whatIf.p_max_inner_mpa), 'MPa'],
              ['a_e', fmtMmFine(whatIf.a_outer_mm), 'mm'],
              ['b_e', fmtMmFine(whatIf.b_outer_mm), 'mm'],
              [
                'a/b_e',
                whatIf.b_outer_mm > 0 ? fmtDimless(whatIf.a_outer_mm / whatIf.b_outer_mm) : '—',
                '',
              ],
              ['p_max,e', fmtMPa(whatIf.p_max_outer_mpa), 'MPa'],
              ['alerts', String(whatIf.alerts.length), '건'],
            ]}
          />
        )}
      </div>
    </div>
  );
}
