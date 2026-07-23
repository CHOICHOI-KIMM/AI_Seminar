// plot.js — 최소 캔버스 플롯 (뷰어 전용).
// ★ R8: 이 파일은 **그리기만** 한다. 물리량 계산 0건 — 배열은 전부 WASM 이 만든 것.
// 좌표 변환(데이터→픽셀)·축 눈금·컬러맵은 렌더링이지 물리가 아니다.

const COLORS = ["#2563eb", "#dc2626", "#059669", "#d97706", "#7c3aed", "#0891b2", "#be185d"];

function niceTicks(min, max, n = 6) {
  if (!(max > min)) return [min];
  const span = max - min;
  const step0 = span / n;
  const mag = Math.pow(10, Math.floor(Math.log10(step0)));
  const step = [1, 2, 5, 10].map((m) => m * mag).find((s) => span / s <= n) || mag * 10;
  const t0 = Math.ceil(min / step) * step;
  const out = [];
  for (let t = t0; t <= max + 1e-12 * span; t += step) out.push(t);
  return out;
}

function fmt(v) {
  if (v === 0) return "0";
  const a = Math.abs(v);
  if (a >= 1e4 || a < 1e-3) return v.toExponential(1);
  return String(Math.round(v * 1000) / 1000);
}

// 선형/로그 x 지원 라인플롯. opts: {series:[{x,y,label,color,dash}], points:[{x,y,label,color}],
//   xLog, xLabel, yLabel, title, shadeMaskSeries(이름): y==1 구간 음영, hlines:[{y,label}]}
export function linePlot(canvas, opts) {
  const ctx = canvas.getContext("2d");
  const W = canvas.width, H = canvas.height;
  const L = 62, R = 14, T = 28, B = 44;
  ctx.clearRect(0, 0, W, H);
  ctx.font = "12px sans-serif";

  const shadeName = opts.shadeMaskSeries || null;
  const drawSeries = (opts.series || []).filter((s) => s.name !== shadeName && s.label !== "__mask__");
  const mask = (opts.series || []).find((s) => s.name === shadeName);

  let xs = [], ys = [];
  for (const s of drawSeries) { xs = xs.concat(s.x); ys = ys.concat(s.y); }
  for (const p of opts.points || []) { xs.push(p.x); ys.push(p.y); }
  if (!xs.length) return;
  const xmin0 = Math.min(...xs), xmax0 = Math.max(...xs);
  let ymin = Math.min(...ys, 0), ymax = Math.max(...ys);
  if (ymax === ymin) ymax = ymin + 1;
  const pad = 0.06 * (ymax - ymin);
  ymin -= pad; ymax += pad;

  const xl = opts.xLog;
  const tx = (x) => xl
    ? L + ((Math.log10(x) - Math.log10(xmin0)) / (Math.log10(xmax0) - Math.log10(xmin0))) * (W - L - R)
    : L + ((x - xmin0) / (xmax0 - xmin0)) * (W - L - R);
  const ty = (y) => H - B - ((y - ymin) / (ymax - ymin)) * (H - T - B);

  // 음영 (fit degrades 등)
  if (mask) {
    ctx.fillStyle = "rgba(220,38,38,0.08)";
    let start = null;
    for (let i = 0; i < mask.x.length; i++) {
      const on = mask.y[i] > 0.5;
      if (on && start === null) start = mask.x[i];
      if ((!on || i === mask.x.length - 1) && start !== null) {
        const end = mask.x[i];
        ctx.fillRect(tx(start), T, tx(end) - tx(start), H - T - B);
        start = null;
      }
    }
  }

  // 축
  ctx.strokeStyle = "#94a3b8"; ctx.lineWidth = 1;
  ctx.strokeRect(L, T, W - L - R, H - T - B);
  ctx.fillStyle = "#475569";
  const xticks = xl
    ? (() => { const o = []; for (let e = Math.ceil(Math.log10(xmin0)); e <= Math.floor(Math.log10(xmax0)); e++) o.push(Math.pow(10, e)); return o.length ? o : [xmin0, xmax0]; })()
    : niceTicks(xmin0, xmax0);
  for (const t of xticks) {
    const px = tx(t);
    if (px < L - 1 || px > W - R + 1) continue;
    ctx.strokeStyle = "#e2e8f0"; ctx.beginPath(); ctx.moveTo(px, T); ctx.lineTo(px, H - B); ctx.stroke();
    ctx.fillText(fmt(t), px - 12, H - B + 16);
  }
  for (const t of niceTicks(ymin, ymax)) {
    const py = ty(t);
    ctx.strokeStyle = "#e2e8f0"; ctx.beginPath(); ctx.moveTo(L, py); ctx.lineTo(W - R, py); ctx.stroke();
    ctx.fillText(fmt(t), 6, py + 4);
  }
  if (opts.xLabel) ctx.fillText(opts.xLabel, (W - L) / 2, H - 8);
  if (opts.yLabel) { ctx.save(); ctx.translate(14, (H + T) / 2); ctx.rotate(-Math.PI / 2); ctx.fillText(opts.yLabel, 0, 0); ctx.restore(); }
  if (opts.title) { ctx.fillStyle = "#0f172a"; ctx.font = "bold 13px sans-serif"; ctx.fillText(opts.title, L, 16); ctx.font = "12px sans-serif"; }

  // 수평 기준선
  for (const h of opts.hlines || []) {
    ctx.strokeStyle = "#64748b"; ctx.setLineDash([4, 4]);
    ctx.beginPath(); ctx.moveTo(L, ty(h.y)); ctx.lineTo(W - R, ty(h.y)); ctx.stroke();
    ctx.setLineDash([]);
    if (h.label) { ctx.fillStyle = "#64748b"; ctx.fillText(h.label, W - R - 70, ty(h.y) - 4); }
  }

  // 시리즈
  drawSeries.forEach((s, si) => {
    ctx.strokeStyle = s.color || COLORS[si % COLORS.length];
    ctx.lineWidth = 1.8;
    if (s.dash) ctx.setLineDash([6, 4]);
    ctx.beginPath();
    let started = false;
    for (let i = 0; i < s.x.length; i++) {
      if (xl && !(s.x[i] > 0)) continue;
      const px = tx(s.x[i]), py = ty(s.y[i]);
      if (!started) { ctx.moveTo(px, py); started = true; } else ctx.lineTo(px, py);
    }
    ctx.stroke();
    ctx.setLineDash([]);
  });

  // 점 (문헌 데이터)
  (opts.points || []).forEach((p) => {
    ctx.fillStyle = p.color || "#0f172a";
    ctx.beginPath(); ctx.arc(tx(p.x), ty(p.y), 4, 0, 2 * Math.PI); ctx.fill();
    ctx.strokeStyle = "#fff"; ctx.lineWidth = 1; ctx.stroke();
  });

  // 범례
  let lx = L + 8, lyy = T + 8;
  drawSeries.forEach((s, si) => {
    if (!s.label) return;
    ctx.fillStyle = s.color || COLORS[si % COLORS.length];
    ctx.fillRect(lx, lyy, 14, 3);
    ctx.fillStyle = "#334155";
    ctx.fillText(s.label, lx + 20, lyy + 6);
    lyy += 16;
  });
}

// 히트맵. data = [row][col] (row=z, col=x). opts:{xLabel,yLabel,title,x0,x1,y0,y1,yDown}
export function heatmap(canvas, data, opts = {}) {
  const ctx = canvas.getContext("2d");
  const W = canvas.width, H = canvas.height;
  const L = 62, R = 70, T = 28, B = 40;
  ctx.clearRect(0, 0, W, H);
  const nz = data.length, nx = data[0]?.length || 0;
  if (!nz || !nx) return;
  let vmin = Infinity, vmax = -Infinity;
  for (const row of data) for (const v of row) { if (v < vmin) vmin = v; if (v > vmax) vmax = v; }
  if (vmax === vmin) vmax = vmin + 1;
  // viridis 근사 3-스톱 (렌더링 전용)
  const colorOf = (t) => {
    const stops = [[68, 1, 84], [33, 145, 140], [253, 231, 37]];
    const s = t < 0.5 ? 0 : 1, u = t < 0.5 ? t * 2 : (t - 0.5) * 2;
    const c0 = stops[s], c1 = stops[s + 1];
    return `rgb(${Math.round(c0[0] + (c1[0] - c0[0]) * u)},${Math.round(c0[1] + (c1[1] - c0[1]) * u)},${Math.round(c0[2] + (c1[2] - c0[2]) * u)})`;
  };
  const cw = (W - L - R) / nx, ch = (H - T - B) / nz;
  for (let r = 0; r < nz; r++) {
    const rr = opts.yDown === false ? nz - 1 - r : r;
    for (let c = 0; c < nx; c++) {
      ctx.fillStyle = colorOf((data[rr][c] - vmin) / (vmax - vmin));
      ctx.fillRect(L + c * cw, T + r * ch, cw + 0.6, ch + 0.6);
    }
  }
  ctx.strokeStyle = "#94a3b8"; ctx.strokeRect(L, T, W - L - R, H - T - B);
  ctx.fillStyle = "#475569"; ctx.font = "12px sans-serif";
  if (opts.title) { ctx.fillStyle = "#0f172a"; ctx.font = "bold 13px sans-serif"; ctx.fillText(opts.title, L, 16); ctx.font = "12px sans-serif"; ctx.fillStyle = "#475569"; }
  if (opts.xLabel) ctx.fillText(opts.xLabel, (W - L) / 2, H - 8);
  if (opts.yLabel) { ctx.save(); ctx.translate(14, (H + T) / 2); ctx.rotate(-Math.PI / 2); ctx.fillText(opts.yLabel, 0, 0); ctx.restore(); }
  // 컬러바
  const cbX = W - R + 14, cbW = 14, cbH = H - T - B;
  for (let i = 0; i < cbH; i++) {
    ctx.fillStyle = colorOf(1 - i / cbH);
    ctx.fillRect(cbX, T + i, cbW, 1.5);
  }
  ctx.strokeRect(cbX, T, cbW, cbH);
  ctx.fillText(fmt(vmax), cbX - 4, T - 4);
  ctx.fillText(fmt(vmin), cbX - 4, T + cbH + 12);
}
