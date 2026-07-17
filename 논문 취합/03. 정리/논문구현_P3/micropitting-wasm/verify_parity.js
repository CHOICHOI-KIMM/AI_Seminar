// Phase 0 기준 ② — WASM 출력 == 네이티브 출력 (계획 §3.2).
//
// 같은 fixture_wear.json 을 같은 run_wear 코어에 통과시키고 결과를 대조한다.
// 사용: node verify_parity.js "<네이티브 출력 JSON>"
//   (네이티브 출력: cargo run --example native_ref)

const fs = require("fs");
const path = require("path");
const wasm = require("./pkg-node/micropitting_wasm.js");

const fixture = fs.readFileSync(path.join(__dirname, "fixture_wear.json"), "utf8");
const wasmOut = wasm.solve_wear_json(fixture);
const nativeOut = process.argv[2];

console.log("WASM   :", wasmOut);

if (!nativeOut) {
  console.log("\n(네이티브 출력 미지정 — 대조 생략)");
  process.exit(0);
}
console.log("NATIVE :", nativeOut);

// 문자열 동일이 가장 강한 판정(f64 직렬화까지 일치). 실패 시에만 수치 허용오차로 하향 진단.
if (wasmOut === nativeOut) {
  console.log("\n[PASS] 기준 ②: WASM 출력 == 네이티브 출력 (문자열 동일)");
  process.exit(0);
}

console.log("\n[!] 문자열 불일치 — 수치 대조로 하향 진단");
const w = JSON.parse(wasmOut);
const n = JSON.parse(nativeOut);
if (!w.ok || !n.ok) {
  console.log(`[FAIL] ok=false  wasm.error=${w.error}  native.error=${n.error}`);
  process.exit(1);
}
const wd = w.result.dh_w.data;
const nd = n.result.dh_w.data;
let maxRel = 0;
for (let i = 0; i < nd.length; i++) {
  const d = Math.abs(wd[i] - nd[i]);
  const rel = nd[i] === 0 ? d : d / Math.abs(nd[i]);
  if (rel > maxRel) maxRel = rel;
}
const meanRel =
  Math.abs(w.result.dh_w_mean - n.result.dh_w_mean) / Math.abs(n.result.dh_w_mean);
console.log(`max rel diff (dh_w) = ${maxRel}`);
console.log(`rel diff (dh_w_mean) = ${meanRel}`);
// 동일 코드·동일 IEEE754 → bit-exact 기대. 미세차라도 원인 규명 전에는 PASS 로 넘기지 않는다.
console.log(maxRel === 0 && meanRel === 0 ? "[PASS] 수치 동일" : "[FAIL] 수치 상이");
process.exit(maxRel === 0 && meanRel === 0 ? 0 : 1);
