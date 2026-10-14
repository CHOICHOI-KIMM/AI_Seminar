// BB 기하 뷰 (Plan §3.6.5.2 S3 · §3.6.4.3 처분표 「Geometry — 개조」)
//
// ─────────────────────────────────────────────────────────────────────
//  이 화면의 검증 임무 — §3.6.4.2
// ─────────────────────────────────────────────────────────────────────
//  대응 Level: **A** (해석적 항등 16건).
//  「`A` · `α₀` · `Σρ_i/e` · `F(ρ)` · `R_i` · `γ` · `n·D_pw` 를 육안으로 재확인」하고,
//  깨졌을 때의 징후는 「`Σρ` 가 음수 · `α₀` 가 공칭각과 다름 · 오스큘레이션이 0,5 미만」이다.
//
//  ⚠ 그래서 **표시 항목을 임의로 고르지 않는다.** Rust `BbGeometryDerived`(9필드)·
//    `BbGeometrySummary`(13필드)의 **전 필드**를 빠짐없이 낸다. 무엇이 검증에 필요한지는
//    §3.6.4.2 가 정하지 이 파일이 정하지 않는다.
//
//  자릿수는 Plan §4-5.3 의 단위별 고정 소수다 (`shared.ts` 포맷 함수). 정밀값 대조는
//  Rust 테스트와 헬스체크 몫이다 (§4-5.1 결정 4).
//
// ─────────────────────────────────────────────────────────────────────
//  데이터원 — §3.6.4.7 ①
// ─────────────────────────────────────────────────────────────────────
//  `bb_compute_geometry` 의 **첫 연결 지점**이다. 이 커맨드는 **하중과 무관**하므로
//  평형을 풀지 않고도(= Solve 를 누르지 않고도) 기하를 볼 수 있다.
//
//  ⚠ 재호출 트리거는 `geometry` 뿐이 아니라 **`bbInput` 전체**다.
//    `BbGeometrySummary` 의 `n_dpw_mm_per_min` 은 `operating.n_inner_rpm` 에서,
//    `ball_mass_g` 는 `material.density_ball_g_cm3` 에서 나온다
//    (`geometry::compute_geometry_summary` 시그니처가 셋을 다 받는다).
//    `geometry` 만 구독하면 rpm·밀도를 바꿨을 때 화면이 **낡은 값을 계속 보여준다.**
//
// ─────────────────────────────────────────────────────────────────────
//  단위 · 자릿수 (Plan §4-5.3)
// ─────────────────────────────────────────────────────────────────────
//  기하 치수 mm 소수 3 · 곡률 1/mm·무차원 소수 4 · 각도 ° 소수 2 (rad 병기 없음, 결정 5) ·
//  질량 g 소수 1 · n·D_pw 정수. 솔버 단위(mm · N · rad, D-10)는 화면 경계에서만 환산한다.
//  클리어런스 값은 지정 방식에 따라 단위가 다르다 — 직경 클리어런스 mm · 초기 접촉각 ° ·
//  축방향 예압 N.

import { useAppState } from '../store';
import { fmtDeg, fmtDimless, fmtInt, fmtMass, fmtMm, fmtN, toDeg, useGeometryResponse } from './shared';
import { AlertList, DetailTable, ErrorBox, Loading } from './ui';

export default function BbGeometryView() {
  const { state } = useAppState();
  const input = state.bbInput;

  const { response, error } = useGeometryResponse(input);

  if (!input) return <Loading />;

  const g = input.geometry;
  const c = g.clearance;
  const [clearanceValue, clearanceUnit] =
    'DiametralMm' in c
      ? [fmtMm(c.DiametralMm), 'mm']
      : 'InitialAngleRad' in c
        ? [fmtDeg(toDeg(c.InitialAngleRad)), '°']
        : [fmtN(c.AxialPreloadN), 'N'];

  return (
    <div className="h-full overflow-auto custom-scrollbar p-4 space-y-4">
      {error && <ErrorBox title="기하 계산 실패" message={error} />}

      {/* 입력 원본 — 화면의 파생값이 어느 입력에서 나온 것인지 대조하기 위한 것이다. */}
      <DetailTable
        title="Input Geometry (BallBearingGeometry)"
        rows={[
          ['Bore d', fmtMm(g.bore_mm), 'mm'],
          ['Outer diameter D', fmtMm(g.outer_diameter_mm), 'mm'],
          ['Width B', fmtMm(g.width_mm), 'mm'],
          ['Ball count Z', String(g.z), ''],
          ['Ball diameter D_w', fmtMm(g.d_w_mm), 'mm'],
          ['Pitch diameter D_pw', fmtMm(g.d_pw_mm), 'mm'],
          ['Inner groove radius r_i', fmtMm(g.r_i_mm), 'mm'],
          ['Outer groove radius r_e', fmtMm(g.r_e_mm), 'mm'],
          ['Nominal contact angle α', fmtDeg(toDeg(g.alpha_nom_rad)), '°'],
          ['Clearance spec', Object.keys(g.clearance)[0], ''],
          ['Clearance value', clearanceValue, clearanceUnit],
        ]}
      />

      {response && (
        <>
          {/* BbGeometryDerived — 전 9필드. 순서는 Rust struct 선언 순서와 동일. */}
          <DetailTable
            title="BbGeometryDerived"
            rows={[
              ['A = r_i + r_e − D_w  (A.3)', fmtMm(response.derived.a_mm), 'mm'],
              ['α₀ initial contact angle  (A.1)', fmtDeg(toDeg(response.derived.alpha_0_rad)), '°'],
              ['R_i tilt moment arm  (A.4)', fmtMm(response.derived.r_i_center_mm), 'mm'],
              ['γ = D_w cos α / D_pw', fmtDimless(response.derived.gamma), ''],
              ['Σρ_i inner curvature sum  (E.4)', fmtDimless(response.derived.sum_rho_i_per_mm), '1/mm'],
              ['Σρ_e outer curvature sum  (E.5)', fmtDimless(response.derived.sum_rho_e_per_mm), '1/mm'],
              ['F_i(ρ) inner curvature difference  (E.6)', fmtDimless(response.derived.f_rho_i), ''],
              ['F_e(ρ) outer curvature difference  (E.7)', fmtDimless(response.derived.f_rho_e), ''],
              ['G_r op equivalent radial clearance', fmtMm(response.derived.g_r_op_mm), 'mm'],
            ]}
          />

          {/* BbGeometrySummary — 전 13필드.
              앞 9개는 Derived 와 같은 값이어야 한다. 일부러 중복해 낸다 —
              두 구조체가 갈라지면 이 화면에서 바로 보인다. */}
          <DetailTable
            title="BbGeometrySummary"
            rows={[
              ['A', fmtMm(response.summary.a_mm), 'mm'],
              ['α₀', fmtDeg(toDeg(response.summary.alpha_0_rad)), '°'],
              ['R_i', fmtMm(response.summary.r_i_center_mm), 'mm'],
              ['γ', fmtDimless(response.summary.gamma), ''],
              ['Σρ_i', fmtDimless(response.summary.sum_rho_i_per_mm), '1/mm'],
              ['Σρ_e', fmtDimless(response.summary.sum_rho_e_per_mm), '1/mm'],
              ['F_i(ρ)', fmtDimless(response.summary.f_rho_i), ''],
              ['F_e(ρ)', fmtDimless(response.summary.f_rho_e), ''],
              ['G_r op', fmtMm(response.summary.g_r_op_mm), 'mm'],
              ['f_i osculation = r_i / D_w', fmtDimless(response.summary.osculation_inner), ''],
              ['f_e osculation = r_e / D_w', fmtDimless(response.summary.osculation_outer), ''],
              ['Ball mass (single)', fmtMass(response.summary.ball_mass_g), 'g'],
              ['n·D_pw (ISO 16281 A.4, D-3)', fmtInt(response.summary.n_dpw_mm_per_min), 'mm/min'],
            ]}
          />

          <AlertList title="Geometry Alerts" alerts={response.alerts} />
        </>
      )}
    </div>
  );
}
