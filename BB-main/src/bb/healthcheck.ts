// BB — 자동 스모크 (Plan §3.6.5.3 ④ / §3.6.5.5 「표현 검사」)
//
// `VITE_BB_HEALTHCHECK=1` 일 때만 앱 기동 직후 1회 동작한다.
// 평상시 `npm run tauri dev` 에는 **아무 영향이 없다.**
//
// 목적 — 「화면이 떴다 + 솔버 왕복이 됐다 + 계약이 맞다」를 **로그만으로** 판정.
//   ① 기본 프리셋을 Rust 에서 만들고 읽어 온다 (프리셋 왕복도 함께 확인)
//   ② `bb_compute_geometry` 를 1회 호출한다 — **P4-S3-4 에서 추가**.
//      S3 에서 `BbGeometryView` 가 이 커맨드에 처음 연결됐다 (§3.6.4.7 ①).
//      `bb_solve_bearing` 과 달리 **하중과 무관**한 별도 경로라 왕복을 따로 확인해야 한다.
//   ③ `bb_solve_bearing` 을 1회 호출한다
//   ③' **위상 스윕을 켠 입력**으로 `bb_solve_bearing` 을 한 번 더 호출한다 — **P4-S4-2 에서 추가**.
//      S4 의 `BbLoadDistView` 가 `result.phase_sweep` 으로 **C-5(주기 2π/Z)** 를 화면에서 본다.
//      그런데 기본 프리셋은 `phase_sweep.enabled = false` 라(`BbPhaseSweep::default()`)
//      ③ 의 왕복만으로는 그 경로가 **한 번도 실행되지 않는다.** 켠 입력을 따로 보내야
//      「스윕이 실제로 채워지는가」가 로그로 판정된다.
//   ③'' `bb_compute_contact` 를 **두 번** 왕복한다 — **P4-S5-2 에서 추가**.
//      이 커맨드는 **등록만 되고 아무도 부르지 않던 마지막 하나**였다(§3.6.3.1).
//      S5 의 `BbStressContourView` 가 (a) `q_n = 0` 하중무관 전처리(Level B 대조표)와
//      (b) what-if 두 곳에서 쓴다. 이것을 넣으면 **BB 커맨드 3종을 헬스체크가 전부 덤는다.**
//      ⚠ 시그니처는 `commands.rs::bb_compute_contact(input: BbInput, q_n: f64)` 이고,
//        Tauri v2 는 인자를 **camelCase** 로 받으므로 JS 키는 `qN` 이다
//        (`tauri-macros` 기본 `ArgumentCase::Camel` — 소스 확인).
//   ④ 받은 JSON 을 **생성 타입(ts-rs)의 형상**과 대조한다
//      — 자동생성은 이름·형상을 묶을 뿐이고, `BbClearanceSpec`·`BbDof` 같은
//        데이터 보유 enum 의 **실제 직렬화 표현**은 런타임에서만 확인된다.

import { invoke } from '@tauri-apps/api/core';
import { info as logInfo, error as logError } from '@tauri-apps/plugin-log';
import type { BbInput } from './generated/BbInput';
import type { BbResult } from './generated/BbResult';
import type { BallBearingKind } from './generated/BallBearingKind';
import type { BbGeometryDerived } from './generated/BbGeometryDerived';
import type { BbGeometrySummary } from './generated/BbGeometrySummary';
import type { BbContactDerived } from './generated/BbContactDerived';
import type { BallResult } from './generated/BallResult';

interface PresetInfo {
  name: string;
  modified: string;
}

/** `commands::ContactResponse` 대응 (Rust 쪽은 ts-rs 대상이 아니라 커맨드 전용 래퍼다). */
interface ContactResponse {
  derived: BbContactDerived;
  q_n: number;
  delta_mm: number;
  a_inner_mm: number;
  b_inner_mm: number;
  p_max_inner_mpa: number;
  a_outer_mm: number;
  b_outer_mm: number;
  p_max_outer_mpa: number;
  alerts: unknown[];
}

const KINDS: readonly BallBearingKind[] = ['Acbb', 'Dgbb', 'FourPoint'];
const ALERT_LEVELS = ['Info', 'Warning', 'Critical'] as const;

function isRecord(v: unknown): v is Record<string, unknown> {
  return typeof v === 'object' && v !== null && !Array.isArray(v);
}

/**
 * `commands::GeometryResponse` 형상 검사 (P4-S3-4).
 *
 * `BbGeometryDerived` 9필드 · `BbGeometrySummary` 13필드를 **전량** 확인한다.
 * 필드 목록은 `src/bb/generated/` 를 그대로 옮긴 것이다 — 생성물이 바뀌면
 * ③ 의 `git diff --exit-code src/bb/generated/` 가 먼저 걸리고,
 * 여기서는 **실제 JSON 에 그 필드가 오는가**를 본다 (자동생성이 못 잡는 쪽).
 */
const DERIVED_FIELDS: readonly (keyof BbGeometryDerived)[] = [
  'a_mm',
  'alpha_0_rad',
  'r_i_center_mm',
  'gamma',
  'sum_rho_i_per_mm',
  'sum_rho_e_per_mm',
  'f_rho_i',
  'f_rho_e',
  'g_r_op_mm',
];
const SUMMARY_FIELDS: readonly (keyof BbGeometrySummary)[] = [
  ...DERIVED_FIELDS,
  'osculation_inner',
  'osculation_outer',
  'ball_mass_g',
  'n_dpw_mm_per_min',
];

function checkGeometryResponseShape(raw: unknown): string[] {
  const bad: string[] = [];
  if (!isRecord(raw)) return ['GeometryResponse 가 객체가 아니다'];

  const derived = raw.derived;
  if (!isRecord(derived)) {
    bad.push('derived 가 객체가 아니다');
  } else {
    for (const f of DERIVED_FIELDS) {
      if (typeof derived[f] !== 'number') bad.push(`derived.${f} 가 number 가 아니다`);
    }
  }

  const summary = raw.summary;
  if (!isRecord(summary)) {
    bad.push('summary 가 객체가 아니다');
  } else {
    for (const f of SUMMARY_FIELDS) {
      if (typeof summary[f] !== 'number') bad.push(`summary.${f} 가 number 가 아니다`);
    }
  }

  // 공유 9필드는 두 구조체에서 **같은 값**이어야 한다.
  // 어긋나면 요약이 파생값을 다시 계산하고 있다는 뜻이고, 그러면 화면 숫자와
  // 검증 숫자가 갈라진다 (§3.6.4.2). 형상검증에 넣어 둔다.
  if (isRecord(derived) && isRecord(summary)) {
    for (const f of DERIVED_FIELDS) {
      if (derived[f] !== summary[f]) {
        bad.push(`derived.${f}(${String(derived[f])}) 와 summary.${f}(${String(summary[f])}) 가 다르다`);
      }
    }
  }

  if (!Array.isArray(raw.alerts)) bad.push('alerts 가 배열이 아니다');
  return bad;
}

/**
 * 받은 JSON 이 `BbResult` 생성 타입의 형상과 맞는지 검사한다.
 * 어긋난 항목의 목록을 돌려준다 (빈 배열이면 합격).
 */
function checkBbResultShape(raw: unknown): string[] {
  const bad: string[] = [];
  if (!isRecord(raw)) return ['BbResult 가 객체가 아니다'];

  // kind 판별자 (§3.6.1.7) — 문자열 단위 enum
  if (typeof raw.kind !== 'string' || !KINDS.includes(raw.kind as BallBearingKind)) {
    bad.push(`kind 가 BallBearingKind 가 아니다: ${JSON.stringify(raw.kind)}`);
  }

  for (const key of ['geometry', 'equilibrium'] as const) {
    if (!isRecord(raw[key])) bad.push(`${key} 가 객체가 아니다`);
  }
  if (typeof raw.elapsed_ms !== 'number') bad.push('elapsed_ms 가 number 가 아니다');

  // Option<T> → `T | null` (undefined 가 아니다)
  if (raw.phase_sweep !== null && !isRecord(raw.phase_sweep)) {
    bad.push('phase_sweep 가 `BbPhaseSweepResult | null` 이 아니다');
  }

  if (!Array.isArray(raw.alerts)) {
    bad.push('alerts 가 배열이 아니다');
  } else {
    for (const [i, a] of raw.alerts.entries()) {
      if (!isRecord(a)) { bad.push(`alerts[${i}] 가 객체가 아니다`); continue; }
      if (typeof a.level !== 'string' || !ALERT_LEVELS.includes(a.level as typeof ALERT_LEVELS[number])) {
        bad.push(`alerts[${i}].level 이 AlertLevel 이 아니다: ${JSON.stringify(a.level)}`);
      }
      if (typeof a.code !== 'string') bad.push(`alerts[${i}].code 가 string 이 아니다`);
      if (typeof a.message !== 'string') bad.push(`alerts[${i}].message 가 string 이 아니다`);
    }
  }

  const eq = raw.equilibrium;
  if (isRecord(eq)) {
    if (typeof eq.converged !== 'boolean') bad.push('equilibrium.converged 가 boolean 이 아니다');
    if (typeof eq.loaded_count !== 'number') bad.push('equilibrium.loaded_count 가 number 가 아니다');
    if (typeof eq.q_max_n !== 'number') bad.push('equilibrium.q_max_n 가 number 가 아니다');
    if (!isRecord(eq.displacement)) {
      bad.push('equilibrium.displacement 가 객체가 아니다 (S0-4 에서 배열→객체로 바뀌었다)');
    } else {
      for (const f of ['dx_mm', 'dy_mm', 'dz_mm', 'ry_rad', 'rz_rad']) {
        if (typeof eq.displacement[f] !== 'number') bad.push(`displacement.${f} 가 number 가 아니다`);
      }
    }
    if (!Array.isArray(eq.ball_results)) bad.push('equilibrium.ball_results 가 배열이 아니다');
  }
  return bad;
}

/**
 * `BbPhaseSweepResult` 형상 검사 (P4-S4-2).
 *
 * **`Option<T>` 가 `null` 이 아니라 실제로 채워졌는가**부터 본다 — S4 의 `BbLoadDistView` 가
 * C-5(주기 `2π/Z`)를 그 필드로만 그린다. 비어 있으면 화면에는 「스윕 꺼짐」만 뜨고
 * 검증 항목 하나가 조용히 사라진다.
 *
 * `curve` 는 `Array<[number, number]>` = `(φ₀, Q_max)` 이력이다. 솔버(`bearing.rs`)는
 * `φ₀ ∈ [0, 2π/Z)` 를 `n_phase` 분할하므로 **길이 = `n_phase`** 이고 **`φ₀ < 2π/Z`** 여야 한다.
 * 값의 옳고 그름(⑤ 육안·Level C-5)이 아니라 **형상과 정의역**만 본다.
 */
function checkPhaseSweepShape(raw: unknown, expectedN: number, z: number): string[] {
  const bad: string[] = [];
  if (!isRecord(raw)) return ['BbResult 가 객체가 아니다'];

  const ps = raw.phase_sweep;
  if (!isRecord(ps)) {
    return [`phase_sweep 가 채워지지 않았다 (enabled=true 인데 ${JSON.stringify(ps)})`];
  }

  for (const f of ['worst_q_max_n', 'worst_q_max_phase_rad', 'worst_p_max_mpa', 'worst_p_max_phase_rad']) {
    const v = ps[f];
    if (typeof v !== 'number' || !Number.isFinite(v)) bad.push(`phase_sweep.${f} 가 유한 number 가 아니다`);
  }

  const span = (2 * Math.PI) / z;
  if (!Array.isArray(ps.curve)) {
    bad.push('phase_sweep.curve 가 배열이 아니다');
  } else {
    if (ps.curve.length !== expectedN) {
      bad.push(`curve 길이가 n_phase 와 다르다: ${ps.curve.length} ≠ ${expectedN}`);
    }
    for (const [i, pt] of ps.curve.entries()) {
      if (!Array.isArray(pt) || pt.length !== 2 || typeof pt[0] !== 'number' || typeof pt[1] !== 'number') {
        bad.push(`curve[${i}] 가 (φ₀, Q_max) 쌍이 아니다: ${JSON.stringify(pt)}`);
        continue;
      }
      if (pt[0] < 0 || pt[0] >= span) {
        bad.push(`curve[${i}].φ₀=${pt[0]} 가 [0, 2π/Z=${span}) 밖이다`);
      }
    }
  }

  const wq = ps.worst_q_max_phase_rad;
  if (typeof wq === 'number' && (wq < 0 || wq >= span)) {
    bad.push(`worst_q_max_phase_rad=${wq} 가 [0, 2π/Z=${span}) 밖이다`);
  }
  return bad;
}

/**
 * `commands::ContactResponse` 형상 + 물리 항등 검사 (P4-S5-2).
 *
 * `BbContactDerived` 14필드를 **전량** 확인하고, 하중 의존 6필드와 `alerts` 를 본다.
 * 값의 옳고 그름(Level B)은 `cargo test` 소관이고 여기서는 **왕복과 계약**을 본다.
 *
 * 다만 다음 두 가지는 **자체 항등**이라 런타임에서 싸게 확인할 수 있어 함께 넣는다:
 *   · `q_n = 0` 이면 타원이 없어야 한다 (`hertz::contact_ellipse` 가 `(0,0,0)` 반환).
 *   · `q_n > 0` 이면 `a ≥ b > 0` 이고 `p_max = 3Q/(2π a b)` 여야 한다 (Theory §6.3).
 *     화면(`BbStressContourView`)이 그리는 **반타원체 압력분포와 같은 항등**이다 —
 *     화면이 지어낸 식으로 그리고 있지 않다는 것이 여기서 증명된다.
 */
const DERIVED_CONTACT_FIELDS: readonly (keyof BbContactDerived)[] = [
  'chi_inner',
  'chi_outer',
  'k_ellip_inner',
  'e_ellip_inner',
  'k_ellip_outer',
  'e_ellip_outer',
  'a_star_inner',
  'b_star_inner',
  'delta_star_inner',
  'a_star_outer',
  'b_star_outer',
  'delta_star_outer',
  'e_star_mpa',
  'c_p_n_per_mm15',
];

const CONTACT_LOAD_FIELDS = [
  'q_n',
  'delta_mm',
  'a_inner_mm',
  'b_inner_mm',
  'p_max_inner_mpa',
  'a_outer_mm',
  'b_outer_mm',
  'p_max_outer_mpa',
] as const;

function checkContactResponseShape(raw: unknown, qRequested: number): string[] {
  const bad: string[] = [];
  if (!isRecord(raw)) return ['ContactResponse 가 객체가 아니다'];

  const d = raw.derived;
  if (!isRecord(d)) {
    bad.push('derived 가 객체가 아니다');
  } else {
    for (const f of DERIVED_CONTACT_FIELDS) {
      const v = d[f];
      if (typeof v !== 'number' || !Number.isFinite(v)) bad.push(`derived.${f} 가 유한 number 가 아니다`);
    }
  }

  for (const f of CONTACT_LOAD_FIELDS) {
    const v = raw[f];
    if (typeof v !== 'number' || !Number.isFinite(v)) bad.push(`${f} 가 유한 number 가 아니다`);
  }
  if (!Array.isArray(raw.alerts)) bad.push('alerts 가 배열이 아니다');

  // 요청한 하중을 그대로 되돌려 주는가.
  // ⚠ 이것이 **인자 키가 실제로 전달됐다는 증거**다 — Tauri 는 누락된 인자를 기본값으로
  //   때우지 않지만, 키 이름을 틀리면(`q_n` 으로 보내면) 커맨드 자체가 거부된다.
  if (raw.q_n !== qRequested) bad.push(`q_n 이 요청값과 다르다: ${String(raw.q_n)} ≠ ${qRequested}`);

  const races: [string, number, number, number][] = [
    ['inner', raw.a_inner_mm as number, raw.b_inner_mm as number, raw.p_max_inner_mpa as number],
    ['outer', raw.a_outer_mm as number, raw.b_outer_mm as number, raw.p_max_outer_mpa as number],
  ];
  for (const [name, a, b, p] of races) {
    if (qRequested === 0) {
      // `hertz::contact_ellipse` 는 `q_n <= 0` 에서 `(0, 0, 0)` 을 돌려준다.
      // 이 경로가 S5 화면의 「하중 무관 전처리」 표를 만든다 — 타원이 나오면 안 된다.
      if (a !== 0 || b !== 0 || p !== 0) {
        bad.push(`q_n=0 인데 ${name} 타원이 0 이 아니다: a=${a} b=${b} p_max=${p}`);
      }
      continue;
    }
    if (!(b > 0) || !(a >= b)) bad.push(`${name}: a ≥ b > 0 이 아니다 (a=${a}, b=${b})`);
    if (a > 0 && b > 0 && p > 0) {
      const pIdent = (3 * qRequested) / (2 * Math.PI * a * b);
      const rel = Math.abs(pIdent - p) / p;
      if (!(rel < 1e-9)) {
        bad.push(`${name}: p_max ≠ 3Q/(2πab) — 솔버 ${p} vs 항등 ${pIdent} (상대차 ${rel})`);
      }
    }
  }
  return bad;
}

/** `BbInput` 쪽에서 런타임에만 확인 가능한 데이터 보유 enum 표현 검사 */
function checkBbInputShape(raw: unknown): string[] {
  const bad: string[] = [];
  if (!isRecord(raw)) return ['BbInput 이 객체가 아니다'];

  if (typeof raw.kind !== 'string' || !KINDS.includes(raw.kind as BallBearingKind)) {
    bad.push(`kind 가 BallBearingKind 가 아니다: ${JSON.stringify(raw.kind)}`);
  }

  // BbClearanceSpec — externally tagged: {"DiametralMm": 0.05} 형태여야 한다
  const geo = raw.geometry;
  if (!isRecord(geo)) {
    bad.push('geometry 가 객체가 아니다');
  } else {
    const c = geo.clearance;
    const tags = ['DiametralMm', 'InitialAngleRad', 'AxialPreloadN'];
    if (!isRecord(c) || Object.keys(c).length !== 1 || !tags.includes(Object.keys(c)[0])) {
      bad.push(`clearance 가 BbClearanceSpec 표현이 아니다: ${JSON.stringify(c)}`);
    } else if (typeof Object.values(c)[0] !== 'number') {
      bad.push(`clearance 의 payload 가 number 가 아니다: ${JSON.stringify(c)}`);
    }
  }

  // BbDof — "Free" 와 {"Prescribed": 0.1} 이 섞인다
  const solver = raw.solver;
  if (isRecord(solver) && isRecord(solver.dof_mask)) {
    for (const [k, v] of Object.entries(solver.dof_mask)) {
      const ok = v === 'Free' || (isRecord(v) && Object.keys(v).length === 1 && 'Prescribed' in v && typeof v.Prescribed === 'number');
      if (!ok) bad.push(`dof_mask.${k} 가 BbDof 표현이 아니다: ${JSON.stringify(v)}`);
    }
  }
  return bad;
}

async function run(): Promise<void> {
  await logInfo('[healthcheck] 시작 (VITE_BB_HEALTHCHECK=1)');

  await invoke('bb_preset_ensure_default');
  const presets = await invoke<PresetInfo[]>('bb_preset_list');
  if (presets.length === 0) throw new Error('기본 프리셋이 없다');
  const name = presets[0].name;

  const rawInput = await invoke<unknown>('bb_preset_load', { name });
  const inputBad = checkBbInputShape(rawInput);
  await logInfo(
    `[healthcheck] preset='${name}' BbInput 형상검증 ${inputBad.length === 0 ? 'PASS' : `FAIL: ${inputBad.join(' | ')}`}`
  );

  const input = rawInput as BbInput;

  // ── bb_compute_geometry 왕복 (P4-S3-4) ────────────────────────────
  // 하중과 무관한 별도 경로다 (§3.6.4.7 ①). `BbGeometryView` 가 쓰는 커맨드.
  const rawGeom = await invoke<unknown>('bb_compute_geometry', { input });
  const geomBad = checkGeometryResponseShape(rawGeom);
  const gr = rawGeom as { summary: BbGeometrySummary; alerts: unknown[] };
  await logInfo(
    `[healthcheck] bb_compute_geometry` +
      ` a_mm=${gr.summary?.a_mm}` +
      ` alpha_0_rad=${gr.summary?.alpha_0_rad}` +
      ` gamma=${gr.summary?.gamma}` +
      ` n_dpw=${gr.summary?.n_dpw_mm_per_min}` +
      ` alerts=${Array.isArray(gr.alerts) ? gr.alerts.length : 'n/a'}`
  );
  await logInfo(
    `[healthcheck] BbGeometryDerived·BbGeometrySummary 형상검증 ${geomBad.length === 0 ? 'PASS' : `FAIL: ${geomBad.join(' | ')}`}`
  );

  const rawResult = await invoke<unknown>('bb_solve_bearing', { input });
  const resultBad = checkBbResultShape(rawResult);
  const r = rawResult as BbResult;

  await logInfo(
    `[healthcheck] bb_solve_bearing kind=${r.kind}` +
      ` converged=${r.equilibrium?.converged}` +
      ` loaded_count=${r.equilibrium?.loaded_count}` +
      ` q_max_n=${r.equilibrium?.q_max_n}` +
      ` elapsed_ms=${r.elapsed_ms}`
  );
  await logInfo(
    `[healthcheck] BbResult 형상검증 ${resultBad.length === 0 ? 'PASS' : `FAIL: ${resultBad.join(' | ')}`}`
  );

  // ── ③' 위상 스윕을 켠 입력으로 한 번 더 (P4-S4-2) ─────────────────
  // 기본 프리셋은 `phase_sweep.enabled = false` 다. S4 의 화면이 C-5 를 보려면
  // 이 경로가 실제로 채워져야 하므로 **켠 입력을 따로 만들어** 왕복시킨다.
  const sweepN = 36;
  const sweepInput: BbInput = {
    ...input,
    solver: { ...input.solver, phase_sweep: { enabled: true, n_phase: sweepN } },
  };
  const rawSweep = await invoke<unknown>('bb_solve_bearing', { input: sweepInput });
  const sweepBad = checkPhaseSweepShape(rawSweep, sweepN, input.geometry.z);
  const sr = rawSweep as BbResult;
  const ps = sr.phase_sweep;
  await logInfo(
    `[healthcheck] bb_solve_bearing (phase_sweep on, n_phase=${sweepN}, Z=${input.geometry.z})` +
      ` curve_len=${ps?.curve.length}` +
      ` worst_q_max_n=${ps?.worst_q_max_n}` +
      ` worst_q_max_phase_rad=${ps?.worst_q_max_phase_rad}` +
      ` worst_p_max_mpa=${ps?.worst_p_max_mpa}` +
      ` period_2pi_over_z=${(2 * Math.PI) / input.geometry.z}` +
      ` curve0_q=${ps?.curve[0]?.[1]}` +
      ` base_q_max_n=${sr.equilibrium?.q_max_n}`
  );
  await logInfo(
    `[healthcheck] BbPhaseSweepResult 형상검증 ${sweepBad.length === 0 ? 'PASS' : `FAIL: ${sweepBad.join(' | ')}`}`
  );

  // ── ③'' bb_compute_contact 왕복 2회 (P4-S5-2) ─────────────────────
  // 등록만 되고 한 번도 불리지 않던 **마지막 커맨드**다(§3.6.3.1). 이것으로 BB 커맨드 3종
  // (`bb_compute_geometry` · `bb_solve_bearing` · `bb_compute_contact`)을 헬스체크가 전부 덮는다.
  //
  //  (a) `q_n = 0` — S5 화면의 **Level B 대조표**(χ · a* · b* · δ*) 경로.
  //      하중을 넣지 않았으므로 타원·응력은 나오지 않아야 한다.
  const rawContact0 = await invoke<unknown>('bb_compute_contact', { input, qN: 0 });
  const contact0Bad = checkContactResponseShape(rawContact0, 0);
  const c0 = rawContact0 as ContactResponse;
  await logInfo(
    `[healthcheck] bb_compute_contact (q_n=0, 하중무관 전처리)` +
      ` chi_inner=${c0.derived?.chi_inner}` +
      ` chi_outer=${c0.derived?.chi_outer}` +
      ` a_star_inner=${c0.derived?.a_star_inner}` +
      ` b_star_inner=${c0.derived?.b_star_inner}` +
      ` delta_star_inner=${c0.derived?.delta_star_inner}` +
      ` a_star_outer=${c0.derived?.a_star_outer}` +
      ` b_star_outer=${c0.derived?.b_star_outer}` +
      ` e_star_mpa=${c0.derived?.e_star_mpa}` +
      ` c_p=${c0.derived?.c_p_n_per_mm15}`
  );

  //  (b) 평형 해의 **최대하중 볼 Q_j** 를 그대로 넣는다 — S5 화면의 「선택 볼 Q_j 넣기」와 같은 경로.
  //      §3.6.4.7 이 「화면과 검증 결과가 **같은 숫자**여야 한다」고 하는 바로 그 지점을
  //      로그로 실증한다: what-if 경로(`bb_compute_contact`)와 평형 경로(`ball_results[]`)가
  //      같은 `Q` 에서 같은 타원을 내지 않으면 두 경로 중 하나가 틀린 것이다.
  const ballsHc: BallResult[] = r.equilibrium?.ball_results ?? [];
  let topHc = -1;
  for (let i = 0; i < ballsHc.length; i++) {
    if (topHc < 0 || ballsHc[i].q_n > ballsHc[topHc].q_n) topHc = i;
  }
  const qTop = topHc >= 0 ? ballsHc[topHc].q_n : 0;
  const rawContactQ = await invoke<unknown>('bb_compute_contact', { input, qN: qTop });
  const contactQBad = checkContactResponseShape(rawContactQ, qTop);
  const cq = rawContactQ as ContactResponse;
  if (topHc >= 0 && qTop > 0) {
    const b0 = ballsHc[topHc];
    const cmp: [string, number, number][] = [
      ['a_inner_mm', cq.a_inner_mm, b0.a_inner_mm],
      ['b_inner_mm', cq.b_inner_mm, b0.b_inner_mm],
      ['p_max_inner_mpa', cq.p_max_inner_mpa, b0.p_max_inner_mpa],
      ['a_outer_mm', cq.a_outer_mm, b0.a_outer_mm],
      ['b_outer_mm', cq.b_outer_mm, b0.b_outer_mm],
      ['p_max_outer_mpa', cq.p_max_outer_mpa, b0.p_max_outer_mpa],
    ];
    for (const [f, x, y] of cmp) {
      const rel = y === 0 ? Math.abs(x) : Math.abs(x - y) / Math.abs(y);
      if (!(rel < 1e-9)) {
        contactQBad.push(`${f}: bb_compute_contact ${x} ≠ ball_results[${topHc}] ${y} (상대차 ${rel})`);
      }
    }
    // χ 는 a/b 그 자체다 (`BbContactDerived` 주석). 「타원비가 1 에 가까움」(§3.6.4.2 징후)의 근원값이라
    // 화면이 내는 `a/b` 와 솔버가 푼 `χ` 가 같은지 여기서 못 박아 둔다.
    if (cq.b_inner_mm > 0 && cq.derived.chi_inner > 0) {
      const chiRel = Math.abs(cq.derived.chi_inner - cq.a_inner_mm / cq.b_inner_mm) / cq.derived.chi_inner;
      if (!(chiRel < 1e-9)) {
        contactQBad.push(
          `chi_inner(${cq.derived.chi_inner}) ≠ a_i/b_i(${cq.a_inner_mm / cq.b_inner_mm}) 상대차 ${chiRel}`
        );
      }
    }
  }
  await logInfo(
    `[healthcheck] bb_compute_contact (q_n=Q_max=${qTop}, 최대하중 볼 #${topHc + 1} 대조)` +
      ` delta_mm=${cq.delta_mm}` +
      ` a_i=${cq.a_inner_mm} b_i=${cq.b_inner_mm} p_i=${cq.p_max_inner_mpa}` +
      ` a_e=${cq.a_outer_mm} b_e=${cq.b_outer_mm} p_e=${cq.p_max_outer_mpa}` +
      ` sigma_hu_mpa=1500` +
      ` alerts=${Array.isArray(cq.alerts) ? cq.alerts.length : 'n/a'}`
  );
  await logInfo(
    `[healthcheck] ContactResponse 형상·항등 검증 ${
      contact0Bad.length === 0 && contactQBad.length === 0
        ? 'PASS'
        : `FAIL: ${[...contact0Bad, ...contactQBad].join(' | ')}`
    }`
  );

  // 판별자 일치 — 입력의 kind 를 솔버가 그대로 반영해야 한다 (A-8c 의 런타임 짝)
  const inKind = isRecord(rawInput) ? rawInput.kind : undefined;
  await logInfo(
    `[healthcheck] kind 판별자 ${inKind === r.kind ? 'PASS' : `FAIL: input=${String(inKind)} result=${String(r.kind)}`}`
  );

  await logInfo(
    `[healthcheck] 종료 — ${
      inputBad.length === 0 &&
      geomBad.length === 0 &&
      resultBad.length === 0 &&
      sweepBad.length === 0 &&
      contact0Bad.length === 0 &&
      contactQBad.length === 0 &&
      inKind === r.kind
        ? 'ALL PASS'
        : 'HAS FAILURES'
    }`
  );
}

/**
 * `VITE_BB_HEALTHCHECK=1` 일 때만 자동 스모크를 1회 실행한다.
 * 그 외에는 **아무 것도 하지 않는다** (평상시 dev 무영향).
 */
export function runHealthcheckIfEnabled(): void {
  if (import.meta.env.VITE_BB_HEALTHCHECK !== '1') return;
  void run().catch((e: unknown) => {
    void logError(`[healthcheck] 실패: ${e instanceof Error ? `${e.name}: ${e.message}` : String(e)}`);
  });
}
