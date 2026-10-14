// BB 프론트 공용 로직·상수 (Plan P4-2d · 글로벌 CLAUDE.md §0.2 상수 SSOT)
//
// 7개 뷰에 복사돼 있던 `num`·`toDeg`·색 상수와 3개 뷰에 복사돼 있던 기하 커맨드 호출을
// 한 곳으로 모았다. **값은 그대로 옮겼다** — 이 파일로 옮기면서 화면이 바뀌면 안 된다.
// P4-5 에서 유효숫자 9자리 `num` 은 아래 「단위별 표시」 함수로 대체되어 삭제됐다.
//
// ⚠ 색 상수는 **데이터 계열색**이다. `index.css` 의 상태 토큰(`--color-warning` 등)과
//    값이 같은 것이 있어도(`C_LOADED` ≡ warning, `C_INNER` ≡ accent) **합치지 않는다**.
//    경고색을 바꿨는데 하중 그래프 색이 바뀌는 결합을 막기 위해서다.
//    Tailwind 클래스를 쓸 수 있는 곳은 토큰 클래스를, 클래스를 못 쓰는 SVG·Plotly 는 이 상수를 쓴다.

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { Alert } from './generated/Alert';
import type { BbInput } from './generated/BbInput';
import type { BbGeometryDerived } from './generated/BbGeometryDerived';
import type { BbGeometrySummary } from './generated/BbGeometrySummary';

// ─── 숫자 표시 ─────────────────────────────────────────────────────────

/** rad → °. 솔버는 rad 로만 낸다(D-10) — 변환은 화면 경계에서만 한다. */
export const toDeg = (rad: number) => (rad * 180) / Math.PI;

// ─── 단위별 표시 (Plan Phase 4-5 §4-5.3) ──────────────────────────────
//
//  TRB 식 단위별 고정 소수. 물리량마다 자릿수가 하나다(요약·상세 구분 없음).
//  입력은 **솔버 단위**(mm · N · rad · N·mm)로 받고, 화면 단위로의 환산은 여기서만 한다(D-10).
//  TRB 는 표시하는 곳마다 toFixed 를 직접 써서 같은 물리량이 위치마다 달랐다 — BB 는 이 함수들만 쓴다.

/**
 * 고정 소수. 0 이 아닌데 반올림하면 0 이 되는 값만 지수(유효숫자 3자리)로 보인다.
 * 「정확히 0」과 「아주 작은 값」(예: D-2a 의 δ_z ≈ 1e-17)을 계속 구별하기 위해서다 (결정 6).
 */
function fixed(v: number, digits: number): string {
  if (!Number.isFinite(v)) return String(v);
  const s = v.toFixed(digits);
  if (v !== 0 && Number(s) === 0) return v.toExponential(2);
  return s;
}

/** 응력·접촉압력 [MPa] → MPa 정수 */
export const fmtMPa = (mpa: number) => fixed(mpa, 0);
/** 탄성계수 [MPa] → GPa 소수 2 (보충, TRB E* 표시) — 응력이 아니므로 fmtMPa 를 쓰지 않는다 */
export const fmtGPa = (mpa: number) => fixed(mpa / 1000, 2);
/** 볼 하중·외부 하중 [N] → N 소수 1 */
export const fmtN = (n: number) => fixed(n, 1);
/** 정격하중 C_r·C_0r·C_u·P [N] → kN 소수 2 */
export const fmtKN = (n: number) => fixed(n / 1000, 2);
/** 모멘트 [N·mm] → N·m 소수 2 */
export const fmtNm = (nmm: number) => fixed(nmm / 1000, 2);
/** 변위·접근량 [mm] → μm 소수 2 */
export const fmtUm = (mm: number) => fixed(mm * 1000, 2);
/** 틸트 [rad] → μrad 소수 2 */
export const fmtUrad = (rad: number) => fixed(rad * 1e6, 2);
/** 각도 [°] → ° 소수 2. rad 는 호출부에서 toDeg 로 바꿔 넘긴다 */
export const fmtDeg = (deg: number) => fixed(deg, 2);
/** 기하 치수 [mm] → mm 소수 3 */
export const fmtMm = (mm: number) => fixed(mm, 3);
/** 접촉타원 반축 [mm] → mm 소수 4 */
export const fmtMmFine = (mm: number) => fixed(mm, 4);
/** 면적 [mm²] → mm² 소수 4 (보충) */
export const fmtMm2 = (mm2: number) => fixed(mm2, 4);
/** 수명 [10⁶ rev] → 소수 2 */
export const fmtMrev = (mrev: number) => fixed(mrev, 2);
/** 수명 [h] → 정수, 10 000 h 초과면 k (TRB formatHours) */
export function fmtHours(h: number): string {
  if (!Number.isFinite(h) || h > 1e9) return '∞';
  if (h > 10000) return `${(h / 1000).toFixed(1)}k`;
  return h.toFixed(0);
}
/** 무차원 계수·곡률 [1/mm] → 소수 4 */
export const fmtDimless = (v: number) => fixed(v, 4);
/** 회전속도 [rpm]·n·D_pw [mm/min]·개수 → 정수 */
export const fmtInt = (v: number) => fixed(v, 0);
/** 동점도 [mm²/s] → 소수 2 (보충) */
export const fmtVisc = (v: number) => fixed(v, 2);
/** 질량 등 [g] → 소수 1 (보충) */
export const fmtMass = (g: number) => fixed(g, 1);
/** 하중-변위 상수 등 강성 → k·M 접미사 소수 1 (보충, TRB formatStiffness) */
export function fmtStiff(v: number): string {
  const a = Math.abs(v);
  if (a >= 1e6) return `${(v / 1e6).toFixed(1)}M`;
  if (a >= 1e3) return `${(v / 1e3).toFixed(1)}k`;
  return fixed(v, 0);
}
/** 소요시간 [ms] → 단위 포함 문자열 (보충, TRB formatTime). 단위 칸은 비운다 */
export function fmtTime(ms: number): string {
  if (ms < 1) return `${(ms * 1000).toFixed(0)} μs`;
  if (ms < 1000) return `${ms.toFixed(0)} ms`;
  return `${(ms / 1000).toFixed(2)} s`;
}
/** 원래 아주 작은 값이 의미인 항목(수렴 잔차·항등 상대차) → 지수, 유효숫자 3자리 */
export const fmtExp = (v: number) => (v === 0 ? '0' : v.toExponential(2));

// ─── 데이터 계열색 ─────────────────────────────────────────────────────

/** 하중을 받는 볼 */
export const C_LOADED = '#f59e0b';
/** 하중을 받지 않는 볼 */
export const C_UNLOADED = '#64748b';
/** 외부 반경하중 방위 (Load Distribution 의 기준선, 축단면 나침반의 모멘트 방위) */
export const C_LOAD_DIR = '#ef4444';
/** 접촉각 `α` 선 · 축단면 나침반의 틸트 방위 */
export const C_ALPHA = '#38bdf8';
/** 기준선 (피치원 · `L_10r` 등 기준값) */
export const C_REF = '#a3e635';
/** 내륜 */
export const C_INNER = '#3b82f6';
/** 외륜 */
export const C_OUTER = '#f97316';
/** 3D — 외부 하중 화살표 */
export const C_FORCE = '#ef4444';
/** 3D — 외부 모멘트 화살표 */
export const C_MOMENT = '#a3e635';
/** 3D — 틸트축 (γ_y, γ_z) */
export const C_TILT = '#38bdf8';
/** 3D — 내륜 실루엣 */
export const C_RING_INNER = '#94a3b8';
/** 3D — 외륜 실루엣 */
export const C_RING_OUTER = '#64748b';
/** 응력 등고선 — 경도 한계 `σ_HU` 경계 */
export const C_HU = '#ef4444';

// ─── 기하 커맨드 ───────────────────────────────────────────────────────

/** `bb_compute_geometry` 반환형 (`commands.rs` 의 `GeometryResponse` 와 같은 모양). */
export interface GeometryResponse {
  derived: BbGeometryDerived;
  summary: BbGeometrySummary;
  alerts: Alert[];
}

/**
 * 입력이 바뀔 때마다 `bb_compute_geometry` 를 부른다 — **150 ms 디바운스 + `cancelled` 플래그**.
 *
 * - 타자 한 글자마다 커맨드를 때리지 않도록 짧게 모은다.
 * - 늦게 도착한 이전 응답이 새 응답을 덮어쓰지 않도록 `cancelled` 로 버린다.
 * - ⚠ 구독 대상은 `input` **전체**다. `geometry` 만 구독하면 낡은 값이 남는다.
 * - 오류는 Rust `validate()` 메시지를 **그대로** 돌려준다 — 프론트가 문구를 고쳐 쓰면
 *   어느 쪽이 거부했는지 알 수 없게 된다 (§3.6.5.3).
 * - `input` 이 없으면 아무 것도 하지 않는다. effect 안에서 동기 setState 를 하면 연쇄 렌더가
 *   되고 `react-hooks/set-state-in-effect` 에도 걸리므로, 호출하는 쪽이 `!input` 을 조기반환한다.
 */
export function useGeometryResponse(input: BbInput | null): {
  response: GeometryResponse | null;
  error: string | null;
} {
  const [response, setResponse] = useState<GeometryResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!input) return;
    let cancelled = false;
    const timer = setTimeout(() => {
      invoke<GeometryResponse>('bb_compute_geometry', { input })
        .then(r => {
          if (cancelled) return;
          setResponse(r);
          setError(null);
        })
        .catch((e: unknown) => {
          if (cancelled) return;
          setError(String(e));
          setResponse(null);
        });
    }, 150);
    return () => {
      cancelled = true;
      clearTimeout(timer);
    };
  }, [input]);

  return { response, error };
}
