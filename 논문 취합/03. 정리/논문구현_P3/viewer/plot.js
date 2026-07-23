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
  if (v === null || !Number.isFinite(v)) return "∞"; // serde_json: f64 ∞/NaN → null
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
  const hasRight = (opts.series || []).some((s) => s.axis === "right");
  const L = 62, R = hasRight ? 64 : 14, T = 48, B = 44;
  ctx.clearRect(0, 0, W, H);
  ctx.font = "12px sans-serif";

  const shadeName = opts.shadeMaskSeries || null;
  const drawSeries = (opts.series || []).filter((s) => s.name !== shadeName && s.label !== "__mask__");
  const mask = (opts.series || []).find((s) => s.name === shadeName);

  const leftSeries = drawSeries.filter((s) => s.axis !== "right");
  const rightSeries = drawSeries.filter((s) => s.axis === "right");
  let xs = [], ys = [];
  for (const s of drawSeries) xs = xs.concat(s.x);
  for (const s of leftSeries) ys = ys.concat(s.y);
  for (const p of opts.points || []) { xs.push(p.x); ys.push(p.y); }
  if (!xs.length) return;
  xs = xs.filter(Number.isFinite); ys = ys.filter(Number.isFinite);
  if (!xs.length || !ys.length) return;
  const xmin0 = Math.min(...xs), xmax0 = Math.max(...xs);
  let ymin = Math.min(...ys, 0), ymax = Math.max(...ys);
  if (ymax === ymin) ymax = ymin + 1;
  const pad = 0.06 * (ymax - ymin);
  ymin -= pad; ymax += pad;

  // 우측 보조축 범위 (Δh_w 등 스케일이 다른 시리즈 — 배율 조작 없이 제 크기로)
  let ys2 = [];
  for (const s of rightSeries) ys2 = ys2.concat(s.y);
  ys2 = ys2.filter(Number.isFinite);
  let ymin2 = 0, ymax2 = 1;
  if (ys2.length) {
    ymin2 = Math.min(...ys2, 0); ymax2 = Math.max(...ys2);
    if (ymax2 === ymin2) ymax2 = ymin2 + 1;
    const p2 = 0.06 * (ymax2 - ymin2); ymin2 -= p2; ymax2 += p2;
  }

  const xl = opts.xLog;
  const tx = (x) => xl
    ? L + ((Math.log10(x) - Math.log10(xmin0)) / (Math.log10(xmax0) - Math.log10(xmin0))) * (W - L - R)
    : L + ((x - xmin0) / (xmax0 - xmin0)) * (W - L - R);
  const ty = (y) => H - B - ((y - ymin) / (ymax - ymin)) * (H - T - B);
  const ty2 = (y) => H - B - ((y - ymin2) / (ymax2 - ymin2)) * (H - T - B);

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
  ctx.textAlign = "right";
  for (const t of niceTicks(ymin, ymax)) {
    const py = ty(t);
    ctx.strokeStyle = "#e2e8f0"; ctx.beginPath(); ctx.moveTo(L, py); ctx.lineTo(W - R, py); ctx.stroke();
    ctx.fillText(fmt(t), L - 8, py + 4); // 축에 붙여 우측정렬 → 좌단 회전 단위라벨과 분리
  }
  ctx.textAlign = "left";
  if (opts.xLabel) ctx.fillText(opts.xLabel, (W - L) / 2, H - 8);
  if (opts.yLabel) { ctx.save(); ctx.translate(12, (H + T) / 2); ctx.rotate(-Math.PI / 2); ctx.textAlign = "center"; ctx.fillText(opts.yLabel, 0, 0); ctx.restore(); }
  if (opts.title) { ctx.fillStyle = "#0f172a"; ctx.font = "bold 13px sans-serif"; ctx.fillText(opts.title, L, 16); ctx.font = "12px sans-serif"; }

  // 우측 보조축 눈금
  if (rightSeries.length) {
    ctx.fillStyle = "#9a3412";
    for (const t of niceTicks(ymin2, ymax2)) {
      const py = ty2(t);
      ctx.fillText(fmt(t), W - R + 5, py + 4);
    }
    if (opts.yLabelRight) {
      ctx.save(); ctx.translate(W - 8, (H + T) / 2); ctx.rotate(-Math.PI / 2);
      ctx.fillText(opts.yLabelRight, 0, 0); ctx.restore();
    }
  }

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
    const tyf = s.axis === "right" ? ty2 : ty;
    let started = false;
    for (let i = 0; i < s.x.length; i++) {
      if (xl && !(s.x[i] > 0)) continue;
      if (!Number.isFinite(s.y[i])) continue;
      const px = tx(s.x[i]), py = tyf(s.y[i]);
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

  // 범례 — 플롯 프레임 **밖** 상단 밴드(제목 오른쪽~프레임 위) 가로 배치 → 곡선과 절대 안 겹침
  const labeled = drawSeries.filter((s) => s.label);
  if (labeled.length) {
    const itemW = labeled.map((s) => 14 + 5 + ctx.measureText(s.label).width + 16);
    const totalW = itemW.reduce((a, b) => a + b, 0);
    let lx = Math.max(L, W - R - totalW);
    const lyy = T - 13; // 프레임(T) 위 밴드
    labeled.forEach((s, k) => {
      const si = drawSeries.indexOf(s);
      ctx.fillStyle = s.color || COLORS[si % COLORS.length];
      ctx.fillRect(lx, lyy, 14, 3);
      ctx.fillStyle = "#334155";
      ctx.fillText(s.label, lx + 19, lyy + 6);
      lx += itemW[k];
    });
  }
}

// 히트맵. data = [row][col] (row=z, col=x). opts:{xLabel,yLabel,title,x0,x1,y0,y1,yDown}
export function heatmap(canvas, data, opts = {}) {
  const ctx = canvas.getContext("2d");
  const W = canvas.width, H = canvas.height;
  const L = 62, R = 70, T = 28, B = 40;
  ctx.clearRect(0, 0, W, H);
  const nz = data.length, nx = data[0]?.length || 0;
  if (!nz || !nx) return;
  // null = serde_json 이 직렬화한 f64 ∞/NaN (예: Dang Van D 분모 τ_f−a·p̂ ≤ 0 → D=∞ = 기준 위배).
  // 숨기지 않고 전용색으로 표시한다(무증상 금지).
  let vmin = Infinity, vmax = -Infinity, hasInf = false;
  for (const row of data) for (const v of row) {
    if (v === null || !Number.isFinite(v)) { hasInf = true; continue; }
    if (v < vmin) vmin = v; if (v > vmax) vmax = v;
  }
  if (!Number.isFinite(vmin)) { vmin = 0; vmax = 1; }
  if (vmax === vmin) vmax = vmin + 1;
  // viridis 근사 3-스톱 (렌더링 전용)
  const colorOf = (t) => {
    const stops = [[68, 1, 84], [33, 145, 140], [253, 231, 37]];
    const s = t < 0.5 ? 0 : 1, u = t < 0.5 ? t * 2 : (t - 0.5) * 2;
    const c0 = stops[s], c1 = stops[s + 1];
    return `rgb(${Math.round(c0[0] + (c1[0] - c0[0]) * u)},${Math.round(c0[1] + (c1[1] - c0[1]) * u)},${Math.round(c0[2] + (c1[2] - c0[2]) * u)})`;
  };
  // ── contourf 스타일: 쌍선형 보간 + 이산 레벨 양자화 (표시 계층 — 데이터 불변) ──
  // 논문 Fig 6(b)(MATLAB contourf) 대응: 셀 블록 대신 픽셀 보간, ~levels 단의 깔끔한 띠.
  const levels = opts.levels || 10;
  const PW = Math.max(1, Math.round(W - L - R)), PH = Math.max(1, Math.round(H - T - B));
  const img = ctx.createImageData(PW, PH);
  const px8 = img.data;
  const colorRGB = (t) => {
    const stops = [[68, 1, 84], [33, 145, 140], [253, 231, 37]];
    const si = t < 0.5 ? 0 : 1, u = t < 0.5 ? t * 2 : (t - 0.5) * 2;
    const c0 = stops[si], c1 = stops[si + 1];
    return [Math.round(c0[0] + (c1[0] - c0[0]) * u), Math.round(c0[1] + (c1[1] - c0[1]) * u), Math.round(c0[2] + (c1[2] - c0[2]) * u)];
  };
  const INF_RGB = [225, 29, 72]; // ∞ 전용색 유지
  const fin = (v) => v !== null && Number.isFinite(v);
  for (let py = 0; py < PH; py++) {
    const gz = nz > 1 ? (py / (PH - 1)) * (nz - 1) : 0;
    const z0 = Math.min(nz - 1, Math.floor(gz)), z1 = Math.min(nz - 1, z0 + 1), fz = gz - z0;
    for (let pxx = 0; pxx < PW; pxx++) {
      const gx = nx > 1 ? (pxx / (PW - 1)) * (nx - 1) : 0;
      const x0 = Math.min(nx - 1, Math.floor(gx)), x1 = Math.min(nx - 1, x0 + 1), fx = gx - x0;
      const v00 = data[z0][x0], v10 = data[z0][x1], v01 = data[z1][x0], v11 = data[z1][x1];
      let rgb;
      if (!fin(v00) || !fin(v10) || !fin(v01) || !fin(v11)) {
        // ∞/NaN 이웃 → 최근접 코너 값으로 폴백 (∞이면 전용색: D 기준위배 띠를 뭉개지 않음)
        const near = fz < 0.5 ? (fx < 0.5 ? v00 : v10) : (fx < 0.5 ? v01 : v11);
        if (fin(near)) {
          const kn = Math.max(0, Math.min(levels - 1, Math.floor(((near - vmin) / (vmax - vmin)) * levels)));
          rgb = colorRGB(levels > 1 ? kn / (levels - 1) : 0.5);
        } else {
          rgb = INF_RGB;
        }
      } else {
        const v = v00 * (1 - fx) * (1 - fz) + v10 * fx * (1 - fz) + v01 * (1 - fx) * fz + v11 * fx * fz;
        const k = Math.max(0, Math.min(levels - 1, Math.floor(((v - vmin) / (vmax - vmin)) * levels)));
        rgb = colorRGB(levels > 1 ? k / (levels - 1) : 0.5);
      }
      const o = (py * PW + pxx) * 4;
      px8[o] = rgb[0]; px8[o + 1] = rgb[1]; px8[o + 2] = rgb[2]; px8[o + 3] = 255;
    }
  }
  ctx.putImageData(img, L, T);
  ctx.strokeStyle = "#94a3b8"; ctx.strokeRect(L, T, W - L - R, H - T - B);
  ctx.fillStyle = "#475569"; ctx.font = "12px sans-serif";
  // 실단위 눈금 (opts.x0/x1 = 가로, opts.y0/y1 = 세로(깊이, 아래로 증가))
  if (opts.x1 !== undefined) {
    for (const t of niceTicks(opts.x0 || 0, opts.x1)) {
      const px = L + ((t - (opts.x0 || 0)) / (opts.x1 - (opts.x0 || 0))) * (W - L - R);
      ctx.beginPath(); ctx.moveTo(px, H - B); ctx.lineTo(px, H - B + 4); ctx.stroke();
      ctx.fillText(fmt(t), px - 10, H - B + 16);
    }
  }
  if (opts.y1 !== undefined) {
    ctx.textAlign = "right";
    for (const t of niceTicks(opts.y0 || 0, opts.y1)) {
      const py = T + ((t - (opts.y0 || 0)) / (opts.y1 - (opts.y0 || 0))) * (H - T - B);
      ctx.beginPath(); ctx.moveTo(L - 4, py); ctx.lineTo(L, py); ctx.stroke();
      ctx.fillText(fmt(t), L - 7, py + 4);
    }
    ctx.textAlign = "left";
  }
  if (opts.title) { ctx.fillStyle = "#0f172a"; ctx.font = "bold 13px sans-serif"; ctx.fillText(opts.title, L, 16); ctx.font = "12px sans-serif"; ctx.fillStyle = "#475569"; }
  if (opts.xLabel) ctx.fillText(opts.xLabel, (W - L) / 2, H - 8);
  if (opts.yLabel) { ctx.save(); ctx.translate(14, (H + T) / 2); ctx.rotate(-Math.PI / 2); ctx.fillText(opts.yLabel, 0, 0); ctx.restore(); }
  // 컬러바
  const cbX = W - R + 14, cbW = 14, cbH = H - T - B;
  for (let k = 0; k < levels; k++) {
    ctx.fillStyle = colorOf(levels > 1 ? k / (levels - 1) : 0.5);
    const y1b = T + cbH - ((k + 1) / levels) * cbH;
    ctx.fillRect(cbX, y1b, cbW, cbH / levels + 0.5);
  }
  ctx.strokeRect(cbX, T, cbW, cbH);
  ctx.fillText(fmt(vmax), cbX - 4, T - 4);
  ctx.fillText(fmt(vmin), cbX - 4, T + cbH + 12);
  if (hasInf) {
    ctx.fillStyle = "#e11d48";
    ctx.fillRect(cbX, T + cbH + 18, cbW, 8);
    ctx.fillStyle = "#991b1b";
    ctx.fillText("= ∞ (D 기준 위배)", cbX + cbW + 4, T + cbH + 26);
  }
}
