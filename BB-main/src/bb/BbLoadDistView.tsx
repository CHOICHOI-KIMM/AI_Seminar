// BB 하중분포 뷰 (Plan §3.6.5.2 S4 · §3.6.4.3 처분표 「Load Distribution — 개조 + 접촉타원 통합」)
//
// ─────────────────────────────────────────────────────────────────────
//  이 화면의 검증 임무 — §3.6.4.2
// ─────────────────────────────────────────────────────────────────────
//  이 뷰는 **핵심 검증 뷰**다. 아래 4개를 화면에서 확인할 수 있어야 한다.
//
//  | 구성 | 확인하는 것 | 대응 Level |
//  |---|---|---|
//  | `Q_j(φ)` 극좌표 | 하중구간의 **방위**와 폭 · 대칭성 | C-4(회전 불변) · C-7(하중구간) · D-2b/2c |
//  | `α_j(φ)` 곡선   | 접촉각이 **볼마다 다르고 하중에 따라 변한다**는 사실 | D-1 · C-2 |
//  | 위상 스윕 곡선  | `Q_max` 가 주기 **2π/Z** 로 진동하는가 | C-5 |
//  | 접촉타원 형상   | 볼별 `a`·`b`·비율 `a/b` | (형상만) |
//
//  ⚠ **접촉타원 내부의 압력분포 `p(x,y)` 는 이 뷰의 소관이 아니다** — `BbStressContourView`(S5).
//    역할 분담은 §3.6.4.4 가 정한다: 여기는 「전 볼을 한눈에」(분포), 저기는 「볼 하나를 자세히」(상세).
//
// ─────────────────────────────────────────────────────────────────────
//  🔴 방위 규약 — 이것을 틀리면 검증 자체가 무의미해진다
// ─────────────────────────────────────────────────────────────────────
//  §3.6.4.2 가 드는 첫 징후가 「**하중구간이 하중 방향과 어긋남**」이다.
//  그런데 **각도축의 0 을 잘못 잡으면 솔버가 정상인데도 어긋나 보인다.**
//  따라서 축 정의를 화면 쪽에서 지어내지 않고 **솔버 소스에서 직접 확인해** 고정했다.
//
//    · `solver/bb/bearing.rs` 헤더:  `R_j = A cos α₀ + δ_y cos φ_j + δ_z sin φ_j`
//      → 볼 j 의 반경 단위벡터가 `(cos φ_j, sin φ_j)` in `(Y, Z)` 다.
//        즉 **`φ = 0` 이 +Y 축, `φ = 90°` 가 +Z 축**이다.
//    · `generated/BallResult.ts` 의 `phi_rad` 주석: 「D-8: φ_1 = 0 이 Y축 방향」 — 같은 결론.
//    · Theory §4.4 확정형의 `F_y = Σ Q cos α cos φ` · `F_z = Σ Q cos α sin φ` 도 같다.
//
//  → 그래서 극좌표의 각도축을 **`φ [°]` 로 직접** 쓰고(회전·방향반전 없음),
//    「θ = 0 은 +Y 축」을 **화면에 명시**하며,
//    외부 반경하중의 방위 `atan2(F_z, F_y)` 를 **같은 축 위에 기준선으로 겹쳐 그린다.**
//    하중구간이 그 선과 정렬되는지가 한눈에 보이는 것 — 그것이 이 뷰의 핵심 임무다.
//
// ─────────────────────────────────────────────────────────────────────
//  단위 정책 (S2 확정 — 바꾸지 말 것)
// ─────────────────────────────────────────────────────────────────────
//  하중 **N**(kN 로 나누지 않는다) · 각도 **°** · 타원 `a`·`b` **mm** · 응력 **MPa** ·
//  **틸트 γ 만 rad**. hover·표는 **유효숫자 9자리** — 줄이면 검증 대조가 불가능하다.
//
// ─────────────────────────────────────────────────────────────────────
//  ⚠ §3.6.1.4 통합 대비 — 컴포넌트를 미리 갈라 둔다
// ─────────────────────────────────────────────────────────────────────
//  나중 통합 시 이 뷰는 **`LoadDistPolar`(전동체 3종 공통) + `ContactEllipse`(BB 전용)** 로 갈라진다.
//  전동체별 `Q(φ)` 극좌표는 롤러(CRB·TRB)에도 그대로 쓰이므로, `LoadDistPolar` 는
//  **`BallResult` 를 모르는 중립 입력**(`{ phi_rad, q_n, loaded }`)만 받도록 지금부터 분리해 둔다.
//  `AlphaCurve`·`PhaseSweepChart`·`ContactEllipse` 는 BB 전용 쪽이다.

import { useAppState } from '../store';
import Plot from '../components/charts/PlotWithCopy';
import { darkLayout, plotConfig } from '../components/charts/plotlyDefaults';
import { DetailTable } from '../components/shared/DetailTable';
import type { BallResult } from './generated/BallResult';
import type { BbPhaseSweepResult } from './generated/BbPhaseSweepResult';

/** 유효숫자 9자리 (또는 지수표기) — 사유는 파일 헤더의 단위 정책 참조. */
function num(v: number): string {
  if (!Number.isFinite(v)) return String(v);
  if (v === 0) return '0';
  const a = Math.abs(v);
  if (a < 1e-4 || a >= 1e9) return v.toExponential(8);
  return v.toPrecision(9);
}

const toDeg = (rad: number) => (rad * 180) / Math.PI;

const C_LOADED = '#f59e0b';
const C_UNLOADED = '#64748b';
const C_LOAD_DIR = '#ef4444';
const C_ALPHA = '#38bdf8';
const C_REF = '#a3e635';
const C_INNER = '#3b82f6';
const C_OUTER = '#f97316';

/**
 * 전동체별 하중 극좌표 — **전동체 3종 공통 컴포넌트**(§3.6.1.4 대비).
 *
 * 입력은 `BallResult` 가 아니라 중립 형태다. 롤러(CRB·TRB)도 `(각위치, 하중, 접촉여부)`
 * 를 가지므로 통합 시 이 컴포넌트가 그대로 재사용된다.
 *
 * `loadPhiDeg` = 외부 반경하중의 방위 `atan2(F_z, F_y)` [°]. 반경하중이 0 이면 `null`.
 */
function LoadDistPolar({
  points,
  loadPhiDeg,
  loadMagN,
}: {
  points: { phi_rad: number; q_n: number; loaded: boolean }[];
  loadPhiDeg: number | null;
  loadMagN: number;
}) {
  const phiDeg = points.map(p => toDeg(p.phi_rad));
  const q = points.map(p => p.q_n);
  const qMax = q.length > 0 ? Math.max(...q) : 0;
  // 볼 위치 링의 반경. 하중이 전부 0 이어도 링은 보여야 한다 (그 자체가 징후다).
  const rRef = qMax > 0 ? qMax * 1.12 : 1;
  const idx = points.map((_, i) => i + 1);

  const data: Plotly.Data[] = [];

  // ① Q_j 막대 — 하중구간의 방위와 폭이 여기서 보인다 (C-7).
  data.push({
    type: 'barpolar',
    r: q,
    theta: phiDeg,
    customdata: idx,
    width: points.length > 0 ? (360 / points.length) * 0.55 : 10,
    marker: { color: C_LOADED, line: { color: '#1e293b', width: 1 } },
    name: 'Q_j',
    hovertemplate: '볼 #%{customdata}<br>φ = %{theta:.9g}°<br>Q = %{r:.9g} N<extra></extra>',
  } as Plotly.Data);

  // ② 볼 위치 링 — **비접촉 볼을 시각적으로 구분**한다. C-7 의 「하중구간」이 이것이다.
  //    막대만 그리면 Q = 0 인 볼은 화면에서 사라져 「몇 번 볼이 빠졌는가」를 못 본다.
  const pushRing = (loaded: boolean) => {
    const sel = points.map((p, i) => ({ p, i })).filter(({ p }) => p.loaded === loaded);
    if (sel.length === 0) return;
    data.push({
      type: 'scatterpolar',
      r: sel.map(() => rRef),
      theta: sel.map(({ i }) => phiDeg[i]),
      customdata: sel.map(({ i }) => [idx[i], q[i]]),
      mode: 'markers',
      marker: {
        size: 10,
        symbol: loaded ? 'circle' : 'circle-open',
        color: loaded ? C_LOADED : C_UNLOADED,
        line: { color: loaded ? '#1e293b' : C_UNLOADED, width: 2 },
      },
      name: loaded ? `접촉 (${sel.length})` : `비접촉 (${sel.length})`,
      hovertemplate:
        `볼 #%{customdata[0]} — ${loaded ? '접촉' : '비접촉'}` +
        '<br>φ = %{theta:.9g}°<br>Q = %{customdata[1]:.9g} N<extra></extra>',
    } as Plotly.Data);
  };
  pushRing(true);
  pushRing(false);

  // ③ 🔴 외부 반경하중 기준선 — 하중구간이 이 방향과 정렬되는지가 이 뷰의 핵심 검증이다.
  if (loadPhiDeg !== null) {
    data.push({
      type: 'scatterpolar',
      r: [0, rRef * 1.15],
      theta: [loadPhiDeg, loadPhiDeg],
      mode: 'lines+markers',
      line: { color: C_LOAD_DIR, width: 3, dash: 'dash' },
      marker: { size: [0, 12], symbol: 'diamond', color: C_LOAD_DIR },
      name: `F_r 방위 ${loadPhiDeg.toFixed(3)}°`,
      hovertemplate: `F_r = ${num(loadMagN)} N<br>φ_F = ${num(loadPhiDeg)}°<extra></extra>`,
    } as Plotly.Data);
  }

  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: { text: 'Q_j(φ) — 볼 하중 극좌표  ·  θ = 0 은 +Y 축', font: { size: 14, color: '#e2e8f0' } },
    polar: {
      bgcolor: 'transparent',
      // ⚠ 각도축은 **φ 그 자체**다. 회전(`rotation`)·방향 반전을 넣지 않는다 —
      //   넣는 순간 「하중구간이 어긋나 보이는 것」이 솔버 탓인지 화면 탓인지 갈리지 않는다.
      angularaxis: {
        direction: 'counterclockwise',
        rotation: 0,
        dtick: 30,
        ticksuffix: '°',
        gridcolor: '#334155',
        linecolor: '#334155',
        tickfont: { size: 11, family: 'JetBrains Mono', color: '#94a3b8' },
      },
      radialaxis: {
        range: [0, rRef * 1.2],
        gridcolor: '#334155',
        linecolor: '#334155',
        tickfont: { size: 11, family: 'JetBrains Mono', color: '#94a3b8' },
        title: { text: 'Q [N]', font: { size: 12, color: '#94a3b8' } },
      },
    },
    margin: { l: 40, r: 40, t: 45, b: 30 },
    showlegend: true,
    legend: { font: { size: 11, color: '#94a3b8' }, bgcolor: 'transparent', x: 0.02, y: 0.98 },
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
 * `α_j(φ)` — **BB 전용**. 접촉각이 볼마다 다르고 하중에 따라 변한다는 사실 자체를 본다
 * (D-1 · C-2). 전부 같은 값이면 틸트가 반영되지 않은 것이다 (§3.6.4.2 의 징후).
 *
 * ⚠ **접촉 볼만** 그린다 — 비접촉 볼의 `α_j` 는 물리적 의미가 없다.
 * ⚠ 점을 선으로 잇지 않는다. 비접촉 볼이 중간에 빠지면 선이 그 구간을 가로질러
 *   **있지도 않은 볼을 있는 것처럼** 보이게 한다.
 */
function AlphaCurve({ balls, alpha0Rad }: { balls: BallResult[]; alpha0Rad: number }) {
  const loaded = balls.map((b, i) => ({ b, i })).filter(({ b }) => b.loaded);
  const alpha0Deg = toDeg(alpha0Rad);

  const data: Plotly.Data[] = [
    {
      type: 'scatter',
      x: [0, 360],
      y: [alpha0Deg, alpha0Deg],
      mode: 'lines',
      line: { color: C_REF, width: 2, dash: 'dot' },
      name: `α₀ = ${alpha0Deg.toFixed(4)}°`,
      hovertemplate: `α₀ = ${num(alpha0Deg)}° (${num(alpha0Rad)} rad)<extra></extra>`,
    } as Plotly.Data,
    {
      type: 'scatter',
      x: loaded.map(({ b }) => toDeg(b.phi_rad)),
      y: loaded.map(({ b }) => toDeg(b.alpha_rad)),
      customdata: loaded.map(({ b, i }) => [i + 1, b.q_n, b.delta_mm]),
      mode: 'markers',
      marker: { size: 10, color: C_ALPHA, line: { color: '#1e293b', width: 1 } },
      name: `α_j (접촉 ${loaded.length})`,
      hovertemplate:
        '볼 #%{customdata[0]}<br>φ = %{x:.9g}°<br>α_j = %{y:.9g}°' +
        '<br>Q = %{customdata[1]:.9g} N<br>δ = %{customdata[2]:.9g} mm<extra></extra>',
    } as Plotly.Data,
  ];

  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: { text: 'α_j(φ) — 운전 접촉각 (접촉 볼만)', font: { size: 14, color: '#e2e8f0' } },
    xaxis: {
      ...darkLayout.xaxis,
      title: { text: 'φ [°]   (φ = 0 은 +Y 축)' },
      range: [0, 360],
      dtick: 30,
    },
    yaxis: { ...darkLayout.yaxis, title: { text: 'α_j [°]' } },
    showlegend: true,
    legend: { font: { size: 11, color: '#94a3b8' }, bgcolor: 'transparent' },
  };

  return (
    <div className="h-[300px]">
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
 * 위상 스윕 — **C-5**. `Q_max` 가 케이지 위상 `φ₀` 에 대해 주기 `2π/Z` 로 진동하는가.
 *
 * ⚠ 솔버는 `φ₀ ∈ [0, 2π/Z)` 를 `n_phase` 분할한다(`bearing.rs`). 즉 **가로축 전체가 정확히 한 주기**다.
 *   그래서 주기 눈금을 `2π/Z` 위치에 세워 「한 주기가 화면 폭과 같은가」를 바로 보게 한다.
 * ⚠ 끝점 `φ₀ = 2π/Z` 는 솔버가 계산하지 않은 점이므로 **곡선을 임의로 닫지 않는다**
 *   (없는 값을 그려 넣으면 그것은 검증이 아니라 연출이다).
 */
function PhaseSweepChart({
  sweep,
  z,
  baseQMaxN,
}: {
  sweep: BbPhaseSweepResult;
  z: number;
  baseQMaxN: number;
}) {
  const periodDeg = z > 0 ? 360 / z : 360;
  const x = sweep.curve.map(([p]) => toDeg(p));
  const y = sweep.curve.map(([, qq]) => qq);

  const data: Plotly.Data[] = [
    {
      type: 'scatter',
      x,
      y,
      mode: 'lines+markers',
      line: { color: C_LOADED, width: 2 },
      marker: { size: 5, color: C_LOADED },
      name: 'Q_max(φ₀)',
      hovertemplate: 'φ₀ = %{x:.9g}°<br>Q_max = %{y:.9g} N<extra></extra>',
    } as Plotly.Data,
  ];

  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: {
      text: `위상 스윕 Q_max(φ₀) — 주기 2π/Z = ${periodDeg.toFixed(6)}°`,
      font: { size: 14, color: '#e2e8f0' },
    },
    xaxis: {
      ...darkLayout.xaxis,
      title: { text: 'φ₀ [°]   (케이지 위상)' },
      range: [0, periodDeg * 1.05],
    },
    yaxis: { ...darkLayout.yaxis, title: { text: 'Q_max [N]' } },
    shapes: [
      {
        type: 'line',
        x0: periodDeg,
        x1: periodDeg,
        y0: 0,
        y1: 1,
        yref: 'paper',
        line: { color: C_REF, width: 2, dash: 'dash' },
      },
    ],
    annotations: [
      {
        x: periodDeg,
        y: 1,
        yref: 'paper',
        text: '2π/Z',
        showarrow: false,
        yanchor: 'bottom',
        font: { size: 11, color: C_REF },
      },
    ],
    showlegend: false,
  };

  return (
    <div className="space-y-2">
      <div className="h-[280px]">
        <Plot
          data={data}
          layout={layout}
          config={plotConfig}
          style={{ width: '100%', height: '100%' }}
          useResizeHandler
        />
      </div>
      <DetailTable
        title="Phase Sweep (BbPhaseSweepResult · 전 5필드)"
        rows={[
          ['주기 2π/Z', num(periodDeg), '°'],
          ['주기 2π/Z', num((2 * Math.PI) / (z > 0 ? z : 1)), 'rad'],
          ['스윕 점수 (curve 길이)', String(sweep.curve.length), ''],
          ['worst Q_max', num(sweep.worst_q_max_n), 'N'],
          ['worst Q_max 발생 φ₀', num(toDeg(sweep.worst_q_max_phase_rad)), '°'],
          ['worst Q_max 발생 φ₀', num(sweep.worst_q_max_phase_rad), 'rad'],
          ['worst p_max', num(sweep.worst_p_max_mpa), 'MPa'],
          ['worst p_max 발생 φ₀', num(toDeg(sweep.worst_p_max_phase_rad)), '°'],
          ['worst p_max 발생 φ₀', num(sweep.worst_p_max_phase_rad), 'rad'],
          // φ₀ = 0 은 기본해(`equilibrium`)와 **같은 배치**다. 두 값을 나란히 내면
          // 스윕 경로와 기본 경로가 갈라졌는지 화면에서 바로 보인다.
          ['curve[0] Q_max (φ₀ = 0)', sweep.curve.length > 0 ? num(sweep.curve[0][1]) : '—', 'N'],
          ['equilibrium.q_max_n (φ₀ = 0)', num(baseQMaxN), 'N'],
        ]}
      />
    </div>
  );
}

/**
 * 접촉타원 **형상** — **BB 전용**(§3.6.1.4 통합 시 분리 대상).
 *
 * ⚠ 여기는 `a`·`b`·`a/b` 의 **형상**까지다. 타원 **내부**의 `p(x, y)` 는 `BbStressContourView`
 *   소관이다 (§3.6.4.4). 여기서 히트맵을 그리면 두 뷰가 같은 것을 두 번 그리게 된다.
 */
function ContactEllipse({ balls }: { balls: BallResult[] }) {
  const idx = balls.map((_, i) => i + 1);
  const bar = (name: string, vals: number[], color: string): Plotly.Data =>
    ({
      type: 'bar',
      x: idx,
      y: vals,
      name,
      marker: { color },
      hovertemplate: `볼 #%{x} — ${name} = %{y:.9g} mm<extra></extra>`,
    }) as Plotly.Data;

  const barLayout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: { text: '접촉타원 반경 a · b (볼별 비교)', font: { size: 14, color: '#e2e8f0' } },
    xaxis: { ...darkLayout.xaxis, title: { text: '볼 번호 j' }, dtick: 1 },
    yaxis: { ...darkLayout.yaxis, title: { text: '[mm]' } },
    barmode: 'group',
    showlegend: true,
    legend: { font: { size: 11, color: '#94a3b8' }, bgcolor: 'transparent' },
  };

  // 실제 축척 윤곽 — 최대하중 볼. 「내·외륜 타원이 뒤바뀜」(§3.6.4.2 징후)이 여기서 보인다.
  let top = -1;
  for (let i = 0; i < balls.length; i++) {
    if (top < 0 || balls[i].q_n > balls[top].q_n) top = i;
  }
  const b0 = top >= 0 ? balls[top] : null;

  const ring = (a: number, b: number, name: string, color: string): Plotly.Data => {
    const t = Array.from({ length: 181 }, (_, k) => (k * 2 * Math.PI) / 180);
    return {
      type: 'scatter',
      x: t.map(v => a * Math.cos(v)),
      y: t.map(v => b * Math.sin(v)),
      mode: 'lines',
      fill: 'toself',
      fillcolor: 'rgba(255,255,255,0.04)',
      line: { color, width: 2 },
      name,
      hovertemplate: `${name}<br>x = %{x:.9g} mm<br>y = %{y:.9g} mm<extra></extra>`,
    } as Plotly.Data;
  };

  const shapeLayout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: {
      text: b0 ? `최대하중 볼 #${top + 1} 의 접촉타원 윤곽 (실제 축척, 내·외륜 겹쳐그리기)` : '접촉타원 윤곽',
      font: { size: 14, color: '#e2e8f0' },
    },
    xaxis: { ...darkLayout.xaxis, title: { text: 'a 방향 [mm]' } },
    yaxis: { ...darkLayout.yaxis, title: { text: 'b 방향 [mm]' }, scaleanchor: 'x', scaleratio: 1 },
    showlegend: true,
    legend: { font: { size: 11, color: '#94a3b8' }, bgcolor: 'transparent' },
  };

  return (
    <div className="space-y-3">
      <div className="h-[300px]">
        <Plot
          data={[
            bar('a inner', balls.map(b => b.a_inner_mm), C_INNER),
            bar('b inner', balls.map(b => b.b_inner_mm), '#93c5fd'),
            bar('a outer', balls.map(b => b.a_outer_mm), C_OUTER),
            bar('b outer', balls.map(b => b.b_outer_mm), '#fdba74'),
          ]}
          layout={barLayout}
          config={plotConfig}
          style={{ width: '100%', height: '100%' }}
          useResizeHandler
        />
      </div>

      {b0 && b0.b_inner_mm > 0 && b0.b_outer_mm > 0 && (
        <div className="h-[320px]">
          <Plot
            data={[
              ring(
                b0.a_inner_mm,
                b0.b_inner_mm,
                `inner (a/b = ${(b0.a_inner_mm / b0.b_inner_mm).toFixed(6)})`,
                C_INNER
              ),
              ring(
                b0.a_outer_mm,
                b0.b_outer_mm,
                `outer (a/b = ${(b0.a_outer_mm / b0.b_outer_mm).toFixed(6)})`,
                C_OUTER
              ),
            ]}
            layout={shapeLayout}
            config={plotConfig}
            style={{ width: '100%', height: '100%' }}
            useResizeHandler
          />
        </div>
      )}
    </div>
  );
}

/** 볼별 전 항목 표 — `BallResult` 11필드를 **빠짐없이** 낸다 (유효숫자 9자리). */
function BallTable({ balls }: { balls: BallResult[] }) {
  const head = [
    'j',
    'φ [°]',
    'loaded',
    'δ_j [mm]',
    'Q_j [N]',
    'α_j [°]',
    'α_j [rad]',
    'a_i [mm]',
    'b_i [mm]',
    'a/b_i',
    'p_max,i [MPa]',
    'a_e [mm]',
    'b_e [mm]',
    'a/b_e',
    'p_max,e [MPa]',
  ];
  return (
    <div>
      <h4 className="text-sm font-semibold text-text-light mb-2 uppercase tracking-wider">
        Ball Results (BallResult · 전 11필드)
      </h4>
      <div className="overflow-x-auto custom-scrollbar">
        <table className="text-[12px] whitespace-nowrap">
          <thead>
            <tr className="border-b border-white/10 text-text-canvas">
              {head.map(h => (
                <th key={h} className="py-1 px-2 text-right font-medium">
                  {h}
                </th>
              ))}
            </tr>
          </thead>
          <tbody className="font-mono tabular-nums text-text-light">
            {balls.map((b, i) => (
              <tr key={i} className={`border-b border-white/[0.03] ${b.loaded ? '' : 'opacity-45'}`}>
                <td className="py-0.5 px-2 text-right">{i + 1}</td>
                <td className="py-0.5 px-2 text-right">{num(toDeg(b.phi_rad))}</td>
                <td className={`py-0.5 px-2 text-right ${b.loaded ? 'text-amber-300' : 'text-slate-400'}`}>
                  {b.loaded ? '접촉' : '비접촉'}
                </td>
                <td className="py-0.5 px-2 text-right">{num(b.delta_mm)}</td>
                <td className="py-0.5 px-2 text-right">{num(b.q_n)}</td>
                <td className="py-0.5 px-2 text-right">{b.loaded ? num(toDeg(b.alpha_rad)) : '—'}</td>
                <td className="py-0.5 px-2 text-right">{b.loaded ? num(b.alpha_rad) : '—'}</td>
                <td className="py-0.5 px-2 text-right">{num(b.a_inner_mm)}</td>
                <td className="py-0.5 px-2 text-right">{num(b.b_inner_mm)}</td>
                <td className="py-0.5 px-2 text-right">
                  {b.b_inner_mm > 0 ? num(b.a_inner_mm / b.b_inner_mm) : '—'}
                </td>
                <td className="py-0.5 px-2 text-right">{num(b.p_max_inner_mpa)}</td>
                <td className="py-0.5 px-2 text-right">{num(b.a_outer_mm)}</td>
                <td className="py-0.5 px-2 text-right">{num(b.b_outer_mm)}</td>
                <td className="py-0.5 px-2 text-right">
                  {b.b_outer_mm > 0 ? num(b.a_outer_mm / b.b_outer_mm) : '—'}
                </td>
                <td className="py-0.5 px-2 text-right">{num(b.p_max_outer_mpa)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      <p className="text-[11px] text-text-canvas/60 mt-1">
        비접촉 볼은 흐리게 표시하고 α_j 를 「—」로 둔다 — 하중을 받지 않는 볼의 접촉각은 물리적 의미가 없다.
      </p>
    </div>
  );
}

export default function BbLoadDistView() {
  const { state } = useAppState();
  const { result, bbInput } = state;

  if (!result) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-text-canvas text-sm">Solve 를 눌러 결과를 만드세요</p>
      </div>
    );
  }

  const eq = result.equilibrium;
  const balls = eq.ball_results;

  // 🔴 외부 반경하중의 방위. `φ` 와 **같은 축**에서 잰다 —
  //    `F_y` 가 `cos φ`, `F_z` 가 `sin φ` 에 곱해지므로 `atan2(F_z, F_y)` 가 그대로 `φ` 다
  //    (Theory §4.4 확정형 · `bearing.rs` 헤더).
  //  ⚠ 하중은 **현재 입력 패널 값**에서 읽는다. 입력을 고친 뒤 Solve 를 누르지 않으면
  //    기준선만 먼저 움직인다 — 그 사실을 화면에 적어 둔다.
  const fy = bbInput?.operating.f_y_n ?? 0;
  const fz = bbInput?.operating.f_z_n ?? 0;
  const loadMagN = Math.hypot(fy, fz);
  const loadPhiDeg = loadMagN > 0 ? ((toDeg(Math.atan2(fz, fy)) % 360) + 360) % 360 : null;

  return (
    <div className="h-full overflow-auto custom-scrollbar p-4 space-y-5">
      {/* ── 🔴 방위 규약 명시 — 이 배너가 없으면 극좌표를 잘못 읽는다 ───────── */}
      <div className="p-2.5 rounded border bg-blue-500/10 border-blue-400/30 text-blue-100 text-[12px] leading-relaxed">
        <p className="font-semibold text-[13px] mb-1">
          방위 규약 (D-8 · Theory §4.4 · solver/bb/bearing.rs)
        </p>
        <p>
          극좌표의 각도축은 <span className="font-mono">φ</span> 그 자체다 —{' '}
          <span className="font-mono font-semibold">θ = 0 은 +Y 축</span>,{' '}
          <span className="font-mono">θ = 90° 는 +Z 축</span>, 반시계 방향으로 증가한다. 볼 각위치는{' '}
          <span className="font-mono">φ_j = 2π(j−1)/Z</span> 이고 반경 단위벡터가{' '}
          <span className="font-mono">(cos φ_j, sin φ_j)</span> in{' '}
          <span className="font-mono">(Y, Z)</span> 다.
        </p>
        <p className="mt-1">
          붉은 파선은 외부 반경하중의 방위 <span className="font-mono">φ_F = atan2(F_z, F_y)</span> ={' '}
          <span className="font-mono">{loadPhiDeg === null ? '— (F_r = 0)' : `${num(loadPhiDeg)}°`}</span>
          {', '}
          <span className="font-mono">
            |F_r| = {num(loadMagN)} N
          </span>{' '}
          (F_y = {num(fy)} N, F_z = {num(fz)} N).{' '}
          <span className="text-blue-200/80">
            하중구간이 이 선을 중심으로 정렬·대칭인지가 C-7 · D-2b/2c 의 육안 판정이다.
          </span>
        </p>
        <p className="mt-1 text-blue-200/70">
          ⚠ 기준선의 하중값은 <b>현재 입력 패널</b>에서 읽는다. 입력을 고쳤다면 Solve 를 다시 눌러야 볼
          하중과 짝이 맞는다.
        </p>
      </div>

      {/* ① Q_j(φ) 극좌표 — C-4 · C-7 · D-2b/2c */}
      <LoadDistPolar points={balls} loadPhiDeg={loadPhiDeg} loadMagN={loadMagN} />

      <DetailTable
        title="Load Zone (C-7) · Equilibrium"
        rows={[
          ['접촉 볼 수 / Z', `${eq.loaded_count} / ${balls.length}`, ''],
          ['Q_max', num(eq.q_max_n), 'N'],
          ['볼 간격 2π/Z', num(balls.length > 0 ? 360 / balls.length : 0), '°'],
          ['φ_F 외부 반경하중 방위', loadPhiDeg === null ? '—' : num(loadPhiDeg), '°'],
          ['|F_r| = √(F_y² + F_z²)', num(loadMagN), 'N'],
          ['converged / iterations', `${String(eq.converged)} / ${eq.iterations}`, ''],
        ]}
      />

      {/* ② α_j(φ) — D-1 · C-2 */}
      <AlphaCurve balls={balls} alpha0Rad={result.geometry.alpha_0_rad} />

      {/* ③ 위상 스윕 — C-5. `SolverParams.phase_sweep.enabled` 일 때만 채워진다. */}
      {result.phase_sweep ? (
        <PhaseSweepChart sweep={result.phase_sweep} z={balls.length} baseQMaxN={eq.q_max_n} />
      ) : (
        <p className="text-[12px] text-text-canvas/60">
          위상 스윕이 꺼져 있다 (<span className="font-mono">solver.phase_sweep.enabled = false</span>). C-5
          (주기 <span className="font-mono">2π/Z</span>)를 화면에서 보려면 입력 패널에서 켜고 다시 Solve 한다.
        </p>
      )}

      {/* ④ 접촉타원 형상 — 내부 압력분포는 S5 소관 (§3.6.4.4) */}
      <ContactEllipse balls={balls} />

      <BallTable balls={balls} />
    </div>
  );
}
