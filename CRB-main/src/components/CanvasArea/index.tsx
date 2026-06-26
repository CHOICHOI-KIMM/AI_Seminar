import { useAppState, type CanvasTab } from '../../store';
import LoadDistChart from '../charts/LoadDistChart';
import StressContourChart from '../charts/StressContourChart';
import LifeChart from '../charts/LifeChart';
import ComparisonChart from '../charts/ComparisonChart';
import BearingView3D from '../BearingView3D';
import SectionView2D from '../SectionView2D';
import ProfileView from '../ProfileView';
import GeometryView from '../GeometryView';
import LubricationView from '../LubricationView';
import DualModeToggle from '../DualModeToggle';
import TransientView from '../TransientView';
import ThermalSpeedView from '../ThermalSpeedView';

const tabs: { key: CanvasTab; label: string }[] = [
  { key: 'geometry', label: 'Geometry' },
  { key: 'profile', label: 'Profile' },
  { key: 'section', label: 'Section' },
  { key: '3d', label: '3D View' },
  { key: 'load', label: 'Load Distribution' },
  { key: 'contour', label: 'Stress Contour' },
  { key: 'lubrication', label: 'Lubrication' },
  { key: 'life', label: 'Life' },
  { key: 'iso15312', label: 'Thermal Speed' },
  { key: 'comparison', label: 'Comparison' },
  { key: 'transient', label: 'Transient' },
];

export default function CanvasArea() {
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
            className={`px-3 py-1.5 text-[13px] font-medium rounded-t transition-colors cursor-pointer ${
              activeTab === tab.key
                ? 'bg-canvas-subtle text-text-light border border-white/10 border-b-0'
                : 'text-text-canvas hover:text-text-light hover:bg-white/5'
            }`}
          >
            {tab.label}
          </button>
        ))}
        <DualModeToggle />
      </div>

      {/* Content */}
      <div className="flex-1 min-h-0 bg-canvas-subtle mx-3 mb-3 rounded-b-lg rounded-tr-lg border border-white/10 overflow-hidden">
        {/* Input-only views — always available */}
        {activeTab === 'section' && <SectionView2D />}
        {activeTab === 'profile' && <ProfileView />}
        {activeTab === 'transient' && <TransientView />}
        {activeTab !== 'section' && activeTab !== 'profile' && activeTab !== 'transient' && (
          !result ? (
            <EmptyState />
          ) : (
            <>
              {activeTab === 'geometry' && <GeometryView />}
              {activeTab === '3d' && <BearingView3D />}
              {activeTab === 'load' && <LoadDistChart />}
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
