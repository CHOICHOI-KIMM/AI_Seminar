// BB — 자동 스모크 (Plan §3.6.5.3 ④ / §3.6.5.5 「표현 검사」)
//
// `VITE_BB_HEALTHCHECK=1` 일 때만 앱 기동 직후 1회 동작한다.
// 평상시 `npm run tauri dev` 에는 **아무 영향이 없다.**
//
// 목적 — 「화면이 떴다 + 솔버 왕복이 됐다 + 계약이 맞다」를 **로그만으로** 판정.
//   ① 기본 프리셋을 Rust 에서 만들고 읽어 온다 (프리셋 왕복도 함께 확인)
//   ② `bb_solve_bearing` 을 1회 호출한다
//   ③ 받은 JSON 을 **생성 타입(ts-rs)의 형상**과 대조한다
//      — 자동생성은 이름·형상을 묶을 뿐이고, `BbClearanceSpec`·`BbDof` 같은
//        데이터 보유 enum 의 **실제 직렬화 표현**은 런타임에서만 확인된다.

import { invoke } from '@tauri-apps/api/core';
import { info as logInfo, error as logError } from '@tauri-apps/plugin-log';
import type { BbInput } from './generated/BbInput';
import type { BbResult } from './generated/BbResult';
import type { BallBearingKind } from './generated/BallBearingKind';

interface PresetInfo {
  name: string;
  modified: string;
}

const KINDS: readonly BallBearingKind[] = ['Acbb', 'Dgbb', 'FourPoint'];
const ALERT_LEVELS = ['Info', 'Warning', 'Critical'] as const;

function isRecord(v: unknown): v is Record<string, unknown> {
  return typeof v === 'object' && v !== null && !Array.isArray(v);
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

  // 판별자 일치 — 입력의 kind 를 솔버가 그대로 반영해야 한다 (A-8c 의 런타임 짝)
  const inKind = isRecord(rawInput) ? rawInput.kind : undefined;
  await logInfo(
    `[healthcheck] kind 판별자 ${inKind === r.kind ? 'PASS' : `FAIL: input=${String(inKind)} result=${String(r.kind)}`}`
  );

  await logInfo(
    `[healthcheck] 종료 — ${inputBad.length === 0 && resultBad.length === 0 && inKind === r.kind ? 'ALL PASS' : 'HAS FAILURES'}`
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
