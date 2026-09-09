// ME2010 Fig 7 / Fig 8 — WASM 실경로 검증 (node). 사용: node verify_lit2010.js
//
// 신선도 가드(verify_phase2.js 와 동형) → lit2010_fig7_json / lit2010_fig8_json 을 `{}` 로
// 호출 → 요약 출력. ok:false 또는 미수렴 점이 하나라도 있으면 exit 1.
//
// ★ 정직성: 여기 숫자는 "솔버가 낸 값" 이다. meta.caveats 를 함께 출력한다 — 현 체인은
//   단일 국소 범프를 자기일관으로 다루지 못한다(M1/M2 부호 규약 상이). 논문 재현으로 읽지 말 것.

const fs = require("fs");
const path = require("path");

const PKG_JS = path.join(__dirname, "pkg-node", "micropitting_wasm.js");
const PKG_WASM = path.join(__dirname, "pkg-node", "micropitting_wasm_bg.wasm");
const SRC = path.join(__dirname, "src", "lib.rs");

const glue = fs.readFileSync(PKG_JS, "utf8");
const REQUIRED = ["lit2010_fig7_json", "lit2010_fig8_json"];
const missing = REQUIRED.filter((f) => !glue.includes(f));
if (missing.length) {
  console.error(`[STALE] pkg-node 에 진입점 없음: ${missing.join(", ")} -> wasm 재빌드 필요.`);
  process.exit(2);
}
if (fs.statSync(SRC).mtimeMs > fs.statSync(PKG_WASM).mtimeMs) {
  console.error("[STALE] src/lib.rs 가 wasm 보다 최신 -> 재빌드 필요.");
  process.exit(2);
}
console.log("신선도 가드 통과 (lit2010 진입점 2종, wasm >= src)\n");

const wasm = require("./pkg-node/micropitting_wasm.js");
let fail = 0;
const e = (x, d = 3) => Number(x).toExponential(d);

// ── Fig 7 ──
let t0 = Date.now();
const f7 = JSON.parse(wasm.lit2010_fig7_json("{}"));
const dt7 = (Date.now() - t0) / 1000;
if (!f7.ok) { console.error("Fig 7 ok:false —", f7.error); fail++; }
else {
  const m = f7.meta, d = f7.diagnostics;
  console.log(`── Fig 7 (${dt7.toFixed(2)} s) pH=${e(m.pH,2)} uMean=${m.uMean} S=${m.S} grid ${m.grid.nx}x${m.grid.ny} hC=${e(m.hC)} m ──`);
  console.log(`  peak p/pH = ${f7.peakPOverPh.toFixed(3)} @ x/b=${f7.peakXOverB.toFixed(3)}   (field max ${f7.peakPOverPhField.toFixed(3)})`);
  console.log(`  min h     = ${e(f7.minH)} m (mid-plane), ${e(f7.minHField)} m (field)`);
  console.log(`  at bump apex: p/pH = ${f7.pOverPhAtBump.toFixed(3)}, h = ${e(f7.hAtBump)} m`);
  console.log(`  phiBl = ${f7.phiBl.toFixed(4)}  contactCount=${d.contactCount} (in bump: ${f7.contactCountInBump})  converged outer=${d.outerConverged} share=${d.shareConverged}  loadRes=${e(d.loadResidual,1)}`);
  if (!(d.outerConverged && d.shareConverged)) { console.error("  [FAIL] Fig 7 미수렴"); fail++; }
  const n = f7.xOverB.length;
  const finite = ["xOverB","pOverPh","hM","hDimless"].every(k => f7[k].length === n && f7[k].every(Number.isFinite));
  if (!finite) { console.error("  [FAIL] 배열 길이/유한성"); fail++; }
}

// ── Fig 8 ──
t0 = Date.now();
const f8 = JSON.parse(wasm.lit2010_fig8_json("{}"));
const dt8 = (Date.now() - t0) / 1000;
if (!f8.ok) { console.error("Fig 8 ok:false —", f8.error); fail++; }
else {
  const m = f8.meta;
  console.log(`\n── Fig 8 (${dt8.toFixed(2)} s, ${f8.points.length} pts) pH=${e(m.pH,2)} S=${m.S} hCAnchor=${e(m.hCAnchor,2)} grid ${m.grid.nx}x${m.grid.ny} ──`);
  console.log("  uMean      hC[m]     load%    area%   count  inBump  conv");
  for (const p of f8.points) {
    console.log(`  ${p.uMean.toFixed(3).padStart(5)}  ${e(p.hC)}  ${p.loadRatioPct.toFixed(3).padStart(7)}  ${p.areaRatioPct.toFixed(3).padStart(7)}  ${String(p.contactCount).padStart(6)}  ${String(p.contactCountInBump).padStart(6)}  ${p.converged}`);
    if (!p.converged) { console.error(`  [FAIL] 미수렴 @ uMean=${p.uMean}`); fail++; }
    if (!(p.phiBl >= 0 && p.phiBl <= 1)) { console.error(`  [FAIL] phiBl 범위 밖 @ ${p.uMean}`); fail++; }
  }
  console.log(`\n  hCFormula: ${m.hCFormula}`);
  console.log(`  hC(H–D 1977 at anchor, independent check): ${e(m.hCHamrockDowsonAtAnchor)} m vs anchor ${e(m.hCAnchor)} m`);
  console.log("\n  caveats (from meta):");
  for (const c of m.caveats) console.log("   - " + c);
}

console.log(`\n${fail === 0 ? "ALL OK" : `FAIL ${fail}`}`);
process.exit(fail === 0 ? 0 : 1);
