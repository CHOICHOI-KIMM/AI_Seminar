// Phase 2 숙제 3건 — WASM 실경로 검증 (계획 R4·R5 + Phase 0 숙제 1).
//
// ★ 신선도 가드 우선: stale pkg 로 옛 코드를 테스트해 "패닉"을 오진한 사고가 실제로 있었다
//   (Phase 1 의 mv/mtime stale 과 동형). 진입점 존재 + 빌드시각을 **먼저** 확인한다.
//
// 사용: node verify_phase2.js

const fs = require("fs");
const path = require("path");

const PKG_JS = path.join(__dirname, "pkg-node", "micropitting_wasm.js");
const PKG_WASM = path.join(__dirname, "pkg-node", "micropitting_wasm_bg.wasm");
const SRC = path.join(__dirname, "src", "lib.rs");

// ── 신선도 가드 ──
const glue = fs.readFileSync(PKG_JS, "utf8");
const REQUIRED = ["solve_wear_json", "solve_stress_fatigue_json", "solve_partial_json"];
const missing = REQUIRED.filter((f) => !glue.includes(f));
if (missing.length) {
  console.error(`[STALE] pkg 에 진입점 없음: ${missing.join(", ")}`);
  console.error("        -> wasm 재빌드 필요. 옛 pkg 로 테스트하면 결과가 거짓이다.");
  process.exit(2);
}
const tWasm = fs.statSync(PKG_WASM).mtimeMs;
const tSrc = fs.statSync(SRC).mtimeMs;
if (tSrc > tWasm) {
  console.error(`[STALE] src/lib.rs 가 wasm 보다 최신 -> 재빌드 필요.`);
  process.exit(2);
}
console.log("신선도 가드 통과 (진입점 3종 존재, wasm >= src)\n");

const wasm = require("./pkg-node/micropitting_wasm.js");

const N = 32, M = 8;
const OP = { p_h: 1.5e9, u_mean: 1.0, u2: 1.01, slide_roll: 0.02, eta0: 0.0094,
             alpha_visc: 20.78e-9, tau0: 3.0e6, temp: 348.15, r_x: 0.01 };
const MAT = { e_red: 115.384615384615e9, nu: 0.3, hardness: 7.0e9, p_lim: 4.0e9 };
const rough = (nx, ny, rq) => ({ nx, ny,
  data: Array.from({ length: nx * ny }, (_, k) => rq * Math.sin(2 * Math.PI * 6 * (k % nx) / nx)) });

let fail = 0;
const check = (cond, msg) => { console.log(`  [${cond ? "PASS" : "FAIL"}] ${msg}`); if (!cond) fail++; };

// ── 숙제 1: 차원 불일치 = 패닉이 아니라 구조적 오류 ──
console.log("── 숙제 1: 차원 불일치가 패닉이 아니라 구조적 오류인가 ──");
const bad = JSON.parse(wasm.solve_wear_json(JSON.stringify({
  grid: { nx: 64, ny: 16, lx: 4e-5, ly: 2e-5 },
  p_tran: { nx: 2, ny: 1, data: [0, 0] },
  op: OP, mat: MAT, phi_bl: 0.3, params: {} })));
check(bad.ok === false && /차원 불일치/.test(bad.error || ""), `오류 반환: "${(bad.error||"").slice(0,44)}…"`);

// Field2 가 grid 보다 **큰** 경우 = 조용한 오독 위험(무증상) — 이쪽이 진짜 위험.
const big = JSON.parse(wasm.solve_wear_json(JSON.stringify({
  grid: { nx: 2, ny: 2, lx: 4e-5, ly: 2e-5 },
  p_tran: { nx: 4, ny: 4, data: new Array(16).fill(1e9) },
  op: OP, mat: MAT, phi_bl: 0.3, params: {} })));
check(big.ok === false, "Field2 > grid (조용한 오독) 도 차단");

// ── 모듈 생존: 패닉이었다면 이후 호출이 불가능하다 ──
console.log("\n── 모듈 생존 (패닉 아님의 증거) ──");
const ok = JSON.parse(wasm.solve_wear_json(JSON.stringify({
  grid: { nx: 4, ny: 2, lx: 4e-5, ly: 2e-5 },
  p_tran: { nx: 4, ny: 2, data: [1.5e9, 0.75e9, 0, -1e8, 1.2e9, 0.3e9, 0, 5e8] },
  op: OP, mat: MAT, phi_bl: 0.3, params: {} })));
check(ok.ok === true, `후속 호출 정상 (dh_w_mean=${ok.ok ? ok.result.dh_w_mean.toExponential(3) : "-"})`);

// ── 숙제 2+3: 부분윤활 — 진짜 오케스트레이터 + 진단 항상 존재 ──
console.log("\n── 숙제 2+3: 부분윤활(M1+M2+M6) ──");
const p = JSON.parse(wasm.solve_partial_json(JSON.stringify({
  grid: { nx: N, ny: M, lx: 5.2e-4, ly: 1.3e-4 },
  rough1: rough(N, M, 0.23e-6), rough2: rough(N, M, 0.06e-6),
  mat: MAT, op: OP, h_bar: 1.4e-7 })));
if (!p.ok) { console.log(`  [FAIL] ${p.error}`); process.exit(1); }
const d = p.diagnostics;
check(!!d, "diagnostics 항상 존재 (비-Option)");
check(d.outerConverged && d.shareConverged, `수렴: outer=${d.outerConverged} share=${d.shareConverged}`);
check(d.loadResidual < 1e-3, `하중보존 잔차 ${d.loadResidual.toExponential(2)} < 1e-3 (CV-M6-Load)`);
check(p.result.phi_bl !== 0, `phi_bl=${p.result.phi_bl.toFixed(4)} != 0 -> 진짜 오케스트레이터(스텁 아님)`);
console.log(`     outerIters=${d.outerIters} shareIters=${d.shareIters} muEff=${d.muEff.toFixed(4)}`);

// ── 입력 검증 ──
console.log("\n── 입력 검증 ──");
const badh = JSON.parse(wasm.solve_partial_json(JSON.stringify({
  grid: { nx: 4, ny: 2, lx: 4e-5, ly: 2e-5 },
  rough1: rough(4, 2, 1e-7), rough2: rough(4, 2, 1e-7),
  mat: MAT, op: OP, h_bar: 0 })));
check(badh.ok === false, `h_bar=0 거부: "${(badh.error||"").slice(0,30)}…"`);

const unknown = JSON.parse(wasm.solve_wear_json(`{"grid":{"nx":1,"ny":1,"lx":1e-5,"ly":1e-5},
  "p_tran":{"nx":1,"ny":1,"data":[1e9]},"op":${JSON.stringify(OP)},"mat":${JSON.stringify(MAT)},
  "phi_bl":0.3,"params":{},"typo_field":1}`));
check(unknown.ok === false, "deny_unknown_fields: 오타 필드가 조용히 무시되지 않음");

console.log(`\n${fail === 0 ? "[ALL PASS]" : `[${fail} FAIL]`}`);
process.exit(fail === 0 ? 0 : 1);
