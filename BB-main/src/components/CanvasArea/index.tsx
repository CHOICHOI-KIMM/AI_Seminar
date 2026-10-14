import { useAppState, type CanvasTab } from '../../store';
import ComparisonChart from '../charts/ComparisonChart';
import ProfileView from '../ProfileView';
import DualModeToggle from '../DualModeToggle';
import TransientView from '../TransientView';
import ThermalSpeedView from '../ThermalSpeedView';

// P4-S1-4 (Plan §3.6.4.3 최소 변경 방침): TRB 잔존 탭(Profile·Thermal Speed·Comparison·Transient)은
//   지우지 않고 그대로 둔다 — 검증 중에는 변경이 적을수록 원인 분리가 쉽다. 일괄 정리는 §3.6.4.6 시점.
//   P4-5 (Plan §4-5.7, 2026-10-14 사용자 결정): 「TRB 잔존」 배지는 지우고, 잔존 5탭은
//   **회색 + 클릭 불가(disabled)** 로 둔다. Lubrication·Thermal Speed 는 BB 결과에서 누르면 앱 전체가
//   빈 화면이 됐다(실측) — 클릭을 막아 그 경로를 없앤다. 탭 자체는 남겨 개조할 목록이 보이게 한다.
//   S3 에서 geometry, S4 에서 load, **S5 에서 contour** 가 BB 뷰로 채워졌다 (셋 다 prop 주입).
//   P4-3a 에서 **section** 이 `bb/BbAxialSectionView` 로 채워졌다 (같은 prop 주입).
//   P4-3b 에서 **3d**, P5-2 에서 **life** 가 같은 방식으로 채워졌다.
//   → 이로써 Geometry·Section·3D·Load·Contour·Life 6탭이 **전부** BB 뷰다.
const tabs: { key: CanvasTab; label: string; disabled?: boolean }[] = [
  { key: 'geometry', label: 'Geometry' },
  { key: 'profile', label: 'Profile', disabled: true },
  { key: 'section', label: 'Section' },
  { key: '3d', label: '3D View' },
  { key: 'load', label: 'Load Distribution' },
  { key: 'contour', label: 'Stress Contour' },
  { key: 'lubrication', label: 'Lubrication', disabled: true },
  { key: 'life', label: 'Life' },
  { key: 'iso15312', label: 'Thermal Speed', disabled: true },
  { key: 'comparison', label: 'Comparison', disabled: true },
  { key: 'transient', label: 'Transient', disabled: true },
];

/**
 * BB 뷰는 **prop 으로 주입받는다** — `App.tsx`(경계 밖)가 넘긴다.
 *
 * ⚠ 여기서 `src/bb/**` 를 직접 import 하면 ESLint 경계 규칙(§3.6.5.6,
 *   `components/**` → `bb/**` 금지)에 걸린다. 그 규칙의 목적은
 *   「TRB 잔존물이 BB 전용물에 의존하면 `components/` 를 통째로 못 지운다」이고,
 *   주입 방식은 그 목적을 **그대로 지키면서** §3.6.4.3 의 「CanvasArea 배선」을
 *   만족한다 — 의존 방향은 여전히 한쪽(`bb/` → `components/`)뿐이다.
 *   S4·S5 의 `load`·`contour` 탭도 같은 방식으로 추가한다.
 */
interface CanvasAreaProps {
  /** `geometry` 탭 내용 (S3: `bb/BbGeometryView`). */
  geometryView: React.ReactNode;
  /** `load` 탭 내용 (S4: `bb/BbLoadDistView`). */
  loadView: React.ReactNode;
  /** `contour` 탭 내용 (S5: `bb/BbStressContourView`). */
  contourView: React.ReactNode;
  /** `section` 탭 내용 (P4-3a: `bb/BbAxialSectionView`). */
  sectionView: React.ReactNode;
  /** `3d` 탭 내용 (P4-3b: `bb/BbBearingView3D`). */
  view3d: React.ReactNode;
  /** `life` 탭 내용 (P5-2: `bb/BbLifeView`). */
  lifeView: React.ReactNode;
}

export default function CanvasArea({
  geometryView,
  loadView,
  contourView,
  sectionView,
  view3d,
  lifeView,
}: CanvasAreaProps) {
  const { state, dispatch } = useAppState();
  const { activeTab, result } = state;

  return (
    <div className="flex flex-col flex-1 min-h-0">
      {/* Tab bar */}
      <div className="flex items-center gap-0.5 px-3 pt-2 pb-0 shrink-0">
        {tabs.map(tab => (
          <button
            key={tab.key}
            onClick={() => dispatch({ type: 'SET_TAB', payload: tab.key })}
            disabled={tab.disabled}
            className={`px-3 py-1.5 text-[13px] font-medium rounded-t transition-colors flex items-center gap-1.5 ${
              activeTab === tab.key
                ? 'bg-canvas-subtle text-text-light border border-white/10 border-b-0 cursor-pointer'
                : tab.disabled
                  ? 'text-text-canvas/40 cursor-not-allowed'
                  : 'text-text-canvas hover:text-text-light hover:bg-white/5 cursor-pointer'
            }`}
          >
            {tab.label}
          </button>
        ))}
        <DualModeToggle />
      </div>

      {/* Content */}
      <div className="flex-1 min-h-0 bg-canvas-subtle mx-3 mb-3 rounded-b-lg rounded-tr-lg border border-white/10 overflow-hidden">
        {/* Input-only views — always available.
            ⚠ `geometry` 가 여기 있는 이유: `bb_compute_geometry` 는 **하중과 무관**이라
               Solve 를 누르지 않고도(= `result` 가 없어도) 볼 수 있다 (§3.6.4.7 ①).
               결과를 기다리면 Level A 를 화면으로 확인하는 경로가 막힌다. */}
        {activeTab === 'geometry' && geometryView}
        {/* ⚠ `section` 도 `geometry` 와 같이 **하중 무관**이다 — `bb_compute_geometry` 만으로
               무하중 단면을 그린다. 결과를 기다리면 Level A 기하를 그림으로 보는 경로가 막힌다. */}
        {activeTab === 'section' && sectionView}
        {activeTab === 'profile' && <ProfileView />}
        {activeTab === 'transient' && <TransientView />}
        {activeTab !== 'geometry' &&
          activeTab !== 'section' &&
          activeTab !== 'profile' &&
          activeTab !== 'transient' && (
          !result ? (
            <EmptyState />
          ) : (
            <>
              {activeTab === '3d' && view3d}
              {activeTab === 'load' && loadView}
              {activeTab === 'contour' && contourView}
              {activeTab === 'life' && lifeView}
              {activeTab === 'iso15312' && <ThermalSpeedView />}
              {activeTab === 'comparison' && <ComparisonChart />}
            </>
          )
        )}
      </div>
    </div>
  );
}

function EmptyState() {
  return (
    <div className="flex items-center justify-center h-full">
      <div className="text-center">
        <p className="text-text-canvas text-sm mb-1">No results yet</p>
        <p className="text-text-canvas/60 text-[13px]">Set parameters and click Solve</p>
      </div>
    </div>
  );
}
