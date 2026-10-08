// BB 프론트 공용 로직·상수 (Plan P4-2d · 글로벌 CLAUDE.md §0.2 상수 SSOT)
//
// 7개 뷰에 복사돼 있던 `num`·`toDeg`·색 상수와 3개 뷰에 복사돼 있던 기하 커맨드 호출을
// 한 곳으로 모았다. **값은 그대로 옮겼다** — 이 파일로 옮기면서 화면이 바뀌면 안 된다.
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

/**
 * 유효숫자 9자리 (또는 지수표기).
 *
 * 화면 값은 **검증 대조용**이다 — Theory·문헌 표와 자릿수까지 맞춰 보므로 줄이면 안 된다.
 */
export function num(v: number): string {
  if (!Number.isFinite(v)) return String(v);
  if (v === 0) return '0';
  const a = Math.abs(v);
  if (a < 1e-4 || a >= 1e9) return v.toExponential(8);
  return v.toPrecision(9);
}

/** rad → °. 솔버는 rad 로만 낸다(D-10) — 변환은 화면 경계에서만 한다. */
export const toDeg = (rad: number) => (rad * 180) / Math.PI;

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
