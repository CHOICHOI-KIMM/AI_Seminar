import { useActiveResult } from '../../hooks/useActiveResult';
import { useAppState } from '../../store';

/** Row helper for parameter table */
function Row({ label, value, unit, highlight }: { label: string; value: string; unit?: string; highlight?: boolean }) {
  return (
    <tr className={highlight ? 'bg-amber-500/10' : 'hover:bg-white/5'}>
      <td className="py-1 pr-4 text-text-canvas text-xs">{label}</td>
      <td className="py-1 text-right font-mono text-xs text-text-light tabular-nums">{value}</td>
      {unit && <td className="py-1 pl-2 text-text-canvas text-xs">{unit}</td>}
    </tr>
  );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div>
      <h3 className="text-xs font-semibold text-text-canvas uppercase tracking-wider mb-2 pb-1 border-b border-white/10">
        {title}
      </h3>
      <table className="w-full">
        <tbody>{children}</tbody>
      </table>
    </div>
  );
}

function fmt(v: number, d = 3) {
  return v.toLocaleString('en-US', { maximumFractionDigits: d, minimumFractionDigits: d });
}
function fmtInt(v: number) {
  return Math.round(v).toLocaleString('en-US');
}

export default function ThermalSpeedView() {
  const result = useActiveResult();
  const { state } = useAppState();

  if (!result) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-text-canvas text-sm">No results yet</p>
      </div>
    );
  }

  const ts = result.thermal_speed;
  const nOp = Math.abs(state.input.operating.n_inner_rpm - state.input.operating.n_outer_rpm);
  const speedRatio = ts.speed_ratio;

  // Traffic-light color for speed ratio
  const ratioColor =
    speedRatio >= 0.9 ? 'text-red-400' : speedRatio >= 0.7 ? 'text-amber-400' : 'text-green-400';

  const statusLabel =
    speedRatio >= 0.9 ? 'Critical — near thermal limit' : speedRatio >= 0.7 ? 'Warning — high thermal load' : 'OK';
  const statusBg =
    speedRatio >= 0.9 ? 'bg-red-500/15 border-red-500/30' : speedRatio >= 0.7 ? 'bg-amber-500/15 border-amber-500/30' : 'bg-green-500/15 border-green-500/30';

  return (
    <div className="flex flex-col h-full overflow-auto px-4 py-3 gap-4">
      {/* Header card */}
      <div className="grid grid-cols-3 gap-3">
        {/* nθr */}
        <div className="bg-white/5 rounded-lg p-3 text-center">
          <p className="text-xs text-text-canvas mb-1">Thermal Speed Rating nθr</p>
          <p className="text-2xl font-bold text-text-light tabular-nums">{fmtInt(ts.n_theta_r)}</p>
          <p className="text-xs text-text-canvas mt-0.5">min⁻¹</p>
        </div>
        {/* Operating speed */}
        <div className="bg-white/5 rounded-lg p-3 text-center">
          <p className="text-xs text-text-canvas mb-1">Operating Speed n</p>
          <p className="text-2xl font-bold text-text-light tabular-nums">{fmtInt(nOp)}</p>
          <p className="text-xs text-text-canvas mt-0.5">min⁻¹</p>
        </div>
        {/* Speed ratio */}
        <div className={`rounded-lg p-3 text-center border ${statusBg}`}>
          <p className="text-xs text-text-canvas mb-1">Speed Ratio n / nθr</p>
          <p className={`text-2xl font-bold tabular-nums ${ratioColor}`}>{fmt(speedRatio, 3)}</p>
          <p className={`text-xs mt-0.5 ${ratioColor}`}>{statusLabel}</p>
        </div>
      </div>

      {/* Detail tables */}
      <div className="grid grid-cols-2 gap-4">
        {/* Reference conditions */}
        <Section title="Reference Conditions (ISO 15312 §5)">
          <Row label="Reference temperature θᵣ" value="70.0" unit="°C" />
          <Row label="Ambient temperature θAᵣ" value="20.0" unit="°C" />
          <Row label="Reference viscosity vᵣ" value={fmt(ts.v_r, 1)} unit="mm²/s" />
          <Row label="Reference load P₁ᵣ = 0.05·C₀ᵣ" value={fmt(ts.p_1r, 1)} unit="N" />
          <Row label="Coefficient f₀ᵣ" value={fmt(ts.f_0r, 1)} />
          <Row label="Coefficient f₁ᵣ" value={ts.f_1r.toExponential(4)} />
        </Section>

        {/* Heat emission */}
        <Section title="Heat Emission (ISO 15312 Eq. 2, 10)">
          <Row label="Mean diameter dm = (D+d)/2" value={fmt(ts.d_m, 2)} unit="mm" />
          <Row label="Heat emitting surface Aᵣ" value={fmtInt(ts.a_r)} unit="mm²" />
          <Row label="Heat flow density qᵣ" value={ts.q_r.toExponential(4)} unit="W/mm²" />
          <Row label="Reference heat flow Φᵣ" value={fmt(ts.phi_r, 3)} unit="W" />
        </Section>

        {/* Friction moments at nθr */}
        <Section title="Friction Moments at nθr (ISO 15312 Eq. 8, 9)">
          <Row label="Load-independent moment M₀ᵣ" value={fmt(ts.m_0r, 3)} unit="N·mm" />
          <Row label="Load-dependent moment M₁ᵣ" value={fmt(ts.m_1r, 3)} unit="N·mm" />
          <Row label="Total friction moment" value={fmt(ts.m_0r + ts.m_1r, 3)} unit="N·mm" />
          <Row label="Power loss Nᵣ = Φᵣ" value={fmt(ts.n_r, 3)} unit="W" />
        </Section>

        {/* Speed assessment */}
        <Section title="Speed Assessment">
          <Row label="Thermal speed rating nθr" value={fmtInt(ts.n_theta_r)} unit="min⁻¹" />
          <Row label="Operating speed n" value={fmtInt(nOp)} unit="min⁻¹" />
          <Row
            label="Speed ratio n / nθr"
            value={fmt(speedRatio, 4)}
            highlight={speedRatio >= 0.7}
          />
          <Row label="Thermal margin (nθr − n)" value={fmtInt(ts.n_theta_r - nOp)} unit="min⁻¹" />
        </Section>
      </div>

      {/* Note */}
      <p className="text-xs text-text-canvas/60 italic">
        ISO 15312:2018 — Thermal speed rating for oil bath lubrication with ISO VG 32 (vᵣ = 12 mm²/s at 70°C),
        reference load P₁ᵣ = 5% C₀ᵣ. Coefficients f₀ᵣ and f₁ᵣ from Table A.1 (TRB dimension series).
        Thermal speed rating is determined by iterative solution of energy balance equation (Eq. 11).
      </p>
    </div>
  );
}
