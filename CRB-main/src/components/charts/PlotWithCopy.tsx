/**
 * Drop-in replacement for react-plotly.js <Plot> that adds a right-click
 * context menu for copying chart data as TSV / CSV / JSON.
 *
 * Usage: replace `import Plot from 'react-plotly.js'`
 *   with  `import Plot from '../charts/PlotWithCopy'`
 *
 * All original <Plot> props are forwarded unchanged.
 */
import { useState, useEffect, useCallback, useRef } from 'react';
import BasePlot from 'react-plotly.js';
import type { PlotParams } from 'react-plotly.js';

type CopyFormat = 'tsv' | 'csv' | 'json';

interface MenuState {
  x: number;
  y: number;
  traces: TraceData[];
}

interface TraceData {
  name: string;
  columns: { header: string; values: (string | number)[] }[];
}

/** Extract copy-friendly data from Plotly trace objects. */
function extractTraces(data: Plotly.Data[]): TraceData[] {
  const out: TraceData[] = [];
  for (const t of data) {
    const trace = t as any;
    const name: string = trace.name || `trace ${out.length + 1}`;

    // 2D scatter / line
    if (Array.isArray(trace.x) && Array.isArray(trace.y)) {
      const xLabel = 'x';
      const yLabel = name;
      out.push({
        name,
        columns: [
          { header: xLabel, values: trace.x },
          { header: yLabel, values: trace.y },
        ],
      });
      continue;
    }

    // surface / heatmap — z matrix
    if (Array.isArray(trace.z) && Array.isArray(trace.z[0])) {
      // Flatten to a single column summary (too large for full copy)
      const rows = trace.z.length;
      const cols = (trace.z[0] as number[]).length;
      out.push({
        name,
        columns: [
          { header: 'info', values: [`${rows}×${cols} matrix — use JSON for full data`] },
        ],
      });
      continue;
    }
  }
  return out;
}

function formatData(traces: TraceData[], fmt: CopyFormat): string {
  if (fmt === 'json') {
    const obj = traces.map(t => ({
      name: t.name,
      ...Object.fromEntries(t.columns.map(c => [c.header, c.values])),
    }));
    return JSON.stringify(obj, null, 2);
  }

  const sep = fmt === 'tsv' ? '\t' : ',';

  // Merge all traces side-by-side
  const allCols: { header: string; values: (string | number)[] }[] = [];
  for (const t of traces) {
    for (const c of t.columns) {
      // Prefix with trace name if multiple traces share same header
      const header = traces.length > 1 ? `${t.name} ${c.header}` : c.header;
      allCols.push({ header, values: c.values });
    }
  }

  const maxLen = Math.max(...allCols.map(c => c.values.length));
  const lines: string[] = [];
  lines.push(allCols.map(c => c.header).join(sep));
  for (let i = 0; i < maxLen; i++) {
    lines.push(allCols.map(c => (i < c.values.length ? c.values[i] : '')).join(sep));
  }
  return lines.join('\n');
}

export default function PlotWithCopy(props: PlotParams) {
  const [menu, setMenu] = useState<MenuState | null>(null);
  const [copiedFmt, setCopiedFmt] = useState<CopyFormat | null>(null);
  const wrapRef = useRef<HTMLDivElement>(null);

  const handleContext = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    const traces = extractTraces(props.data as Plotly.Data[]);
    if (traces.length === 0) return;
    setCopiedFmt(null);
    setMenu({ x: e.clientX, y: e.clientY, traces });
  }, [props.data]);

  const closeMenu = useCallback(() => { setMenu(null); setCopiedFmt(null); }, []);

  const handleCopy = useCallback(async (fmt: CopyFormat) => {
    if (!menu) return;
    const text = formatData(menu.traces, fmt);
    await navigator.clipboard.writeText(text);
    setCopiedFmt(fmt);
    setTimeout(closeMenu, 600);
  }, [menu, closeMenu]);

  useEffect(() => {
    if (!menu) return;
    const onKey = (e: KeyboardEvent) => { if (e.key === 'Escape') closeMenu(); };
    const onClick = () => closeMenu();
    window.addEventListener('keydown', onKey);
    window.addEventListener('click', onClick);
    return () => { window.removeEventListener('keydown', onKey); window.removeEventListener('click', onClick); };
  }, [menu, closeMenu]);

  const totalPts = menu?.traces.reduce((s, t) => s + Math.max(...t.columns.map(c => c.values.length)), 0) ?? 0;

  return (
    <div ref={wrapRef} onContextMenu={handleContext} style={{ width: '100%', height: '100%' }}>
      <BasePlot {...props} />

      {menu && (
        <div
          className="fixed z-50 min-w-[180px] bg-gray-900 border border-white/15 rounded-lg shadow-xl py-1 text-sm"
          style={{ left: menu.x, top: menu.y }}
          onClick={e => e.stopPropagation()}
        >
          <div className="px-3 py-1.5 text-[11px] text-gray-400 border-b border-white/10 truncate">
            {menu.traces.length} trace{menu.traces.length > 1 ? 's' : ''} · {totalPts} pts
          </div>
          {(['tsv', 'csv', 'json'] as CopyFormat[]).map(fmt => (
            <button
              key={fmt}
              className="w-full text-left px-3 py-1.5 text-gray-200 hover:bg-white/10 transition-colors flex items-center gap-2 cursor-pointer"
              onClick={() => handleCopy(fmt)}
            >
              <span className="text-gray-500 text-xs w-4">{copiedFmt === fmt ? '✓' : '📋'}</span>
              {copiedFmt === fmt
                ? <span className="text-emerald-400">Copied!</span>
                : `Copy as ${fmt.toUpperCase()}`}
            </button>
          ))}
        </div>
      )}
    </div>
  );
}
