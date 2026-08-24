// BB 입력 패널 (Plan §3.6.5.2 S2 · §3.6.4.3 최소 변경)
//
// 기존 TRB `components/InputPanel` (1 696줄) 을 개조하지 않고 **새로 만든다**.
// 개조보다 신규가 변경량이 훨씬 적기 때문이다 (§3.6.4.3 처분표).
// 기존 파일은 **지우지 않는다** — 미개조 8탭이 아직 그 데이터 모델을 쓴다.
//
// ─────────────────────────────────────────────────────────────────────
//  단위 정책 (확정 — 자의적으로 바꾸지 말 것)
// ─────────────────────────────────────────────────────────────────────
//  이 단계의 목적은 **솔버 검증**이다. 따라서 **화면 숫자 = 검증 숫자**를
//  최우선한다 (§3.6.4.2). 솔버 내부 단위는 mm · N · rad (D-10) 이므로
//  화면도 그대로 간다.
//
//  | 대상                                  | 표시 단위 |
//  |---------------------------------------|-----------|
//  | 기하 (bore·OD·width·D_w·D_pw·r_i·r_e) | mm        |
//  | 하중 f_*_n                            | N         |
//  | 모멘트 m_*_nmm                        | N·mm      |
//  | 변위형 지정값 (BbDof::Prescribed x·y·z)| mm        |
//  | 접촉각 계열 (alpha_nom_rad,           |           |
//  |   BbClearanceSpec::InitialAngleRad)   | ° (내부 rad) |
//  | 틸트 γ (BbDof::Prescribed gy·gz)      | **rad 유지** |
//
//  ⚠ **틸트만 rad 인 이유** — 「각도는 °」의 유일한 예외다.
//    Level **D-2a·D-2d 판정이 rad 로 출력**된다. °로 바꿔 보여주면 화면 숫자와
//    검증 숫자가 달라져 대조가 한 단계 꼬인다. 틸트(misalignment)를 rad 또는
//    mrad 로 읽는 것은 공학 관행이기도 하다 (ISO 16281 의 ψ 도 rad).
//    접촉각은 반대다 — 0,698 rad 는 사람이 읽을 수 없으므로 °로 보여준다.
//
//  ⚠ **환산 코드는 접촉각 계열에만 존재한다.** 그 외 어떤 필드에도
//    환산이 있어서는 안 된다 (있으면 위 원칙이 깨진 것이다).
//
// ─────────────────────────────────────────────────────────────────────
//  필드 순서
// ─────────────────────────────────────────────────────────────────────
//  `src/bb/generated/` (ts-rs 자동생성) 를 SSOT 로 삼고, 각 그룹 안의 필드
//  순서를 **Rust struct 선언 순서와 동일**하게 둔다. 순서를 임의로 정하면
//  그 자체가 자의적 판단이고, Rust 쪽이 바뀌었을 때 대조가 어려워진다.
//
// ─────────────────────────────────────────────────────────────────────
//  입력 검증
// ─────────────────────────────────────────────────────────────────────
//  프론트는 **형식 검증(숫자 파싱 실패)만** 한다. min/max 같은 물리적 타당성
//  범위를 프론트에서 임의로 만들지 않는다 — 근거 없는 자의적 판단이 된다.
//  물리적 타당성은 Rust `validate()` 가 판정하고, 그 **오류 메시지를 그대로**
//  화면에 띄운다 (§3.6.5.3).

import { useCallback, useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useAppState } from '../store';
import type { BbInput } from './generated/BbInput';
import type { BbResult } from './generated/BbResult';
import type { BallBearingGeometry } from './generated/BallBearingGeometry';
import type { BallBearingKind } from './generated/BallBearingKind';
import type { BbClearanceSpec } from './generated/BbClearanceSpec';
import type { BbDof } from './generated/BbDof';
import type { BbDofMask } from './generated/BbDofMask';
import type { BbOperatingConditions } from './generated/BbOperatingConditions';
import type { BbPreloadModel } from './generated/BbPreloadModel';
import type { BbSolverParams } from './generated/BbSolverParams';
import type { Material } from './generated/Material';

interface PresetInfo {
  name: string;
  modified: string;
}

// 접촉각 계열 전용 환산. 이 두 개 말고 다른 환산은 이 파일에 없어야 한다.
const toDeg = (rad: number) => (rad * 180) / Math.PI;
const toRad = (deg: number) => (deg * Math.PI) / 180;

// ── BbDofMask 빠른 선택 ────────────────────────────────────────────────
// Rust `impl BbDofMask` 의 연관상수와 **같은 값**이어야 한다
// (src-tauri/src/solver/bb/types.rs — FULL / ISO_3DOF).
// ISO_3DOF 는 δ_z·γ_y 를 0 으로 구속해 ISO 16281 A.2.2 와 항등이 되는 모드이고,
// Level D-1 (Harris Table 7.4 대조) 이 이 모드에서 수행된다.
const DOF_FULL: BbDofMask = { x: 'Free', y: 'Free', z: 'Free', gy: 'Free', gz: 'Free' };
const DOF_ISO_3DOF: BbDofMask = {
  x: 'Free',
  y: 'Free',
  z: { Prescribed: 0.0 },
  gy: { Prescribed: 0.0 },
  gz: 'Free',
};

// 비활성 변종의 사유. Rust `BbInput::validate()` 가 이미 거부한다 —
// UI 는 그 사실을 **반영만** 하고 독자적으로 판단하지 않는다.
const KIND_DISABLED_REASON: Partial<Record<BallBearingKind, string>> = {
  Dgbb:
    '심구(DGBB)는 아직 지원하지 않습니다. 솔버 코어(기하·접촉·5-DOF 평형)는 ACBB 와 ' +
    '동일하게 동작하나 ISO 281 X/Y 계수(α = 0 행)를 확보하지 못해 수명 산출이 불가능합니다 — 신 P5(수명) 범위.',
  FourPoint:
    '4점접촉(4PCBB)은 아직 지원하지 않습니다. 궤도당 곡률중심이 2개이고 볼당 접촉이 최대 4점이라 ' +
    '평형 모듈이 미구현입니다 — 기하·접촉(Hertz)은 그대로 재사용 가능합니다.',
};

const KINDS: readonly BallBearingKind[] = ['Acbb', 'Dgbb', 'FourPoint'];

type ClearanceTag = 'DiametralMm' | 'InitialAngleRad' | 'AxialPreloadN';
const CLEARANCE_TAGS: readonly ClearanceTag[] = ['DiametralMm', 'InitialAngleRad', 'AxialPreloadN'];
const CLEARANCE_UNIT: Record<ClearanceTag, string> = {
  DiametralMm: 'mm',
  InitialAngleRad: '°', // 내부는 rad — 표시만 °
  AxialPreloadN: 'N',
};
const CLEARANCE_LABEL: Record<ClearanceTag, string> = {
  DiametralMm: '직경 클리어런스',
  InitialAngleRad: '초기 접촉각 α₀',
  AxialPreloadN: '축방향 예압',
};

function clearanceTag(spec: BbClearanceSpec): ClearanceTag {
  return Object.keys(spec)[0] as ClearanceTag;
}
function clearanceValue(spec: BbClearanceSpec): number {
  return Object.values(spec)[0] as number;
}

type DofKey = keyof BbDofMask;
const DOF_KEYS: readonly DofKey[] = ['x', 'y', 'z', 'gy', 'gz'];
// δ 는 mm, γ 는 rad — 위 단위 정책의 예외 항목이 여기에 그대로 드러난다.
const DOF_UNIT: Record<DofKey, string> = { x: 'mm', y: 'mm', z: 'mm', gy: 'rad', gz: 'rad' };
const DOF_LABEL: Record<DofKey, string> = {
  x: 'δ_x (축)',
  y: 'δ_y',
  z: 'δ_z',
  gy: 'γ_y',
  gz: 'γ_z',
};

// ── 소품 ──────────────────────────────────────────────────────────────

/**
 * 숫자 입력 한 칸.
 *
 * **형식 검증만** 한다 — 파싱에 실패하면 직전 값으로 되돌린다.
 * `min`/`max` 를 일부러 두지 않았다 (위 헤더 「입력 검증」 참조).
 */
function NumField({
  label,
  value,
  unit,
  onChange,
  title,
}: {
  label: string;
  value: number;
  unit: string;
  onChange: (v: number) => void;
  title?: string;
}) {
  const [text, setText] = useState(String(value));
  const [focused, setFocused] = useState(false);
  // 부모(프리셋 로드 등)가 값을 바꾸면 편집 중이 아닐 때만 표시를 따라간다.
  // `useEffect` 대신 **렌더 중 상태 조정** 패턴을 쓴다 (React 공식 권장 —
  // effect 안의 setState 는 연쇄 렌더를 부르고 `react-hooks/set-state-in-effect` 에 걸린다).
  const [syncedValue, setSyncedValue] = useState(value);
  if (!focused && value !== syncedValue) {
    setSyncedValue(value);
    setText(String(value));
  }

  const commit = (raw: string) => {
    const n = Number(raw);
    if (raw.trim() !== '' && Number.isFinite(n)) onChange(n);
    else setText(String(value)); // 형식 오류 → 직전 값 복원
  };

  return (
    <div className="flex items-center justify-between gap-2 py-0.5" title={title}>
      <label className="text-[13px] text-text-muted whitespace-nowrap min-w-0 truncate flex-1">
        {label}
      </label>
      <div className="flex items-center gap-1">
        <input
          type="number"
          value={text}
          onChange={e => {
            setText(e.target.value);
            const n = Number(e.target.value);
            if (e.target.value.trim() !== '' && Number.isFinite(n)) onChange(n);
          }}
          onFocus={() => setFocused(true)}
          onBlur={e => {
            setFocused(false);
            commit(e.target.value);
          }}
          onKeyDown={e => e.key === 'Enter' && commit((e.target as HTMLInputElement).value)}
          className="w-24 px-1.5 py-0.5 text-[13px] text-right bg-white border border-slate-200 rounded font-mono tabular-nums
                     focus:outline-none focus:border-accent focus:ring-1 focus:ring-accent/30 text-text-dark"
        />
        <span className="text-xs text-text-muted w-10">{unit}</span>
      </div>
    </div>
  );
}

function Section({
  title,
  open,
  onToggle,
  children,
}: {
  title: string;
  open: boolean;
  onToggle: () => void;
  children: React.ReactNode;
}) {
  return (
    <div className="border-b border-panel-border">
      <button
        onClick={onToggle}
        className="w-full flex items-center justify-between px-3 py-2 text-[13px] font-semibold text-text-dark hover:bg-black/5 cursor-pointer"
      >
        <span>{title}</span>
        <span className="text-text-muted">{open ? '▾' : '▸'}</span>
      </button>
      {open && <div className="px-3 pb-3">{children}</div>}
    </div>
  );
}

// ── 본체 ──────────────────────────────────────────────────────────────

export default function BbInputPanel() {
  const { state, dispatch } = useAppState();
  const { loading } = state;

  // P4-S3-1: BB 입력은 **store 로 승격**됐다 (`state.bbInput`).
  //   `BbGeometryView` 가 `bb_compute_geometry` 를 부르려면 입력을 공유해야 하기
  //   때문이며, S2 설계선택 #1 이 예고한 그대로다.
  // ⚠ `store.input`(TRB `BearingInput`) 은 손대지 않는다 — 별도 필드로 공존시킨다.
  const input = state.bbInput;
  const setInput = useCallback(
    (next: BbInput) => dispatch({ type: 'SET_BB_INPUT', payload: next }),
    [dispatch],
  );

  // 기본값을 TS 로 다시 적지 않는다 — Rust `presets.rs` 가 유일한 출처다.
  // 따라서 프리셋이 로드되기 전까지는 폼을 그리지 않는다.
  const [presets, setPresets] = useState<PresetInfo[]>([]);
  const [selectedPreset, setSelectedPreset] = useState('');
  const [openSections, setOpenSections] = useState<Set<string>>(
    () => new Set(['geometry', 'operating']),
  );
  const [saveFlash, setSaveFlash] = useState(false);

  // 라디오를 전환해도 값이 날아가지 않도록 변종별 초안을 들고 있는다.
  // ⚠ 세 지정방식 사이를 **자동 환산하지 않는다** — α₀ ↔ 직경클리어런스 ↔ 예압
  //   환산은 솔버 내부 관계식(식 A.1 및 그 역산)이라 프론트가 흉내내면
  //   화면 숫자와 솔버 숫자가 갈라진다. 미선택 변종의 초기값은 **0** 이다
  //   (0 은 셋 다 물리적으로 성립하는 중립값이고, 아니면 validate() 가 잡는다).
  const [clearanceDraft, setClearanceDraft] = useState<Record<ClearanceTag, number>>({
    DiametralMm: 0,
    InitialAngleRad: 0,
    AxialPreloadN: 0,
  });
  // `Free` ↔ `Prescribed` 토글 시 되돌아올 지정값.
  const [dofDraft, setDofDraft] = useState<Record<DofKey, number>>({
    x: 0,
    y: 0,
    z: 0,
    gy: 0,
    gz: 0,
  });

  const adoptInput = useCallback((next: BbInput) => {
    setInput(next);
    const tag = clearanceTag(next.geometry.clearance);
    setClearanceDraft(prev => ({ ...prev, [tag]: clearanceValue(next.geometry.clearance) }));
    setDofDraft(prev => {
      const merged = { ...prev };
      for (const k of DOF_KEYS) {
        const d = next.solver.dof_mask[k];
        if (typeof d === 'object') merged[k] = d.Prescribed;
      }
      return merged;
    });
  }, [setInput]);

  const refreshPresets = useCallback(async () => {
    const list = await invoke<PresetInfo[]>('bb_preset_list');
    setPresets(list);
    return list;
  }, []);

  const loadPreset = useCallback(
    async (name: string) => {
      const data = await invoke<BbInput>('bb_preset_load', { name });
      adoptInput(data);
      setSelectedPreset(name);
      invoke('bb_preset_save_last', { name }).catch(() => {});
    },
    [adoptInput],
  );

  // 기동 시: 기본 프리셋 보장 → 마지막 프리셋(없으면 첫 번째) 복원
  useEffect(() => {
    (async () => {
      try {
        await invoke('bb_preset_ensure_default');
        const list = await refreshPresets();
        const last = await invoke<string | null>('bb_preset_get_last');
        const name = last && list.some(p => p.name === last) ? last : list[0]?.name;
        if (name) await loadPreset(name);
      } catch (e) {
        dispatch({ type: 'SET_ERROR', payload: `프리셋 초기화 실패: ${e}` });
      }
    })();
    // 기동 시 1회만.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const toggleSection = (key: string) =>
    setOpenSections(prev => {
      const next = new Set(prev);
      if (next.has(key)) next.delete(key);
      else next.add(key);
      return next;
    });

  // ── 부분 갱신 도우미 ──
  // 부분 갱신은 `UPDATE_BB_INPUT`(얕은 병합)에 **완성된 하위 객체**를 실어 보낸다.
  const patchGeometry = (p: Partial<BallBearingGeometry>) => {
    if (!input) return;
    dispatch({ type: 'UPDATE_BB_INPUT', payload: { geometry: { ...input.geometry, ...p } } });
  };
  const patchMaterial = (p: Partial<Material>) => {
    if (!input) return;
    dispatch({ type: 'UPDATE_BB_INPUT', payload: { material: { ...input.material, ...p } } });
  };
  const patchOperating = (p: Partial<BbOperatingConditions>) => {
    if (!input) return;
    dispatch({ type: 'UPDATE_BB_INPUT', payload: { operating: { ...input.operating, ...p } } });
  };
  const patchSolver = (p: Partial<BbSolverParams>) => {
    if (!input) return;
    dispatch({ type: 'UPDATE_BB_INPUT', payload: { solver: { ...input.solver, ...p } } });
  };

  const setClearance = (tag: ClearanceTag, value: number) => {
    setClearanceDraft(prev => ({ ...prev, [tag]: value }));
    patchGeometry({ clearance: { [tag]: value } as BbClearanceSpec });
  };

  const setDof = (key: DofKey, dof: BbDof) => {
    if (!input) return;
    patchSolver({ dof_mask: { ...input.solver.dof_mask, [key]: dof } });
  };

  // ── 프리셋 조작 ──
  const handlePresetChange = async (name: string) => {
    setSelectedPreset(name);
    if (!name) return;
    try {
      await loadPreset(name);
      dispatch({ type: 'SET_ERROR', payload: null });
    } catch (e) {
      dispatch({ type: 'SET_ERROR', payload: `프리셋 로드 실패: ${e}` });
    }
  };

  const handleSaveAs = async () => {
    if (!input) return;
    const name = window.prompt('새 프리셋 이름:', selectedPreset);
    if (!name) return;
    try {
      await invoke('bb_preset_save', { name, input });
      await refreshPresets();
      setSelectedPreset(name);
      invoke('bb_preset_save_last', { name }).catch(() => {});
      setSaveFlash(true);
      setTimeout(() => setSaveFlash(false), 1200);
    } catch (e) {
      dispatch({ type: 'SET_ERROR', payload: `프리셋 저장 실패: ${e}` });
    }
  };

  const handleOverwrite = async () => {
    if (!input || !selectedPreset) return;
    try {
      await invoke('bb_preset_save', { name: selectedPreset, input });
      await refreshPresets();
      setSaveFlash(true);
      setTimeout(() => setSaveFlash(false), 1200);
    } catch (e) {
      dispatch({ type: 'SET_ERROR', payload: `프리셋 저장 실패: ${e}` });
    }
  };

  // ── Solve ──
  const handleSolve = async () => {
    if (!input) return;
    dispatch({ type: 'SET_LOADING', payload: true });
    dispatch({ type: 'SET_ERROR', payload: null });
    try {
      const result = await invoke<BbResult>('bb_solve_bearing', { input });
      dispatch({ type: 'SET_RESULT', payload: result });
    } catch (e) {
      // Rust `validate()` / 솔버 오류 메시지를 **그대로** 보여준다.
      // 프론트가 문구를 고쳐 쓰면 어느 쪽이 거부했는지 알 수 없게 된다 (§3.6.5.3).
      dispatch({ type: 'SET_ERROR', payload: String(e) });
    } finally {
      dispatch({ type: 'SET_LOADING', payload: false });
    }
  };

  if (!input) {
    return (
      <div className="p-3 text-[13px] text-text-muted">
        프리셋을 불러오는 중…
        {state.error && <div className="mt-2 text-red-600 whitespace-pre-wrap">{state.error}</div>}
      </div>
    );
  }

  const g = input.geometry;
  const m = input.material;
  const op = input.operating;
  const sp = input.solver;
  const cTag = clearanceTag(g.clearance);
  const dofFullActive = DOF_KEYS.every(k => sp.dof_mask[k] === 'Free');

  return (
    <div className="text-text-dark">
      {/* ── 프리셋 ─────────────────────────────────────────────── */}
      <div className="px-3 py-2 border-b border-panel-border space-y-1.5">
        <div className="text-[11px] uppercase tracking-wide text-text-muted">Preset</div>
        <select
          value={selectedPreset}
          onChange={e => handlePresetChange(e.target.value)}
          className="w-full px-1.5 py-1 text-[13px] bg-white border border-slate-200 rounded
                     focus:outline-none focus:border-accent cursor-pointer"
        >
          {presets.length === 0 && <option value="">(프리셋 없음)</option>}
          {presets.map(p => (
            <option key={p.name} value={p.name}>
              {p.name}
            </option>
          ))}
        </select>
        {/* Plan §3.6.5.4 ② — 가정 기하임을 화면에 드러낸다 (T-6 미해결) */}
        {selectedPreset.includes('assumed') && (
          <div
            className="inline-block px-1.5 py-0.5 rounded text-[11px] bg-amber-100 text-amber-800"
            title="Z 와 D_w 가 실 카탈로그 미확인 가정값이다 (Plan T-6). 다만 이 기하가 Level C·D 검증 픽스처와 같아 화면 숫자를 검증 결과와 직접 대조할 수 있다."
          >
            가정 기하
          </div>
        )}
        <div className="flex gap-1">
          <button
            onClick={handleOverwrite}
            disabled={!selectedPreset}
            className="flex-1 px-2 py-1 text-[12px] rounded border border-slate-200 hover:bg-black/5 disabled:opacity-40 cursor-pointer"
          >
            {saveFlash ? '저장됨' : '덮어쓰기'}
          </button>
          <button
            onClick={handleSaveAs}
            className="flex-1 px-2 py-1 text-[12px] rounded border border-slate-200 hover:bg-black/5 cursor-pointer"
          >
            새 이름으로
          </button>
        </div>
      </div>

      {/* ── 변종 ───────────────────────────────────────────────── */}
      <div className="px-3 py-2 border-b border-panel-border space-y-1">
        <div className="text-[11px] uppercase tracking-wide text-text-muted">Kind</div>
        <div className="flex gap-1">
          {KINDS.map(k => {
            const reason = KIND_DISABLED_REASON[k];
            return (
              <button
                key={k}
                disabled={!!reason}
                title={reason ?? 'Angular Contact Ball Bearing'}
                onClick={() => dispatch({ type: 'UPDATE_BB_INPUT', payload: { kind: k } })}
                className={`flex-1 px-1 py-1 text-[12px] rounded border ${
                  input.kind === k
                    ? 'border-accent bg-accent/10 font-semibold'
                    : 'border-slate-200 hover:bg-black/5'
                } disabled:opacity-40 disabled:cursor-not-allowed cursor-pointer`}
              >
                {k}
              </button>
            );
          })}
        </div>
      </div>

      {/* ── ① 기하 (BallBearingGeometry 선언 순서) ───────────────── */}
      <Section title="① 기하 (Geometry)" open={openSections.has('geometry')} onToggle={() => toggleSection('geometry')}>
        <NumField label="내경 d" value={g.bore_mm} unit="mm" onChange={v => patchGeometry({ bore_mm: v })} />
        <NumField label="외경 D" value={g.outer_diameter_mm} unit="mm" onChange={v => patchGeometry({ outer_diameter_mm: v })} />
        <NumField label="폭 B" value={g.width_mm} unit="mm" onChange={v => patchGeometry({ width_mm: v })} />
        <NumField label="볼 수 Z" value={g.z} unit="" onChange={v => patchGeometry({ z: Math.round(v) })} />
        <NumField label="볼 직경 D_w" value={g.d_w_mm} unit="mm" onChange={v => patchGeometry({ d_w_mm: v })} />
        <NumField label="피치직경 D_pw" value={g.d_pw_mm} unit="mm" onChange={v => patchGeometry({ d_pw_mm: v })} />
        <NumField
          label="내륜 홈반경 r_i"
          value={g.r_i_mm}
          unit="mm"
          title="ISO 16281 Annex B.2 참조기하 기본값 = 0,52 D_w"
          onChange={v => patchGeometry({ r_i_mm: v })}
        />
        <NumField
          label="외륜 홈반경 r_e"
          value={g.r_e_mm}
          unit="mm"
          title="ISO 16281 Annex B.2 참조기하 기본값 = 0,53 D_w"
          onChange={v => patchGeometry({ r_e_mm: v })}
        />
        <NumField
          label="공칭 접촉각 α"
          value={toDeg(g.alpha_nom_rad)}
          unit="°"
          title="ISO 281 정격하중 계산 전용. 내부 하중분포에는 초기 접촉각 α₀ 를 쓴다 (ISO 16281 Clause 3.5 NOTE). 내부 저장은 rad."
          onChange={v => patchGeometry({ alpha_nom_rad: toRad(v) })}
        />

        {/* BbClearanceSpec — 생성 타입이 `{ "DiametralMm": number } | …` 이므로 그 표현 그대로 만든다 */}
        <div className="mt-2 pt-2 border-t border-panel-border">
          <div className="text-[12px] font-semibold mb-1">클리어런스 / 예압</div>
          {CLEARANCE_TAGS.map(tag => (
            <label key={tag} className="flex items-center gap-1.5 text-[13px] py-0.5 cursor-pointer">
              <input
                type="radio"
                name="bb-clearance"
                checked={cTag === tag}
                onChange={() => setClearance(tag, clearanceDraft[tag])}
                className="cursor-pointer"
              />
              <span className="text-text-muted">{CLEARANCE_LABEL[tag]}</span>
            </label>
          ))}
          <NumField
            label={CLEARANCE_LABEL[cTag]}
            // 접촉각 계열만 환산한다.
            value={cTag === 'InitialAngleRad' ? toDeg(clearanceValue(g.clearance)) : clearanceValue(g.clearance)}
            unit={CLEARANCE_UNIT[cTag]}
            title={
              cTag === 'InitialAngleRad'
                ? '초기 접촉각 α₀. 내부 저장은 rad, 표시만 °.'
                : undefined
            }
            onChange={v => setClearance(cTag, cTag === 'InitialAngleRad' ? toRad(v) : v)}
          />
        </div>
      </Section>

      {/* ── ② 재질 (Material 선언 순서) ──────────────────────────── */}
      <Section title="② 재질 (Material)" open={openSections.has('material')} onToggle={() => toggleSection('material')}>
        <NumField label="볼 E" value={m.e_ball_mpa} unit="MPa" onChange={v => patchMaterial({ e_ball_mpa: v })} />
        <NumField label="레이스웨이 E" value={m.e_ring_mpa} unit="MPa" onChange={v => patchMaterial({ e_ring_mpa: v })} />
        <NumField label="포아송비 ν" value={m.nu} unit="" onChange={v => patchMaterial({ nu: v })} />
        <NumField label="경도" value={m.hrc} unit="HRC" onChange={v => patchMaterial({ hrc: v })} />
        <NumField label="볼 밀도" value={m.density_ball_g_cm3} unit="g/cm³" onChange={v => patchMaterial({ density_ball_g_cm3: v })} />
        <NumField label="링 밀도" value={m.density_ring_g_cm3} unit="g/cm³" onChange={v => patchMaterial({ density_ring_g_cm3: v })} />
      </Section>

      {/* ── ③ 운전조건 (BbOperatingConditions 선언 순서) ─────────── */}
      <Section title="③ 운전조건 (Operating)" open={openSections.has('operating')} onToggle={() => toggleSection('operating')}>
        <div className="text-[11px] text-text-muted pb-1">
          좌표계 D-7 (ISO, X = 회전축). 내륜에 작용하는 하중을 양으로 본다.
        </div>
        <NumField label="축하중 F_x" value={op.f_x_n} unit="N" onChange={v => patchOperating({ f_x_n: v })} />
        <NumField label="반경하중 F_y" value={op.f_y_n} unit="N" onChange={v => patchOperating({ f_y_n: v })} />
        <NumField label="반경하중 F_z" value={op.f_z_n} unit="N" onChange={v => patchOperating({ f_z_n: v })} />
        <NumField label="모멘트 M_y" value={op.m_y_nmm} unit="N·mm" onChange={v => patchOperating({ m_y_nmm: v })} />
        <NumField label="모멘트 M_z" value={op.m_z_nmm} unit="N·mm" onChange={v => patchOperating({ m_z_nmm: v })} />
        <NumField label="내륜 회전속도" value={op.n_inner_rpm} unit="r/min" onChange={v => patchOperating({ n_inner_rpm: v })} />
        <NumField label="외륜 회전속도" value={op.n_outer_rpm} unit="r/min" onChange={v => patchOperating({ n_outer_rpm: v })} />
        <NumField label="운전 온도" value={op.temperature_c} unit="°C" onChange={v => patchOperating({ temperature_c: v })} />
      </Section>

      {/* ── ④ 솔버 (BbSolverParams 선언 순서) ────────────────────── */}
      <Section title="④ 솔버 (Solver)" open={openSections.has('solver')} onToggle={() => toggleSection('solver')}>
        <NumField label="수렴 판정값" value={sp.convergence_tol} unit="" onChange={v => patchSolver({ convergence_tol: v })} />
        <NumField label="최대 반복" value={sp.max_iterations} unit="" onChange={v => patchSolver({ max_iterations: Math.round(v) })} />

        {/* dof_mask */}
        <div className="mt-2 pt-2 border-t border-panel-border">
          <div className="flex items-center justify-between mb-1">
            <span className="text-[12px] font-semibold">자유도 구속 (D-1)</span>
            <span className="flex gap-1">
              <button
                onClick={() => patchSolver({ dof_mask: DOF_FULL })}
                title="5-DOF 전 자유도 해방 (Rust BbDofMask::FULL)"
                className={`px-1.5 py-0.5 text-[11px] rounded border cursor-pointer ${
                  dofFullActive ? 'border-accent bg-accent/10' : 'border-slate-200 hover:bg-black/5'
                }`}
              >
                FULL
              </button>
              <button
                onClick={() => patchSolver({ dof_mask: DOF_ISO_3DOF })}
                title="δ_z·γ_y 를 0 으로 구속 → ISO 16281 A.2.2 와 항등. Level D-1 (Harris Table 7.4) 이 이 모드다. (Rust BbDofMask::ISO_3DOF)"
                className="px-1.5 py-0.5 text-[11px] rounded border border-slate-200 hover:bg-black/5 cursor-pointer"
              >
                ISO_3DOF
              </button>
            </span>
          </div>
          {DOF_KEYS.map(k => {
            const dof = sp.dof_mask[k];
            const prescribed = typeof dof === 'object';
            return (
              <div key={k} className="flex items-center gap-1.5 py-0.5">
                <span className="text-[13px] text-text-muted w-16 shrink-0">{DOF_LABEL[k]}</span>
                <button
                  onClick={() =>
                    setDof(k, prescribed ? 'Free' : ({ Prescribed: dofDraft[k] } as BbDof))
                  }
                  title={
                    prescribed
                      ? '변위 제어 — 이 방향의 반력이 결과가 된다'
                      : '하중 제어 — 이 방향의 외력이 주어지고 변위가 해가 된다'
                  }
                  className={`px-1.5 py-0.5 text-[11px] rounded border cursor-pointer shrink-0 ${
                    prescribed ? 'border-accent bg-accent/10' : 'border-slate-200 hover:bg-black/5'
                  }`}
                >
                  {prescribed ? 'Prescribed' : 'Free'}
                </button>
                {prescribed && (
                  <input
                    type="number"
                    value={dof.Prescribed}
                    onChange={e => {
                      const n = Number(e.target.value);
                      if (e.target.value.trim() !== '' && Number.isFinite(n)) {
                        setDofDraft(prev => ({ ...prev, [k]: n }));
                        setDof(k, { Prescribed: n });
                      }
                    }}
                    className="w-16 px-1 py-0.5 text-[12px] text-right bg-white border border-slate-200 rounded font-mono tabular-nums
                               focus:outline-none focus:border-accent"
                  />
                )}
                {prescribed && <span className="text-[11px] text-text-muted">{DOF_UNIT[k]}</span>}
              </div>
            );
          })}
          <div className="text-[11px] text-text-muted pt-1">
            γ 는 <b>rad</b> 로 표시한다 — Level D-2a·D-2d 판정이 rad 로 출력되므로 화면 숫자와 검증 숫자를 같게 두기 위해서다.
          </div>
        </div>

        {/* phase_sweep */}
        <div className="mt-2 pt-2 border-t border-panel-border">
          <label className="flex items-center gap-1.5 text-[13px] cursor-pointer">
            <input
              type="checkbox"
              checked={sp.phase_sweep.enabled}
              onChange={e =>
                patchSolver({ phase_sweep: { ...sp.phase_sweep, enabled: e.target.checked } })
              }
              className="cursor-pointer"
            />
            <span>케이지 위상 스윕 (D-8)</span>
          </label>
          {sp.phase_sweep.enabled && (
            <NumField
              label="분할 수 n_phase"
              value={sp.phase_sweep.n_phase}
              unit=""
              title="φ₀ 를 [0, 2π/Z) 로 분할해 Q_max·p_H·수명의 최악값과 발생 위상을 찾는다."
              onChange={v =>
                patchSolver({ phase_sweep: { ...sp.phase_sweep, n_phase: Math.round(v) } })
              }
            />
          )}
        </div>

        {/* preload_model */}
        <div className="mt-2 pt-2 border-t border-panel-border">
          <div className="text-[12px] font-semibold mb-1">축방향 예압 모델 (D-2)</div>
          {(['Spring', 'Rigid'] as BbPreloadModel[]).map(pm => (
            <label key={pm} className="flex items-center gap-1.5 text-[13px] py-0.5 cursor-pointer">
              <input
                type="radio"
                name="bb-preload-model"
                checked={sp.preload_model === pm}
                onChange={() => patchSolver({ preload_model: pm })}
                className="cursor-pointer"
              />
              <span className="text-text-muted">
                {pm === 'Spring'
                  ? 'Spring — 하중 제어 (웨이브 와셔·스프링)'
                  : 'Rigid — 변위 제어 (스페이서·로크너트)'}
              </span>
            </label>
          ))}
          {cTag !== 'AxialPreloadN' && (
            <div className="text-[11px] text-text-muted pt-1">
              이 설정은 클리어런스를 <b>축방향 예압</b>으로 지정했을 때만 의미가 있다.
            </div>
          )}
        </div>

        {/* c_r_n / c_0r_n — Option<f64> */}
        <div className="mt-2 pt-2 border-t border-panel-border">
          {(
            [
              ['c_r_n', '동정격 C_r', 'ISO 281 식으로 자체 산출'],
              ['c_0r_n', '정정격 C_0r', 'ISO 76 식으로 자체 산출'],
            ] as const
          ).map(([key, label, auto]) => (
            <div key={key} className="py-0.5">
              <label className="flex items-center gap-1.5 text-[13px] cursor-pointer">
                <input
                  type="checkbox"
                  checked={sp[key] !== null}
                  onChange={e => patchSolver({ [key]: e.target.checked ? 0 : null })}
                  className="cursor-pointer"
                />
                <span className="text-text-muted">
                  {label} 직접 지정 <span className="text-[11px]">(끄면 {auto})</span>
                </span>
              </label>
              {sp[key] !== null && (
                <NumField
                  label={label}
                  value={sp[key] as number}
                  unit="N"
                  onChange={v => patchSolver({ [key]: v })}
                />
              )}
            </div>
          ))}
        </div>
      </Section>

      {/* ── Solve ──────────────────────────────────────────────── */}
      <div className="p-3 space-y-2">
        <button
          onClick={handleSolve}
          disabled={loading}
          className="w-full py-2 rounded bg-accent text-white text-[13px] font-semibold
                     hover:brightness-110 disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer"
        >
          {loading ? '해석 중…' : 'Solve'}
        </button>
        {state.error && (
          <div className="px-2 py-1.5 rounded bg-red-50 border border-red-200 text-[12px] text-red-700 whitespace-pre-wrap">
            {state.error}
          </div>
        )}
      </div>
    </div>
  );
}
