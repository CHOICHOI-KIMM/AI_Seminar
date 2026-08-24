import { useReducer, useCallback } from 'react';
import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from '@tauri-apps/plugin-fs';
import { AppContext, appReducer, type AppState } from './store';
import { defaultInput } from './defaults';
import { openProjectFile, type ProjectFile, PROJECT_VERSION } from './project';
// P4-S2-3: 좌측 상시 입력 패널을 BB 전용으로 교체 (Plan §3.6.5.2 S2).
// 기존 `components/InputPanel` 은 TRB 필드(surface_finish.mean 등)를 읽어
// BB 프리셋이 로드되면 렌더 중 throw 했다 — 에러 바운더리가 없는 React 19 는
// 트리 전체를 언마운트하므로 화면이 통째로 비었다 (S1 헬스체크에서 검출).
// **파일은 지우지 않는다** — 최소 변경 방침(§3.6.4.3), 삭제는 §3.6.4.6 에서 일괄.
import BbInputPanel from './bb/BbInputPanel';
import CanvasArea from './components/CanvasArea';
// P4-S3-3: 우측 상시 요약 카드를 BB 전용으로 교체 (Plan §3.6.5.2 S3).
// 기존 `components/ResultsCard` 는 BB 에 없는 `result.life`·`static_rating`·
// `k_radial`·`mode` 를 읽어 **Solve 를 누르는 순간** 렌더 중 throw 했다
// (S2 가 「남은 위험」으로 예고). **파일은 지우지 않는다** — §3.6.4.3.
import BbResultsCard from './bb/BbResultsCard';
import AlertPanel from './components/AlertPanel';
import ProgressBar from './components/ProgressBar';

const initialState: AppState = {
  input: defaultInput,
  // P4-S3-1: BB 입력은 Rust 프리셋이 유일한 출처라 초기값이 없다 (`BbInputPanel` 이 기동 시 로드).
  bbInput: null,
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
            <h1 className="text-sm font-semibold tracking-wide">BB Contact Analysis</h1>
            <span className="text-xs text-text-canvas">Angular Contact Ball Bearing (ACBB)</span>
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
            <BbInputPanel />
            <AlertPanel />
          </aside>

          {/* Center: Canvas (Dark) */}
          <main className="flex-1 bg-canvas flex flex-col min-w-0 relative">
            <CanvasArea />
            <ProgressBar />
          </main>

          {/* Right: Results Sidebar */}
          <BbResultsCard />
        </div>
      </div>
    </AppContext>
  );
}
