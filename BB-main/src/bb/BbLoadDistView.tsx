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
//
//  ⚠ 접촉타원(반경 a·b · 윤곽 · 압력분포)은 이 뷰에서 뺐다(2026-10-14 사용자 지시) — `BbStressContourView` 가 맡는다.
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
//  → 그래서 극좌표의 각도축은 **`φ [°]` 그 자체**이고(값은 손대지 않는다),
//    「θ = 0 은 +Y 축」을 **화면에 명시**하며,
//    외부 반경하중의 방위 `atan2(F_z, F_y)` 를 **같은 축 위에 기준선으로 겹쳐 그린다.**
//    하중구간이 그 선과 정렬되는지가 한눈에 보이는 것 — 그것이 이 뷰의 핵심 임무다.
//
// ─────────────────────────────────────────────────────────────────────
//  🔴 각도 **표시** 규약 — §3.6.4.10 (확정 2026-08-24)
// ─────────────────────────────────────────────────────────────────────
//  **0° 를 화면 아래에 두고 각도는 우측으로 증가**시킨다. 자의적 선택이 아니라 **규격 준수**다.
//
//    · Theory §4 — ISO 16281 **Annex A.2.2 NOTE**:
//      「우수좌표계 기준이며 X 축이 베어링 공칭 회전축과 일치한다.
//        **문서 내 모든 그림에서 Y 축은 지면 아래쪽을 향한다.**」
//    · `φ = 0` 이 `+Y`(D-8) → **0° 는 화면 아래**.
//    · 우수좌표계에서 `X(회전축) = 화면 밖` · `Y = 아래` → `Z = X × Y = 오른쪽`
//      → `φ = 90°`(= `+Z`)가 **오른쪽** → **각도가 우측으로 증가**.
//    두 요구가 하나의 규격 근거로 동시에 설명된다.
//
//  ⚠️ **데이터는 바뀌지 않는다.** `φ_j` 도 `atan2(F_z, F_y)` 도 그대로다 — **표시 방향만**이다.
//    따라서 §3.6.4.2 의 검증 임무(하중구간 방위 정합)는 영향을 받지 않는다.
//    오히려 ISO 그림과 방향이 같아져 대조가 쉬워진다.
//
// ─────────────────────────────────────────────────────────────────────
//  단위·자릿수 (Plan Phase 4-5 §4-5.3)
// ─────────────────────────────────────────────────────────────────────
//  표시는 `shared.ts` 의 단위별 함수만 쓴다 — 하중 N 소수 1 · 각도 ° 소수 2(rad 병기 없음) ·
//  반축 a·b mm 소수 4 · 변위 δ μm 소수 2 · 응력 MPa 정수 · 무차원 소수 4.
//  Plotly hover 도 같은 자릿수다. 정밀 대조는 Rust 테스트와 헬스체크 몫이다 (§4-5.1 결정 4).
//
// ─────────────────────────────────────────────────────────────────────
//  ⚠ §3.6.1.4 통합 대비 — 컴포넌트를 미리 갈라 둔다
// ─────────────────────────────────────────────────────────────────────
//  나중 통합 시 이 뷰는 **`LoadDistPolar`(전동체 3종 공통)** 와 BB 전용 부분으로 갈라진다.
//  전동체별 `Q(φ)` 극좌표는 롤러(CRB·TRB)에도 그대로 쓰이므로, `LoadDistPolar` 는
//  **`BallResult` 를 모르는 중립 입력**(`{ phi_rad, q_n, loaded }`)만 받도록 지금부터 분리해 둔다.
//  `AlphaCurve`·`PhaseSweepChart` 는 BB 전용 쪽이다.

import { useAppState } from '../store';
import Plot from '../components/charts/PlotWithCopy';
import { darkLayout, plotConfig } from '../components/charts/plotlyDefaults';
import { DetailTable } from './ui';
import type { BallResult } from './generated/BallResult';
import type { BbPhaseSweepResult } from './generated/BbPhaseSweepResult';
import { toDeg, fmtN, fmtDeg, fmtUm, fmtMPa, fmtMmFine, fmtDimless, C_LOADED, C_UNLOADED, C_LOAD_DIR, C_ALPHA, C_REF, loadColorCss, LOAD_COLORSCALE } from './shared';


/**
 * 전동체별 하중 극좌표 — **전동체 3종 공통 컴포넌트**(§3.6.1.4 대비).
 *
 * 입력은 `BallResult` 가 아니라 중립 형태다. 롤러(CRB·TRB)도 `(각위치, 하중, 접촉여부)`
 * 를 가지므로 통합 시 이 컴포넌트가 그대로 재사용된다.
 *
 * `loadPhiDeg` = 외부 반경하중의 방위 `atan2(F_z, F_y)` [°]. 반경하중이 0 이면 `null`.
 *
 * ⚠ 화면의 「방위 규약」 배너와 차트 캡션은 2026-08-24 사용자 지시로 제거했다.
 *   규약 자체는 그대로다 — `polar.angularaxis` 의 `rotation:-90` ·
 *   `direction:'counterclockwise'` 가 방향을 만들고, 근거는 위 주석에 남아 있다.
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

  // ① Q_j envelope — 하중구간의 방위와 폭이 여기서 보인다 (C-7). §3.6.4.11
  //
  //  `barpolar`(부채꼴)를 envelope 으로 바꿨다. TRB `LoadDistChart` 와 같은 방식이다:
  //    `scatterpolar` + `mode:'lines'` + `fill:'toself'` — **첫 점을 끝에 다시 붙여 닫는다.**
  //
  //  ⚠ **`line_shape:'spline'` 을 쓰지 말 것.** TRB 그림이 매끄러운 이유는 그리기 방식이 아니라
  //    **데이터 밀도**(`angular_distribution` 의 촘촘한 각도 격자)다. BB 에는 `ball_results[]`
  //    **16점**뿐이므로 스플라인은 하중구간 경계에서 **overshoot** 해 **없는 하중을 만들고 음수로
  //    내려간다** — C-7 육안 판정이 망가진다. 그래서 **16점 직선 연결**이다.
  //  ⚠ **`phase_sweep.curve` 를 여기에 쓰지 말 것.** 그것은 「위상 회전 시 **최대** 볼하중」이지
  //    「**방위별** 하중분포」가 아니다 — **다른 물리량이다.**
  //  ※ 매끄러운 곡선이 필요하면 Rust 가 방위별 분포를 반환해야 한다 (§3.6.4.11 후속 별건).
  //
  //  φ 오름차순으로 정렬해 잇는다 — 입력 순서가 각도 순서라는 보장에 기대지 않는다.
  const env = phiDeg
    .map((phi, i) => ({ phi, q: q[i], j: idx[i] }))
    .sort((a, b) => a.phi - b.phi);
  if (env.length > 0) {
    const closed = [...env, env[0]]; // ← 첫 점을 끝에 다시 붙여 다각형을 닫는다
    data.push({
      type: 'scatterpolar',
      r: closed.map(e => e.q),
      theta: closed.map(e => e.phi),
      customdata: closed.map(e => e.j),
      mode: 'lines',
      fill: 'toself',
      fillcolor: 'rgba(245,158,11,0.15)',
      line: { color: C_LOADED, width: 2 },
      name: 'Q_j envelope',
      hovertemplate: '볼 #%{customdata}<br>φ = %{theta:.2f}°<br>Q = %{r:.1f} N<extra></extra>',
    } as Plotly.Data);

    // envelope 의 **꼭짓점**(= 실제 계산된 볼 하중). 이것을 찍어 두어야
    // 「곡선은 16점을 직선으로 이은 것」이라는 사실이 화면에서 드러난다 —
    // 매끄러워 보이는 선을 연속 분포로 오독하는 것을 막는다.
    data.push({
      type: 'scatterpolar',
      r: env.map(e => e.q),
      theta: env.map(e => e.phi),
      customdata: env.map(e => e.j),
      mode: 'markers',
      marker: { size: 6, color: C_LOADED, line: { color: '#1e293b', width: 1 } },
      name: `Q_j (볼 ${env.length}점)`,
      hovertemplate: '볼 #%{customdata}<br>φ = %{theta:.2f}°<br>Q = %{r:.1f} N<extra></extra>',
    } as Plotly.Data);
  }

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
        '<br>φ = %{theta:.2f}°<br>Q = %{customdata[1]:.1f} N<extra></extra>',
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
      name: `F_r 방위 ${fmtDeg(loadPhiDeg)}°`,
      hovertemplate: `F_r = ${fmtN(loadMagN)} N<br>φ_F = ${fmtDeg(loadPhiDeg)}°<extra></extra>`,
    } as Plotly.Data);
  }

  // 반경축(눈금 숫자 500·1000 …)은 90° 선(오른쪽 수평)에 고정한다 — 가독성이 가장 좋다(2026-10-14 사용자 지시).
  //   `radialaxis.angle` 은 각도축 좌표(φ 그대로)다.
  // 반경 눈금: 격자 원은 범위 끝까지 두되 숫자는 Q_max 이하만 단다 — 볼 링(1.12·Q_max)의 점과 안 겹치게.
  const rawStep = (rRef * 1.2) / 5;
  const mag = 10 ** Math.floor(Math.log10(rawStep));
  const step = ([1, 2, 2.5, 5, 10].find(m => m * mag >= rawStep) ?? 10) * mag;
  const tickvals: number[] = [];
  for (let v = 0; v <= rRef * 1.2 + 1e-9 * step; v += step) tickvals.push(Number(v.toPrecision(12)));
  const ticktext = tickvals.map(v => (v <= qMax ? String(v) : ''));

  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    // 제목은 **`Load Distribution`** 으로 단순화한다 (§3.6.4.11). 축 규약은 위 주석과
    // `angularaxis` 설정이 지킨다 (화면 배너·부제는 2026-08-24 사용자 지시로 제거).
    // 제목은 그래프 영역(paper) 기준 중앙 = 극좌표 중심(180° 눈금) 위 — 범례가 왼쪽 여백을 늘려도 맞는다.
    title: { text: 'Load Distribution', font: { size: 14, color: '#e2e8f0' }, xref: 'paper', x: 0.5 },
    polar: {
      bgcolor: 'transparent',
      // 🔴 각도 **표시** 규약 — §3.6.4.10 (구 주석의 「회전을 넣지 않는다」는 폐기됐다).
      //   각도축의 **값**은 여전히 `φ` 그 자체다(데이터 무변경). 바꾸는 것은 **화면 배치**뿐이며,
      //   그 배치는 ISO 16281 A.2.2 NOTE(「모든 그림에서 Y 축은 지면 아래」)가 정한다.
      //   `φ=0`=+Y → 아래 · 우수좌표계 `Z = X × Y` → `φ=90°`=+Z → 오른쪽.
      //
      //   plotly.js 의 각도→화면 변환을 소스로 확인했다 (v3.4.0):
      //     `plots/polar/set_convert.js` setConvertAngular:
      //        dir = {clockwise:-1, counterclockwise:+1}[direction] ;  g = dir·θ + rotation
      //     `traces/scatterpolar/plot.js:41-42`:  x = r·cos(g) ,  y = r·sin(g)   (y 는 위가 양)
      //   → rotation = -90°, dir = +1 이면
      //        θ=0   → g = -90° → (x,y) = (0, -r)  → **아래**
      //        θ=90° → g =   0° → (x,y) = (+r, 0)  → **오른쪽**
      //   즉 「0°는 아래, 90°는 오른쪽」이 성립한다. (계산이 아니라 소스의 변환식으로 확인)
      angularaxis: {
        direction: 'counterclockwise',
        rotation: -90,
        dtick: 45,
        ticksuffix: '°',
        gridcolor: '#334155',
        linecolor: '#334155',
        tickfont: { size: 11, family: 'JetBrains Mono', color: '#94a3b8' },
      },
      radialaxis: {
        range: [0, rRef * 1.2],
        angle: 90,
        tickvals,
        ticktext,
        gridcolor: '#334155',
        linecolor: '#334155',
        tickfont: { size: 11, family: 'JetBrains Mono', color: '#94a3b8' },
        title: { text: 'Q [N]', font: { size: 12, color: '#94a3b8' } },
      },
    },
    margin: { l: 40, r: 40, t: 70, b: 30 },
    showlegend: true,
    // 범례는 그림 틀(container)의 좌상단 모서리에 붙인다 — 극좌표 원·각도 눈금과 겹치지 않게.
    legend: {
      font: { size: 11, color: '#94a3b8' }, bgcolor: 'transparent',
      xref: 'container', yref: 'container', x: 0, y: 1, xanchor: 'left', yanchor: 'top',
    },
  };

  return (
    <div className="space-y-1">
      <div className="h-[420px]">
        <Plot
          data={data}
          layout={layout}
          config={plotConfig}
          style={{ width: '100%', height: '100%' }}
          useResizeHandler
        />
      </div>
    </div>
  );
}

/**
 * `α_j(φ)` — **BB 전용**. 접촉각이 볼마다 다르고 하중에 따라 변한다는 사실 자체를 본다
 * (D-1 · C-2). 전부 같은 값이면 틸트가 반영되지 않은 것이다 (§3.6.4.2 의 징후).
 *
 * ⚠ **접촉 볼만** 그린다 — 비접촉 볼의 `α_j` 는 물리적 의미가 없다.
 * 점은 **스플라인으로 잇는다** (2026-10-14 사용자 지시). 단:
 *   - 비접촉 볼 자리는 `null` 로 **선을 끊는다** — 선이 그 구간을 가로질러 있지도 않은 볼을 있는 것처럼
 *     보이게 하지 않기 위해서다(이전에 선을 쓰지 않은 이유).
 *   - φ 는 한 바퀴 주기이므로 첫 볼 값을 φ + 360° 에 한 번 더 둬 0°↔360° 가 이어지게 한다
 *     (같은 볼의 값이지 새로 지어낸 값이 아니다).
 *   - 스플라인은 점 사이를 부드럽게 보이게 할 뿐이다. 계산된 값은 점(마커)이다.
 */
/**
 * 볼별 작용 하중 막대 (Plan §4-5.9) — TRB 의 「극좌표 + 롤러별 하중 막대」 배치를 옮겼다.
 * 색은 3D 뷰와 **같은 함수**(`shared.loadColorCss`)로 칠하고, 같은 범위(0 … Q_max)의 컬러바를 단다.
 * 비접촉 볼은 높이 0 이다(Q_j = 0).
 */
function BallLoadBars({ balls }: { balls: BallResult[] }) {
  const qMax = Math.max(0, ...balls.map(b => b.q_n));
  const data: Plotly.Data[] = [
    {
      type: 'bar',
      x: balls.map((_, i) => i + 1),
      y: balls.map(b => b.q_n),
      marker: { color: balls.map(b => loadColorCss(b.q_n, qMax)) },
      customdata: balls.map(b => [fmtN(b.q_n), fmtDeg(toDeg(b.phi_rad))]),
      hovertemplate: '볼 #%{x}<br>φ = %{customdata[1]}°<br>Q = %{customdata[0]} N<extra></extra>',
      showlegend: false,
    } as Plotly.Data,
    // 컬러바 전용(보이지 않는 점) — 막대 색과 같은 hue 표본을 같은 범위로.
    {
      type: 'scatter',
      x: [1],
      y: [0],
      mode: 'markers',
      marker: {
        size: 0,
        opacity: 0,
        color: [0, qMax],
        cmin: 0,
        cmax: qMax,
        colorscale: LOAD_COLORSCALE,
        showscale: true,
        colorbar: { title: { text: 'N' }, thickness: 12, len: 0.8, tickfont: { size: 11, color: '#94a3b8' } },
      },
      hoverinfo: 'skip',
      showlegend: false,
    } as Plotly.Data,
  ];
  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: { text: '볼별 하중 Q_j', font: { size: 14, color: '#e2e8f0' } },
    margin: { ...darkLayout.margin, t: 70 }, // 극좌표와 같은 상단 여백 — 제목과 그래프 간격
    xaxis: { ...darkLayout.xaxis, title: { text: '볼 번호' }, dtick: 1 },
    yaxis: { ...darkLayout.yaxis, title: { text: 'Q [N]' }, rangemode: 'tozero' },
    bargap: 0.15,
  };
  return (
    <div className="h-[420px]">
      <Plot data={data} layout={layout} config={plotConfig} style={{ width: '100%', height: '100%' }} useResizeHandler />
    </div>
  );
}

function AlphaCurve({ balls, alpha0Rad }: { balls: BallResult[]; alpha0Rad: number }) {
  const loaded = balls.map((b, i) => ({ b, i })).filter(({ b }) => b.loaded);
  const alpha0Deg = toDeg(alpha0Rad);
  // 선: 전 볼을 φ 순으로, 비접촉은 null(끊김). 첫 볼을 +360° 에 반복해 한 바퀴를 닫는다.
  const ring = balls
    .map(b => ({ phi: ((toDeg(b.phi_rad) % 360) + 360) % 360, a: b.loaded ? toDeg(b.alpha_rad) : null }))
    .sort((p, q) => p.phi - q.phi);
  if (ring.length > 0) ring.push({ phi: ring[0].phi + 360, a: ring[0].a });

  const data: Plotly.Data[] = [
    {
      type: 'scatter',
      x: [0, 360],
      y: [alpha0Deg, alpha0Deg],
      mode: 'lines',
      line: { color: C_REF, width: 2, dash: 'dot' },
      name: `α₀ = ${fmtDeg(alpha0Deg)}°`,
      hovertemplate: `α₀ = ${fmtDeg(alpha0Deg)}°<extra></extra>`,
    } as Plotly.Data,
    {
      type: 'scatter',
      x: ring.map(r => r.phi),
      y: ring.map(r => r.a),
      mode: 'lines',
      line: { color: C_ALPHA, width: 2, shape: 'spline' },
      connectgaps: false,
      showlegend: false,
      hoverinfo: 'skip',
    } as Plotly.Data,
    {
      type: 'scatter',
      x: loaded.map(({ b }) => toDeg(b.phi_rad)),
      y: loaded.map(({ b }) => toDeg(b.alpha_rad)),
      // δ 는 화면 단위(μm) 문자열로 넘긴다 — 환산을 shared.ts 한 곳에 둔다.
      customdata: loaded.map(({ b, i }) => [i + 1, b.q_n, fmtUm(b.delta_mm)]),
      mode: 'markers',
      marker: { size: 10, color: C_ALPHA, line: { color: '#1e293b', width: 1 } },
      name: `α_j (접촉 ${loaded.length})`,
      hovertemplate:
        '볼 #%{customdata[0]}<br>φ = %{x:.2f}°<br>α_j = %{y:.2f}°' +
        '<br>Q = %{customdata[1]:.1f} N<br>δ = %{customdata[2]} μm<extra></extra>',
    } as Plotly.Data,
  ];

  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: { text: 'α_j(φ) 운전 접촉각', font: { size: 14, color: '#e2e8f0' } },
    xaxis: {
      ...darkLayout.xaxis,
      title: { text: 'φ [°]' },
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
      hovertemplate: 'φ₀ = %{x:.2f}°<br>Q_max = %{y:.1f} N<extra></extra>',
    } as Plotly.Data,
  ];

  const layout: Partial<Plotly.Layout> = {
    ...darkLayout,
    title: { text: '위상 스윕 Q_max(φ₀)', font: { size: 14, color: '#e2e8f0' } },
    xaxis: {
      ...darkLayout.xaxis,
      title: { text: 'φ₀ [°]' },
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
        title="Phase Sweep"
        rows={[
          ['Sweep period (2π/Z)', fmtDeg(periodDeg), '°'],
          ['Sweep points', String(sweep.curve.length), ''],
          ['Worst max ball load (Q_max)', fmtN(sweep.worst_q_max_n), 'N'],
          ['Phase at worst Q_max (φ₀)', fmtDeg(toDeg(sweep.worst_q_max_phase_rad)), '°'],
          ['Worst max contact pressure (p_max)', fmtMPa(sweep.worst_p_max_mpa), 'MPa'],
          ['Phase at worst p_max (φ₀)', fmtDeg(toDeg(sweep.worst_p_max_phase_rad)), '°'],
          // φ₀ = 0 은 기본해(`equilibrium`)와 **같은 배치**다. 두 값을 나란히 내면
          // 스윕 경로와 기본 경로가 갈라졌는지 화면에서 바로 보인다.
          ['Sweep max ball load at φ₀ = 0 (Q_max)', sweep.curve.length > 0 ? fmtN(sweep.curve[0][1]) : '—', 'N', '스윕 curve[0]'],
          ['Equilibrium max ball load (Q_max)', fmtN(baseQMaxN), 'N', '기본해 φ₀ = 0 — 위 값과 같아야 한다'],
        ]}
      />
    </div>
  );
}

/**
 * 볼별 전 항목 표 — `BallResult` 11필드를 **빠짐없이** 낸다.
 *
 * 비접촉 볼은 흐리게 하고 α_j 를 「—」로 둔다 — 하중을 받지 않는 볼의 접촉각은 물리적 의미가 없다.
 */
function BallTable({ balls }: { balls: BallResult[] }) {
  const head = [
    'j',
    'φ [°]',
    'loaded',
    'δ_j [μm]',
    'Q_j [N]',
    'α_j [°]',
    'a_i [mm]',
    'b_i [mm]',
    'Inner ellipse ratio (a/b_i)',
    'p_max,i [MPa]',
    'a_e [mm]',
    'b_e [mm]',
    'Outer ellipse ratio (a/b_e)',
    'p_max,e [MPa]',
  ];
  return (
    <div>
      <h4 className="text-sm font-medium text-text-light mb-2">Ball Results</h4>
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
                <td className="py-0.5 px-2 text-right">{fmtDeg(toDeg(b.phi_rad))}</td>
                <td className={`py-0.5 px-2 text-right ${b.loaded ? 'text-amber-300' : 'text-slate-400'}`}>
                  {b.loaded ? '접촉' : '비접촉'}
                </td>
                <td className="py-0.5 px-2 text-right">{fmtUm(b.delta_mm)}</td>
                <td className="py-0.5 px-2 text-right">{fmtN(b.q_n)}</td>
                <td className="py-0.5 px-2 text-right">{b.loaded ? fmtDeg(toDeg(b.alpha_rad)) : '—'}</td>
                <td className="py-0.5 px-2 text-right">{fmtMmFine(b.a_inner_mm)}</td>
                <td className="py-0.5 px-2 text-right">{fmtMmFine(b.b_inner_mm)}</td>
                <td className="py-0.5 px-2 text-right">
                  {b.b_inner_mm > 0 ? fmtDimless(b.a_inner_mm / b.b_inner_mm) : '—'}
                </td>
                <td className="py-0.5 px-2 text-right">{fmtMPa(b.p_max_inner_mpa)}</td>
                <td className="py-0.5 px-2 text-right">{fmtMmFine(b.a_outer_mm)}</td>
                <td className="py-0.5 px-2 text-right">{fmtMmFine(b.b_outer_mm)}</td>
                <td className="py-0.5 px-2 text-right">
                  {b.b_outer_mm > 0 ? fmtDimless(b.a_outer_mm / b.b_outer_mm) : '—'}
                </td>
                <td className="py-0.5 px-2 text-right">{fmtMPa(b.p_max_outer_mpa)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

export default function BbLoadDistView() {
  const { state } = useAppState();
  const { result, resultInput } = state;

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
  // ⚠ `bbInput`(편집 중) 이 아니라 `resultInput`(Solve 시점 스냅샷) 을 읽는다.
  //    그래야 기준선과 볼 하중의 시점이 항상 일치한다 (store.ts `resultInput` 주석).
  const fy = resultInput?.operating.f_y_n ?? 0;
  const fz = resultInput?.operating.f_z_n ?? 0;
  const loadMagN = Math.hypot(fy, fz);
  const loadPhiDeg = loadMagN > 0 ? ((toDeg(Math.atan2(fz, fy)) % 360) + 360) % 360 : null;

  return (
    <div className="h-full overflow-auto custom-scrollbar p-4 space-y-5">
      {/* ① Q_j(φ) 극좌표 — C-4 · C-7 · D-2b/2c — 오른쪽에 볼별 하중 막대 (Plan §4-5.9) */}
      <div className="grid grid-cols-2 gap-4">
        <LoadDistPolar points={balls} loadPhiDeg={loadPhiDeg} loadMagN={loadMagN} />
        <BallLoadBars balls={balls} />
      </div>

      <DetailTable
        title="Load Zone"
        rows={[
          ['Loaded balls (Z_l / Z)', `${eq.loaded_count} / ${balls.length}`, ''],
          ['Max ball load (Q_max)', fmtN(eq.q_max_n), 'N'],
          ['Ball spacing (Δφ)', fmtDeg(balls.length > 0 ? 360 / balls.length : 0), '°', 'Δφ = 2π/Z'],
          ['Radial load direction (φ_F)', loadPhiDeg === null ? '—' : fmtDeg(loadPhiDeg), '°'],
          ['Radial load (|F_r|)', fmtN(loadMagN), 'N', '|F_r| = √(F_y² + F_z²)'],
          ['Converged / iterations', `${String(eq.converged)} / ${eq.iterations}`, ''],
        ]}
      />

      {/* ② α_j(φ) — D-1 · C-2 */}
      <AlphaCurve balls={balls} alpha0Rad={result.geometry.alpha_0_rad} />

      {/* ③ 위상 스윕 — C-5. `SolverParams.phase_sweep.enabled` 일 때만 채워진다. */}
      {result.phase_sweep && (
        <PhaseSweepChart sweep={result.phase_sweep} z={balls.length} baseQMaxN={eq.q_max_n} />
      )}


      <BallTable balls={balls} />
    </div>
  );
}
