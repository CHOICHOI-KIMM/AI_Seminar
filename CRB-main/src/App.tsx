import { useReducer, useCallback } from 'react';
import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from '@tauri-apps/plugin-fs';
import { AppContext, appReducer, type AppState } from './store';
import { defaultInput } from './defaults';
import { openProjectFile, type ProjectFile, PROJECT_VERSION } from './project';
import InputPanel from './components/InputPanel';
import CanvasArea from './components/CanvasArea';
import ResultsCard from './components/ResultsCard';
import AlertPanel from './components/AlertPanel';
import ProgressBar from './components/ProgressBar';

const initialState: AppState = {
  input: defaultInput,
  result: null,
  dualResult: null,
  transientResult: null,
  dualViewMode: 'gen1',
  loading: false,
  progress: null,
  error: null,
  activeTab: 'load',
  resultsPanelOpen: true,
};

export default function App() {
  const [state, dispatch] = useReducer(appReducer, initialState);

  const handleSave = useCallback(async () => {
    const path = await save({
      title: 'Save Project',
      defaultPath: 'bearing.trb.json',
      filters: [{ name: 'TRB Project', extensions: ['trb.json'] }],
    });
    if (!path) return;
    const project: ProjectFile = {
      version: PROJECT_VERSION,
      created: new Date().toISOString(),
      input: state.input,
    };
    await writeTextFile(path, JSON.stringify(project, null, 2));
  }, [state.input]);

  const handleLoad = useCallback(async () => {
    const input = await openProjectFile();
    if (input) {
      dispatch({ type: 'SET_INPUT', payload: input });
      dispatch({ type: 'CLEAR_RESULTS' });
    }
  }, [dispatch]);

  const handleReset = useCallback(() => {
    dispatch({ type: 'SET_INPUT', payload: defaultInput });
    dispatch({ type: 'CLEAR_RESULTS' });
  }, [dispatch]);

  return (
    <AppContext value={{ state, dispatch }}>
      <div className="flex flex-col h-screen">
        {/* Top Bar */}
        <header className="flex items-center justify-between px-4 py-2 bg-bar text-bar-text border-b border-white/10 shrink-0">
          <div className="flex items-center gap-3">
            <h1 className="text-sm font-semibold tracking-wide">TRB Contact Analysis</h1>
            <span className="text-xs text-text-canvas">Tapered Roller Bearing</span>
          </div>
          <div className="flex items-center gap-2 text-xs">
            <button onClick={handleLoad} className="px-2 py-1 rounded hover:bg-white/10 text-text-canvas transition-colors cursor-pointer" title="Open project file">
              Load
            </button>
            <button onClick={handleSave} className="px-2 py-1 rounded hover:bg-white/10 text-text-canvas transition-colors cursor-pointer" title="Save project file">
              Save
            </button>
            <button onClick={handleReset} className="px-2 py-1 rounded hover:bg-white/10 text-orange-300 transition-colors cursor-pointer" title="Reset to defaults">
              Reset
            </button>
            <span className="tabular-nums text-text-canvas ml-2">v0.1.0</span>
          </div>
        </header>

        {/* Main Content */}
        <div className="flex flex-1 min-h-0">
          {/* Left: Input Panel (Light) */}
          <aside className="w-72 shrink-0 bg-panel border-r border-panel-border overflow-y-auto custom-scrollbar">
            <InputPanel />
            <AlertPanel />
          </aside>

          {/* Center: Canvas (Dark) */}
          <main className="flex-1 bg-canvas flex flex-col min-w-0 relative">
            <CanvasArea />
            <ProgressBar />
          </main>

          {/* Right: Results Sidebar */}
          <ResultsCard />
        </div>
      </div>
    </AppContext>
  );
}
