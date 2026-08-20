import { useState } from 'react';
import { useAppState } from '../../store';
import TransientTimeChart from './TransientTimeChart';
import RollerDynamicsChart from './RollerDynamicsChart';
import DamageRiskView from './DamageRiskView';
import SliceSlidingContour from './SliceSlidingContour';

type SubTab = 'time' | 'dynamics' | 'damage' | 'contour';

const subTabs: { key: SubTab; label: string }[] = [
  { key: 'time', label: 'Time History' },
  { key: 'dynamics', label: 'Roller Dynamics' },
  { key: 'damage', label: 'Damage & Risk' },
  { key: 'contour', label: 'Slice SRR' },
];

export default function TransientView() {
  const { state } = useAppState();
  const { transientResult } = state;
  const [activeSubTab, setActiveSubTab] = useState<SubTab>('time');

  if (!transientResult) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="text-center">
          <p className="text-text-canvas text-sm mb-1">No transient results</p>
          <p className="text-text-canvas/60 text-xs">
            Load a CSV time series and click Solve Transient
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full">
      {/* Sub-tab bar */}
      <div className="flex gap-1 px-3 pt-2 pb-1 shrink-0">
        {subTabs.map(tab => (
          <button
            key={tab.key}
            onClick={() => setActiveSubTab(tab.key)}
            className={`px-3 py-1 text-[11px] font-medium rounded transition-colors cursor-pointer ${
              activeSubTab === tab.key
                ? 'bg-accent/20 text-accent'
                : 'text-text-canvas hover:text-text-light hover:bg-white/5'
            }`}
          >
            {tab.label}
          </button>
        ))}
        <span className="ml-auto text-xs text-text-canvas/60 font-mono self-center">
          {transientResult.snapshots.length} snapshots · {transientResult.elapsed_ms.toFixed(0)}ms
        </span>
      </div>

      {/* Content */}
      <div className="flex-1 min-h-0 overflow-auto px-3 pb-3">
        {activeSubTab === 'time' && <TransientTimeChart result={transientResult} />}
        {activeSubTab === 'dynamics' && <RollerDynamicsChart result={transientResult} />}
        {activeSubTab === 'damage' && <DamageRiskView result={transientResult} />}
        {activeSubTab === 'contour' && <SliceSlidingContour result={transientResult} />}
      </div>
    </div>
  );
}
