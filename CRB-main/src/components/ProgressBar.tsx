import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { useAppState, type SolverProgress } from '../store';

export default function ProgressBar() {
  const { state, dispatch } = useAppState();
  const { loading, progress } = state;

  useEffect(() => {
    const unlisten = listen<SolverProgress>('solver-progress', (event) => {
      dispatch({ type: 'SET_PROGRESS', payload: event.payload });
    });
    return () => { unlisten.then(fn => fn()); };
  }, [dispatch]);

  if (!loading) return null;

  const percent = progress?.percent ?? 0;
  const stage = progress?.stage ?? 'Initializing';
  const detail = progress?.detail ?? '';

  return (
    <div className="absolute bottom-0 left-0 right-0 z-50">
      {/* Text info */}
      <div className="flex items-center justify-between px-3 py-1 text-xs bg-slate-900/90 backdrop-blur-sm">
        <div className="flex items-center gap-2">
          <span className="text-blue-400 font-semibold">{stage}</span>
          <span className="text-text-canvas">{detail}</span>
        </div>
        <span className="text-text-canvas font-mono tabular-nums">{percent.toFixed(0)}%</span>
      </div>
      {/* Bar */}
      <div className="h-1 bg-slate-800">
        <div
          className="h-full bg-gradient-to-r from-blue-500 to-cyan-400 transition-all duration-150 ease-out"
          style={{ width: `${percent}%` }}
        />
      </div>
    </div>
  );
}
