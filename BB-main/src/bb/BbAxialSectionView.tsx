// BB 축단면 뷰 (Plan §3.6.4.8 설계 · §4 Phase 4-3 P4-3a · §3.6.3.3 대체 판정)
//
// ─────────────────────────────────────────────────────────────────────
//  이 화면의 검증 임무 — §3.6.4.2
// ─────────────────────────────────────────────────────────────────────
//  §3.6.4.2 의 7항목 중 **유일하게 화면에 없던 것이 D-2d(틸트축 정합)** 이다.
//  이 뷰가 그것을 맡는다. 함께 보는 것은 D-1(하중 후 접촉각 변화)·C-2 다.
//
//  | 구성 | 확인하는 것 | 대응 Level |
//  |---|---|---|
//  | 곡률중심 `O_i`·`O_e` 와 `A = r_i + r_e − D_w` | Theory §2 (A.3) 의 교과서 그림 그 자체 | **A** |
//  | `α₀` 선 ↔ `α_j` 선 겹쳐그리기 | 접촉각이 **벌어지는 방향** | **D-1** · C-2 |
//  | `δ_x` 화살표 + 틸트 기여항 | 축변위와 틸트가 `X_j` 를 어떻게 만드는가 | D-1 |
//  | **틸트 방위 ↔ 모멘트 방위 나침반** | **틸트축이 모멘트와 맞는가** | **D-2d** |
//
//  깨졌을 때의 징후(§3.6.4.2): 「`α_j` 선이 `α₀` 와 겹치면 하중이 안 걸린 것」·
//  「틸트 방향이 모멘트와 반대」.
//
// ─────────────────────────────────────────────────────────────────────
//  🔴 좌표계·부호 규약 — 이것을 틀리면 그림이 조용히 거짓말을 한다
// ─────────────────────────────────────────────────────────────────────
//  · **가로축 = X = 회전축**(D-7, ISO 규약), **세로축 = 반경 r**.
//  · 🔴 **§3.6.4.10 각도 표시 규약 — 0° 는 화면 아래, 각도는 우측으로 증가.**
//    근거는 자의적 선택이 아니라 규격이다. ISO 16281 Annex A.2.2 NOTE:
//      「우수좌표계이며 X 축이 공칭 회전축과 일치한다.
//        **문서 내 모든 그림에서 Y 축은 지면 아래쪽을 향한다.**」
//    `φ = 0` 이 `+Y`(D-8)이므로 **0° = 화면 아래**이고,
//    `X(회전축) = 화면 밖`·`Y = 아래` 에서 `Z = X × Y = 오른쪽` 이라
//    `φ = 90°`(= `+Z`)가 오른쪽 → **각도가 우측으로 증가**한다.
//    이 뷰에서의 귀결은 셋이다:
//      ① **반경 바깥 방향 = 화면 아래**. 그래서 `φ_j = 0`(= `+Y`)인 볼은
//         **아래쪽 반단면**에 놓인다. (SVG 그룹의 `scale(1,-1)` 을 **없앴다** —
//         수학좌표 `(x, r)` 이 그대로 SVG `(x, y)` 라 `r` 이 커지면 화면 아래다.)
//      ② 접촉각 `α` 는 **반경방향(= 화면 아래, 0°)에서 축방향(= 화면 오른쪽, +X)** 으로 잰다.
//         `α₀`·`α_j` 호는 **아래 방향 기준선에서 출발해 오른쪽으로** 벌어진다 —
//         전역 규약과 **같은 증가 방향**이다.
//      ③ 「X 오른쪽 · Y 아래」를 동시에 만족하려면 우수좌표계상 **시선이 `+Z` 방향**
//         (= `−Z` 쪽에서 본다). 이것은 ①② 가 정해지면 **강제되는 결과**이지 선택이 아니다.
//  · 나침반(D-2d)도 같은 규약이다 — `θ = atan2(Z, Y)`, 0° 아래, 90° 오른쪽.
//  · ⚠ **데이터는 바뀌지 않는다.** `φ_j`·`atan2(F_z,F_y)`·`atan2(γ_z,γ_y)` 값은 그대로다.
//    **표시 방향만** 규약에 맞춘 것이다 (§3.6.4.10 의 단서).
//  · 곡률중심의 축방향 배치는 Theory §4.4 확정형에서 나온다:
//      `X_j = A sin α₀ + δ_x − R_i(γ_z cos φ_j − γ_y sin φ_j)`
//    `X_j` 는 **`O_e` → `O_i` 벡터의 축성분**이고 `δ_x > 0`(내륜이 +X 로 이동)이면
//    커진다. 따라서 **`O_i` 는 `O_e` 보다 +X 쪽**에 있다. 단위벡터로 쓰면
//      `n = (sin α₀, cos α₀)` in (축 X, 반경 r),  `O_i = C + (r_i − D_w/2)·n`,
//      `O_e = C − (r_e − D_w/2)·n`,  `|O_i − O_e| = r_i + r_e − D_w = A` ✅ (A.3)
//    그리고 `O_i` 의 반경 = `D_pw/2 + (r_i − D_w/2) cos α₀` = **`R_i`** ✅ (A.4).
//    → 두 항등이 그림에서 **동시에** 맞아야 한다. 아래 검산표가 그것을 숫자로 낸다.
//  · 볼–궤도 접촉점: `P_i = C − (D_w/2)·n` (반경 안쪽 = 내륜),
//    `P_e = C + (D_w/2)·n` (반경 바깥 = 외륜). 부호가 뒤집히면 내·외륜이 바뀐다.
//
// ─────────────────────────────────────────────────────────────────────
//  🔴 D-2d 를 어떻게 보이는가 (근거)
// ─────────────────────────────────────────────────────────────────────
//  ① **수치**: 틸트 방위 `atan2(γ_z, γ_y)` 와 외부 모멘트 방위 `atan2(M_z, M_y)` 를
//     나란히 내고 그 차 `Δ` 를 낸다. 모멘트는 **`state.resultInput.operating`**
//     (Solve 시점 스냅샷)에서 읽는다 — `bbInput` 은 편집 중이라 시점이 어긋난다.
//  ② **그림**: (Y, Z) 평면 나침반에 두 벡터를 같은 길이로 겹쳐 그린다. 정합하면
//     두 화살표가 포개진다.
//  ③ **기대 부호의 근거** — Theory §4.4 확정형에서 유도된다.
//     `X_j = … + R_i γ_y sin φ_j` 이므로 `γ_y > 0` 은 `sin φ > 0` 쪽 볼의 `Q` 를 키우고,
//     내부 모멘트 `M_y = +R_i Σ Q sin α sin φ > 0` 이 된다. 평형식이 「외부 = 내부」이므로
//     `M_y > 0 ⟹ γ_y > 0`. `M_z` 도 같은 방식으로 `M_z > 0 ⟹ γ_z > 0`
//     (`X_j` 의 `−R_i γ_z cos φ` 항 → `cos φ < 0` 쪽 `Q` 증가 → `M_z = −R_i Σ Q sin α cos φ > 0`).
//     → **순수 모멘트 하중에서 두 방위는 같아야 한다.** `Δ ≈ 180°` 면 부호가 뒤집힌 것이다.
//  ⚠ 반경하중이 함께 걸리면 응답이 등방이 아니므로 `Δ = 0` 이 **요구되지 않는다.**
//     그래서 화면은 「반경하중 유무」를 함께 표시하고, 순수 모멘트일 때만 정합을 단정한다.
//     (자의적 판정을 피하기 위한 조치다 — 화면이 물리를 지어내면 검증이 무의미해진다.)
//  ⚠ 순수 모멘트라도 **`Δ = 0` 이 엄밀히 요구되지는 않는다.** 볼이 `Z` 개로 **이산**이라
//     응답은 연속 등방이 아니라 **Z-중 대칭**만 갖는다. 모멘트 방위가 대칭축(`φ_j`)과
//     어긋나면 작은 `Δ` 가 남는 것이 정상이다. 따라서 판정을 4단으로 나눈다:
//       `|Δ| < 1°` 정합 ✅ / `|Δ| < 90°` 부호 정합·이산성 🟡 /
//       `||Δ|−180°| < 15°` 반대 ❌ / 그 밖 ⚠ 솔버 의심.
//     (인수 시점 코드는 `|Δ| ≥ 1°` 를 곧바로 「솔버 의심」으로 몰아 **이산성을 솔버 오류로
//      오인**시킬 수 있었다. 화면이 없는 결함을 지어내면 안 되므로 고쳤다.)
//
// ─────────────────────────────────────────────────────────────────────
//  축척과 과장 — 무엇을 실제로 그리고 무엇을 과장했는가
// ─────────────────────────────────────────────────────────────────────
//  · **각도는 절대 과장하지 않는다.** `α₀`·`α_j` 선은 **참값 각도**로 그린다.
//    각도를 과장하면 「벌어지는 방향」이라는 이 뷰의 임무 자체가 거짓이 된다.
//  · **변위만 과장**한다. `δ_x`·틸트항·반경변위 화살표에 배율 `×M` 을 곱하고
//    **배율을 화면에 명시**한다 (`M` 은 1-2-5 계열로 반올림한 값).
//  · `A` 는 `D_pw/2` 대비 1 % 규모라 전체 반단면에서는 점으로 보인다. 그래서
//    **패널을 둘로 나눈다** — 개요(실척)와 곡률중심 상세(**뷰 확대**, 형상 왜곡 없음).
//
// ─────────────────────────────────────────────────────────────────────
//  단위 정책 (S2 확정 — 바꾸지 말 것)
// ─────────────────────────────────────────────────────────────────────
//  길이 **mm** · 하중 **N** · 응력 **MPa** · **접촉각만 °**(내부 rad) · **틸트 γ 는 rad 유지**.
//  표·hover 는 **유효숫자 9자리** — 검증 대조가 목적이라 자릿수를 줄이면 안 된다.
//
// ─────────────────────────────────────────────────────────────────────
//  ♻ 재사용 출처 (Plan §4 Phase 4-3 조사 결과)
// ─────────────────────────────────────────────────────────────────────
//  `components/SectionView2D/index.tsx` 의 **순수 유틸**을 이 파일로 **복사**했다
//  (원본은 손대지 않는다 — TRB 잔존 탭이 계속 쓴다):
//    · `viewBox` 프레임 · `<marker id="arr">` · `Txt` · `WidthDim`
//    · `AngleArc` → **시작각 인자를 추가**해 일반화(원본은 수평 0° 고정). 이 뷰의 각도는
//      **수직 기준**이라 그대로는 보각(90°−α)을 그리게 되어 틀린 그림이 된다.
//  P4-2c 에서 **신설**한 것 (원본에 대응물이 없다):
//    · `layoutGutter`/`GutterLabel` — 라벨을 도형 밖 여백에만 두고 최소간격을 강제한다.
//      겹침을 「피하는」 것이 아니라 **겹칠 수 있는 자리에 두지 않는** 방식이다.
//    · `VerticalDim`(`WidthDim` 의 축 대칭판) · `HatchDefs`(45° 해칭) · `AxisMarker`(Z⊗→X, Y↓)
//  P4-2c 에서 **버린** 것: `CalloutDim`·`SlantDim`·`pickMagnification`·2패널 구성·
//    곡률중심 `O_i`·`O_e` 마커·변위 과장 오버레이(δ_x·틸트·O_i′).
//  복사하지 **않은** 것: `computeSectionGeometry`·`SectionHalf`·`RibHeightDim`·`LweDim`·
//  `Annotations` 본문(γ 경사축·사다리꼴 롤러·리브 전량) — BB 에 대응 개념이 없다.
//  색 팔레트·`num()`·`toDeg` 는 `BbLoadDistView` 와 **같은 값**을 쓴다.

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useAppState } from '../store';
import { DetailTable } from '../components/shared/DetailTable';
import type { Alert } from './generated/Alert';
import type { BallBearingGeometry } from './generated/BallBearingGeometry';
import type { BbGeometryDerived } from './generated/BbGeometryDerived';

/** `commands::GeometryResponse` 대응 — `BbGeometryView` 와 같은 형태다. */
interface GeometryResponse {
  derived: BbGeometryDerived;
  alerts: Alert[];
}

/** 유효숫자 9자리 (또는 지수표기) — 사유는 파일 헤더의 단위 정책 참조. */
function num(v: number): string {
  if (!Number.isFinite(v)) return String(v);
  if (v === 0) return '0';
  const a = Math.abs(v);
  if (a < 1e-4 || a >= 1e9) return v.toExponential(8);
  return v.toPrecision(9);
}

const toDeg = (rad: number) => (rad * 180) / Math.PI;

/** (−180, 180] 로 접는다 — 방위차 Δ 표시용. */
function wrapDeg(d: number): number {
  let x = ((d + 180) % 360 + 360) % 360 - 180;
  if (x === -180) x = 180;
  return x;
}

// ─── 색 팔레트 — `BbLoadDistView` 와 동일 (§3.6.1.4 통합 대비) ─────────
const C_LOADED = '#f59e0b';
const C_UNLOADED = '#64748b';
const C_LOAD_DIR = '#ef4444';
const C_ALPHA = '#38bdf8';
const C_REF = '#a3e635';
const C_INNER = '#3b82f6';
const C_OUTER = '#f97316';


// ─── 기하 재구성 (Plan §4 Phase 4-3: raceway 프로파일이 결과에 없다) ───

interface SectionFrame {
  rBore: number;
  rOD: number;
  halfB: number;
  rPitch: number;
  ballR: number;
  ri: number;
  re: number;
  aMm: number;
  alpha0: number;
  /** 접촉선 단위벡터 `n = (sin α₀, cos α₀)` — 헤더의 부호 규약 참조. */
  nx: number;
  ny: number;
  C: [number, number];
  Oi: [number, number];
  Oe: [number, number];
  Pi: [number, number];
  Pe: [number, number];
  /** `O_i` 의 반경 — (A.4) 의 `R_i` 와 같아야 한다. */
  riCenterCheck: number;
}

function buildFrame(g: BallBearingGeometry, d: BbGeometryDerived): SectionFrame {
  const ballR = g.d_w_mm / 2;
  const rPitch = g.d_pw_mm / 2;
  const alpha0 = d.alpha_0_rad;
  const nx = Math.sin(alpha0);
  const ny = Math.cos(alpha0);
  const C: [number, number] = [0, rPitch];
  const li = g.r_i_mm - ballR; // O_i 까지의 거리
  const le = g.r_e_mm - ballR; // O_e 까지의 거리
  const Oi: [number, number] = [C[0] + li * nx, C[1] + li * ny];
  const Oe: [number, number] = [C[0] - le * nx, C[1] - le * ny];
  const Pi: [number, number] = [C[0] - ballR * nx, C[1] - ballR * ny];
  const Pe: [number, number] = [C[0] + ballR * nx, C[1] + ballR * ny];
  return {
    rBore: g.bore_mm / 2,
    rOD: g.outer_diameter_mm / 2,
    halfB: g.width_mm / 2,
    rPitch,
    ballR,
    ri: g.r_i_mm,
    re: g.r_e_mm,
    aMm: d.a_mm,
    alpha0,
    nx,
    ny,
    C,
    Oi,
    Oe,
    Pi,
    Pe,
    riCenterCheck: Oi[1],
  };
}

/** 원호를 폴리라인으로 — SVG `A` 플래그(뒤집힌 좌표계에서 sweep 해석이 헷갈린다)를 피한다. */
function arcPath(cx: number, cy: number, r: number, a0: number, a1: number, seg = 64): string {
  const pts: string[] = [];
  for (let i = 0; i <= seg; i++) {
    const a = a0 + (a1 - a0) * (i / seg);
    pts.push(`${cx + r * Math.cos(a)},${cy + r * Math.sin(a)}`);
  }
  return 'M ' + pts.join(' L ');
}

// ─── SVG 프리미티브 (SectionView2D 복사 — 파일 헤더의 재사용 출처 참조) ───

function Txt({
  x,
  y,
  text,
  fs,
  fill = '#e2e8f0',
  anchor = 'start',
}: {
  x: number;
  y: number;
  text: string;
  fs: number;
  fill?: string;
  anchor?: 'start' | 'middle' | 'end';
}) {
  // ⚠ 원본 `SectionView2D` 는 부모가 `scale(1,-1)` 이라 텍스트를 다시 뒤집었다.
  //    이 뷰는 **§3.6.4.10 각도 규약**에 따라 부모의 뒤집기를 없앴으므로
  //    (반경 바깥 = 화면 아래) 여기서도 되뒤집지 **않는다**.
  return (
    <g transform={`translate(${x}, ${y})`}>
      <text
        fill={fill}
        fontSize={fs}
        fontFamily="'JetBrains Mono', monospace"
        dominantBaseline="middle"
        textAnchor={anchor}
        fontWeight={500}
      >
        {text}
      </text>
    </g>
  );
}

/** 수평 양방향 치수선 (SectionView2D 342–358 복사). */
function WidthDim({
  y,
  xLeft,
  xRight,
  yTick,
  label,
  s,
  fs,
  marker,
}: {
  y: number;
  xLeft: number;
  xRight: number;
  yTick: number;
  label: string;
  s: number;
  fs: number;
  /** 이 패널 전용 `<marker>` id (패널마다 축척이 달라 공유하면 안 된다). */
  marker: string;
}) {
  return (
    <g>
      <line x1={xLeft} y1={yTick} x2={xLeft} y2={y + s * 3} stroke="#475569" strokeWidth={s * 0.4} />
      <line x1={xRight} y1={yTick} x2={xRight} y2={y + s * 3} stroke="#475569" strokeWidth={s * 0.4} />
      <line
        x1={xLeft}
        y1={y}
        x2={xRight}
        y2={y}
        stroke="#94a3b8"
        strokeWidth={s * 0.4}
        markerStart={`url(#${marker})`}
        markerEnd={`url(#${marker})`}
      />
      <Txt x={(xLeft + xRight) / 2} y={y + fs * 0.7} text={label} fs={fs} anchor="middle" />
    </g>
  );
}

/**
 * 각도 호 + 가이드선 (SectionView2D 412–447 복사, **시작각 인자 추가**).
 *
 * ⚠ 원본은 수평(0°)에서만 출발한다. 접촉각 `α` 는 **반경방향(수직 = 90°)** 에서
 *   재므로 그대로 쓰면 보각을 그린다 — 그림이 조용히 틀린다. 그래서 `a0Deg` 를 뒀다.
 */
function AngleArc({
  cx,
  cy,
  radius,
  a0Deg,
  a1Deg,
  label,
  s,
  fs,
  color,
}: {
  cx: number;
  cy: number;
  radius: number;
  a0Deg: number;
  a1Deg: number;
  label: string;
  s: number;
  fs: number;
  color: string;
}) {
  const a0 = (a0Deg * Math.PI) / 180;
  const a1 = (a1Deg * Math.PI) / 180;
  const guideLen = radius * 1.35;
  const mid = (a0 + a1) / 2;
  return (
    <g>
      <line
        x1={cx}
        y1={cy}
        x2={cx + guideLen * Math.cos(a0)}
        y2={cy + guideLen * Math.sin(a0)}
        stroke={color}
        strokeWidth={s * 0.4}
        strokeOpacity={0.5}
        strokeDasharray={`${s * 1.5} ${s * 0.8}`}
      />
      <line
        x1={cx}
        y1={cy}
        x2={cx + guideLen * Math.cos(a1)}
        y2={cy + guideLen * Math.sin(a1)}
        stroke={color}
        strokeWidth={s * 0.4}
        strokeOpacity={0.5}
        strokeDasharray={`${s * 1.5} ${s * 0.8}`}
      />
      <path d={arcPath(cx, cy, radius, a0, a1, 48)} fill="none" stroke={color} strokeWidth={s * 0.5} />
      <Txt
        x={cx + radius * 1.18 * Math.cos(mid)}
        y={cy + radius * 1.18 * Math.sin(mid)}
        text={label}
        fs={fs * 0.85}
        fill={color}
        anchor="start"
      />
    </g>
  );
}

/**
 * `<marker>` — SectionView2D 202–208 복사, **id 를 패널별로 분리**.
 *
 * ⚠ 인수 시점 코드는 두 패널이 **같은 `id="arr"`** 를 뿜었다. SVG 의 `url(#…)` 은
 *   문서에서 **처음 만난** 요소를 잡으므로 상세 패널의 화살표가 개요 패널의 마커를
 *   참조했다. 두 패널은 사용자좌표 축척이 서로 다르고(`markerUnits` 기본값이
 *   `strokeWidth`) `markerWidth` 가 각 패널의 `fs` 로 정해지므로 **화살촉 크기가
 *   조용히 틀린다.** 그래서 `idPrefix` 를 받아 유일 id 를 만든다.
 *   (미사용이던 `arrHi` 마커는 참조하는 곳이 없어 함께 제거했다.)
 */
function ArrowDefs({ id, size }: { id: string; size: number }) {
  return (
    <defs>
      <marker
        id={id}
        viewBox="0 0 6 3"
        refX="6"
        refY="1.5"
        markerWidth={size}
        markerHeight={size * 0.5}
        orient="auto-start-reverse"
        fill="#94a3b8"
      >
        <path d="M0,0 L6,1.5 L0,3 Z" />
      </marker>
    </defs>
  );
}

interface LoadedOverlay {
  /** 축방향 총 증분 `ΔX = δ_x + 틸트항` [mm] */
  dX: number;
  /** 축방향 중 `δ_x` 만 [mm] */
  dxOnly: number;
  /** 틸트 기여항 `−R_i(γ_z cos φ − γ_y sin φ)` [mm] */
  tiltTerm: number;
  /** 반경방향 증분 `ΔR = δ_y cos φ + δ_z sin φ` [mm] */
  dR: number;
  /** 솔버가 낸 운전 접촉각 `α_j` [rad] */
  alphaJ: number;
  loaded: boolean;
}

// ─── 여백 배치기 — 라벨 겹침을 **구조적으로** 없앤다 ─────────────────
//
//  이전 구현은 `<text>` 18개가 전부 하드코딩 오프셋이었고 halo·충돌검사가
//  하나도 없어, 베어링 치수나 창 크기가 바뀌면 글자가 선·도형과 겹쳤다.
//  해법은 「겹치면 피한다」가 아니라 **「겹칠 수 있는 자리에 두지 않는다」** 다:
//    ① 라벨은 **도형 영역 밖(좌·우 여백)** 에만 놓는다 → 선과 겹칠 수 없다.
//    ② 같은 여백 안에서는 **최소 간격 `minGap` 을 강제**한다 → 라벨끼리 겹칠 수 없다.
//  ①이 「선 ↔ 글자」를, ②가 「글자 ↔ 글자」를 막는다. 둘이 합쳐 겹침이 0이 된다.

interface GutterItem {
  /** 지시 대상의 y (여기에 최대한 가깝게 두려 한다) */
  yWant: number;
  /** 지시선이 닿을 도형 위의 점 */
  dotX: number;
  dotY: number;
  label: string;
  color?: string;
}

/** `yWant` 순으로 정렬한 뒤 `minGap` 미만으로 붙은 것을 아래로 밀어낸다. */
function layoutGutter(items: GutterItem[], minGap: number): (GutterItem & { y: number })[] {
  const sorted = [...items].sort((a, b) => a.yWant - b.yWant);
  const out: (GutterItem & { y: number })[] = [];
  let prev = -Infinity;
  for (const it of sorted) {
    const y = Math.max(it.yWant, prev + minGap);
    out.push({ ...it, y });
    prev = y;
  }
  // 아래로만 밀면 전체가 처진다 — 밀린 총량의 절반을 위로 되돌려 중심을 맞춘다.
  const drift = out.length > 0 ? out[out.length - 1].y - out[out.length - 1].yWant : 0;
  if (drift > 0) for (const it of out) it.y -= drift / 2;
  return out;
}

/** 여백 라벨 하나 — 지시선(수평 엘보) + 글자. 글자는 여백 안에서만 산다. */
function GutterLabel({
  item,
  gutterX,
  side,
  s,
  fs,
}: {
  item: GutterItem & { y: number };
  /** 글자가 놓일 x (여백 안쪽 경계) */
  gutterX: number;
  side: 'left' | 'right';
  s: number;
  fs: number;
}) {
  const col = item.color ?? '#94a3b8';
  // 엘보: 라벨 → 수평 → 대상. 꺾임점은 대상 쪽으로 60 %.
  const elbowX = gutterX + (item.dotX - gutterX) * 0.55;
  return (
    <g>
      <circle cx={item.dotX} cy={item.dotY} r={s * 1.1} fill={col} />
      <polyline
        points={`${gutterX},${item.y} ${elbowX},${item.y} ${item.dotX},${item.dotY}`}
        fill="none"
        stroke={col}
        strokeWidth={s * 0.45}
        strokeDasharray={`${s * 2.2} ${s * 1.1}`}
        opacity={0.85}
      />
      <Txt
        x={gutterX + (side === 'left' ? -s * 1.5 : s * 1.5)}
        y={item.y}
        text={item.label}
        fs={fs}
        fill={col}
        anchor={side === 'left' ? 'end' : 'start'}
      />
    </g>
  );
}

/** 수직 치수선 — `WidthDim`(수평)의 축 대칭판. `D_pw/2` · `R_i` 용. */
function VerticalDim({
  x,
  yTop,
  yBot,
  xTick,
  label,
  s,
  fs,
  marker,
}: {
  x: number;
  yTop: number;
  yBot: number;
  /** 연장선이 도형에 닿는 x */
  xTick: number;
  label: string;
  s: number;
  fs: number;
  marker: string;
}) {
  return (
    <g>
      <line x1={xTick} y1={yTop} x2={x - s * 3} y2={yTop} stroke="#475569" strokeWidth={s * 0.4} />
      <line x1={xTick} y1={yBot} x2={x - s * 3} y2={yBot} stroke="#475569" strokeWidth={s * 0.4} />
      <line
        x1={x}
        y1={yTop}
        x2={x}
        y2={yBot}
        stroke="#94a3b8"
        strokeWidth={s * 0.4}
        markerStart={`url(#${marker})`}
        markerEnd={`url(#${marker})`}
      />
      {/* 라벨은 치수선 **옆**에 세로 중앙 — 치수선과 겹치지 않는다. */}
      <Txt x={x - s * 1.6} y={(yTop + yBot) / 2} text={label} fs={fs} anchor="end" />
    </g>
  );
}

/** 45° 해칭 — ISO 도면의 링 단면 표기. 패널마다 id 가 유일해야 한다. */
function HatchDefs({ id, pitch, s }: { id: string; pitch: number; s: number }) {
  return (
    <defs>
      <pattern id={id} patternUnits="userSpaceOnUse" width={pitch} height={pitch} patternTransform="rotate(45)">
        <line x1={0} y1={0} x2={0} y2={pitch} stroke="#64748b" strokeWidth={s * 0.35} opacity={0.75} />
      </pattern>
    </defs>
  );
}

/** 좌표 마커 — `Z⊗ → X`, `Y↓`. ISO 16281 Fig A.1 a) 의 원점 표기. */
function AxisMarker({ x, y, len, s, fs }: { x: number; y: number; len: number; s: number; fs: number }) {
  const r = fs * 0.45;
  return (
    <g>
      {/* Z: 지면 밖에서 안으로 = ⊗ */}
      <circle cx={x} cy={y} r={r} fill="none" stroke="#e2e8f0" strokeWidth={s * 0.45} />
      <line x1={x - r * 0.7} y1={y - r * 0.7} x2={x + r * 0.7} y2={y + r * 0.7} stroke="#e2e8f0" strokeWidth={s * 0.45} />
      <line x1={x - r * 0.7} y1={y + r * 0.7} x2={x + r * 0.7} y2={y - r * 0.7} stroke="#e2e8f0" strokeWidth={s * 0.45} />
      <Txt x={x - r * 1.6} y={y - r * 1.3} text="Z" fs={fs * 0.9} anchor="end" />
      {/* X: 오른쪽 = 회전축 */}
      <line x1={x + r * 1.3} y1={y} x2={x + len} y2={y} stroke="#e2e8f0" strokeWidth={s * 0.5} />
      <path d={`M${x + len},${y} L${x + len - fs * 0.5},${y - fs * 0.22} L${x + len - fs * 0.5},${y + fs * 0.22} Z`} fill="#e2e8f0" />
      <Txt x={x + len + fs * 0.35} y={y} text="X" fs={fs * 0.9} anchor="start" />
      {/* Y: 아래 (ISO 16281 A.2.2 NOTE) */}
      <line x1={x} y1={y + r * 1.3} x2={x} y2={y + len} stroke="#e2e8f0" strokeWidth={s * 0.5} />
      <path d={`M${x},${y + len} L${x - fs * 0.22},${y + len - fs * 0.5} L${x + fs * 0.22},${y + len - fs * 0.5} Z`} fill="#e2e8f0" />
      <Txt x={x + fs * 0.35} y={y + len + fs * 0.3} text="Y" fs={fs * 0.9} anchor="start" />
    </g>
  );
}

// ─── ISO 16281 Fig A.1 a) 스타일 전단면 (단일 패널) ──────────────────
//
//  P4-2c 확정 (2026-08-24): 패널을 **하나만** 둔다. 곡률중심 `O_i`·`O_e` 마커는
//  그리지 않는다 — ISO 원도면도 표기하지 않으며, 볼 지름에 비해 간격이 너무 작아
//  실척에서 점 두 개가 겹쳐 보일 뿐이다. `A` 는 ISO 와 같이 **접촉선 위의 치수**로만 낸다.
//  (`O_i`·`O_e` 의 수치 검산은 아래 「기하 항등」 표가 계속 맡는다.)
//
//  기하는 **한 번만** 그리고 `scale(1,-1)` 로 미러링해 전단면을 만든다.
//  ⚠ 미러 그룹 안에는 `<text>` 를 **절대 넣지 않는다** — 뒤집혀 읽을 수 없게 된다.
//     글자는 전부 최상위(여백)에 있고, 그래서 미러링이 라벨을 건드리지 않는다.

const MK_ISO = 'bbsec-arr-iso';
const HATCH_I = 'bbsec-hatch-inner';
const HATCH_E = 'bbsec-hatch-outer';

function IsoSectionPanel({
  f,
  ov,
  phiRad,
}: {
  f: SectionFrame;
  ov: LoadedOverlay | null;
  phiRad: number;
}) {
  // ── 좌표계: x = 축(오른쪽 +X), y = 반경(아래 +Y). 축은 y = 0. ──
  const gutter = f.rOD * 0.62; // 라벨 전용 여백 — 도형은 여기 들어오지 않는다
  const xDraw = f.halfB * 1.05;
  const xMin = -xDraw - gutter;
  const xMax = xDraw + gutter;
  const yPad = f.rOD * 0.18;
  const yMin = -f.rOD - yPad;
  const yMax = f.rOD + yPad;

  const span = xMax - xMin;
  const fs = f.rOD * 0.052;
  const s = f.rOD * 0.008;
  const minGap = fs * 1.5;

  // ── 궤도 홈 원호 — 접촉점 근방만. 폴리곤이 아니다 (§3.6.3.3). ──
  const spanRad = (52 * Math.PI) / 180;
  const bi = Math.atan2(-f.ny, -f.nx); // O_i → P_i
  const be = Math.atan2(f.ny, f.nx); // O_e → P_e
  const arcI = arcPath(f.Oi[0], f.Oi[1], f.ri, bi - spanRad, bi + spanRad);
  const arcE = arcPath(f.Oe[0], f.Oe[1], f.re, be - spanRad, be + spanRad);

  // 홈 원호의 끝점 반경 = 그 링의 랜드(land) 반경.
  const endI = [bi - spanRad, bi + spanRad].map(a => f.Oi[1] + f.ri * Math.sin(a));
  const endE = [be - spanRad, be + spanRad].map(a => f.Oe[1] + f.re * Math.sin(a));
  const landI = Math.max(...endI); // 내륜 바깥 경계 (반경 큰 쪽 = 화면 아래)
  const landE = Math.min(...endE); // 외륜 안쪽 경계
  const arcIx = [bi - spanRad, bi + spanRad].map(a => f.Oi[0] + f.ri * Math.cos(a));
  const arcEx = [be - spanRad, be + spanRad].map(a => f.Oe[0] + f.re * Math.cos(a));

  // ── 링 단면 폐곡선 ──
  //  내륜: bore ~ 랜드, 가운데가 홈으로 파여 있다.
  const ringI =
    `M ${-f.halfB},${f.rBore} L ${f.halfB},${f.rBore} L ${f.halfB},${landI} ` +
    `L ${Math.max(...arcIx)},${landI} ` +
    arcPath(f.Oi[0], f.Oi[1], f.ri, bi + spanRad, bi - spanRad).replace('M ', 'L ') +
    ` L ${Math.min(...arcIx)},${landI} L ${-f.halfB},${landI} Z`;
  //  외륜: 랜드 ~ OD.
  //  ⚠ 폐곡선은 **+x 끝 → 홈 → −x 끝** 순으로 돌아야 자기교차하지 않는다.
  //     내륜은 `bi+span` 이 +x 끝이지만 **외륜은 `be−span` 이 +x 끝**이라 방향이 반대다:
  //     `bi = atan2(−cos α₀, −sin α₀) ∈ [−135°, −90°]` · `be = atan2(cos α₀, sin α₀) ∈ [45°, 90°]`
  //     로 부호가 갈리기 때문이다. 같은 방향으로 쓰면 외륜 단면이 나비넥타이처럼 꼬인다.
  const ringE =
    `M ${-f.halfB},${f.rOD} L ${f.halfB},${f.rOD} L ${f.halfB},${landE} ` +
    `L ${Math.max(...arcEx)},${landE} ` +
    arcPath(f.Oe[0], f.Oe[1], f.re, be - spanRad, be + spanRad).replace('M ', 'L ') +
    ` L ${Math.min(...arcEx)},${landE} L ${-f.halfB},${landE} Z`;

  // ── 접촉선: 볼 중심을 지나 α₀ 방향. 여기서는 **OD 까지 길게** 뽑는다. ──
  //  1° 남짓한 `α₀ ↔ α_j` 차이는 짧은 선에서는 눈에 보이지 않는다.
  //  선을 길게 그리면 끝점 간 거리가 벌어져 **육안으로 판별 가능**해진다 (D-1).
  const rayLen = f.rOD * 0.62;
  const rayEnd = (a: number): [number, number] => [f.C[0] + rayLen * Math.sin(a), f.C[1] + rayLen * Math.cos(a)];
  const rayStart = (a: number): [number, number] => [f.C[0] - rayLen * 0.55 * Math.sin(a), f.C[1] - rayLen * 0.55 * Math.cos(a)];
  const e0 = rayEnd(f.alpha0);
  const s0 = rayStart(f.alpha0);
  const eJ = ov ? rayEnd(ov.alphaJ) : null;

  // ── 기하 그룹: 한 번 그리고 미러링한다. 텍스트 없음. ──
  const geom = (
    <>
      <path d={ringI} fill={`url(#${HATCH_I})`} stroke={C_INNER} strokeWidth={s * 0.75} opacity={0.95} />
      <path d={ringE} fill={`url(#${HATCH_E})`} stroke={C_OUTER} strokeWidth={s * 0.75} opacity={0.95} />
      {/* 홈 원호를 한 번 더 진하게 — 어느 선이 궤도인지 분명해야 한다 */}
      <path d={arcI} fill="none" stroke={C_INNER} strokeWidth={s * 1.15} />
      <path d={arcE} fill="none" stroke={C_OUTER} strokeWidth={s * 1.15} />
      {/* 볼 */}
      <circle cx={f.C[0]} cy={f.C[1]} r={f.ballR} fill="#e2e8f0" fillOpacity={0.1} stroke="#cbd5e1" strokeWidth={s * 0.9} />
      <circle cx={f.C[0]} cy={f.C[1]} r={s * 1.2} fill="#cbd5e1" />
      {/* 피치원선 (1점쇄선) */}
      <line
        x1={-f.halfB * 1.02}
        y1={f.rPitch}
        x2={f.halfB * 1.02}
        y2={f.rPitch}
        stroke={C_REF}
        strokeWidth={s * 0.4}
        strokeDasharray={`${s * 6} ${s * 2} ${s} ${s * 2}`}
        opacity={0.7}
      />
      {/* 무하중 접촉선 α₀ */}
      <line x1={s0[0]} y1={s0[1]} x2={e0[0]} y2={e0[1]} stroke={C_ALPHA} strokeWidth={s * 0.85} />
      {/* 운전 접촉선 α_j (하중 후) */}
      {eJ && (
        <line
          x1={f.C[0]}
          y1={f.C[1]}
          x2={eJ[0]}
          y2={eJ[1]}
          stroke={ov?.loaded ? C_LOADED : C_UNLOADED}
          strokeWidth={s * 0.85}
          strokeDasharray={`${s * 3} ${s * 2}`}
        />
      )}
    </>
  );

  // ── 여백 라벨 ──
  const right: GutterItem[] = [
    { yWant: f.rBore, dotX: f.halfB * 0.55, dotY: f.rBore, label: `d = ${num(f.rBore * 2)}`, color: C_INNER },
    { yWant: landI, dotX: arcIx[0], dotY: f.Oi[1] + f.ri * Math.sin(bi - spanRad * 0.8), label: `r_i = ${num(f.ri)}`, color: C_INNER },
    { yWant: f.rPitch, dotX: f.halfB * 0.75, dotY: f.rPitch, label: `D_pw = ${num(f.rPitch * 2)}`, color: C_REF },
    { yWant: f.C[1] + f.ballR * 0.7, dotX: f.C[0] + f.ballR * 0.7, dotY: f.C[1] + f.ballR * 0.7, label: `D_w = ${num(f.ballR * 2)}`, color: '#cbd5e1' },
    { yWant: landE, dotX: arcEx[1], dotY: f.Oe[1] + f.re * Math.sin(be + spanRad * 0.8), label: `r_e = ${num(f.re)}`, color: C_OUTER },
    { yWant: f.rOD, dotX: f.halfB * 0.35, dotY: f.rOD, label: `D = ${num(f.rOD * 2)}`, color: C_OUTER },
  ];
  const left: GutterItem[] = [
    { yWant: f.C[1] - f.ballR * 0.2, dotX: (s0[0] + f.C[0]) / 2, dotY: (s0[1] + f.C[1]) / 2, label: `α₀ = ${num(toDeg(f.alpha0))}°`, color: C_ALPHA },
    { yWant: f.C[1] + f.ballR * 0.9, dotX: (f.Oi[0] + f.Oe[0]) / 2, dotY: (f.Oi[1] + f.Oe[1]) / 2, label: `A = r_i+r_e−D_w = ${num(f.aMm)}`, color: '#a78bfa' },
  ];
  if (ov && eJ) {
    left.push({
      yWant: eJ[1],
      dotX: (f.C[0] + eJ[0]) / 2,
      dotY: (f.C[1] + eJ[1]) / 2,
      label: `α_j = ${num(toDeg(ov.alphaJ))}° (볼 φ=${num(toDeg(phiRad))}°)`,
      color: ov.loaded ? C_LOADED : C_UNLOADED,
    });
  }
  const rightL = layoutGutter(right, minGap);
  const leftL = layoutGutter(left, minGap);
  const gxR = xDraw + gutter * 0.42;
  const gxL = -xDraw - gutter * 0.42;

  return (
    <svg
      viewBox={`${xMin} ${yMin} ${span} ${yMax - yMin}`}
      preserveAspectRatio="xMidYMid meet"
      style={{ width: '100%', maxHeight: '62vh', display: 'block' }}
    >
      <ArrowDefs id={MK_ISO} size={fs * 0.5} />
      <HatchDefs id={HATCH_I} pitch={f.rOD * 0.035} s={s} />
      <HatchDefs id={HATCH_E} pitch={f.rOD * 0.035} s={s} />

      {/* 회전축 X 중심선 (1점쇄선) — 전단면의 대칭축 */}
      <line
        x1={xMin + gutter * 0.35}
        y1={0}
        x2={xMax - gutter * 0.35}
        y2={0}
        stroke="#64748b"
        strokeWidth={s * 0.45}
        strokeDasharray={`${s * 8} ${s * 2.5} ${s * 1.2} ${s * 2.5}`}
      />

      {/* 아래쪽 반단면 (+Y) — 치수는 이쪽에 단다 */}
      <g>{geom}</g>
      {/* 위쪽 반단면 (−Y) — 같은 기하의 거울상. 텍스트 없음 */}
      <g transform="scale(1,-1)">{geom}</g>

      {/* 수직 치수선 — 축(y=0) 에서 잰다 */}
      <VerticalDim
        x={-xDraw - gutter * 0.08}
        yTop={0}
        yBot={f.rPitch}
        xTick={-f.halfB}
        label={`D_pw/2 = ${num(f.rPitch)}`}
        s={s}
        fs={fs * 0.85}
        marker={MK_ISO}
      />
      <VerticalDim
        x={xDraw + gutter * 0.08}
        yTop={0}
        yBot={f.riCenterCheck}
        xTick={f.halfB}
        label={`R_i = ${num(f.riCenterCheck)}`}
        s={s}
        fs={fs * 0.85}
        marker={MK_ISO}
      />

      {/* 폭 B 치수선 (아래쪽 여백) */}
      <WidthDim
        y={f.rOD + yPad * 0.55}
        xLeft={-f.halfB}
        xRight={f.halfB}
        yTick={f.rOD}
        label={`B = ${num(f.halfB * 2)}`}
        s={s}
        fs={fs * 0.85}
        marker={MK_ISO}
      />

      {/* α₀ 각도호 — 반경 기준선(아래)에서 축(+X) 쪽으로 */}
      <AngleArc
        cx={f.C[0]}
        cy={f.C[1]}
        radius={rayLen * 0.42}
        a0Deg={90}
        a1Deg={90 - toDeg(f.alpha0)}
        label=""
        s={s}
        fs={fs}
        color={C_ALPHA}
      />

      {/* 여백 라벨 — 도형 영역에 절대 진입하지 않는다 */}
      {rightL.map((it, i) => (
        <GutterLabel key={`r${i}`} item={it} gutterX={gxR} side="right" s={s} fs={fs * 0.82} />
      ))}
      {leftL.map((it, i) => (
        <GutterLabel key={`l${i}`} item={it} gutterX={gxL} side="left" s={s} fs={fs * 0.82} />
      ))}

      {/* 좌표 마커 Z⊗ → X, Y↓ */}
      <AxisMarker x={0} y={0} len={f.rOD * 0.3} s={s} fs={fs} />
    </svg>
  );
}

function TiltCompass({
  tiltDeg,
  momDeg,
  phiDeg,
}: {
  tiltDeg: number | null;
  momDeg: number | null;
  phiDeg: number;
}) {
  const R = 46;
  const cx = 62;
  const cy = 62;
  // 🔴 §3.6.4.10 각도 표시 규약 — **0° = +Y = 화면 아래**, 각도는 **우측(+Z)** 으로 증가.
  //    근거: ISO 16281 A.2.2 NOTE 「모든 그림에서 Y 축은 지면 아래쪽」 + 우수좌표계
  //    (X = 회전축 = 화면 밖) ⟹ Z = X × Y = 오른쪽.
  //    방위각 θ = atan2(Z, Y) 이므로 화면 좌표는 x = cx + r sinθ, y = cy + r cosθ 다
  //    (SVG 는 y 가 아래 방향이라 cos 항에 −를 붙이지 **않는다**).
  const pt = (deg: number, r: number): [number, number] => {
    const a = (deg * Math.PI) / 180;
    return [cx + r * Math.sin(a), cy + r * Math.cos(a)];
  };
  const [phx, phy] = pt(phiDeg, R);
  return (
    <svg width={124} height={124} className="shrink-0">
      <circle cx={cx} cy={cy} r={R} fill="none" stroke="#334155" strokeWidth={1} />
      <line x1={cx - R} y1={cy} x2={cx + R} y2={cy} stroke="#334155" strokeWidth={0.8} />
      <line x1={cx} y1={cy - R} x2={cx} y2={cy + R} stroke="#334155" strokeWidth={0.8} />
      <text x={cx} y={cy + R + 10} fill="#94a3b8" fontSize={9} fontFamily="monospace" textAnchor="middle">
        +Y · 0°
      </text>
      <text x={cx + R - 1} y={cy - 5} fill="#94a3b8" fontSize={9} fontFamily="monospace" textAnchor="end">
        +Z · 90°
      </text>
      {/* 선택한 볼의 방위 φ_j */}
      <line x1={cx} y1={cy} x2={phx} y2={phy} stroke={C_LOADED} strokeWidth={1} strokeDasharray="3 2" />
      <circle cx={phx} cy={phy} r={2.5} fill={C_LOADED} />
      {momDeg !== null && (
        <line
          x1={cx}
          y1={cy}
          x2={pt(momDeg, R * 0.86)[0]}
          y2={pt(momDeg, R * 0.86)[1]}
          stroke={C_LOAD_DIR}
          strokeWidth={3}
          strokeLinecap="round"
        />
      )}
      {tiltDeg !== null && (
        <line
          x1={cx}
          y1={cy}
          x2={pt(tiltDeg, R * 0.62)[0]}
          y2={pt(tiltDeg, R * 0.62)[1]}
          stroke={C_ALPHA}
          strokeWidth={2}
          strokeLinecap="round"
        />
      )}
      <circle cx={cx} cy={cy} r={2} fill="#e2e8f0" />
    </svg>
  );
}

// ─── 본체 ──────────────────────────────────────────────────────────

export default function BbAxialSectionView() {
  const { state } = useAppState();
  const { bbInput, result, resultInput } = state;

  const [derived, setDerived] = useState<BbGeometryDerived | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [selected, setSelected] = useState<number | null>(null);

  // `BbGeometryView`(85–110) 와 **같은 패턴** — 150 ms 디바운스 + `cancelled` 플래그.
  // ⚠ 구독 대상은 `bbInput` **전체**다. `geometry` 만 구독하면 낡은 값이 남는다.
  useEffect(() => {
    if (!bbInput) return;
    let cancelled = false;
    const timer = setTimeout(() => {
      invoke<GeometryResponse>('bb_compute_geometry', { input: bbInput })
        .then(r => {
          if (cancelled) return;
          setDerived(r.derived);
          setError(null);
        })
        .catch((e: unknown) => {
          if (cancelled) return;
          // Rust 메시지를 **그대로** 낸다 (§3.6.5.3).
          setError(String(e));
          setDerived(null);
        });
    }, 150);
    return () => {
      cancelled = true;
      clearTimeout(timer);
    };
  }, [bbInput]);

  if (!bbInput) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-text-canvas text-sm">프리셋을 불러오는 중…</p>
      </div>
    );
  }

  const g = bbInput.geometry;
  const balls = result?.equilibrium.ball_results ?? [];

  // 기본 선택 = **최대하중 볼**. `α_j` 가 볼마다 다르므로 어느 볼인지 항상 명시한다.
  let top = -1;
  for (let i = 0; i < balls.length; i++) {
    if (top < 0 || balls[i].q_n > balls[top].q_n) top = i;
  }
  const zCount = balls.length > 0 ? balls.length : Math.max(1, g.z);
  const sel = selected !== null && selected >= 0 && selected < zCount ? selected : top >= 0 ? top : 0;
  const ball = sel < balls.length ? balls[sel] : null;
  const phiRad = ball ? ball.phi_rad : (2 * Math.PI * sel) / zCount;

  // ── D-2d: 틸트 방위 ↔ 모멘트 방위 ──────────────────────────────
  const disp = result?.equilibrium.displacement ?? null;
  const op = resultInput?.operating ?? null;
  const tiltMag = disp ? Math.hypot(disp.ry_rad, disp.rz_rad) : 0;
  const momMag = op ? Math.hypot(op.m_y_nmm, op.m_z_nmm) : 0;
  const radMag = op ? Math.hypot(op.f_y_n, op.f_z_n) : 0;
  const tiltDeg = disp && tiltMag > 0 ? toDeg(Math.atan2(disp.rz_rad, disp.ry_rad)) : null;
  const momDeg = op && momMag > 0 ? toDeg(Math.atan2(op.m_z_nmm, op.m_y_nmm)) : null;
  const deltaDeg = tiltDeg !== null && momDeg !== null ? wrapDeg(tiltDeg - momDeg) : null;
  // 반경하중이 함께 걸리면 등방 응답이 아니므로 Δ = 0 이 **요구되지 않는다** (헤더 ⚠).
  const pureMoment = momMag > 0 && radMag === 0;

  // ── 하중 후 오버레이 ───────────────────────────────────────────
  let ov: LoadedOverlay | null = null;
  if (derived && disp && ball) {
    const tiltTerm = -derived.r_i_center_mm * (disp.rz_rad * Math.cos(phiRad) - disp.ry_rad * Math.sin(phiRad));
    ov = {
      dxOnly: disp.dx_mm,
      tiltTerm,
      dX: disp.dx_mm + tiltTerm,
      dR: disp.dy_mm * Math.cos(phiRad) + disp.dz_mm * Math.sin(phiRad),
      alphaJ: ball.alpha_rad,
      loaded: ball.loaded,
    };
  }

  if (error) {
    return (
      <div className="h-full overflow-auto custom-scrollbar p-4">
        <div className="p-2 rounded border bg-red-500/10 border-red-400/30 text-red-200">
          <p className="text-[13px] font-medium">기하 계산 실패</p>
          <p className="text-xs opacity-80 whitespace-pre-wrap break-words">{error}</p>
        </div>
      </div>
    );
  }
  if (!derived) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-text-canvas text-sm">
          <code className="font-mono">bb_compute_geometry</code> 계산 중…
        </p>
      </div>
    );
  }

  const f = buildFrame(g, derived);

  // 상세 패널의 잘라내기 반폭 — A 기준. 과장된 변위가 이 안에 들어오도록 배율을 정한다.

  // ── 검산: 그림이 쓰는 값과 솔버 값이 같은가 (Theory §4.4 확정형 재계산) ──
  const xJ = f.aMm * Math.sin(f.alpha0) + (ov?.dX ?? 0);
  const rJ = f.aMm * Math.cos(f.alpha0) + (ov?.dR ?? 0);
  const alphaRecomputed = Math.atan2(xJ, rJ);
  const deltaRecomputed = Math.max(0, Math.hypot(xJ, rJ) - f.aMm);
  const alphaRelDiff = ball && ball.alpha_rad !== 0 ? (alphaRecomputed - ball.alpha_rad) / ball.alpha_rad : null;
  const deltaRelDiff = ball && ball.delta_mm !== 0 ? (deltaRecomputed - ball.delta_mm) / ball.delta_mm : null;
  const aIdentity = f.ri + f.re - f.ballR * 2;
  const riIdentity = f.rPitch + (f.ri - f.ballR) * Math.cos(f.alpha0);

  const stale = result !== null && resultInput !== null && resultInput !== bbInput;

  return (
    <div className="h-full overflow-auto custom-scrollbar p-4 space-y-5">
      {/* ── 검증 임무 명시 ─────────────────────────────────────────── */}
      <div className="p-2.5 rounded border bg-blue-500/10 border-blue-400/30 text-blue-100 text-[12px] leading-relaxed">
        <p className="font-semibold text-[13px] mb-1">
          축단면 (ISO 16281 Fig A.1 a) 스타일 전단면) — Theory §2 (A.1)(A.3)(A.4) · §4.4 확정형
        </p>
        <p>
          가로축이 <span className="font-mono">X</span>(회전축), 세로축이 반경이며 <b>중심선을 축으로 상·하 전단면</b>을
          그린다. 궤도는 <b>홈반경 원호</b>(<span className="font-mono">r_i</span>·
          <span className="font-mono">r_e</span>)로 그리며 폴리곤이 아니다. 치수는 <b>아래쪽 반단면</b>에만 단다.
        </p>
        <p className="mt-1">
          <b>각도는 과장하지 않는다.</b> <span className="font-mono">α₀</span>(하늘)와{' '}
          <span className="font-mono">α_j</span>(주황 파선)는 <b>참값 각도</b>로 겹쳐 그린다. 두 선이 포개져 보이면{' '}
          <b>하중이 걸리지 않은 것</b>이다(§3.6.4.2 의 징후). 1° 남짓한 차이가 보이도록{' '}
          <b>접촉선을 OD 근처까지 길게</b> 뽑았다 — 길이는 표시용이고 <b>각도만이 이 선의 정보</b>다.
        </p>
        <p className="mt-1 text-blue-200/80">
          단면은 <b>선택한 볼의 방위 φ_j 에서 자른 것</b>이다. 볼마다{' '}
          <span className="font-mono">α_j</span> 가 다르므로 어느 볼인지가 항상 표시된다. 곡률중심{' '}
          <span className="font-mono">O_i</span>·<span className="font-mono">O_e</span> 는 볼 지름 대비 간격이 1 % 규모라
          실척에서 점이 겹치므로 <b>그리지 않고</b>, <span className="font-mono">A = r_i + r_e − D_w</span> 를{' '}
          <b>접촉선 위의 치수</b>로만 낸다(ISO 원도면과 같다). 수치 검산은 아래 <b>기하 항등</b> 표가 맡는다.
        </p>
      </div>

      {stale && (
        <div className="p-2 rounded border bg-amber-500/10 border-amber-400/30 text-amber-200 text-[12px]">
          입력이 Solve 이후 변경되었다. <b>단면 형상은 현재 입력</b>(<span className="font-mono">bbInput</span>)
          기준이고, <b>하중 후 오버레이(α_j·δ·틸트)는 Solve 시점 스냅샷</b>
          (<span className="font-mono">resultInput</span>) 기준이다. 두 시점이 어긋난 상태이므로 다시 Solve 할 것.
        </div>
      )}

      {/* ── 볼 선택 ────────────────────────────────────────────────── */}
      <div className="flex flex-wrap items-center gap-3">
        <label className="text-[12px] text-text-canvas">
          단면 위치 (볼 선택)
          <select
            value={sel}
            onChange={e => setSelected(Number(e.target.value))}
            className="ml-2 bg-canvas-subtle border border-white/15 rounded px-2 py-1 text-[12px] font-mono text-text-light"
          >
            {balls.length > 0
              ? balls.map((b, i) => (
                  <option key={i} value={i}>
                    #{i + 1} · φ = {toDeg(b.phi_rad).toFixed(2)}° · Q = {b.q_n.toPrecision(6)} N
                    {b.loaded ? '' : ' · 비접촉'}
                    {i === top ? '  ← 최대하중' : ''}
                  </option>
                ))
              : Array.from({ length: zCount }, (_, i) => (
                  <option key={i} value={i}>
                    #{i + 1} · φ = {((360 * i) / zCount).toFixed(2)}° (결과 없음)
                  </option>
                ))}
          </select>
        </label>
        {top >= 0 && (
          <button
            onClick={() => setSelected(top)}
            className="px-2 py-1 rounded text-[12px] border border-white/15 text-text-canvas hover:text-text-light hover:bg-white/5 cursor-pointer"
            title="기본값 — 최대하중 볼"
          >
            최대하중 볼로
          </button>
        )}
        <span className="text-[12px] text-text-canvas/70 font-mono">
          φ_j = {num(toDeg(phiRad))}° ({num(phiRad)} rad) · φ = 0 은 +Y 축 (D-8) = 화면 아래 (§3.6.4.10)
        </span>
        {!result && (
          <span className="text-[12px] text-amber-300/80">
            결과 없음 — <b>무하중 단면</b>만 그린다 (기하는 Solve 없이도 보인다).
          </span>
        )}
      </div>

      {/* ── ISO 스타일 전단면 (단일 패널) ─────────────────────────── */}
      <div className="rounded border border-white/10 bg-black/20 p-2">
        <p className="text-[11px] text-text-canvas/60 mb-1">
          <b>실척 전단면</b>. 링 외곽은 <span className="font-mono">d</span>·<span className="font-mono">D</span>·
          <span className="font-mono">B</span> 만으로 그린 포락선이다(숄더 치수는 입력에 없어 그리지 않는다).{' '}
          <b>모든 문자는 도형 바깥 여백에만</b> 두고 지시선으로 연결한다 — 선·글자가 겹칠 수 있는 자리에 아예
          두지 않는 방식이다.
        </p>
        <IsoSectionPanel f={f} ov={ov} phiRad={phiRad} />
      </div>

      {/* ── D-2d ───────────────────────────────────────────────────── */}
      <div className="rounded border border-white/10 bg-black/20 p-3">
        <p className="text-[13px] font-semibold text-text-light mb-1">
          D-2d — 틸트축이 모멘트와 맞는가 (§3.6.4.2 의 유일한 미커버 항목)
        </p>
        {!result || !op ? (
          <p className="text-[12px] text-text-canvas/70">Solve 를 눌러야 판정할 수 있다.</p>
        ) : (
          <div className="flex flex-wrap items-start gap-4">
            <div className="shrink-0">
              <TiltCompass tiltDeg={tiltDeg} momDeg={momDeg} phiDeg={toDeg(phiRad)} />
              <div className="text-[10px] leading-snug space-y-0.5 mt-0.5 w-[124px]">
                <p style={{ color: C_LOAD_DIR }}>━ 모멘트 방위 (외부, resultInput)</p>
                <p style={{ color: C_ALPHA }}>━ 틸트 방위 (평형해)</p>
                <p style={{ color: C_LOADED }}>┄ 선택한 볼 φ_j</p>
                <p className="text-text-canvas/60">0° = +Y = 아래, 우측(+Z) 증가</p>
              </div>
            </div>
            <div className="min-w-[22rem] flex-1">
              <DetailTable
                title="방위 대조 — (Y, Z) 평면 · 방위 0° = +Y = 화면 아래, 우측(+Z) 증가 (§3.6.4.10)"
                rows={[
                  ['틸트 γ_y', num(disp?.ry_rad ?? 0), 'rad'],
                  ['틸트 γ_z', num(disp?.rz_rad ?? 0), 'rad'],
                  ['틸트 크기 |γ|', num(tiltMag), 'rad'],
                  [
                    '틸트 방위 atan2(γ_z, γ_y)  〔0°=+Y=아래〕',
                    tiltDeg === null ? '정의 없음 (|γ| = 0)' : num(tiltDeg),
                    '°',
                  ],
                  ['모멘트 M_y (resultInput)', num(op.m_y_nmm), 'N·mm'],
                  ['모멘트 M_z (resultInput)', num(op.m_z_nmm), 'N·mm'],
                  [
                    '모멘트 방위 atan2(M_z, M_y)  〔0°=+Y=아래〕',
                    momDeg === null ? '정의 없음 (|M| = 0)' : num(momDeg),
                    '°',
                  ],
                  ['Δ = 틸트 − 모멘트 (규약 무관 — 차이값)', deltaDeg === null ? '—' : num(deltaDeg), '°'],
                  ['반경하중 |F_r| (등방성 판정용)', num(radMag), 'N'],
                  ['볼 수 Z (이산성 — Δ 허용폭 근거)', String(zCount), '개'],
                ]}
              />
              <div className="mt-2 text-[12px] leading-relaxed">
                {deltaDeg === null ? (
                  <p className="text-text-canvas/70">
                    모멘트 또는 틸트가 0 이라 방위가 정의되지 않는다 — 이 조건에서는 D-2d 를 판정할 수 없다.
                  </p>
                ) : pureMoment ? (
                  Math.abs(deltaDeg) < 1 ? (
                    <p className="text-emerald-300">
                      ✅ <b>정합</b> — 순수 모멘트 하중에서 틸트 방위가 모멘트 방위와 같다 (|Δ| ={' '}
                      <span className="font-mono">{num(Math.abs(deltaDeg))}</span>° &lt; 1°). Theory §4.4 확정형에서{' '}
                      <span className="font-mono">M_y &gt; 0 ⟹ γ_y &gt; 0</span> ·{' '}
                      <span className="font-mono">M_z &gt; 0 ⟹ γ_z &gt; 0</span> 이 유도되며, 화면이 그것을 재확인한다.
                    </p>
                  ) : Math.abs(Math.abs(deltaDeg) - 180) < 15 ? (
                    <p className="text-red-300">
                      ❌ <b>Δ ≈ 180°</b> (|Δ| = <span className="font-mono">{num(Math.abs(deltaDeg))}</span>°) — 틸트가
                      모멘트와 <b>반대로 섰다</b>. §3.6.4.2 가 드는 징후「틸트 방향이 모멘트와 반대」에 해당한다. 부호
                      규약(Theory §4.4 의 <span className="font-mono">M_y = +R_i Σ Q sin α sin φ</span>) 을 먼저 확인할 것.
                    </p>
                  ) : Math.abs(deltaDeg) < 90 ? (
                    <p className="text-emerald-300/90">
                      🟡 <b>부호는 정합</b> (|Δ| = <span className="font-mono">{num(Math.abs(deltaDeg))}</span>° &lt; 90°)
                      이나 <b>정확히 0 은 아니다</b>. 볼이 <b>Z = {zCount} 개로 이산</b>이라 응답이 연속 등방이 아니고{' '}
                      <b>Z-중 대칭</b>만 갖는다 — 모멘트 방위가 대칭축과 어긋나면 Δ 가 정확히 0 이 되지 <b>않는</b> 것이
                      정상이다. <b>이 편차만으로 솔버를 의심하지 말 것.</b> 엄밀 대조가 필요하면 모멘트 방위를 볼 위치(φ_j
                      = 360°·k/Z)에 맞추고 다시 볼 것.
                    </p>
                  ) : (
                    <p className="text-amber-300">
                      ⚠ 순수 모멘트인데 |Δ| 가 <b>90° 이상 180° 미만</b>이다 (|Δ| ={' '}
                      <span className="font-mono">{num(Math.abs(deltaDeg))}</span>°). 이산성(Z = {zCount})으로 설명되는
                      규모가 아니다. <b>화면을 고쳐 맞추지 말고 솔버 쪽을 먼저 의심할 것.</b>
                    </p>
                  )
                ) : (
                  <p className="text-text-canvas/70">
                    반경하중이 함께 걸려 있어 응답이 <b>등방이 아니다</b> — 이 조건에서는 Δ = 0 이{' '}
                    <b>요구되지 않는다.</b> D-2d 판정은 <span className="font-mono">F_y = F_z = 0</span> 인{' '}
                    <b>순수 모멘트 조건</b>에서 하라. (지금 Δ ={' '}
                    <span className="font-mono">{num(deltaDeg)}</span>°)
                  </p>
                )}
              </div>
            </div>
          </div>
        )}
      </div>

      {/* ── 검산표 ─────────────────────────────────────────────────── */}
      <DetailTable
        title="기하 항등 — 그림이 쓰는 값 ↔ 솔버 값 (Theory §2)"
        rows={[
          ['A (솔버, a_mm)', num(f.aMm), 'mm'],
          ['A 재계산 = r_i + r_e − D_w  (A.3)', num(aIdentity), 'mm'],
          ['A 상대차', num(f.aMm !== 0 ? (aIdentity - f.aMm) / f.aMm : 0), '—'],
          ['α₀ (솔버, alpha_0_rad)', num(toDeg(f.alpha0)), '°'],
          ['α₀ (rad)', num(f.alpha0), 'rad'],
          ['R_i (솔버, r_i_center_mm)', num(derived.r_i_center_mm), 'mm'],
          ['R_i = 그림의 O_i 반경  (A.4)', num(f.riCenterCheck), 'mm'],
          ['R_i 재계산 = D_pw/2 + (r_i − D_w/2) cos α₀', num(riIdentity), 'mm'],
          [
            'R_i 상대차 (그림 ↔ 솔버)',
            num(derived.r_i_center_mm !== 0 ? (f.riCenterCheck - derived.r_i_center_mm) / derived.r_i_center_mm : 0),
            '—',
          ],
          ['O_i 축위치', num(f.Oi[0]), 'mm'],
          ['O_e 축위치', num(f.Oe[0]), 'mm'],
          ['O_e 반경 = D_pw/2 − (r_e − D_w/2) cos α₀', num(f.Oe[1]), 'mm'],
          ['내륜 접촉점 P_i (x, r)', `${num(f.Pi[0])}, ${num(f.Pi[1])}`, 'mm'],
          ['외륜 접촉점 P_e (x, r)', `${num(f.Pe[0])}, ${num(f.Pe[1])}`, 'mm'],
        ]}
      />

      {ov && ball && (
        <DetailTable
          title={`하중 후 — 볼 #${sel + 1} (φ_j = ${num(toDeg(phiRad))}°) · Theory §4.4 확정형 재계산`}
          rows={[
            ['접촉 여부', ball.loaded ? '접촉' : '비접촉', '—'],
            ['Q_j 볼 하중', num(ball.q_n), 'N'],
            ['δ_x (Displacement.dx_mm)', num(ov.dxOnly), 'mm'],
            ['틸트항 −R_i(γ_z cos φ_j − γ_y sin φ_j)', num(ov.tiltTerm), 'mm'],
            ['ΔX = δ_x + 틸트항', num(ov.dX), 'mm'],
            ['ΔR = δ_y cos φ_j + δ_z sin φ_j', num(ov.dR), 'mm'],
            ['X_j = A sin α₀ + ΔX', num(xJ), 'mm'],
            ['R_j = A cos α₀ + ΔR', num(rJ), 'mm'],
            ['α_j (솔버, alpha_rad)', num(toDeg(ball.alpha_rad)), '°'],
            ['α_j 재계산 = atan2(X_j, R_j)', num(toDeg(alphaRecomputed)), '°'],
            ['α_j 상대차', alphaRelDiff === null ? '—' : num(alphaRelDiff), '—'],
            ['α_j − α₀ (벌어진 양)', num(toDeg(ball.alpha_rad - f.alpha0)), '°'],
            ['δ_j (솔버, delta_mm)', num(ball.delta_mm), 'mm'],
            ['δ_j 재계산 = max(0, √(X_j²+R_j²) − A)', num(deltaRecomputed), 'mm'],
            ['δ_j 상대차', deltaRelDiff === null ? '—' : num(deltaRelDiff), '—'],
          ]}
        />
      )}

      <p className="text-[11px] text-text-canvas/50 leading-relaxed">
        재사용: <span className="font-mono">components/SectionView2D</span> 의 순수 유틸(
        <span className="font-mono">Txt</span>·<span className="font-mono">WidthDim</span>·
        <span className="font-mono">AngleArc</span>·<span className="font-mono">viewBox</span> 프레임)을 복사했다(원본
        미수정). <span className="font-mono">AngleArc</span> 는 시작각 인자를 받도록 일반화했다 — 접촉각이{' '}
        <b>수직 기준</b>이라 그대로 쓰면 보각(90°−α)을 그려 <b>조용히 틀린 그림</b>이 되기 때문이다. P4-2c 에서{' '}
        <span className="font-mono">layoutGutter</span>·<span className="font-mono">GutterLabel</span>·
        <span className="font-mono">VerticalDim</span>·<span className="font-mono">HatchDefs</span>·
        <span className="font-mono">AxisMarker</span> 를 신설했다. 형상 계산부(γ 경사축·사다리꼴 롤러·리브)는
        복사하지 않았다.
      </p>
    </div>
  );
}
