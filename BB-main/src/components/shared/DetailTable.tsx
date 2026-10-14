// 행 = [라벨, 값, 단위, 툴팁?] — 툴팁은 라벨에서 뺀 식·식번호·설명을 마우스 오버로 보인다.
export function DetailTable({ title, rows }: { title: string; rows: [string, string, string, string?][] }) {
  return (
    <div>
      <h4 className="text-sm font-medium text-text-light mb-2">
        {title}
      </h4>
      <table className="text-[13px]">
        <tbody>
          {rows.map(([label, value, unit, tip], i) => (
            <tr key={i} className="border-b border-white/[0.03]">
              <td className="py-0.5 pr-6 text-text-canvas whitespace-nowrap" title={tip}>{label}</td>
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
