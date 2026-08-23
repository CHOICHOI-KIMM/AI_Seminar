import { useAppState } from '../store';
import type { BearingResult } from '../types/bearing';

/**
 * Returns the active BearingResult based on dual view mode toggle.
 *
 * ⚠ P4-S1-4 (Plan §3.6.4.3): `store.result` 는 이제 **`BbResult`** 다.
 *   이 훅을 쓰는 곳은 전부 **미개조 TRB 잔존 뷰**(회색 표시 8탭 + ResultsCard)이며,
 *   그것들은 이미 죽은 커맨드에 묶여 동작하지 않는다. 최소 변경 방침상 지금
 *   지우지 않으므로, 타입 경계만 캐스트로 표시해 둔다.
 *   개조된 BB 뷰는 이 훅을 쓰지 않고 `store.result` 를 직접 읽는다.
 *   `useActiveResult` 자체의 제거는 §3.6.4.6 일괄 정리 시점이다.
 */
export function useActiveResult(): BearingResult | null {
  const { state } = useAppState();
  if (!state.dualResult) return state.result as unknown as BearingResult | null;
  return state.dualViewMode === 'gen3'
    ? state.dualResult.gen3_result
    : state.dualResult.gen1_result;
}
