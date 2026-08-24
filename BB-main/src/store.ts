import { createContext, useContext } from 'react';
import type { BearingInput, DualModeComparison, TransientResult } from './types/bearing';
// P4-S1-4: 해석 결과 타입을 Rust(solver/bb/types.rs) 자동생성본으로 교체 (Plan §3.6.5.5).
// dualResult·transientResult 는 최소 변경 방침(§3.6.4.3)에 따라 TRB 타입 그대로 둔다.
import type { BbResult } from './bb/generated/BbResult';
// P4-S3-1: BB 입력을 store 로 승격 (Plan §3.6.5.2 S3 · S2 설계선택 #1 의 예고대로).
// ⚠ 기존 `input: BearingInput`(TRB) 은 **그대로 둔다** — 타입을 바꾸면 `@ts-nocheck`
//    가 아닌 `ThermalSpeedView`·`project.ts` 가 깨져 최소 변경 방침(§3.6.4.3)에 어긋난다.
//    두 필드는 당분간 공존하고, 일괄 정리는 §3.6.4.6 시점에 한다.
import type { BbInput } from './bb/generated/BbInput';

export type CanvasTab = 'geometry' | 'profile' | 'section' | '3d' | 'load' | 'contour' | 'lubrication' | 'life' | 'iso15312' | 'comparison' | 'transient';
export type DualViewMode = 'gen1' | 'gen3';

export interface SolverProgress {
  stage: string;
  detail: string;
  percent: number;
}

export interface AppState {
  input: BearingInput;
  /**
   * BB 입력 (Rust `solver/bb/types.rs` 대응 자동생성 타입).
   *
   * `null` 인 이유 — 기본값을 TS 로 다시 적지 않는다. Rust `presets.rs` 가 유일한
   * 출처이며(S2 확정), 프리셋이 로드되기 전까지는 값이 존재하지 않는다.
   */
  bbInput: BbInput | null;
  result: BbResult | null;
  /**
   * `result` 를 만들어낸 **Solve 시점의 입력 스냅샷**.
   *
   * `bbInput` 은 사용자가 계속 편집하므로 `result` 와 시점이 어긋난다.
   * 결과와 짝이 맞는 입력을 읽어야 하는 뷰(예: `BbLoadDistView` 의 `F_r` 방위
   * 기준선)는 `bbInput` 이 아니라 **이것**을 써야 한다. `SET_RESULT` 가 채운다.
   */
  resultInput: BbInput | null;
  dualResult: DualModeComparison | null;
  transientResult: TransientResult | null;
  dualViewMode: DualViewMode;
  loading: boolean;
  progress: SolverProgress | null;
  error: string | null;
  activeTab: CanvasTab;
  resultsPanelOpen: boolean;
}

export type AppAction =
  | { type: 'SET_INPUT'; payload: BearingInput }
  | { type: 'UPDATE_INPUT'; payload: Partial<BearingInput> }
  | { type: 'SET_BB_INPUT'; payload: BbInput }
  | { type: 'UPDATE_BB_INPUT'; payload: Partial<BbInput> }
  | { type: 'SET_RESULT'; payload: BbResult }
  | { type: 'SET_DUAL_RESULT'; payload: DualModeComparison }
  | { type: 'SET_LOADING'; payload: boolean }
  | { type: 'SET_PROGRESS'; payload: SolverProgress | null }
  | { type: 'SET_ERROR'; payload: string | null }
  | { type: 'SET_TAB'; payload: CanvasTab }
  | { type: 'SET_DUAL_VIEW_MODE'; payload: DualViewMode }
  | { type: 'SET_TRANSIENT_RESULT'; payload: TransientResult }
  | { type: 'CLEAR_RESULTS' }
  | { type: 'TOGGLE_RESULTS_PANEL' };

export function appReducer(state: AppState, action: AppAction): AppState {
  switch (action.type) {
    case 'SET_INPUT':
      return { ...state, input: action.payload };
    case 'UPDATE_INPUT':
      return { ...state, input: { ...state.input, ...action.payload } };
    case 'SET_BB_INPUT':
      return { ...state, bbInput: action.payload };
    case 'UPDATE_BB_INPUT':
      // 아직 프리셋이 안 들어왔으면(=null) 부분 갱신은 의미가 없으므로 무시한다.
      return state.bbInput ? { ...state, bbInput: { ...state.bbInput, ...action.payload } } : state;
    case 'SET_RESULT':
      // 결과와 짝이 맞는 입력을 함께 고정한다 (`resultInput` 주석 참조).
      return { ...state, result: action.payload, resultInput: state.bbInput, dualResult: null, error: null };
    case 'SET_DUAL_RESULT':
      // ⚠ `solve_bearing_dual` 은 이미 죽은 커맨드라 이 경로는 실행되지 않는다.
      // 최소 변경 방침상 dual 계열을 지금 제거하지 않으므로, TRB 결과를
      // BbResult 슬롯에 넣는 부분만 캐스트로 표시해 둔다 (§3.6.4.6 에서 일괄 정리).
      return { ...state, dualResult: action.payload, result: action.payload.gen1_result as unknown as BbResult, error: null };
    case 'SET_LOADING':
      return { ...state, loading: action.payload, progress: action.payload ? state.progress : null };
    case 'SET_PROGRESS':
      return { ...state, progress: action.payload };
    case 'SET_ERROR':
      return { ...state, error: action.payload, loading: false };
    case 'SET_TAB':
      return { ...state, activeTab: action.payload };
    case 'SET_TRANSIENT_RESULT':
      return { ...state, transientResult: action.payload, error: null };
    case 'SET_DUAL_VIEW_MODE':
      return { ...state, dualViewMode: action.payload };
    case 'CLEAR_RESULTS':
      return { ...state, result: null, resultInput: null, dualResult: null, transientResult: null, error: null };
    case 'TOGGLE_RESULTS_PANEL':
      return { ...state, resultsPanelOpen: !state.resultsPanelOpen };
    default:
      return state;
  }
}

export const AppContext = createContext<{
  state: AppState;
  dispatch: React.Dispatch<AppAction>;
} | null>(null);

export function useAppState() {
  const ctx = useContext(AppContext);
  if (!ctx) throw new Error('useAppState must be used within AppContext');
  return ctx;
}
