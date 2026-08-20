import { useAppState } from '../store';
import type { BearingResult } from '../types/bearing';

/** Returns the active BearingResult based on dual view mode toggle. */
export function useActiveResult(): BearingResult | null {
  const { state } = useAppState();
  if (!state.dualResult) return state.result;
  return state.dualViewMode === 'gen3'
    ? state.dualResult.gen3_result
    : state.dualResult.gen1_result;
}
