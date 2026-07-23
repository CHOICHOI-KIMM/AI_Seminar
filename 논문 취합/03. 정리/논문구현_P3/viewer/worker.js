// worker.js — WASM 호출 전담 (module worker). UI 블로킹 방지 (R2 대응: 512×64 ≈ 1.3s).
// ★ R8: 물리 계산 0건 — 모든 수치는 WASM 경계 너머 micropitting_model 이 낸다.
//   여기서 하는 유일한 "생성"은 거칠기 **입력 합성**(사인 리플, 물리 법칙 아님 — 시험입력).

import init, {
  solve_chain_json,
  reference_curve_json,
  reference_tables_json,
} from "./pkg/micropitting_wasm.js";

const wstep = (n) => fetch(`/__viewer__/w/${n}`).catch(() => {});
wstep("moduleEval");
const ready = init().then((r) => { wstep("initDone"); return r; }, (e) => { wstep("initFail/" + encodeURIComponent(String(e)).slice(0, 80)); throw e; });

// 거칠기 입력 합성 — Rq 지정 사인 리플 (진폭 = Rq·√2 는 사인파의 RMS 정의, 입력 규격이지 물리 모델 아님).
function sineRough(nx, ny, rq, waves) {
  const amp = rq * Math.SQRT2;
  const data = new Array(nx * ny);
  for (let j = 0; j < ny; j++)
    for (let i = 0; i < nx; i++)
      data[i + j * nx] = amp * Math.sin((2 * Math.PI * waves * i) / nx);
  return { nx, ny, data };
}

self.onmessage = async (ev) => {
  const { id, cmd, payload } = ev.data;
  wstep(`msg/${cmd}`);
  try {
    await ready;
    let result;
    if (cmd === "chain") {
      // payload: {grid:{nx,ny,lx,ly}, rq1, rq2, waves, mat, op, h_bar, nz}
      const { grid, rq1, rq2, waves, mat, op, h_bar, nz } = payload;
      const args = {
        grid,
        rough1: sineRough(grid.nx, grid.ny, rq1, waves),
        rough2: sineRough(grid.nx, grid.ny, rq2, waves),
        mat,
        op,
        h_bar,
        nz,
      };
      result = JSON.parse(solve_chain_json(JSON.stringify(args)));
      // 입력 echo — WASM 에 투입된 **바로 그 배열**의 j0 행을 그대로 반환(표시=투입, 재계산 금지).
      if (result.ok) {
        const j0 = result.sliceJ ?? 0;
        result.inputEcho = {
          rough1: args.rough1.data.slice(j0 * grid.nx, (j0 + 1) * grid.nx),
          rough2: args.rough2.data.slice(j0 * grid.nx, (j0 + 1) * grid.nx),
        };
      }
    } else if (cmd === "refCurve") {
      result = JSON.parse(reference_curve_json(payload.kind, JSON.stringify(payload.params || {})));
    } else if (cmd === "refTables") {
      result = JSON.parse(reference_tables_json());
    } else {
      throw new Error(`unknown cmd: ${cmd}`);
    }
    self.postMessage({ id, ok: true, result });
  } catch (e) {
    self.postMessage({ id, ok: false, error: String(e) });
  }
};
