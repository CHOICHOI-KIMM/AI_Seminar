/* tslint:disable */
/* eslint-disable */

/**
 * 참조곡선: kind + params JSON → CurveResp JSON.
 */
export function reference_curve_json(kind: string, params_json: string): string;

/**
 * 정적 문헌 데이터 일괄 JSON.
 */
export function reference_tables_json(): string;

/**
 * 전체 체인(탭3): ChainArgs JSON → ChainResp JSON (뷰어 슬라이스만, unwornGeometry 플래그).
 */
export function solve_chain_json(input_json: string): string;

/**
 * 부분윤활 M1+M2+M6: `PartialArgs` JSON → `PartialResponse` JSON (**diagnostics 항상 포함**).
 */
export function solve_partial_json(input_json: string): string;

/**
 * M3→M4 체인: `StressFatigueArgs` JSON → `StressFatigueSummary` JSON.
 */
export function solve_stress_fatigue_json(input_json: string): string;

/**
 * M5 경마모: `WearArgs` JSON → `WearResponse` JSON.
 */
export function solve_wear_json(input_json: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly reference_curve_json: (a: number, b: number, c: number, d: number) => [number, number];
    readonly reference_tables_json: () => [number, number];
    readonly solve_chain_json: (a: number, b: number) => [number, number];
    readonly solve_partial_json: (a: number, b: number) => [number, number];
    readonly solve_stress_fatigue_json: (a: number, b: number) => [number, number];
    readonly solve_wear_json: (a: number, b: number) => [number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
