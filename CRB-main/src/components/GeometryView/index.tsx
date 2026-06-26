import { useActiveResult } from '../../hooks/useActiveResult';
import { useAppState } from '../../store';
import { DetailTable } from '../shared/DetailTable';
import type {
  BearingInput, SliceGeometry, CrownType, RacewayProfile,
} from '../../types/bearing';

export default function GeometryView() {
  const result = useActiveResult();
  const { state } = useAppState();

  if (!result) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-text-canvas text-sm">No results yet</p>
      </div>
    );
  }

  const geo = result.geometry;
  const input = state.input;
  const mg = input.macro_geom;
  const rg = input.raceway_geom;
  const rp = input.roller_profile;
  const mat = input.material;

  return (
    <div className="h-full overflow-auto custom-scrollbar p-4 space-y-4">
      {/* Bearing Macro Geometry */}
      <DetailTable title="Bearing Macro Geometry" rows={[
        ['Bore diameter (d)', `${mg.d.toFixed(1)}`, 'mm'],
        ['Outer diameter (D)', `${mg.outer_diameter.toFixed(1)}`, 'mm'],
        ['Assembly height (T)', `${mg.t.toFixed(2)}`, 'mm'],
        ['Pitch diameter (d_pw)', `${mg.d_pw.toFixed(2)}`, 'mm'],
        ['Number of rollers (Z)', `${mg.z}`, ''],
        ['Contact angle (α)', `${mg.alpha.toFixed(2)}`, 'deg'],
        ['Radial clearance (g_r)', `${mg.g_r.toFixed(1)}`, 'μm'],
        ['Roller spacing (Δψ)', `${(360 / mg.z).toFixed(3)}`, 'deg'],
      ]} />

      {/* Roller Geometry */}
      <DetailTable title="Roller Geometry" rows={[
        ['D_we,max (large end)', `${mg.d_we_max.toFixed(3)}`, 'mm'],
        ['D_we,min (small end)', `${mg.d_we_min.toFixed(3)}`, 'mm'],
        ['D_we,mean', `${geo.d_we_mean.toFixed(3)}`, 'mm'],
        ['Effective length (L_we)', `${mg.l_we.toFixed(3)}`, 'mm'],
        ['Taper angle (half)', `${geo.roller_taper_angle_deg.toFixed(4)}`, 'deg'],
        ['Taper angle (half)', `${(geo.roller_taper_angle_rad * 1000).toFixed(4)}`, 'mrad'],
        ['Full cone angle (2φ)', `${geo.cone_angle_deg.toFixed(4)}`, 'deg'],
        ['L_we / D_we', `${geo.contact_length_ratio.toFixed(3)}`, ''],
        ['D_we / d_pw (γ)', `${geo.gamma_dw.toFixed(6)}`, ''],
        ['Sphere radius (R_sph)', `${rp.r_sph.toFixed(2)}`, 'mm'],
        ['Surface roughness (Ra)', `${rp.sigma_roller.toFixed(3)}`, 'μm'],
      ]} />

      {/* Roller Crown Profile */}
      <DetailTable title="Roller Crown Profile" rows={[
        ['Crown type', formatCrownType(rp.crown_type), ''],
        ['Crown drop (δ_c)', `${rp.delta_c.toFixed(1)}`, 'μm'],
        ['DUB large-end (δ_dub,L)', `${rp.delta_dub_l.toFixed(1)}`, 'μm'],
        ['DUB small-end (δ_dub,S)', `${rp.delta_dub_s.toFixed(1)}`, 'μm'],
        ['DUB length large (l_dub,L)', `${rp.l_dub_l.toFixed(2)}`, 'mm'],
        ['DUB length small (l_dub,S)', `${rp.l_dub_s.toFixed(2)}`, 'mm'],
      ]} />

      {/* Raceway Geometry */}
      <DetailTable title="Raceway Geometry" rows={[
        ['Inner contact angle (α_i)', `${rg.alpha_i.toFixed(3)}`, 'deg'],
        ['Outer contact angle (α_o)', `${rg.alpha_o.toFixed(3)}`, 'deg'],
        ['Δα = α_o − α_i', `${(rg.alpha_o - rg.alpha_i).toFixed(3)}`, 'deg'],
        ['Inner raceway radius (r_i)', `${rg.r_i.toFixed(3)}`, 'mm'],
        ['Outer raceway radius (r_o)', `${rg.r_o.toFixed(3)}`, 'mm'],
        ['Undercut diameter (d_uc)', `${rg.d_uc.toFixed(2)}`, 'mm'],
        ['Undercut length (l_uc)', `${rg.l_uc.toFixed(2)}`, 'mm'],
      ]} />

      {/* Raceway Profiles */}
      <DetailTable title="Raceway Profiles" rows={[
        ['Inner raceway profile', formatRacewayProfile(input.raceway_profile_inner), ''],
        ['  └ Offset (δ_rw)', `${input.raceway_profile_inner.delta_rw.toFixed(1)}`, 'μm'],
        ['  └ Taper (w_a)', `${input.raceway_profile_inner.w_a.toFixed(3)}`, 'μm/mm'],
        ['  └ Ra roughness', `${input.raceway_profile_inner.ra.toFixed(3)}`, 'μm'],
        ['Outer raceway profile', formatRacewayProfile(input.raceway_profile_outer), ''],
        ['  └ Offset (δ_rw)', `${input.raceway_profile_outer.delta_rw.toFixed(1)}`, 'μm'],
        ['  └ Taper (w_a)', `${input.raceway_profile_outer.w_a.toFixed(3)}`, 'μm/mm'],
        ['  └ Ra roughness', `${input.raceway_profile_outer.ra.toFixed(3)}`, 'μm'],
      ]} />

      {/* Material */}
      <DetailTable title="Material Properties" rows={[
        ['Roller elastic modulus (E)', `${mat.e_roller.toFixed(0)}`, 'GPa'],
        ['Ring elastic modulus (E)', `${mat.e_ring.toFixed(0)}`, 'GPa'],
        ["Poisson's ratio (ν)", `${mat.nu.toFixed(2)}`, ''],
        ['Hardness (HRC)', `${mat.hrc.toFixed(0)}`, 'HRC'],
        ['E* (Johnson)', `${geo.e_star_gpa.toFixed(2)}`, 'GPa'],
        ["E' (Harris = 2E*)", `${(geo.e_star_gpa * 2).toFixed(2)}`, 'GPa'],
      ]} />

      {/* Component Weights */}
      <DetailTable title="Component Weights (approx.)" rows={[
        ['Single roller', `${geo.mass_roller_g.toFixed(2)}`, 'g'],
        ['All rollers (×' + mg.z + ')', `${geo.mass_rollers_total_g.toFixed(1)}`, 'g'],
        ['Inner race (cone)', `${geo.mass_inner_race_g.toFixed(1)}`, 'g'],
        ['Outer race (cup)', `${geo.mass_outer_race_g.toFixed(1)}`, 'g'],
        ['Total (no cage)', `${geo.mass_total_g.toFixed(1)}`, 'g'],
        ['Total (no cage)', `${(geo.mass_total_g / 1000).toFixed(3)}`, 'kg'],
      ]} />

      {/* Rib Geometry */}
      <RibGeometryDetail input={input} />

      {/* Applied Loads */}
      <DetailTable title="Applied Loads (computed)" rows={[
        ['Resultant radial (F_r)', `${geo.f_r_kn.toFixed(3)}`, 'kN'],
        ['Axial (F_a)', `${geo.f_a_kn.toFixed(3)}`, 'kN'],
        ['F_a / F_r', geo.f_r_kn > 0 ? `${(geo.f_a_kn / geo.f_r_kn).toFixed(4)}` : '-', ''],
        ['Misalignment (γ)', `${(geo.gamma_rad * 180 * 60 / Math.PI).toFixed(3)}`, 'arcmin'],
        ['Misalignment (γ)', `${(geo.gamma_rad * 1e6).toFixed(1)}`, 'μrad'],
      ]} />

      {/* Solver Settings */}
      <DetailTable title="Solver Settings" rows={[
        ['Number of slices', `${input.solver.n_slices}`, ''],
        ['Convergence tolerance', `${input.solver.convergence_tol.toExponential(1)}`, ''],
        ['Max iterations', `${input.solver.max_iterations}`, ''],
        ['Rib contact mode', `${input.solver.rib_contact_mode}`, ''],
        ['Beam type', `${input.solver.beam_type}`, ''],
      ]} />

      {/* Slice Geometry Table */}
      <SliceGeometryTable slices={geo.slice_geometries} />
    </div>
  );
}

// ─── Helpers ──────────────────────────────────────────────────────

function formatCrownType(ct: CrownType): string {
  if ('Logarithmic' in ct) return `Logarithmic (a_log=${ct.Logarithmic.a_log.toFixed(4)})`;
  if ('Circular' in ct) return `Circular (R=${ct.Circular.r_crown.toFixed(1)} mm)`;
  if ('Parabolic' in ct) return `Parabolic (c₂=${ct.Parabolic.c2.toExponential(3)})`;
  if ('Custom' in ct) return `Custom (${ct.Custom.profile.length} pts)`;
  if ('Polynomial' in ct) return `Polynomial (${ct.Polynomial.coeffs.length} coeffs)`;
  return 'Unknown';
}

function formatRacewayProfile(rp: RacewayProfile): string {
  if (rp.custom_profile) return `Custom (${rp.custom_profile.length} pts)`;
  if (rp.polynomial_coeffs) return `Polynomial (${rp.polynomial_coeffs.length} coeffs)`;
  if (rp.delta_rw !== 0 || rp.w_a !== 0) return 'Offset/Taper';
  return 'Nominal (flat)';
}

function RibGeometryDetail({ input }: { input: BearingInput }) {
  const mg = input.macro_geom;
  const rg = input.raceway_geom;
  const rp = input.roller_profile;

  const gammaRad = ((rg.alpha_i + rg.alpha_o) / 2) * Math.PI / 180;
  const gammaDeg = (rg.alpha_i + rg.alpha_o) / 2;
  const rBase = mg.d_pw / 2 + (mg.l_we / 2) * Math.sin(gammaRad) - (mg.d_we_max / 2) * Math.cos(gammaRad);

  const betaDeg = (rg.alpha_o - rg.alpha_i) / 2;
  const hC = mg.h_c != null ? Math.max(0, Math.min(mg.h_rib, mg.h_c)) : mg.h_rib / 2;

  const rContact = rBase + hC;
  const sinAlphaRib = Math.sin(mg.alpha_rib * Math.PI / 180);
  const rRibCircAuto = sinAlphaRib > 1e-6 ? rContact / sinAlphaRib : Infinity;
  const rRibCircUsed = rg.r_rib_circ ?? rRibCircAuto;
  const isAuto = rg.r_rib_circ === null;

  const rX = 1 / (1 / rp.r_sph - 1 / rg.r_rib);
  const rY = rRibCircUsed > rp.r_sph ? 1 / (1 / rp.r_sph - 1 / rRibCircUsed) : rp.r_sph;

  const rRollerBottom = rBase;
  const rRibTip = rRollerBottom + mg.h_rib;

  return (
    <div>
      <h4 className="text-xs font-semibold text-text-light mb-2 uppercase tracking-wider">
        Rib Contact Geometry
      </h4>
      <table className="text-xs">
        <tbody>
          {/* Input parameters */}
          <tr className="border-b border-white/[0.03]">
            <td className="py-0.5 pr-6 text-text-canvas" colSpan={3}>
              <span className="text-xs uppercase tracking-wider text-text-canvas/60">Input</span>
            </td>
          </tr>
          {([
            ['Rib height (h_rib)', `${mg.h_rib.toFixed(2)}`, 'mm'],
            ['Rib angle (α_rib)', `${mg.alpha_rib.toFixed(2)}`, 'deg'],
            ['Roller sphere (R_sph)', `${rp.r_sph.toFixed(2)}`, 'mm'],
            ['Rib fillet (R_rib)', `${rg.r_rib.toFixed(2)}`, 'mm'],
            ['R_rib_circ mode', isAuto ? 'Auto' : 'Manual', ''],
            ...(isAuto ? [] : [['R_rib_circ (input)', `${rg.r_rib_circ!.toFixed(2)}`, 'mm']]),
            ['Inner contact angle (α_i)', `${rg.alpha_i.toFixed(3)}`, 'deg'],
            ['Outer contact angle (α_o)', `${rg.alpha_o.toFixed(3)}`, 'deg'],
          ] as [string, string, string][]).map(([label, value, unit], i) => (
            <tr key={`in-${i}`} className="border-b border-white/[0.03]">
              <td className="py-0.5 pr-6 text-text-canvas whitespace-nowrap">{label}</td>
              <td className="py-0.5 text-right font-mono text-text-light tabular-nums">{value}</td>
              <td className="py-0.5 pl-1.5 text-text-canvas font-mono whitespace-nowrap">{unit}</td>
            </tr>
          ))}

          {/* Derived values */}
          <tr className="border-b border-white/[0.03]">
            <td className="py-1.5 pr-6 text-text-canvas" colSpan={3}>
              <span className="text-xs uppercase tracking-wider text-text-canvas/60">Derived</span>
            </td>
          </tr>
          {([
            ['Roller tilt angle (γ)', `${gammaDeg.toFixed(3)}`, 'deg'],
            ['Roller half cone angle (β)', `${betaDeg.toFixed(3)}`, 'deg'],
            ['Cone distance (R)', Math.abs(Math.sin(gammaRad)) > 1e-6 ? `${((mg.d_pw / 2) / Math.sin(gammaRad)).toFixed(2)}` : '-', 'mm'],
            ['Rib base radius (r_base)', `${rBase.toFixed(3)}`, 'mm'],
            ['  └ pitch (d_pw/2)', `${(mg.d_pw / 2).toFixed(3)}`, 'mm'],
            ['  └ axial offset (+l_we/2·sinγ)', `+${((mg.l_we / 2) * Math.sin(gammaRad)).toFixed(3)}`, 'mm'],
            ['  └ sphere proj (−d_we/2·cosγ)', `−${((mg.d_we_max / 2) * Math.cos(gammaRad)).toFixed(3)}`, 'mm'],
            ['Contact height (h_c)', `${hC.toFixed(3)}`, 'mm'],
            ['Contact point radius (r_c = r_base + h_c)', `${rContact.toFixed(3)}`, 'mm'],
            ['Rib tip radius (r_tip)', `${rRibTip.toFixed(3)}`, 'mm'],
            ['Rib radial span', `${(rRibTip - rRollerBottom).toFixed(3)}`, 'mm'],
            ['r_c relative position', `${(((rContact - rRollerBottom) / mg.h_rib) * 100).toFixed(1)}`, '% from base'],
            ['R_rib_circ (auto = r_c/sinα_rib)', `${rRibCircAuto.toFixed(2)}`, 'mm'],
            ['R_rib_circ (used)', `${isFinite(rRibCircUsed) ? rRibCircUsed.toFixed(2) : '∞'}`, 'mm'],
          ] as [string, string, string][]).map(([label, value, unit], i) => (
            <tr key={`d-${i}`} className="border-b border-white/[0.03]">
              <td className="py-0.5 pr-6 text-text-canvas whitespace-nowrap">{label}</td>
              <td className="py-0.5 text-right font-mono text-text-light tabular-nums">{value}</td>
              <td className="py-0.5 pl-1.5 text-text-canvas font-mono whitespace-nowrap">{unit}</td>
            </tr>
          ))}

          {/* Range check */}
          <tr className="border-b border-white/[0.03]">
            <td className="py-1.5 pr-6 text-text-canvas" colSpan={3}>
              <span className="text-xs uppercase tracking-wider text-text-canvas/60">Range Check</span>
            </td>
          </tr>
          {([
            ['Roller bottom (r_min)', `${rRollerBottom.toFixed(3)}`, 'mm'],
            ['Rib tip (r_max)', `${rRibTip.toFixed(3)}`, 'mm'],
            ['r_c position', `${rContact.toFixed(3)}`, 'mm'],
            ['Status', rContact >= rRollerBottom && rContact <= rRibTip ? 'OK' : 'OUT OF RANGE', ''],
          ] as [string, string, string][]).map(([label, value, unit], i) => (
            <tr key={`r-${i}`} className={`border-b border-white/[0.03] ${
              label === 'Status' && value !== 'OK' ? 'text-red-400' : ''
            }`}>
              <td className="py-0.5 pr-6 text-text-canvas whitespace-nowrap">{label}</td>
              <td className={`py-0.5 text-right font-mono tabular-nums ${
                label === 'Status' ? (value === 'OK' ? 'text-emerald-400' : 'text-red-400') : 'text-text-light'
              }`}>{value}</td>
              <td className="py-0.5 pl-1.5 text-text-canvas font-mono whitespace-nowrap">{unit}</td>
            </tr>
          ))}

          {/* Equivalent radii */}
          <tr className="border-b border-white/[0.03]">
            <td className="py-1.5 pr-6 text-text-canvas" colSpan={3}>
              <span className="text-xs uppercase tracking-wider text-text-canvas/60">Equivalent Radii (Hertz)</span>
            </td>
          </tr>
          {([
            ['R_x (meridional)', `${rX.toFixed(3)}`, 'mm'],
            ['R_y (circumferential)', `${rY.toFixed(3)}`, 'mm'],
            ['R_x / R_y', rY > 0 ? `${(rX / rY).toFixed(3)}` : '-', ''],
            ['Σρ = 1/R_x + 1/R_y', `${(1 / rX + 1 / rY).toFixed(6)}`, 'mm⁻¹'],
          ] as [string, string, string][]).map(([label, value, unit], i) => (
            <tr key={`eq-${i}`} className="border-b border-white/[0.03]">
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

function SliceGeometryTable({ slices }: { slices: SliceGeometry[] }) {
  if (!slices || slices.length === 0) return null;

  return (
    <div>
      <h4 className="text-xs font-semibold text-text-light mb-2 uppercase tracking-wider">
        Slice Geometries ({slices.length} slices)
      </h4>
      <div className="overflow-x-auto">
        <table className="w-full text-xs font-mono">
          <thead>
            <tr className="text-text-canvas border-b border-white/10">
              <th className="px-2 py-1 text-left">#</th>
              <th className="px-2 py-1 text-right">x [mm]</th>
              <th className="px-2 py-1 text-right">R_roller [mm]</th>
              <th className="px-2 py-1 text-right">R_eq_i [mm]</th>
              <th className="px-2 py-1 text-right">R_eq_o [mm]</th>
              <th className="px-2 py-1 text-right">Δz_i [μm]</th>
              <th className="px-2 py-1 text-right">Δz_o [μm]</th>
              <th className="px-2 py-1 text-right">width [mm]</th>
            </tr>
          </thead>
          <tbody>
            {slices.map(s => (
              <tr key={s.k} className="border-b border-white/5 hover:bg-white/5">
                <td className="px-2 py-0.5 text-text-canvas">{s.k}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{s.x_axial.toFixed(3)}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{s.r_roller.toFixed(4)}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{s.r_eq_inner.toFixed(4)}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{s.r_eq_outer.toFixed(4)}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{s.delta_z_total_inner.toFixed(3)}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{s.delta_z_total_outer.toFixed(3)}</td>
                <td className="px-2 py-0.5 text-right text-text-light">{s.slice_width.toFixed(4)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
