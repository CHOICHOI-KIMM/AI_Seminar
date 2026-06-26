import { createContext, useContext } from 'react';
import type { BearingInput, BearingResult, DualModeComparison, TransientResult } from './types/bearing';

export type CanvasTab = 'geometry' | 'profile' | 'section' | '3d' | 'load' | 'contour' | 'lubrication' | 'life' | 'iso15312' | 'comparison' | 'transient';
export type DualViewMode = 'gen1' | 'gen3';

export interface SolverProgress {
  stage: string;
  detail: string;
  percent: number;
}

export interface AppState {
  input: BearingInput;
  result: BearingResult | null;
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
  | { type: 'SET_RESULT'; payload: BearingResult }
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
    case 'SET_RESULT':
      return { ...state, result: action.payload, dualResult: null, error: null };
    case 'SET_DUAL_RESULT':
      return { ...state, dualResult: action.payload, result: action.payload.gen1_result, error: null };
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
      return { ...state, result: null, dualResult: null, transientResult: null, error: null };
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
