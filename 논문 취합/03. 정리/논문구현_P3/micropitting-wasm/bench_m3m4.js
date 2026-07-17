// Phase 0 §3.3 — M3→M4 계량 (계획 §3.3: "병목은 solve_fatigue").
//
// 목적: Phase 3 그리드 사이징 근거 확보. rayon 이 꺼진 단일스레드 WASM 에서
// solve_fatigue(nz·ny 컬럼 × nx 이력 MCE 최소화)의 실제 비용을 잰다.
// 벽시계는 JS 가 잰다 — 크레이트에 시계가 없고, wasm32 에서 std::time 도 불가.
//
// 사용: node bench_m3m4.js

const wasm = require("./pkg-node/micropitting_wasm.js");

// 합성 하중장(계량용 — 물리 결과가 아니라 타이밍 부하). 정현 리플 + Hertz 포락.
function makeFields(nx, ny) {
  const p = new Array(nx * ny);
  const q = new Array(nx * ny);
  for (let j = 0; j < ny; j++) {
    for (let i = 0; i < nx; i++) {
      const xr = (2 * i) / nx - 1; // -1..1
      const env = Math.max(0, 1 - xr * xr);
      const ripple = 1 + 0.2 * Math.sin((2 * Math.PI * 8 * i) / nx);
      const pv = 1.5e9 * Math.sqrt(env) * ripple;
      p[i + j * nx] = pv;
      q[i + j * nx] = 0.05 * pv; // μ_ehl = 0.05 (원논문 Table 1) 수준의 트랙션
    }
  }
  return [p, q];
}

function args(nx, ny, nz) {
  const [pd, qd] = makeFields(nx, ny);
  return JSON.stringify({
    grid: { nx, ny, lx: 5.2e-4, ly: 1.3e-4 },
    p_tran: { nx, ny, data: pd },
    q_tran: { nx, ny, data: qd },
    op: {
      p_h: 1.5e9, u_mean: 1.0, u2: 1.01, slide_roll: 0.02,
      eta0: 0.0094, alpha_visc: 20.78e-9, tau0: 3.0e6, temp: 348.15, r_x: 0.01,
    },
    mat: { e_red: 115.384615384615e9, nu: 0.3, hardness: 7.0e9, p_lim: 4.0e9 },
    nz,
  });
}

const CASES = [
  [128, 16, 15],
  [256, 32, 15],
  [512, 64, 15],
];

console.log("nx    ny   nz   wall(ms)   b(m)         maxVM(Pa)     maxD");
console.log("-".repeat(70));
for (const [nx, ny, nz] of CASES) {
  const input = args(nx, ny, nz);
  const t0 = process.hrtime.bigint();
  const out = wasm.solve_stress_fatigue_json(input);
  const t1 = process.hrtime.bigint();
  const ms = Number(t1 - t0) / 1e6;
  const r = JSON.parse(out);
  if (!r.ok) {
    console.log(`${nx}\t${ny}\t${nz}\tFAIL: ${r.error}`);
    continue;
  }
  console.log(
    `${String(nx).padEnd(5)} ${String(ny).padEnd(4)} ${String(nz).padEnd(4)} ` +
      `${ms.toFixed(0).padStart(8)}   ${r.b.toExponential(3)}   ` +
      `${r.maxVonMises.toExponential(3)}   ${r.maxDangVanD.toExponential(3)}`
  );
}
