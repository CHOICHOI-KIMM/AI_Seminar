export function DetailTable({ title, rows }: { title: string; rows: [string, string, string][] }) {
  return (
    <div>
      <h4 className="text-sm font-semibold text-text-light mb-2 uppercase tracking-wider">
        {title}
      </h4>
      <table className="text-[13px]">
        <tbody>
          {rows.map(([label, value, unit], i) => (
            <tr key={i} className="border-b border-white/[0.03]">
              <td className="py-0.5 pr-6 text-text-canvas whitespace-nowrap">{label}</td>
              <td className="py-0.5 text-right font-mono text-text-light tabular-nums">{value}</td>
              <td className="py-0.5 pl-1.5 text-text-canvas font-mono whitespace-nowrap">{unit}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export function formatHours(hours: number): string {
  if (!isFinite(hours) || hours > 1e9) return '∞';
  if (hours > 10000) return `${(hours / 1000).toFixed(1)}k`;
  return hours.toFixed(0);
}
