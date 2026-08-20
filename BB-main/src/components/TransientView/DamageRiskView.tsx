import type { TransientResult, RiskLevel } from '../../types/bearing';

interface Props {
  result: TransientResult;
}

const riskColors: Record<RiskLevel, { bg: string; text: string; border: string }> = {
  Low: { bg: 'bg-emerald-500/15', text: 'text-emerald-300', border: 'border-emerald-500/30' },
  Medium: { bg: 'bg-amber-500/15', text: 'text-amber-300', border: 'border-amber-500/30' },
  High: { bg: 'bg-orange-500/15', text: 'text-orange-300', border: 'border-orange-500/30' },
  Critical: { bg: 'bg-red-500/15', text: 'text-red-300', border: 'border-red-500/30' },
};

export default function DamageRiskView({ result }: Props) {
  const { damage_summary, risk_assessment } = result;

  return (
    <div className="space-y-4 pt-2">
      {/* Overall Risk Badge */}
      {risk_assessment && (
        <div className={`rounded-lg border p-3 ${riskColors[risk_assessment.overall_risk_level].bg} ${riskColors[risk_assessment.overall_risk_level].border}`}>
          <div className="flex items-center justify-between mb-2">
            <span className="text-xs font-semibold text-text-light">Overall Risk</span>
            <span className={`text-sm font-bold ${riskColors[risk_assessment.overall_risk_level].text}`}>
              {risk_assessment.overall_risk_level}
            </span>
          </div>
          {risk_assessment.recommendations.length > 0 && (
            <ul className="space-y-1">
              {risk_assessment.recommendations.map((r, i) => (
                <li key={i} className="text-[11px] text-text-canvas leading-tight">• {r}</li>
              ))}
            </ul>
          )}
        </div>
      )}

      {/* WEC Risk */}
      {risk_assessment && (
        <RiskCard title="WEC Risk (Guo 2021)" level={risk_assessment.wec_guo.risk_level}>
          <MetricRow label="Risk Index" value={risk_assessment.wec_guo.risk_index.toFixed(3)} />
          <MetricRow label="Slip-Load Fraction" value={risk_assessment.wec_guo.slip_load_fraction.toFixed(3)} />
          <MetricRow label="Q_max during slip" value={`${risk_assessment.wec_guo.q_max_during_slip.toFixed(0)} N`} />
          <MetricRow label="High-load slip events" value={String(risk_assessment.wec_guo.high_load_slip_events)} />
          <MetricRow label="Energy Ratio (Argonne)" value={risk_assessment.wec_energy_ratio.toFixed(4)} />
        </RiskCard>
      )}

      {/* Smearing Risk */}
      {risk_assessment && (
        <RiskCard title="Smearing Risk" level={risk_assessment.smearing.risk_level}>
          <MetricRow label="Max SRR" value={`${(risk_assessment.smearing.max_srr * 100).toFixed(3)} %`} />
          <MetricRow label="Flash Temp Rise" value={`${risk_assessment.smearing.max_flash_temp_rise.toFixed(1)} °C`} />
          <MetricRow label="Peak Slide Velocity" value={`${risk_assessment.smearing.peak_slide_velocity.toFixed(3)} m/s`} />
          <MetricRow label="Total Slide Distance" value={`${risk_assessment.smearing.total_slide_distance.toFixed(4)} m`} />
        </RiskCard>
      )}

      {/* Damage Summary */}
      <div className="border border-white/10 rounded-lg p-3">
        <p className="text-xs font-semibold text-text-light mb-2">Damage Summary</p>
        <MetricRow label="Total Slip Events" value={String(damage_summary.total_slip_events)} />
        <MetricRow label="Total Slip Duration" value={`${damage_summary.total_slip_duration_s.toFixed(4)} s`} />
        <MetricRow label="Max SRR Overall" value={`${(damage_summary.max_slip_ratio_overall * 100).toFixed(3)} %`} />
        <MetricRow label="WEC Risk Index (simple)" value={damage_summary.wec_risk_index.toFixed(4)} />
      </div>

      {/* Per-roller damage table */}
      {damage_summary.roller_damage.length > 0 && (
        <div className="border border-white/10 rounded-lg p-3">
          <p className="text-xs font-semibold text-text-light mb-2">Per-Roller Damage</p>
          <div className="overflow-x-auto">
            <table className="w-full text-xs font-mono">
              <thead>
                <tr className="text-text-canvas border-b border-white/10">
                  <th className="py-1 px-1 text-left">Roller</th>
                  <th className="py-1 px-1 text-right">Slip Events</th>
                  <th className="py-1 px-1 text-right">Slip Duration [ms]</th>
                  <th className="py-1 px-1 text-right">Friction Energy [mJ]</th>
                  <th className="py-1 px-1 text-right">Max Q in Slip [N]</th>
                </tr>
              </thead>
              <tbody>
                {damage_summary.roller_damage
                  .filter(rd => rd.slip_event_count > 0)
                  .map(rd => (
                    <tr key={rd.j} className="text-text-light border-b border-white/5 hover:bg-white/5">
                      <td className="py-0.5 px-1">{rd.j}</td>
                      <td className="py-0.5 px-1 text-right">{rd.slip_event_count}</td>
                      <td className="py-0.5 px-1 text-right">{(rd.total_slip_duration_s * 1000).toFixed(1)}</td>
                      <td className="py-0.5 px-1 text-right">{(rd.cumulative_friction_energy_j * 1000).toFixed(2)}</td>
                      <td className="py-0.5 px-1 text-right">{rd.max_contact_load_during_slip_n.toFixed(0)}</td>
                    </tr>
                  ))}
              </tbody>
            </table>
            {damage_summary.roller_damage.every(rd => rd.slip_event_count === 0) && (
              <p className="text-xs text-text-canvas/60 text-center py-2">No slip events detected</p>
            )}
          </div>
        </div>
      )}
    </div>
  );
}

function RiskCard({ title, level, children }: { title: string; level: RiskLevel; children: React.ReactNode }) {
  const colors = riskColors[level];
  return (
    <div className={`border rounded-lg p-3 ${colors.bg} ${colors.border}`}>
      <div className="flex items-center justify-between mb-2">
        <p className="text-xs font-semibold text-text-light">{title}</p>
        <span className={`text-[11px] font-bold ${colors.text}`}>{level}</span>
      </div>
      <div className="space-y-0.5">{children}</div>
    </div>
  );
}

function MetricRow({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex items-center justify-between">
      <span className="text-[11px] text-text-canvas">{label}</span>
      <span className="text-[11px] text-text-light font-mono tabular-nums">{value}</span>
    </div>
  );
}
