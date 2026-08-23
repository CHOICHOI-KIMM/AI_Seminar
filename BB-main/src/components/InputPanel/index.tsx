// @ts-nocheck
// CRB Phase 1.4 stub: 이 컴포넌트는 TRB 데이터 모델을 참조 중 → Phase 6 (Frontend UI 변경) 에서 CRB 로 정식 재작성 예정
import { useState, useCallback, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { readTextFile } from '@tauri-apps/plugin-fs';
import { useAppState } from '../../store';
import type { BearingInput, BearingResult, DualModeComparison, CrownType, TransientResult, LoadTimePoint, TransientInput, LoadSourceType, SineWaveConfig } from '../../types/bearing';
import { DEFAULT_SINE_CONFIG, generateSineLoadSeries } from '../../types/bearing';
import FieldGroup from './FieldGroup';


interface PresetInfo {
  name: string;
  modified: string;
}

export default function InputPanel() {
  const { state, dispatch } = useAppState();
  const { input, loading } = state;
  const [openSections, setOpenSections] = useState<Set<string>>(() => new Set());
  const [elapsed, setElapsed] = useState(0);
  useEffect(() => {
    if (!loading) { setElapsed(0); return; }
    const t0 = Date.now();
    const id = setInterval(() => setElapsed((Date.now() - t0) / 1000), 200);
    return () => clearInterval(id);
  }, [loading]);

  // 토글 시 이전 값 보존용 ref
  const prevRibCircRef = useRef<number | null>(null);

  const toggleSection = useCallback((key: string) => {
    setOpenSections(prev => {
      const next = new Set(prev);
      if (next.has(key)) next.delete(key);
      else next.add(key);
      return next;
    });
  }, []);

  // ── Preset management ──
  const [presets, setPresets] = useState<PresetInfo[]>([]);
  const [selectedPreset, setSelectedPreset] = useState('');
  const [saveFlash, setSaveFlash] = useState(false);

  const refreshPresets = useCallback(async () => {
    try {
      const list = await invoke<PresetInfo[]>('bb_preset_list');
      setPresets(list);
      if (list.length > 0 && !list.find(p => p.name === selectedPreset)) {
        setSelectedPreset(list[0].name);
      }
    } catch { /* ignore */ }
  }, [selectedPreset]);

  // 앱 시작 시 마지막 사용 프리셋 자동 로드
  useEffect(() => {
    (async () => {
      await refreshPresets();
      try {
        const lastName = await invoke<string | null>('bb_preset_get_last');
        if (lastName) {
          const data = await invoke<BearingInput>('bb_preset_load', { name: lastName });
          dispatch({ type: 'SET_INPUT', payload: data });
          setSelectedPreset(lastName);
          return;
        }
      } catch { /* 프리셋 삭제된 경우 등 */ }
      // last_preset 없으면 첫 번째 프리셋 자동 로드
      try {
        const list = await invoke<PresetInfo[]>('bb_preset_list');
        if (list.length > 0) {
          const firstName = list[0].name;
          const data = await invoke<BearingInput>('bb_preset_load', { name: firstName });
          dispatch({ type: 'SET_INPUT', payload: data });
          setSelectedPreset(firstName);
          invoke('bb_preset_save_last', { name: firstName }).catch(() => {});
        }
      } catch { /* ignore */ }
    })();
  }, []);

  // B안: 드롭다운 선택 즉시 로드
  const handlePresetChange = useCallback(async (name: string) => {
    setSelectedPreset(name);
    if (!name) return;
    try {
      const data = await invoke<BearingInput>('bb_preset_load', { name });
      dispatch({ type: 'SET_INPUT', payload: data });
      invoke('bb_preset_save_last', { name }).catch(() => {});
    } catch (e) {
      dispatch({ type: 'SET_ERROR', payload: `Preset load failed: ${e}` });
    }
  }, [dispatch]);

  // 새 이름으로 저장
  const handleSaveNewPreset = useCallback(async () => {
    const name = window.prompt('새 프리셋 이름:', '');
    if (!name) return;
    try {
      await invoke('bb_preset_save', { name, input });
      await refreshPresets();
      setSelectedPreset(name);
      invoke('bb_preset_save_last', { name }).catch(() => {});
    } catch (e) {
      dispatch({ type: 'SET_ERROR', payload: `Preset save failed: ${e}` });
    }
  }, [input, refreshPresets, dispatch]);

  // 현재 선택된 프리셋에 덮어쓰기
  const handleOverwritePreset = useCallback(async () => {
    if (!selectedPreset) return;
    try {
      await invoke('bb_preset_save', { name: selectedPreset, input });
      invoke('bb_preset_save_last', { name: selectedPreset }).catch(() => {});
      await refreshPresets();
      setSaveFlash(true);
      setTimeout(() => setSaveFlash(false), 1200);
    } catch (e) {
      dispatch({ type: 'SET_ERROR', payload: `Preset overwrite failed: ${e}` });
    }
  }, [selectedPreset, input, refreshPresets, dispatch]);

  // 선택된 프리셋의 원래 값으로 복구
  const handleResetPreset = useCallback(async () => {
    if (!selectedPreset) return;
    try {
      const data = await invoke<BearingInput>('bb_preset_load', { name: selectedPreset });
      dispatch({ type: 'SET_INPUT', payload: data });
    } catch (e) {
      dispatch({ type: 'SET_ERROR', payload: `Preset reset failed: ${e}` });
    }
  }, [selectedPreset, dispatch]);

  const handleDeletePreset = useCallback(async () => {
    if (!selectedPreset) return;
    if (!window.confirm(`"${selectedPreset}" 프리셋을 삭제하시겠습니까?`)) return;
    try {
      await invoke('bb_preset_delete', { name: selectedPreset });
      invoke('bb_preset_save_last', { name: '' }).catch(() => {});
      await refreshPresets();
    } catch (e) {
      dispatch({ type: 'SET_ERROR', payload: `Preset delete failed: ${e}` });
    }
  }, [selectedPreset, refreshPresets, dispatch]);

  const updateGeom = useCallback((field: string, value: number) => {
    dispatch({
      type: 'UPDATE_INPUT',
      payload: { macro_geom: { ...input.macro_geom, [field]: value } },
    });
  }, [input.macro_geom, dispatch]);

  const updateRaceway = useCallback((field: string, value: number) => {
    dispatch({
      type: 'UPDATE_INPUT',
      payload: { raceway_geom: { ...input.raceway_geom, [field]: value } },
    });
  }, [input.raceway_geom, dispatch]);

  const updateRollerProfile = useCallback((field: string, value: number) => {
    dispatch({
      type: 'UPDATE_INPUT',
      payload: { roller_profile: { ...input.roller_profile, [field]: value } },
    });
  }, [input.roller_profile, dispatch]);

  const updateRacewayInner = useCallback((field: string, value: number) => {
    dispatch({
      type: 'UPDATE_INPUT',
      payload: { raceway_profile_inner: { ...input.raceway_profile_inner, [field]: value } },
    });
  }, [input.raceway_profile_inner, dispatch]);

  const updateRacewayOuter = useCallback((field: string, value: number) => {
    dispatch({
      type: 'UPDATE_INPUT',
      payload: { raceway_profile_outer: { ...input.raceway_profile_outer, [field]: value } },
    });
  }, [input.raceway_profile_outer, dispatch]);

  const updateOp = useCallback((field: string, value: number) => {
    dispatch({
      type: 'UPDATE_INPUT',
      payload: { operating: { ...input.operating, [field]: value } },
    });
  }, [input.operating, dispatch]);

  const updateMat = useCallback((field: string, value: number) => {
    dispatch({
      type: 'UPDATE_INPUT',
      payload: { material: { ...input.material, [field]: value } },
    });
  }, [input.material, dispatch]);

  // Crown type helpers
  const crownTypeName = (ct: CrownType): string => {
    if ('Logarithmic' in ct) return 'Logarithmic';
    if ('Circular' in ct) return 'Circular';
    if ('Parabolic' in ct) return 'Parabolic';
    if ('Polynomial' in ct) return 'Polynomial';
    return 'Custom';
  };

  const setCrownType = (name: string) => {
    let crown_type: CrownType;
    switch (name) {
      case 'Logarithmic': crown_type = { Logarithmic: { a_log: 0.0002 } }; break;
      case 'Circular': crown_type = { Circular: { r_crown: 5000 } }; break;
      case 'Custom': crown_type = { Custom: { profile: [] } }; break;
      case 'Polynomial': crown_type = { Polynomial: { coeffs: [0, 0, 0, 0, 0] } }; break;
      default: crown_type = { Parabolic: { c2: 0.01 } }; break;
    }
    dispatch({
      type: 'UPDATE_INPUT',
      payload: { roller_profile: { ...input.roller_profile, crown_type } },
    });
  };

  // Derived parameter from δ_c (read-only display)
  const derivedCrownParam = (() => {
    const ct = input.roller_profile.crown_type;
    const dc = input.roller_profile.delta_c;
    const halfL = input.macro_geom.l_we / 2;
    const hl2 = halfL * halfL;
    if (dc <= 0 || hl2 <= 0) return null;
    if ('Logarithmic' in ct) return { label: 'A_log', value: (dc / Math.log(1 / (1 - 0.81))).toFixed(4) };
    if ('Circular' in ct) return { label: 'R_crown', value: (hl2 / (2 * dc / 1000)).toFixed(1) + ' mm' };
    if ('Parabolic' in ct) return { label: 'c₂', value: (dc / hl2).toFixed(6) };
    return null;
  })();

  const solve = async () => {
    console.log('[Solve] start, mode=', input.solver.run_mode, 'split=', input.solver.use_split_contact);
    dispatch({ type: 'SET_LOADING', payload: true });
    dispatch({ type: 'SET_ERROR', payload: null });
    // Yield to let React render the loading state before starting heavy computation
    await new Promise(r => setTimeout(r, 50));
    try {
      const isDual = input.solver.run_mode === 'Dual';
      if (isDual) {
        const result = await invoke<DualModeComparison>('solve_bearing_dual', { input });
        dispatch({ type: 'SET_DUAL_RESULT', payload: result });
      } else {
        const result = await invoke<BearingResult>('bb_solve_bearing', { input });
        dispatch({ type: 'SET_RESULT', payload: result });
      }
      console.log('[Solve] done');
    } catch (e) {
      console.error('[Solve] error:', e);
      dispatch({ type: 'SET_ERROR', payload: String(e) });
    } finally {
      dispatch({ type: 'SET_LOADING', payload: false });
    }
  };

  const setRunMode = (mode: string) => {
    const run_mode = mode === 'Dual' ? 'Dual' as const : { Single: mode as 'Gen1' | 'Gen3' };
    dispatch({
      type: 'UPDATE_INPUT',
      payload: { solver: { ...input.solver, run_mode } },
    });
  };

  const setBeamType = (bt: string) => {
    dispatch({
      type: 'UPDATE_INPUT',
      payload: { solver: { ...input.solver, beam_type: bt as 'Timoshenko' | 'EulerBernoulli' } },
    });
  };

  // ── Transient ──
  const [loadSourceType, setLoadSourceType] = useState<LoadSourceType>('sine');
  const [loadSeries, setLoadSeries] = useState<LoadTimePoint[]>([]);
  const [sineConfig, setSineConfig] = useState<SineWaveConfig>({ ...DEFAULT_SINE_CONFIG });
  const [dtMax, setDtMax] = useState(0.001);
  const [enableDynamics, setEnableDynamics] = useState(true);
  const [snapshotInterval, setSnapshotInterval] = useState(1);

  // 사인파 config 변경 시 자동 생성
  useEffect(() => {
    if (loadSourceType === 'sine') {
      try {
        const pts = generateSineLoadSeries(sineConfig);
        setLoadSeries(pts);
      } catch {
        setLoadSeries([]);
      }
    }
  }, [loadSourceType, sineConfig]);

  const updateSineChannel = useCallback((ch: keyof SineWaveConfig, field: 'mean' | 'amplitude', value: number) => {
    setSineConfig(prev => ({
      ...prev,
      [ch]: { ...(prev[ch] as { mean: number; amplitude: number }), [field]: value },
    }));
  }, []);

  const loadCsv = useCallback(async () => {
    const path = await open({
      title: 'Load Time Series CSV',
      filters: [{ name: 'CSV', extensions: ['csv'] }],
    });
    if (!path) return;
    try {
      const csvText = await readTextFile(path);
      const points = await invoke<LoadTimePoint[]>('parse_load_csv', { csvText });
      setLoadSeries(points);
    } catch (e) {
      dispatch({ type: 'SET_ERROR', payload: `CSV parse error: ${e}` });
    }
  }, [dispatch]);

  const solveTransient = useCallback(async () => {
    if (loadSeries.length < 2) {
      dispatch({ type: 'SET_ERROR', payload: 'Load at least 2 time points' });
      return;
    }
    const transientInput: TransientInput = {
      load_series: loadSeries,
      dt_max: dtMax,
      enable_roller_dynamics: enableDynamics,
      snapshot_interval: snapshotInterval,
    };
    const fullInput = { ...input, transient: transientInput };
    dispatch({ type: 'SET_LOADING', payload: true });
    dispatch({ type: 'SET_ERROR', payload: null });
    try {
      const result = await invoke<TransientResult>('solve_transient', { input: fullInput });
      dispatch({ type: 'SET_TRANSIENT_RESULT', payload: result });
      dispatch({ type: 'SET_TAB', payload: 'transient' });
    } catch (e) {
      dispatch({ type: 'SET_ERROR', payload: String(e) });
    } finally {
      dispatch({ type: 'SET_LOADING', payload: false });
    }
  }, [input, loadSeries, dtMax, enableDynamics, snapshotInterval, dispatch]);

  const currentMode = input.solver.run_mode === 'Dual' ? 'Dual' :
    (input.solver.run_mode as { Single: string }).Single;

  return (
    <div className="flex flex-col h-full">
      {/* Preset Bar — 선택 즉시 로드, +/− 버튼 */}
      <div className="px-3 pt-2 pb-1 border-b border-panel-border">
        <div className="flex items-center gap-1">
          <select
            value={selectedPreset}
            onChange={e => handlePresetChange(e.target.value)}
            className="flex-1 min-w-0 px-1.5 py-1 text-xs bg-white border border-slate-200 rounded
                       focus:outline-none focus:border-accent text-text-dark cursor-pointer truncate"
          >
            {presets.map(p => (
              <option key={p.name} value={p.name}>{p.name}</option>
            ))}
            {presets.length === 0 && <option value="">No presets</option>}
          </select>
          <button
            onClick={handleResetPreset}
            disabled={!selectedPreset}
            className="w-6 h-6 flex items-center justify-center text-xs rounded bg-amber-400 text-white hover:bg-amber-500 disabled:bg-slate-100 disabled:text-slate-300 cursor-pointer transition-colors"
            title="선택된 프리셋의 원래 값으로 복구"
          >↺</button>
          <button
            onClick={handleOverwritePreset}
            disabled={!selectedPreset}
            className={`w-6 h-6 flex items-center justify-center text-xs rounded cursor-pointer transition-colors ${saveFlash ? 'bg-emerald-500 text-white' : 'bg-slate-200 text-slate-600 hover:bg-accent hover:text-white'} disabled:bg-slate-100 disabled:text-slate-300`}
            title="현재 입력값을 선택된 프리셋에 덮어쓰기"
          >{saveFlash ? '✓' : '💾'}</button>
          <button
            onClick={handleSaveNewPreset}
            className="w-6 h-6 flex items-center justify-center text-sm rounded font-bold bg-emerald-500 text-white hover:bg-emerald-600 cursor-pointer transition-colors"
            title="현재 입력값을 새 프리셋으로 저장"
          >+</button>
          <button
            onClick={handleDeletePreset}
            disabled={!selectedPreset}
            className="w-6 h-6 flex items-center justify-center text-sm rounded font-bold bg-red-400 text-white hover:bg-red-500 disabled:bg-slate-200 disabled:text-slate-400 cursor-pointer transition-colors"
            title="선택된 프리셋 삭제"
          >−</button>
        </div>
      </div>

      {/* Solve Button */}
      <div className="p-3 border-b border-panel-border">
        <button
          onClick={solve}
          disabled={loading}
          className={`w-full py-2.5 px-4 rounded-md text-sm font-semibold transition-all cursor-pointer ${
            loading
              ? 'bg-amber-600 text-white cursor-not-allowed animate-pulse border-2 border-amber-400'
              : 'bg-accent text-white hover:bg-accent-hover active:scale-[0.98]'
          }`}
        >
          {loading ? (
            <span className="flex items-center justify-center gap-2">
              <svg className="animate-spin h-4 w-4" viewBox="0 0 24 24" fill="none">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
              </svg>
              Solving... {elapsed >= 1 ? `(${elapsed.toFixed(0)}s)` : ''}
            </span>
          ) : 'Solve'}
        </button>
        {loading && (
          <div className="mt-1.5 w-full bg-slate-700 rounded-full h-1.5 overflow-hidden">
            <div className="bg-amber-400 h-full rounded-full animate-pulse" style={{ width: '100%' }} />
          </div>
        )}

        {/* Mode selector */}
        <div className="flex gap-1 mt-2">
          {['Gen1', 'Gen3', 'Dual'].map(mode => (
            <button
              key={mode}
              onClick={() => setRunMode(mode)}
              className={`flex-1 py-1 text-[13px] rounded font-medium transition-colors cursor-pointer ${
                currentMode === mode
                  ? 'bg-accent text-white'
                  : 'bg-panel-muted text-text-muted hover:bg-slate-200'
              }`}
            >
              {mode}
            </button>
          ))}
        </div>

        {/* Inline error/warning display */}
        {state.error && (
          <div className="mt-2 p-2 rounded bg-red-50 border border-red-200">
            <p className="text-[11px] text-red-700 break-words leading-tight">{state.error}</p>
          </div>
        )}
        {!state.error && state.result?.alerts && state.result.alerts.length > 0 && (
          <div className="mt-2 space-y-1">
            {state.result.alerts.slice(0, 3).map((a, i) => (
              <div key={i} className={`p-1.5 rounded border text-[11px] break-words leading-tight ${
                a.level === 'Critical' ? 'bg-red-50 border-red-200 text-red-700'
                  : a.level === 'Warning' ? 'bg-amber-50 border-amber-200 text-amber-700'
                  : 'bg-blue-50 border-blue-200 text-blue-700'
              }`}>
                <span className="font-semibold">{a.category}:</span> {a.message}
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Accordion Sections */}
      <div className="flex-1 overflow-y-auto custom-scrollbar">
        {/* ── Geometry (parent group) ── */}
        <AccordionSection title="Geometry" sectionKey="geom" openSections={openSections} onToggle={toggleSection}>
          {/* 공통 제원 */}
          <SubAccordion title="Common" sectionKey="geom-common" openSections={openSections} onToggle={toggleSection}>
            <FieldGroup label="Bore d" value={input.macro_geom.d} unit="mm" onChange={v => updateGeom('d', v)} />
            <FieldGroup label="OD" value={input.macro_geom.outer_diameter} unit="mm" onChange={v => updateGeom('outer_diameter', v)} />
            <FieldGroup label="Width T" value={input.macro_geom.t} unit="mm" onChange={v => updateGeom('t', v)} />
            <FieldGroup label="Contact angle α" value={input.macro_geom.alpha} unit="deg" onChange={v => updateGeom('alpha', v)} step={0.5} />
            <FieldGroup label="Rollers Z" value={input.macro_geom.z} unit="" onChange={v => updateGeom('z', v)} step={1} />
            <FieldGroup label="d_pw" value={input.macro_geom.d_pw} unit="mm" onChange={v => updateGeom('d_pw', v)} />
            <FieldGroup label="Clearance G_r" value={input.macro_geom.g_r} unit="μm" onChange={v => updateGeom('g_r', v)} step={0.5} />
          </SubAccordion>

          {/* 롤러 */}
          <SubAccordion title="Roller" sectionKey="geom-roller" openSections={openSections} onToggle={toggleSection}>
            <FieldGroup label="D_we max" value={input.macro_geom.d_we_max} unit="mm" onChange={v => updateGeom('d_we_max', v)} />
            <FieldGroup label="D_we min" value={input.macro_geom.d_we_min} unit="mm" onChange={v => updateGeom('d_we_min', v)} />
            <FieldGroup label="L_we" value={input.macro_geom.l_we} unit="mm" onChange={v => updateGeom('l_we', v)} />
            <FieldGroup label="R_sph (roller end)" value={input.roller_profile.r_sph} unit="mm" onChange={v => updateRollerProfile('r_sph', v)} step={5} />
          </SubAccordion>

          {/* 레이스웨이 */}
          <SubAccordion title="Raceway" sectionKey="geom-raceway" openSections={openSections} onToggle={toggleSection}>
            <FieldGroup label="α inner" value={input.raceway_geom.alpha_i} unit="deg" onChange={v => updateRaceway('alpha_i', v)} step={0.5} />
            <FieldGroup label="α outer" value={input.raceway_geom.alpha_o} unit="deg" onChange={v => updateRaceway('alpha_o', v)} step={0.5} />
<FieldGroup label="Rib height" value={input.macro_geom.h_rib} unit="mm" onChange={v => updateGeom('h_rib', v)} step={0.5} />
            <FieldGroup label="Rib angle" value={input.macro_geom.alpha_rib} unit="deg" onChange={v => updateGeom('alpha_rib', v)} step={0.5} />
            <div className="flex items-center justify-between py-0.5">
              <span className="text-[13px] text-text-muted">h_c (contact ht.)</span>
              <div className="flex items-center gap-1">
                <button
                  className={`px-1.5 py-0.5 text-xs rounded border ${
                    input.macro_geom.h_c === null
                      ? 'bg-accent text-white border-accent'
                      : 'bg-white text-text-muted border-slate-200 hover:border-accent'
                  }`}
                  onClick={() => dispatch({ type: 'UPDATE_INPUT', payload: { macro_geom: { ...input.macro_geom, h_c: null } } })}
                >Auto</button>
                <button
                  className={`px-1.5 py-0.5 text-xs rounded border ${
                    input.macro_geom.h_c !== null
                      ? 'bg-accent text-white border-accent'
                      : 'bg-white text-text-muted border-slate-200 hover:border-accent'
                  }`}
                  onClick={() => dispatch({ type: 'UPDATE_INPUT', payload: { macro_geom: { ...input.macro_geom, h_c: input.macro_geom.h_rib / 2 } } })}
                >Manual</button>
              </div>
            </div>
            {input.macro_geom.h_c !== null && (
              <FieldGroup label="  value" value={input.macro_geom.h_c} unit="mm" onChange={v => updateGeom('h_c', v)} step={0.1} />
            )}
            {input.macro_geom.h_c === null && (
              <div className="text-xs text-text-muted pl-2 py-0.5">= h_rib/2 = {(input.macro_geom.h_rib / 2).toFixed(2)} mm</div>
            )}
            <FieldGroup label="r rib meridional" value={input.raceway_geom.r_rib} unit="mm" onChange={v => updateRaceway('r_rib', v)} step={100} />
            <div className="flex items-center justify-between py-0.5">
              <span className="text-[13px] text-text-muted">r rib circ.</span>
              <div className="flex items-center gap-1">
                <button
                  className={`px-1.5 py-0.5 text-xs rounded border ${
                    input.raceway_geom.r_rib_circ === null
                      ? 'bg-accent text-white border-accent'
                      : 'bg-white text-text-muted border-slate-200 hover:border-accent'
                  }`}
                  onClick={() => {
                    if (input.raceway_geom.r_rib_circ !== null) {
                      prevRibCircRef.current = input.raceway_geom.r_rib_circ;
                    }
                    dispatch({
                      type: 'UPDATE_INPUT',
                      payload: { raceway_geom: { ...input.raceway_geom, r_rib_circ: null } },
                    });
                  }}
                >Auto</button>
                <button
                  className={`px-1.5 py-0.5 text-xs rounded border ${
                    input.raceway_geom.r_rib_circ !== null
                      ? 'bg-accent text-white border-accent'
                      : 'bg-white text-text-muted border-slate-200 hover:border-accent'
                  }`}
                  onClick={() => dispatch({
                    type: 'UPDATE_INPUT',
                    payload: { raceway_geom: { ...input.raceway_geom, r_rib_circ: prevRibCircRef.current ?? (() => { const g = (input.raceway_geom.alpha_i + input.raceway_geom.alpha_o) / 2 * Math.PI / 180; const hc = input.macro_geom.h_c ?? input.macro_geom.h_rib / 2; const rc = input.macro_geom.d_pw / 2 + input.macro_geom.l_we / 2 * Math.sin(g) - input.macro_geom.d_we_max / 2 * Math.cos(g) + hc; return Math.round(rc / Math.sin(input.macro_geom.alpha_rib * Math.PI / 180)); })() } },
                  })}
                >Manual</button>
              </div>
            </div>
            {input.raceway_geom.r_rib_circ !== null && (
              <FieldGroup label="  value" value={input.raceway_geom.r_rib_circ} unit="mm" onChange={v => updateRaceway('r_rib_circ', v)} step={1} />
            )}
            {input.raceway_geom.r_rib_circ === null && (
              <div className="text-xs text-text-muted pl-2 py-0.5">
                ≈ {(() => { const g = (input.raceway_geom.alpha_i + input.raceway_geom.alpha_o) / 2 * Math.PI / 180; const hc = input.macro_geom.h_c ?? input.macro_geom.h_rib / 2; const rc = input.macro_geom.d_pw / 2 + input.macro_geom.l_we / 2 * Math.sin(g) - input.macro_geom.d_we_max / 2 * Math.cos(g) + hc; return (rc / Math.sin(input.macro_geom.alpha_rib * Math.PI / 180)).toFixed(1); })()} mm (r_c={(() => { const g = (input.raceway_geom.alpha_i + input.raceway_geom.alpha_o) / 2 * Math.PI / 180; const hc = input.macro_geom.h_c ?? input.macro_geom.h_rib / 2; return (input.macro_geom.d_pw / 2 + input.macro_geom.l_we / 2 * Math.sin(g) - input.macro_geom.d_we_max / 2 * Math.cos(g) + hc).toFixed(2); })()})
              </div>
            )}
            <FieldGroup label="Undercut d_uc" value={input.raceway_geom.d_uc} unit="mm" onChange={v => updateRaceway('d_uc', v)} step={0.1} />
            <FieldGroup label="Undercut L_uc" value={input.raceway_geom.l_uc} unit="mm" onChange={v => updateRaceway('l_uc', v)} step={0.1} />
          </SubAccordion>

          {/* 프로파일 */}
          <SubAccordion title="Profile" sectionKey="geom-profile" openSections={openSections} onToggle={toggleSection}>
            <div className="flex items-center justify-between py-1 mb-1">
              <span className="text-[13px] text-text-muted font-medium">Roughness</span>
              <select
                value={input.operating.roughness_input_mode}
                onChange={e => {
                  const mode = e.target.value as 'Ra' | 'Rq';
                  const old = input.operating.roughness_input_mode;
                  if (mode === old) return;
                  const factor = mode === 'Rq' ? 1.25 : 1.0 / 1.25;
                  dispatch({ type: 'UPDATE_INPUT', payload: { operating: {
                    ...input.operating,
                    roughness_input_mode: mode,
                    rq_roller: +(input.operating.rq_roller * factor).toFixed(4),
                    rq_inner: +(input.operating.rq_inner * factor).toFixed(4),
                    rq_outer: +(input.operating.rq_outer * factor).toFixed(4),
                  }}});
                }}
                className="w-32 px-1.5 py-0.5 text-[13px] bg-white border border-slate-200 rounded focus:outline-none focus:border-accent text-text-dark cursor-pointer"
              >
                <option value="Ra">Ra (arithmetic)</option>
                <option value="Rq">Rq (RMS)</option>
              </select>
            </div>
            <SubHeader>Roller Crown</SubHeader>
            <div className="flex items-center justify-between py-0.5">
              <span className="text-[13px] text-text-muted">Crown type</span>
              <select
                value={crownTypeName(input.roller_profile.crown_type)}
                onChange={e => setCrownType(e.target.value)}
                className="w-24 px-1.5 py-0.5 text-[13px] bg-white border border-slate-200 rounded
                           focus:outline-none focus:border-accent text-text-dark cursor-pointer"
              >
                {['Logarithmic', 'Circular', 'Parabolic', 'Polynomial', 'Custom'].map(t => (
                  <option key={t} value={t}>{t}</option>
                ))}
              </select>
            </div>
            <FieldGroup label="Crown drop δ_c" value={input.roller_profile.delta_c} unit="μm" onChange={v => updateRollerProfile('delta_c', v)} step={0.5} />
            {derivedCrownParam && (
              <div className="flex items-center justify-between py-0.5">
                <span className="text-xs text-text-muted italic">→ {derivedCrownParam.label}</span>
                <span className="text-xs text-accent font-mono">{derivedCrownParam.value}</span>
              </div>
            )}
            <FieldGroup label="Dub-off L (large)" value={input.roller_profile.delta_dub_l} unit="μm" onChange={v => updateRollerProfile('delta_dub_l', v)} step={0.5} />
            <FieldGroup label="Dub-off S (small)" value={input.roller_profile.delta_dub_s} unit="μm" onChange={v => updateRollerProfile('delta_dub_s', v)} step={0.5} />
            <FieldGroup label="L_dub large" value={input.roller_profile.l_dub_l} unit="mm" onChange={v => updateRollerProfile('l_dub_l', v)} step={0.1} />
            <FieldGroup label="L_dub small" value={input.roller_profile.l_dub_s} unit="mm" onChange={v => updateRollerProfile('l_dub_s', v)} step={0.1} />
            <FieldGroup label={`${input.operating.roughness_input_mode} roller`} value={input.operating.rq_roller} unit="μm" onChange={v => updateOp('rq_roller', v)} step={0.05} />

            {/* Custom roller profile editor */}
            {'Custom' in input.roller_profile.crown_type && (
              <ProfileEditor
                label="Roller Profile"
                points={input.roller_profile.crown_type.Custom.profile}
                onChange={pts => dispatch({
                  type: 'UPDATE_INPUT',
                  payload: { roller_profile: { ...input.roller_profile, crown_type: { Custom: { profile: pts } } } },
                })}
              />
            )}

            {/* Polynomial coefficient editor */}
            {'Polynomial' in input.roller_profile.crown_type && (
              <PolynomialEditor
                label="Roller Polynomial"
                coeffs={input.roller_profile.crown_type.Polynomial.coeffs}
                onChange={coeffs => dispatch({
                  type: 'UPDATE_INPUT',
                  payload: { roller_profile: { ...input.roller_profile, crown_type: { Polynomial: { coeffs } } } },
                })}
              />
            )}

            <SubHeader>Inner Raceway</SubHeader>
            <FieldGroup label="Crowning δ_rw" value={input.raceway_profile_inner.delta_rw} unit="μm" onChange={v => updateRacewayInner('delta_rw', v)} step={0.5} />
            <FieldGroup label="Waviness W_a" value={input.raceway_profile_inner.w_a} unit="μm" onChange={v => updateRacewayInner('w_a', v)} step={0.1} />
            <FieldGroup label={`${input.operating.roughness_input_mode} inner`} value={input.operating.rq_inner} unit="μm" onChange={v => updateOp('rq_inner', v)} step={0.05} />
            <ProfileToggleEditor
              label="Inner Custom Profile"
              points={input.raceway_profile_inner.custom_profile}
              onChange={pts => dispatch({
                type: 'UPDATE_INPUT',
                payload: { raceway_profile_inner: { ...input.raceway_profile_inner, custom_profile: pts } },
              })}
            />
            <PolynomialToggleEditor
              label="Inner Polynomial"
              coeffs={input.raceway_profile_inner.polynomial_coeffs}
              onChange={coeffs => dispatch({
                type: 'UPDATE_INPUT',
                payload: { raceway_profile_inner: { ...input.raceway_profile_inner, polynomial_coeffs: coeffs } },
              })}
            />

            <SubHeader>Outer Raceway</SubHeader>
            <FieldGroup label="Crowning δ_rw" value={input.raceway_profile_outer.delta_rw} unit="μm" onChange={v => updateRacewayOuter('delta_rw', v)} step={0.5} />
            <FieldGroup label="Waviness W_a" value={input.raceway_profile_outer.w_a} unit="μm" onChange={v => updateRacewayOuter('w_a', v)} step={0.1} />
            <FieldGroup label={`${input.operating.roughness_input_mode} outer`} value={input.operating.rq_outer} unit="μm" onChange={v => updateOp('rq_outer', v)} step={0.05} />
            <ProfileToggleEditor
              label="Outer Custom Profile"
              points={input.raceway_profile_outer.custom_profile}
              onChange={pts => dispatch({
                type: 'UPDATE_INPUT',
                payload: { raceway_profile_outer: { ...input.raceway_profile_outer, custom_profile: pts } },
              })}
            />
            <PolynomialToggleEditor
              label="Outer Polynomial"
              coeffs={input.raceway_profile_outer.polynomial_coeffs}
              onChange={coeffs => dispatch({
                type: 'UPDATE_INPUT',
                payload: { raceway_profile_outer: { ...input.raceway_profile_outer, polynomial_coeffs: coeffs } },
              })}
            />
          </SubAccordion>
        </AccordionSection>

        {/* ── Operating Conditions ── */}
        <AccordionSection title="Operating" sectionKey="load" openSections={openSections} onToggle={toggleSection}>
          <FieldGroup label="Design life" value={input.operating.design_life_hours} unit="hr" onChange={v => updateOp('design_life_hours', v)} step={10} />
          <FieldGroup label="F_x (radial X)" value={input.operating.f_x} unit="kN" onChange={v => updateOp('f_x', v)} step={0.5} />
          <FieldGroup label="F_y (radial Y)" value={input.operating.f_y} unit="kN" onChange={v => updateOp('f_y', v)} step={0.5} />
          <FieldGroup label="F_a (axial)" value={input.operating.f_a} unit="kN" onChange={v => updateOp('f_a', v)} step={0.5} />
          <FieldGroup label="M_x (moment X)" value={input.operating.m_x} unit="kN·m" onChange={v => updateOp('m_x', v)} step={0.1} />
          <FieldGroup label="M_y (moment Y)" value={input.operating.m_y} unit="kN·m" onChange={v => updateOp('m_y', v)} step={0.1} />
          <FieldGroup label="n inner" value={input.operating.n_inner_rpm} unit="rpm" onChange={v => updateOp('n_inner_rpm', v)} step={50} />
          <FieldGroup label="n outer" value={input.operating.n_outer_rpm} unit="rpm" onChange={v => updateOp('n_outer_rpm', v)} step={50} />
          <FieldGroup label="Misalign γ" value={input.operating.gamma} unit="arcmin" onChange={v => updateOp('gamma', v)} step={0.5} />
          <FieldGroup label="Temp" value={input.operating.t_op} unit="°C" onChange={v => updateOp('t_op', v)} step={5} />

          <SubHeader>Lubricant</SubHeader>
          <div className="flex items-center justify-between py-0.5">
            <span className="text-[13px] text-text-muted">Type</span>
            <select
              value={input.operating.lubrication_type}
              onChange={e => {
                const type = e.target.value as 'Oil' | 'Grease';
                dispatch({
                  type: 'UPDATE_INPUT',
                  payload: {
                    operating: {
                      ...input.operating,
                      lubrication_type: type,
                      starvation_factor: type === 'Grease' ? 0.7 : 1.0,
                    },
                  },
                });
              }}
              className="w-24 px-1.5 py-0.5 text-[13px] bg-white border border-slate-200 rounded
                         focus:outline-none focus:border-accent text-text-dark cursor-pointer"
            >
              <option value="Oil">Oil</option>
              <option value="Grease">Grease</option>
            </select>
          </div>
          <FieldGroup label="ν₄₀" value={input.operating.nu_40} unit="mm²/s" onChange={v => updateOp('nu_40', v)} />
          <FieldGroup label="ν₁₀₀" value={input.operating.nu_100} unit="mm²/s" onChange={v => updateOp('nu_100', v)} />
          <FieldGroup label="α_pv" value={input.operating.alpha_pv} unit="1/GPa" onChange={v => updateOp('alpha_pv', v)} step={1} />
          <FieldGroup label="ρ_oil" value={input.operating.rho_oil} unit="kg/m³" onChange={v => updateOp('rho_oil', v)} step={10} />
          <FieldGroup label="φ_s (starvation)" value={input.operating.starvation_factor} unit="" onChange={v => updateOp('starvation_factor', v)} step={0.05} />

          <SubHeader>Contamination (ISO 281 Annex A)</SubHeader>
          <FieldGroup label="e_c override" value={input.solver.e_c} unit="" onChange={v =>
            dispatch({ type: 'UPDATE_INPUT', payload: { solver: { ...input.solver, e_c: v } } })
          } step={0.1} />
          <div className="text-[11px] text-text-muted mb-1">0 = auto (ISO 281 Annex A), &gt;0 = manual</div>
          {input.solver.e_c <= 0 && (<>
            <div className="flex items-center justify-between py-0.5">
              <span className="text-[13px] text-text-muted">Oil supply</span>
              <select
                value={input.solver.oil_supply_method}
                onChange={e => dispatch({ type: 'UPDATE_INPUT', payload: { solver: { ...input.solver, oil_supply_method: e.target.value as 'OilBath' | 'CirculatingWithFilter' | 'Grease' } } })}
                className="w-40 px-1.5 py-0.5 text-[13px] bg-white border border-slate-200 rounded focus:outline-none focus:border-accent text-text-dark cursor-pointer"
              >
                <option value="OilBath">Oil bath / Splash</option>
                <option value="CirculatingWithFilter">Circulating + on-line filter</option>
                <option value="Grease">Grease</option>
              </select>
            </div>
            <div className="flex items-center justify-between py-0.5">
              <span className="text-[13px] text-text-muted">Cleanliness</span>
              <select
                value={input.solver.contamination_level}
                onChange={e => dispatch({ type: 'UPDATE_INPUT', payload: { solver: { ...input.solver, contamination_level: e.target.value as any } } })}
                className="w-40 px-1.5 py-0.5 text-[11px] bg-white border border-slate-200 rounded focus:outline-none focus:border-accent text-text-dark cursor-pointer"
              >
                <option value="HighCleanliness">{'\u2014/13/10 High cleanliness'}</option>
                <option value="NormalCleanliness">{'\u2014/15/12 Normal (typical)'}</option>
                <option value="SlightContamination">{'\u2014/17/14 Slight contam.'}</option>
                <option value="SevereContamination">{'\u2014/19/16 Severe contam.'}</option>
                <option value="VeryHeavyContamination">{'\u2014/21/18 Very heavy'}</option>
              </select>
            </div>
          </>)}
          <SubHeader>Preload</SubHeader>
          <div className="flex items-center justify-between py-0.5">
            <span className="text-[13px] text-text-muted">Mode</span>
            <select
              value={input.operating.preload_mode}
              onChange={e => {
                const mode = e.target.value as 'DisplacementFromForce' | 'DisplacementFromForceIterative' | 'Displacement';
                dispatch({
                  type: 'UPDATE_INPUT',
                  payload: {
                    operating: {
                      ...input.operating,
                      preload_mode: mode,
                      delta_preload_um: mode === 'Displacement' ? input.operating.delta_preload_um : 0.0,
                    },
                  },
                });
              }}
              className="w-32 px-1.5 py-0.5 text-[13px] bg-white border border-slate-200 rounded
                         focus:outline-none focus:border-accent text-text-dark cursor-pointer"
            >
              <option value="DisplacementFromForce">Disp. from Force</option>
              <option value="DisplacementFromForceIterative">Disp. from Force (iter.)</option>
              <option value="Displacement">Displacement</option>
            </select>
          </div>
          {input.operating.preload_mode === 'Displacement' && (
            <FieldGroup label="δ_preload" value={input.operating.delta_preload_um} unit="μm" onChange={v => updateOp('delta_preload_um', v)} step={1} />
          )}
        </AccordionSection>

        {/* ── Material ── */}
        <AccordionSection title="Material" sectionKey="material" openSections={openSections} onToggle={toggleSection}>
          <FieldGroup label="E roller" value={input.material.e_roller} unit="GPa" onChange={v => updateMat('e_roller', v)} />
          <FieldGroup label="E ring" value={input.material.e_ring} unit="GPa" onChange={v => updateMat('e_ring', v)} />
          <FieldGroup label="ν (Poisson)" value={input.material.nu} unit="" onChange={v => updateMat('nu', v)} step={0.01} />
          <FieldGroup label="HRC" value={input.material.hrc} unit="" onChange={v => updateMat('hrc', v)} step={1} />
          <FieldGroup label="ρ roller" value={input.material.density_roller} unit="g/cm³" onChange={v => updateMat('density_roller', v)} step={0.01} />
          <FieldGroup label="ρ ring" value={input.material.density_ring} unit="g/cm³" onChange={v => updateMat('density_ring', v)} step={0.01} />
        </AccordionSection>

        {/* ── Solver ── */}
        <AccordionSection title="Solver" sectionKey="solver" openSections={openSections} onToggle={toggleSection}>
          <FieldGroup label="Slices" value={input.solver.n_slices} unit="" onChange={v =>
            dispatch({ type: 'UPDATE_INPUT', payload: { solver: { ...input.solver, n_slices: v } } })
          } step={5} />
          <FieldGroup label="Max iter" value={input.solver.max_iterations} unit="" onChange={v =>
            dispatch({ type: 'UPDATE_INPUT', payload: { solver: { ...input.solver, max_iterations: v } } })
          } step={50} />
          <FieldGroup label="Conv. tol" value={input.solver.convergence_tol} unit="" onChange={v =>
            dispatch({ type: 'UPDATE_INPUT', payload: { solver: { ...input.solver, convergence_tol: v } } })
          } step={0.0001} />
          <FieldGroup label="Δψ (angular)" value={input.solver.angular_increment_deg} unit="deg" onChange={v =>
            dispatch({ type: 'UPDATE_INPUT', payload: { solver: { ...input.solver, angular_increment_deg: v } } })
          } step={0.5} />
          {/* Beam type */}
          <div className="flex items-center justify-between py-1">
            <span className="text-[13px] text-text-muted">Beam type</span>
            <div className="flex gap-1">
              {(['Timoshenko', 'EulerBernoulli'] as const).map(bt => (
                <button
                  key={bt}
                  onClick={() => setBeamType(bt)}
                  className={`px-2 py-0.5 text-xs rounded font-medium cursor-pointer ${
                    input.solver.beam_type === bt
                      ? 'bg-accent text-white'
                      : 'bg-panel-muted text-text-muted hover:bg-slate-200'
                  }`}
                >
                  {bt === 'EulerBernoulli' ? 'Euler-B' : 'Timo.'}
                </button>
              ))}
            </div>
          </div>

          {/* Rib contact mode */}
          <div className="flex items-center justify-between py-1">
            <span className="text-[13px] text-text-muted">Rib contact</span>
            <div className="flex gap-1">
              {([
                { key: 'PostProcess', label: 'Post' },
                { key: 'Coupled', label: 'Coupled' },
              ] as const).map(({ key, label }) => (
                <button
                  key={key}
                  onClick={() => dispatch({
                    type: 'UPDATE_INPUT',
                    payload: { solver: { ...input.solver, rib_contact_mode: key } },
                  })}
                  className={`px-2 py-0.5 text-xs rounded font-medium cursor-pointer ${
                    input.solver.rib_contact_mode === key
                      ? 'bg-accent text-white'
                      : 'bg-panel-muted text-text-muted hover:bg-slate-200'
                  }`}
                >
                  {label}
                </button>
              ))}
            </div>
          </div>

          {/* Split contact mode */}
          <div className="flex items-center justify-between py-1">
            <span className="text-[13px] text-text-muted">Split contact</span>
            <button
              onClick={() => dispatch({
                type: 'UPDATE_INPUT',
                payload: { solver: { ...input.solver, use_split_contact: !input.solver.use_split_contact } },
              })}
              className={`px-2 py-0.5 text-xs rounded font-medium cursor-pointer ${
                input.solver.use_split_contact
                  ? 'bg-accent text-white'
                  : 'bg-panel-muted text-text-muted hover:bg-slate-200'
              }`}
            >
              {input.solver.use_split_contact ? 'ON' : 'OFF'}
            </button>
          </div>

          {/* ═══ LUBRICATION SETTINGS (5 logical groups) ═══ */}

          {/* ── Group 1: EHL Film Model ── */}
          <SubHeader>EHL Film Model</SubHeader>
          <div className="flex items-center justify-between py-1">
            <span className="text-[13px] text-text-muted">Method</span>
            <div className="flex gap-1">
              {([
                { key: 'Method1_DH' as const, label: 'M1 (DH)' },
                { key: 'Method2_MK' as const, label: 'M2 (MK)' },
                { key: 'Method3_NVM' as const, label: 'M3 (NVM)' },
              ]).map(({ key, label }) => (
                <button
                  key={key}
                  onClick={() => dispatch({
                    type: 'UPDATE_INPUT',
                    payload: { operating: { ...input.operating, lubrication_model: key } },
                  })}
                  className={`px-2 py-0.5 text-xs rounded font-medium cursor-pointer ${
                    input.operating.lubrication_model === key
                      ? 'bg-violet-500 text-white'
                      : 'bg-panel-muted text-text-muted hover:bg-slate-200'
                  }`}
                  title={key === 'Method1_DH' ? 'Dowson-Higginson (1977)' : key === 'Method2_MK' ? 'Masjedi-Khonsari (2015)' : 'Nijenbanning-Venner-Moes (1994)'}
                >
                  {label}
                </button>
              ))}
            </div>
          </div>
          <FieldGroup label="φ_s (starvation)" value={input.operating.starvation_factor} unit="" onChange={v => updateOp('starvation_factor', v)} step={0.05} />
          {(input.operating.lubrication_model === 'Method2_MK' || input.operating.lubrication_model === 'Method3_NVM') && (
            <>
              <div className="flex items-center justify-between py-0.5">
                <span className="text-[13px] text-text-muted">Preset</span>
                <select
                  value=""
                  onChange={e => {
                    const preset = e.target.value;
                    if (!preset) return;
                    const presets: Record<string, { tau: number; z: number; k: number; beta: number }> = {
                      mineral: { tau: 5.0, z: 0.67, k: 0.15, beta: 0.04 },
                      pao:     { tau: 8.0, z: 0.50, k: 0.14, beta: 0.03 },
                      ester:   { tau: 4.0, z: 0.55, k: 0.16, beta: 0.035 },
                    };
                    const p = presets[preset];
                    if (p) {
                      dispatch({
                        type: 'UPDATE_INPUT',
                        payload: { operating: { ...input.operating, tau_eyring: p.tau, z_roelands: p.z, k_fluid: p.k, beta_visc: p.beta } },
                      });
                    }
                  }}
                  className="w-24 px-1.5 py-0.5 text-[13px] bg-white border border-slate-200 rounded focus:outline-none focus:border-accent text-text-dark cursor-pointer"
                >
                  <option value="">Select...</option>
                  <option value="mineral">Mineral oil</option>
                  <option value="pao">PAO</option>
                  <option value="ester">Ester</option>
                </select>
              </div>
              <div className="flex items-center justify-between py-1">
                <span className="text-[13px] text-text-muted">Traction</span>
                <div className="flex gap-1">
                  {([
                    { key: 'Eyring' as const, label: 'Eyring', tip: 'τ = τ₀·sinh⁻¹(η·γ̇/τ₀). Default; suited to raceway low-SRR contacts.' },
                    { key: 'CarreauYasuda' as const, label: 'Carreau', tip: 'η_eff = η_∞ + (η_0−η_∞)·[1+(λγ̇)^a]^((n−1)/a). Recommended for rib-end / high-SRR / EV ULV.' },
                  ]).map(({ key, label, tip }) => (
                    <button
                      key={key}
                      onClick={() => dispatch({
                        type: 'UPDATE_INPUT',
                        payload: { operating: { ...input.operating, traction_model: key } },
                      })}
                      className={`px-2 py-0.5 text-xs rounded font-medium cursor-pointer ${
                        input.operating.traction_model === key
                          ? 'bg-violet-500 text-white'
                          : 'bg-panel-muted text-text-muted hover:bg-slate-200'
                      }`}
                      title={tip}
                    >
                      {label}
                    </button>
                  ))}
                </div>
              </div>
              {input.operating.traction_model === 'Eyring' && (
                <div className="flex items-center gap-1">
                  <div className="flex-1">
                    <FieldGroup label="τ_Eyring" value={input.operating.tau_eyring} unit="MPa" onChange={v => updateOp('tau_eyring', v)} step={0.5} />
                  </div>
                  <button
                    onClick={() => {
                      const alpha = input.operating.alpha_pv * 1e-9;
                      const lambdaLim = input.operating.lubrication_type === 'Grease' ? 0.040 : 0.047;
                      const tauE = alpha > 0 ? (2 * lambdaLim / alpha) / 1e6 : 5.0;
                      updateOp('tau_eyring', Math.round(tauE * 10) / 10);
                    }}
                    className="px-1.5 py-0.5 text-[10px] rounded bg-blue-500/20 text-blue-300 hover:bg-blue-500/30 cursor-pointer whitespace-nowrap"
                    title="Auto-estimate from α_pv (Arana 2019: τ_E = 2Λ/α)"
                  >Auto(α)</button>
                </div>
              )}
              {input.operating.traction_model === 'CarreauYasuda' && (
                <>
                  <FieldGroup
                    label="η_∞/η_0"
                    value={input.operating.carreau_eta_inf_ratio}
                    unit=""
                    onChange={v => updateOp('carreau_eta_inf_ratio', v)}
                    step={0.001}
                  />
                  <FieldGroup
                    label="λ_relax"
                    value={input.operating.carreau_lambda_s * 1e9}
                    unit="ns"
                    onChange={v => updateOp('carreau_lambda_s', v * 1e-9)}
                    step={10}
                  />
                  <FieldGroup
                    label="n (power-law)"
                    value={input.operating.carreau_n}
                    unit=""
                    onChange={v => updateOp('carreau_n', v)}
                    step={0.05}
                  />
                  <FieldGroup
                    label="a (Yasuda)"
                    value={input.operating.carreau_a}
                    unit=""
                    onChange={v => updateOp('carreau_a', v)}
                    step={0.5}
                  />
                </>
              )}
              <FieldGroup label="z (Roelands)" value={input.operating.z_roelands} unit="" onChange={v => updateOp('z_roelands', v)} step={0.01} />
              <FieldGroup label="k_fluid" value={input.operating.k_fluid} unit="W/(m·K)" onChange={v => updateOp('k_fluid', v)} step={0.01} />
              <FieldGroup label="β_visc" value={input.operating.beta_visc} unit="1/K" onChange={v => updateOp('beta_visc', v)} step={0.005} />
            </>
          )}

          {/* ── Friction model selection (always visible) ── */}
          <div className="flex items-center justify-between py-1">
            <span className="text-[13px] text-text-muted">Friction</span>
            <div className="flex gap-1">
              {([
                { key: 'PalmgrenLike' as const, label: 'Palmgren', tip: 'μ_rr·Q·u (default). Simple, no viscosity dependence. Tends to over-predict torque vs. catalogue.' },
                { key: 'BibouletHoupert' as const, label: 'BH 2010', tip: 'Biboulet-Houpert 2010 per-contact F_R = f(IVR, EHL, M). Analytical, captures viscous-hydrodynamic rolling. Best for non-SKF bearings, Gen3 split, research.' },
                { key: 'SkfAdvanced' as const, label: 'SKF', tip: 'SKF Catalogue 2018 bearing-level: M_rr = G_rr·(νn)^0.6. Industry-calibrated, matches SKF Bearing Calculator. Series-specific (302~Other).' },
              ]).map(({ key, label, tip }) => (
                <button
                  key={key}
                  onClick={() => dispatch({
                    type: 'UPDATE_INPUT',
                    payload: { operating: { ...input.operating, friction_model: key } },
                  })}
                  className={`px-2 py-0.5 text-xs rounded font-medium cursor-pointer ${
                    input.operating.friction_model === key
                      ? 'bg-violet-500 text-white'
                      : 'bg-panel-muted text-text-muted hover:bg-slate-200'
                  }`}
                  title={tip}
                >
                  {label}
                </button>
              ))}
            </div>
          </div>
          {/* Thermal-correction selector — only meaningful when BH 2010 is active */}
          {input.operating.friction_model === 'BibouletHoupert' && (
            <div className="flex items-center justify-between py-0.5">
              <span className="text-[13px] text-text-muted">Thermal φ_T</span>
              <div className="flex gap-1">
                {([
                  { key: 'Wilson1979' as const, label: 'Wilson', tip: 'Wilson 1979 φ_T = 1/(1+0.1·L^0.64). Conservative; matches the φ_T already used for film thickness.' },
                  { key: 'Aihara1987' as const, label: 'Aihara', tip: 'Aihara 1987 φ_T = 1/(1+0.29·L^0.78). Calibrated for TRB rolling torque; matches Schwarz 2023 measurements within 8%.' },
                  { key: 'None' as const, label: 'None', tip: 'Isothermal — no thermal correction.' },
                ]).map(({ key, label, tip }) => (
                  <button
                    key={key}
                    onClick={() => dispatch({
                      type: 'UPDATE_INPUT',
                      payload: { operating: { ...input.operating, thermal_correction: key } },
                    })}
                    className={`px-2 py-0.5 text-xs rounded font-medium cursor-pointer ${
                      input.operating.thermal_correction === key
                        ? 'bg-violet-500 text-white'
                        : 'bg-panel-muted text-text-muted hover:bg-slate-200'
                    }`}
                    title={tip}
                  >
                    {label}
                  </button>
                ))}
              </div>
            </div>
          )}
          <div className="flex items-center justify-between py-0.5">
            <span className="text-[13px] text-text-muted">SKF series</span>
            <select
              value={input.operating.skf_trb_series}
              onChange={e => dispatch({
                type: 'UPDATE_INPUT',
                payload: { operating: { ...input.operating, skf_trb_series: e.target.value as any } },
              })}
              className="w-32 px-1.5 py-0.5 text-[13px] bg-white border border-slate-200 rounded focus:outline-none focus:border-accent text-text-dark cursor-pointer"
              title="SKF Catalogue Table 2d series (R/S geometric constants)"
            >
              <option value="Series302">302 (light)</option>
              <option value="Series303">303 (medium)</option>
              <option value="Series313">313 (X)</option>
              <option value="Series320">320 X</option>
              <option value="Series322">322</option>
              <option value="Series322B">322 B</option>
              <option value="Series323">323</option>
              <option value="Series323B">323 B</option>
              <option value="Other">Other</option>
            </select>
          </div>
          <div className="flex items-center justify-between py-0.5">
            <span className="text-[13px] text-text-muted">SKF lubrication</span>
            <select
              value={input.operating.skf_lubrication}
              onChange={e => dispatch({
                type: 'UPDATE_INPUT',
                payload: { operating: { ...input.operating, skf_lubrication: e.target.value as any } },
              })}
              className="w-32 px-1.5 py-0.5 text-[13px] bg-white border border-slate-200 rounded focus:outline-none focus:border-accent text-text-dark cursor-pointer"
              title="Sets K_rs (kinematic starvation): 3e-8 oil bath/jet, 6e-8 grease/oil-air"
            >
              <option value="OilBath">Oil bath</option>
              <option value="OilJet">Oil jet</option>
              <option value="Grease">Grease</option>
              <option value="OilAir">Oil-air</option>
            </select>
          </div>
          <FieldGroup
            label="SKF Y factor"
            value={input.operating.skf_y_factor}
            unit=""
            onChange={v => updateOp('skf_y_factor', v)}
            step={0.1}
          />

          {/* ── Group 2: Surface & Roughness ── */}
          <SubHeader>Surface & Roughness</SubHeader>
          <div className="flex items-center justify-between py-0.5">
            <span className="text-[13px] text-text-muted">Finish</span>
            <select
              value={input.operating.surface_finish}
              onChange={e => dispatch({
                type: 'UPDATE_INPUT',
                payload: { operating: { ...input.operating, surface_finish: e.target.value as any } },
              })}
              className="w-32 px-1.5 py-0.5 text-[13px] bg-white border border-slate-200 rounded focus:outline-none focus:border-accent text-text-dark cursor-pointer"
            >
              <option value="Standard">Standard (Rq 0.3+)</option>
              <option value="FineGround">Fine ground (0.15)</option>
              <option value="Superfinish">Superfinish (&lt;0.1)</option>
            </select>
          </div>
          <div className="flex items-center justify-between py-0.5">
            <span className="text-[13px] text-text-muted">Additive</span>
            <select
              value={input.operating.additive_type}
              onChange={e => dispatch({
                type: 'UPDATE_INPUT',
                payload: { operating: { ...input.operating, additive_type: e.target.value as any } },
              })}
              className="w-32 px-1.5 py-0.5 text-[13px] bg-white border border-slate-200 rounded focus:outline-none focus:border-accent text-text-dark cursor-pointer"
            >
              <option value="None">None</option>
              <option value="EP">EP (×0.8)</option>
              <option value="AW">AW (×0.7)</option>
            </select>
          </div>

          {/* ── Group 3: Film Decay ── */}
          <SubHeader>Film Decay</SubHeader>
          <div className="flex items-center justify-between py-1">
            <span className="text-[13px] text-text-muted">Van Zoelen</span>
            <button
              onClick={() => dispatch({
                type: 'UPDATE_INPUT',
                payload: { operating: { ...input.operating, film_decay_enabled: !input.operating.film_decay_enabled } },
              })}
              className={`px-2 py-0.5 text-xs rounded font-medium cursor-pointer ${
                input.operating.film_decay_enabled
                  ? 'bg-emerald-500 text-white'
                  : 'bg-panel-muted text-text-muted hover:bg-slate-200'
              }`}
              title="Van Zoelen side-flow decay model (2012)"
            >
              {input.operating.film_decay_enabled ? 'ON' : 'OFF'}
            </button>
          </div>
          {input.operating.film_decay_enabled && (
            <>
              <FieldGroup label="Operating time" value={input.operating.film_decay_time_hours} unit="hr" onChange={v => updateOp('film_decay_time_hours', v)} step={100} />
              <FieldGroup label="Skew angle" value={input.operating.skew_angle_deg} unit="°" onChange={v => updateOp('skew_angle_deg', v)} step={0.5} />
              <FieldGroup label="Replenishment R" value={input.operating.replenishment_rate_nm_s} unit="nm/s" onChange={v => updateOp('replenishment_rate_nm_s', v)} step={0.001} />
              <div className="text-[11px] text-text-muted/60 pl-1 pb-1">
                {input.operating.lubrication_type === 'Grease'
                  ? 'Grease: uses base oil viscosity (ν₄₀/ν₁₀₀)'
                  : 'Oil: uses oil viscosity directly'}
              </div>
            </>
          )}

          {/* ── Group 4: Life Rating ── */}
          <SubHeader>Life Rating</SubHeader>
          <div className="flex items-center justify-between py-0.5">
            <span className="text-[13px] text-text-muted">{'\u03BA method'}</span>
            <select
              value={input.solver.kappa_method}
              onChange={e => dispatch({ type: 'UPDATE_INPUT', payload: { solver: { ...input.solver, kappa_method: e.target.value as 'ViscosityRatio' | 'FilmThicknessRatio' } } })}
              className="w-40 px-1.5 py-0.5 text-[13px] bg-white border border-slate-200 rounded focus:outline-none focus:border-accent text-text-dark cursor-pointer"
            >
              <option value="ViscosityRatio">{'\u03BD/\u03BD\u2081 (ISO 281)'}</option>
              <option value="FilmThicknessRatio">{'\u039B\u00B9\u00B7\u00B3 (ISO/TR 1281-2)'}</option>
            </select>
          </div>
          {input.operating.lubrication_model === 'Method2_MK' && input.solver.kappa_method === 'FilmThicknessRatio' && (
            <div className="text-[11px] text-amber-400/80 pl-1 pb-1">
              ⚠ M2 + FilmThicknessRatio: roughness double-counted in κ
            </div>
          )}

          <SubHeader>Load Ratings</SubHeader>
          {/* C_r (dynamic) — auto/manual toggle */}
          <NullableField
            label="C_r (dynamic)"
            value={input.solver.c_r_kn}
            unit="kN"
            step={0.1}
            onChangeValue={v => dispatch({
              type: 'UPDATE_INPUT',
              payload: { solver: { ...input.solver, c_r_kn: v } },
            })}
          />
          {/* C₀ᵣ (static) — auto/manual toggle */}
          <NullableField
            label="C₀ᵣ (static)"
            value={input.solver.c_0r_kn}
            unit="kN"
            step={0.1}
            onChangeValue={v => dispatch({
              type: 'UPDATE_INPUT',
              payload: { solver: { ...input.solver, c_0r_kn: v } },
            })}
          />
          <FieldGroup label="S₀ min" value={input.solver.f_s_min} unit="" onChange={v =>
            dispatch({ type: 'UPDATE_INPUT', payload: { solver: { ...input.solver, f_s_min: v } } })
          } step={0.1} />
          <SubHeader>ISO 15312 Thermal Speed</SubHeader>
          <FieldGroup label="f₀ᵣ (viscous)" value={input.solver.f_0r} unit="" onChange={v =>
            dispatch({ type: 'UPDATE_INPUT', payload: { solver: { ...input.solver, f_0r: v } } })
          } step={0.5} />
          <FieldGroup label="f₁ᵣ (load)" value={input.solver.f_1r} unit="" onChange={v =>
            dispatch({ type: 'UPDATE_INPUT', payload: { solver: { ...input.solver, f_1r: v } } })
          } step={0.00005} />
        </AccordionSection>

        {/* ── Transient ── */}
        <AccordionSection title="Transient" sectionKey="transient" openSections={openSections} onToggle={toggleSection}>
          {/* Source Type Tabs */}
          <div className="flex rounded-md overflow-hidden border border-slate-200 mb-2">
            {(['sine', 'csv'] as LoadSourceType[]).map(t => (
              <button
                key={t}
                onClick={() => setLoadSourceType(t)}
                className={`flex-1 py-1 text-[12px] font-medium cursor-pointer transition-colors ${
                  loadSourceType === t
                    ? 'bg-accent text-white'
                    : 'bg-slate-50 text-text-muted hover:bg-slate-100'
                }`}
              >
                {t === 'sine' ? 'Sine Wave' : 'Custom (CSV)'}
              </button>
            ))}
          </div>

          {/* ── Sine Wave Config ── */}
          {loadSourceType === 'sine' && (
            <div className="space-y-1">
              <FieldGroup label="Frequency" value={sineConfig.frequency_hz} unit="Hz" onChange={v => setSineConfig(p => ({ ...p, frequency_hz: v }))} step={1} />
              <FieldGroup label="Duration" value={sineConfig.duration_s} unit="s" onChange={v => setSineConfig(p => ({ ...p, duration_s: v }))} step={0.1} />
              <FieldGroup label="Pts/cycle" value={sineConfig.points_per_cycle} unit="" onChange={v => setSineConfig(p => ({ ...p, points_per_cycle: Math.max(4, Math.round(v)) }))} step={10} />

              {/* Channel table */}
              <div className="mt-1">
                <div className="grid grid-cols-3 gap-1 text-[10px] text-text-muted font-semibold uppercase tracking-wider px-0.5 mb-0.5">
                  <span>Channel</span><span className="text-center">Mean</span><span className="text-center">Amplitude</span>
                </div>
                {([
                  ['f_x', 'Fx', 'kN'],
                  ['f_y', 'Fy', 'kN'],
                  ['f_a', 'Fa', 'kN'],
                  ['m_x', 'Mx', 'kN·m'],
                  ['m_y', 'My', 'kN·m'],
                  ['n_rpm', 'RPM', 'rpm'],
                ] as [keyof SineWaveConfig, string, string][]).map(([ch, label, unit]) => {
                  const p = sineConfig[ch] as { mean: number; amplitude: number };
                  return (
                    <div key={ch} className="grid grid-cols-[60px_1fr_1fr] gap-1 items-center">
                      <span className="text-[11px] text-text-muted truncate">{label} <span className="text-[9px]">[{unit}]</span></span>
                      <input
                        type="number"
                        value={p.mean}
                        onChange={e => updateSineChannel(ch, 'mean', parseFloat(e.target.value) || 0)}
                        className="w-full px-1 py-0.5 text-[12px] text-right rounded border border-slate-200 bg-white focus:border-accent focus:outline-none"
                      />
                      <input
                        type="number"
                        value={p.amplitude}
                        onChange={e => updateSineChannel(ch, 'amplitude', parseFloat(e.target.value) || 0)}
                        className="w-full px-1 py-0.5 text-[12px] text-right rounded border border-slate-200 bg-white focus:border-accent focus:outline-none"
                      />
                    </div>
                  );
                })}
              </div>

              {/* Sine preview */}
              {loadSeries.length > 0 && (
                <SinePreviewMini points={loadSeries} />
              )}
            </div>
          )}

          {/* ── CSV Mode ── */}
          {loadSourceType === 'csv' && (
            <>
              <div className="flex items-center gap-2 py-1">
                <button
                  onClick={loadCsv}
                  className="flex-1 py-1.5 text-[13px] rounded bg-slate-100 text-text-dark hover:bg-slate-200 font-medium cursor-pointer transition-colors"
                >
                  Load CSV
                </button>
                <span className="text-xs text-text-muted font-mono">
                  {loadSeries.length > 0 ? `${loadSeries.length} pts` : 'No data'}
                </span>
              </div>
            </>
          )}

          {/* Common: load series info */}
          {loadSeries.length > 0 && (
            <div className="text-[11px] text-text-muted space-y-0.5 mb-1">
              <div>t: {loadSeries[0].t_s.toFixed(4)}s → {loadSeries[loadSeries.length - 1].t_s.toFixed(4)}s ({loadSeries.length} pts)</div>
              <div>n: {Math.min(...loadSeries.map(p => p.n_rpm)).toFixed(0)} ~ {Math.max(...loadSeries.map(p => p.n_rpm)).toFixed(0)} rpm</div>
            </div>
          )}

          <FieldGroup label="Δt max" value={dtMax} unit="s" onChange={setDtMax} step={0.0001} />
          <FieldGroup label="Snapshot interval" value={snapshotInterval} unit="" onChange={v => setSnapshotInterval(Math.max(1, Math.round(v)))} step={1} />

          {/* Dynamics toggle */}
          <div className="flex items-center justify-between py-1">
            <span className="text-[13px] text-text-muted">Roller dynamics</span>
            <button
              onClick={() => setEnableDynamics(!enableDynamics)}
              className={`px-2 py-0.5 text-xs rounded font-medium cursor-pointer ${
                enableDynamics
                  ? 'bg-accent/15 text-accent'
                  : 'bg-slate-100 text-slate-400 hover:bg-slate-200'
              }`}
            >
              {enableDynamics ? 'ON' : 'OFF'}
            </button>
          </div>

          {/* Solve Transient button */}
          <button
            onClick={solveTransient}
            disabled={loading || loadSeries.length < 2}
            className={`w-full py-1.5 mt-2 rounded-md text-[13px] font-medium transition-all cursor-pointer ${
              loading || loadSeries.length < 2
                ? 'bg-slate-200 text-slate-400 cursor-not-allowed'
                : 'bg-orange-500 text-white hover:bg-orange-600 active:scale-[0.98]'
            }`}
          >
            {loading ? 'Solving...' : 'Solve Transient'}
          </button>
        </AccordionSection>
      </div>
    </div>
  );
}

/* ── Sub-components ── */

function SubHeader({ children }: { children: React.ReactNode }) {
  return (
    <div className="text-xs font-semibold text-accent uppercase tracking-wider pt-2 pb-0.5 border-t border-slate-100 mt-1">
      {children}
    </div>
  );
}

function AccordionSection({
  title, sectionKey, openSections, onToggle, children,
}: {
  title: string; sectionKey: string; openSections: Set<string>; onToggle: (key: string) => void;
  children: React.ReactNode;
}) {
  const isOpen = openSections.has(sectionKey);
  return (
    <div>
      <button
        onClick={() => onToggle(sectionKey)}
        className="w-full flex items-center justify-between px-3 py-2 text-[13px] font-semibold text-text-dark uppercase tracking-wider border-b border-panel-border hover:bg-panel-muted transition-colors cursor-pointer"
      >
        <span>{title}</span>
        <span className={`text-xs transition-transform ${isOpen ? 'rotate-90' : ''}`}>▶</span>
      </button>
      {isOpen && <div className="px-3 pb-3 pt-1 space-y-0.5">{children}</div>}
    </div>
  );
}

function SubAccordion({
  title, sectionKey, openSections, onToggle, children,
}: {
  title: string; sectionKey: string; openSections: Set<string>; onToggle: (key: string) => void;
  children: React.ReactNode;
}) {
  const isOpen = openSections.has(sectionKey);
  return (
    <div className="border-l-2 border-accent/30 ml-0.5">
      <button
        onClick={() => onToggle(sectionKey)}
        className="w-full flex items-center justify-between px-2 py-1.5 text-[11px] font-semibold text-accent uppercase tracking-wider hover:bg-accent/5 transition-colors cursor-pointer"
      >
        <span>{title}</span>
        <span className={`text-[11px] transition-transform ${isOpen ? 'rotate-90' : ''}`}>▶</span>
      </button>
      {isOpen && <div className="px-2 pb-2 pt-0.5 space-y-0.5">{children}</div>}
    </div>
  );
}

/** Nullable number field with Auto/Manual toggle (for Option<f64> fields) */
function NullableField({ label, value, unit, step = 1, onChangeValue }: {
  label: string;
  value: number | null;
  unit: string;
  step?: number;
  onChangeValue: (v: number | null) => void;
}) {
  const isAuto = value === null;
  const [text, setText] = useState(value !== null ? String(value) : '');

  const toggleAuto = () => {
    if (isAuto) {
      onChangeValue(0);
      setText('0');
    } else {
      onChangeValue(null);
    }
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setText(e.target.value);
    const n = e.target.valueAsNumber;
    if (!isNaN(n) && isFinite(n)) onChangeValue(n);
  };

  const commit = () => {
    const n = parseFloat(text);
    if (!isNaN(n)) onChangeValue(n);
    else setText(value !== null ? String(value) : '');
  };

  return (
    <div className="flex items-center justify-between gap-2 py-0.5">
      <label className="text-xs text-text-muted whitespace-nowrap min-w-0 truncate flex-1">{label}</label>
      <div className="flex items-center gap-1">
        <button
          onClick={toggleAuto}
          className={`px-1.5 py-0.5 text-[11px] rounded font-medium cursor-pointer ${
            isAuto ? 'bg-emerald-100 text-emerald-700' : 'bg-slate-100 text-slate-500 hover:bg-slate-200'
          }`}
          title={isAuto ? 'Auto-calculated from geometry' : 'Manual override'}
        >
          {isAuto ? 'Auto' : 'Man.'}
        </button>
        {!isAuto && (
          <>
            <input
              type="number"
              value={text}
              step={step}
              onChange={handleChange}
              onBlur={commit}
              onKeyDown={e => e.key === 'Enter' && commit()}
              className="w-16 px-1.5 py-0.5 text-xs text-right bg-white border border-slate-200 rounded font-mono tabular-nums
                         focus:outline-none focus:border-accent focus:ring-1 focus:ring-accent/30 text-text-dark"
            />
            {unit && <span className="text-xs text-text-muted w-6">{unit}</span>}
          </>
        )}
      </div>
    </div>
  );
}

/** Profile coordinate editor for roller CrownType::Custom */
function ProfileEditor({ label, points, onChange }: {
  label: string;
  points: [number, number][];
  onChange: (pts: [number, number][]) => void;
}) {
  const addPoint = () => onChange([...points, [0, 0]]);
  const removePoint = (idx: number) => onChange(points.filter((_, i) => i !== idx));
  const updatePoint = (idx: number, axis: 0 | 1, val: number) => {
    const next = points.map((p, i) => i === idx ? (axis === 0 ? [val, p[1]] : [p[0], val]) as [number, number] : p);
    onChange(next);
  };

  const handlePaste = (e: React.ClipboardEvent) => {
    const text = e.clipboardData.getData('text');
    const lines = text.trim().split(/\r?\n/).map(l => l.split(/[\t,;\s]+/).map(Number));
    const valid = lines.filter(l => l.length >= 2 && l.every(n => !isNaN(n)));
    if (valid.length > 0) {
      e.preventDefault();
      onChange(valid.map(l => [l[0], l[1]] as [number, number]));
    }
  };

  return (
    <div className="mt-1 border border-slate-200 rounded p-1.5" onPaste={handlePaste}>
      <div className="flex items-center justify-between mb-1">
        <span className="text-xs text-text-muted font-medium">{label}</span>
        <button onClick={addPoint} className="text-[11px] px-1.5 py-0.5 bg-accent/10 text-accent rounded hover:bg-accent/20 cursor-pointer">+ Add</button>
      </div>
      {points.length === 0 && (
        <div className="text-[11px] text-slate-400 text-center py-1">No points. Paste X,Δz data or add manually.</div>
      )}
      {points.length > 0 && (
        <div className="space-y-0.5 max-h-32 overflow-y-auto custom-scrollbar">
          <div className="flex gap-1 text-[10px] text-slate-400 uppercase px-0.5">
            <span className="w-16 text-center">X [mm]</span>
            <span className="w-16 text-center">Δz [μm]</span>
          </div>
          {points.map((pt, i) => (
            <div key={i} className="flex gap-1 items-center">
              <input type="number" value={pt[0]} step={0.1}
                onChange={e => updatePoint(i, 0, e.target.valueAsNumber || 0)}
                className="w-16 px-1 py-0.5 text-xs text-right bg-white border border-slate-200 rounded font-mono focus:outline-none focus:border-accent" />
              <input type="number" value={pt[1]} step={0.1}
                onChange={e => updatePoint(i, 1, e.target.valueAsNumber || 0)}
                className="w-16 px-1 py-0.5 text-xs text-right bg-white border border-slate-200 rounded font-mono focus:outline-none focus:border-accent" />
              <button onClick={() => removePoint(i)} className="text-[11px] text-red-400 hover:text-red-600 cursor-pointer px-0.5">×</button>
            </div>
          ))}
        </div>
      )}
      <div className="text-[10px] text-slate-400 mt-0.5">Tip: Paste tab/comma-separated X,Δz data</div>
    </div>
  );
}

/** 4th-order polynomial coefficient editor: val(x) = p1·x⁴ + p2·x³ + p3·x² + p4·x + p5 */
function PolynomialEditor({ label, coeffs, onChange }: {
  label: string;
  coeffs: number[];
  onChange: (coeffs: number[]) => void;
}) {
  const labels = ['p1 (x⁴)', 'p2 (x³)', 'p3 (x²)', 'p4 (x)', 'p5 (const)'];
  const padded = [...coeffs, ...Array(5 - coeffs.length).fill(0)].slice(0, 5) as number[];

  const updateCoeff = (idx: number, val: number) => {
    const next = [...padded];
    next[idx] = val;
    onChange(next);
  };

  return (
    <div className="mt-1 border border-slate-200 rounded p-1.5">
      {label && (
        <div className="text-xs text-text-muted font-medium mb-1">{label}</div>
      )}
      <div className="text-[10px] text-slate-400 mb-1 font-mono">
        val(x) = p1·x⁴ + p2·x³ + p3·x² + p4·x + p5 [μm]
      </div>
      <div className="space-y-0.5">
        {labels.map((lbl, i) => (
          <div key={i} className="flex items-center gap-1">
            <span className="text-[11px] text-text-muted w-14 text-right">{lbl}</span>
            <input
              type="number"
              value={padded[i]}
              step={0.001}
              onChange={e => updateCoeff(i, e.target.valueAsNumber || 0)}
              className="flex-1 px-1 py-0.5 text-xs text-right bg-white border border-slate-200 rounded font-mono
                         focus:outline-none focus:border-accent"
            />
          </div>
        ))}
      </div>
    </div>
  );
}

/** Polynomial editor with ON/OFF toggle (for raceway polynomial_coeffs) */
function PolynomialToggleEditor({ label, coeffs, onChange }: {
  label: string;
  coeffs: number[] | null;
  onChange: (coeffs: number[] | null) => void;
}) {
  const enabled = coeffs !== null;
  const prevRef = useRef<number[] | null>(null);
  return (
    <div className="mt-1">
      <div className="flex items-center gap-2 py-0.5">
        <label className="text-xs text-text-muted flex-1">{label}</label>
        <button
          onClick={() => {
            if (enabled) {
              prevRef.current = coeffs;
              onChange(null);
            } else {
              onChange(prevRef.current ?? [0, 0, 0, 0, 0]);
            }
          }}
          className={`px-1.5 py-0.5 text-[11px] rounded font-medium cursor-pointer ${
            enabled ? 'bg-accent/15 text-accent' : 'bg-slate-100 text-slate-400 hover:bg-slate-200'
          }`}
        >
          {enabled ? 'ON' : 'OFF'}
        </button>
      </div>
      {enabled && (
        <PolynomialEditor label="" coeffs={coeffs} onChange={c => onChange(c)} />
      )}
    </div>
  );
}

/** Raceway custom profile with enable/disable toggle (for Option<Vec<(f64,f64)>> fields) */
function ProfileToggleEditor({ label, points, onChange }: {
  label: string;
  points: [number, number][] | null;
  onChange: (pts: [number, number][] | null) => void;
}) {
  const enabled = points !== null;
  const prevRef = useRef<[number, number][] | null>(null);
  return (
    <div className="mt-1">
      <div className="flex items-center gap-2 py-0.5">
        <label className="text-xs text-text-muted flex-1">{label}</label>
        <button
          onClick={() => {
            if (enabled) {
              prevRef.current = points;
              onChange(null);
            } else {
              onChange(prevRef.current ?? []);
            }
          }}
          className={`px-1.5 py-0.5 text-[11px] rounded font-medium cursor-pointer ${
            enabled ? 'bg-accent/15 text-accent' : 'bg-slate-100 text-slate-400 hover:bg-slate-200'
          }`}
        >
          {enabled ? 'ON' : 'OFF'}
        </button>
      </div>
      {enabled && (
        <ProfileEditor label="" points={points} onChange={onChange as (pts: [number, number][]) => void} />
      )}
    </div>
  );
}

/** Mini SVG preview chart for sine wave load series */
function SinePreviewMini({ points }: { points: LoadTimePoint[] }) {
  if (points.length < 2) return null;

  const W = 220, H = 60, PAD = 4;
  const t0 = points[0].t_s;
  const tEnd = points[points.length - 1].t_s;
  const dt = tEnd - t0 || 1;

  // 가장 변동이 큰 채널 자동 선택
  const channels: { key: keyof LoadTimePoint; label: string; color: string }[] = [
    { key: 'f_a', label: 'Fa', color: '#3b82f6' },
    { key: 'f_x', label: 'Fx', color: '#ef4444' },
    { key: 'f_y', label: 'Fy', color: '#22c55e' },
    { key: 'n_rpm', label: 'RPM', color: '#f59e0b' },
  ];

  const activeChannels = channels.filter(ch => {
    const vals = points.map(p => p[ch.key] as number);
    const mn = Math.min(...vals), mx = Math.max(...vals);
    return mx - mn > 1e-9;
  });

  if (activeChannels.length === 0) {
    // 변동 없으면 Fa를 기본 표시
    activeChannels.push(channels[0]);
  }

  const makePath = (ch: { key: keyof LoadTimePoint; color: string }) => {
    const vals = points.map(p => p[ch.key] as number);
    const mn = Math.min(...vals), mx = Math.max(...vals);
    const range = mx - mn || 1;
    return points.map((p, i) => {
      const x = PAD + ((p.t_s - t0) / dt) * (W - 2 * PAD);
      const y = H - PAD - ((vals[i] - mn) / range) * (H - 2 * PAD);
      return `${i === 0 ? 'M' : 'L'}${x.toFixed(1)},${y.toFixed(1)}`;
    }).join(' ');
  };

  return (
    <div className="mt-1 bg-slate-50 rounded border border-slate-200 p-1">
      <svg width={W} height={H} className="w-full" viewBox={`0 0 ${W} ${H}`} preserveAspectRatio="none">
        {activeChannels.map(ch => (
          <path key={ch.key} d={makePath(ch)} fill="none" stroke={ch.color} strokeWidth="1.5" />
        ))}
      </svg>
      <div className="flex gap-2 justify-center mt-0.5">
        {activeChannels.map(ch => (
          <span key={ch.key} className="text-[9px] font-medium" style={{ color: ch.color }}>{ch.label}</span>
        ))}
      </div>
    </div>
  );
}
