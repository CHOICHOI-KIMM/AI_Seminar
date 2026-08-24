import { useAppState, type CanvasTab } from '../../store';
import StressContourChart from '../charts/StressContourChart';
import LifeChart from '../charts/LifeChart';
import ComparisonChart from '../charts/ComparisonChart';
import BearingView3D from '../BearingView3D';
import SectionView2D from '../SectionView2D';
import ProfileView from '../ProfileView';
import LubricationView from '../LubricationView';
import DualModeToggle from '../DualModeToggle';
import TransientView from '../TransientView';
import ThermalSpeedView from '../ThermalSpeedView';

// P4-S1-4 (Plan §3.6.4.3 최소 변경 방침):
//   `legacy: true` = **아직 BB 로 개조되지 않은 TRB 잔존 탭**.
//   회색으로 표시하고 「TRB 잔존」 배지를 달되 **클릭은 허용**하고 내용은 그대로 둔다.
//   지우지 않는 이유 — 검증 중에는 변경이 적을수록 「솔버가 틀린 건가, 화면이
//   틀린 건가」의 원인 분리가 쉽기 때문이다. 일괄 정리는 §3.6.4.6 시점에 한다.
//   부수 효과로 **개조할 목록이 화면에 그대로 보인다.**
//   `legacy` 가 없는 3탭(geometry·load·contour)이 S3~S5 에서 BB 뷰가 붙을 자리다.
//   S3 에서 geometry, **S4 에서 load** 가 BB 뷰로 채워졌다 (둘 다 prop 주입).
const tabs: { key: CanvasTab; label: string; legacy?: boolean }[] = [
  { key: 'geometry', label: 'Geometry' },
  { key: 'profile', label: 'Profile', legacy: true },
  { key: 'section', label: 'Section', legacy: true },
  { key: '3d', label: '3D View', legacy: true },
  { key: 'load', label: 'Load Distribution' },
  { key: 'contour', label: 'Stress Contour' },
  { key: 'lubrication', label: 'Lubrication', legacy: true },
  { key: 'life', label: 'Life', legacy: true },
  { key: 'iso15312', label: 'Thermal Speed', legacy: true },
  { key: 'comparison', label: 'Comparison', legacy: true },
  { key: 'transient', label: 'Transient', legacy: true },
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
}

export default function CanvasArea({ geometryView, loadView }: CanvasAreaProps) {
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
            title={tab.legacy ? 'TRB 잔존 — 아직 BB 로 개조되지 않은 화면입니다' : undefined}
            className={`px-3 py-1.5 text-[13px] font-medium rounded-t transition-colors cursor-pointer flex items-center gap-1.5 ${
              activeTab === tab.key
                ? 'bg-canvas-subtle text-text-light border border-white/10 border-b-0'
                : tab.legacy
                  ? 'text-text-canvas/40 hover:text-text-canvas/70 hover:bg-white/5'
                  : 'text-text-canvas hover:text-text-light hover:bg-white/5'
            }`}
          >
            <span className={tab.legacy && activeTab !== tab.key ? 'opacity-70' : undefined}>{tab.label}</span>
            {tab.legacy && (
              <span className="px-1 py-px rounded-sm text-[9px] font-semibold uppercase tracking-wide bg-white/10 text-text-canvas/60 border border-white/10">
                TRB 잔존
              </span>
            )}
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
        {activeTab === 'section' && <SectionView2D />}
        {activeTab === 'profile' && <ProfileView />}
        {activeTab === 'transient' && <TransientView />}
        {activeTab !== 'geometry' && activeTab !== 'section' && activeTab !== 'profile' && activeTab !== 'transient' && (
          !result ? (
            <EmptyState />
          ) : (
            <>
              {activeTab === '3d' && <BearingView3D />}
              {activeTab === 'load' && loadView}
              {activeTab === 'contour' && <StressContourChart />}
              {activeTab === 'lubrication' && <LubricationView />}
              {activeTab === 'life' && <LifeChart />}
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
