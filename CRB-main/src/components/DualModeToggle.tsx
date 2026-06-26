import { useAppState, type DualViewMode } from '../store';

const modes: { key: DualViewMode; label: string; color: string }[] = [
  { key: 'gen1', label: 'Gen1', color: 'bg-blue-500/20 text-blue-300 border-blue-500/40' },
  { key: 'gen3', label: 'Gen3', color: 'bg-emerald-500/20 text-emerald-300 border-emerald-500/40' },
];

export default function DualModeToggle() {
  const { state, dispatch } = useAppState();
  if (!state.dualResult) return null;

  return (
    <div className="flex items-center gap-1 ml-2 pl-2 border-l border-white/10">
      <span className="text-xs text-text-canvas mr-1">View:</span>
      {modes.map(m => (
        <button
          key={m.key}
          onClick={() => dispatch({ type: 'SET_DUAL_VIEW_MODE', payload: m.key })}
          className={`px-2 py-0.5 text-xs font-mono rounded border transition-colors cursor-pointer ${
            state.dualViewMode === m.key
              ? m.color
              : 'border-transparent text-text-canvas hover:text-text-light'
          }`}
        >
          {m.label}
        </button>
      ))}
    </div>
  );
}
