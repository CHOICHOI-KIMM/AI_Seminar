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
//  ⚠ **유효숫자 6자리 이상.** 이 뷰의 목적은 Level A 시험 결과와 화면을 **대조**하는
//    것이다. 자릿수를 줄이면 대조 자체가 불가능해진다 (`num()` 은 9자리 또는 지수표기).
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
//  단위 정책 (S2 확정 — 바꾸지 말 것)
// ─────────────────────────────────────────────────────────────────────
//  길이 mm · 하중 N · 응력 MPa · **접촉각만 °**(내부 rad).
//  단 α₀ 는 **° 와 rad 를 나란히** 낸다 — °는 사람이 읽기 위한 것이고,
//  rad 는 Level A 시험 출력과 같은 숫자라 대조에 그것이 필요하다.

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useAppState } from '../store';
import { DetailTable } from '../components/shared/DetailTable';
import type { Alert } from './generated/Alert';
import type { AlertLevel } from './generated/AlertLevel';
import type { BbGeometryDerived } from './generated/BbGeometryDerived';
import type { BbGeometrySummary } from './generated/BbGeometrySummary';

/** `commands::GeometryResponse` 대응 (Rust 쪽은 ts-rs 대상이 아니라 커맨드 전용 래퍼다). */
interface GeometryResponse {
  derived: BbGeometryDerived;
  summary: BbGeometrySummary;
  alerts: Alert[];
}

/**
 * 유효숫자 9자리 (또는 지수표기).
 *
 * §3.6.4.2 의 검증 매핑상 이 화면의 숫자는 **Level A 시험 결과와 직접 대조**된다.
 * 반올림해 보여주면 대조가 불가능하므로 자릿수를 줄이지 않는다.
 */
function num(v: number): string {
  if (!Number.isFinite(v)) return String(v);
  if (v === 0) return '0';
  const a = Math.abs(v);
  if (a < 1e-4 || a >= 1e9) return v.toExponential(8);
  return v.toPrecision(9);
}

const toDeg = (rad: number) => (rad * 180) / Math.PI;

// `AlertPanel` 과 **같은 표현**을 쓴다 (§3.6.5.2 S3).
// 그 컴포넌트는 `store.result.alerts` 만 읽으므로 기하 경고를 넘길 수 없어
// 표현만 옮겨 온다. 색·아이콘은 어두운 캔버스 배경에 맞춰 대응시켰다.
const ALERT_COLORS: Record<AlertLevel, string> = {
  Info: 'bg-blue-500/10 border-blue-400/30 text-blue-200',
  Warning: 'bg-amber-500/10 border-amber-400/30 text-amber-200',
  Critical: 'bg-red-500/10 border-red-400/30 text-red-200',
};
const ALERT_ICONS: Record<AlertLevel, string> = { Info: 'i', Warning: '!', Critical: '!!' };

export default function BbGeometryView() {
  const { state } = useAppState();
  const input = state.bbInput;

  const [response, setResponse] = useState<GeometryResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // `input` 이 아직 없으면 아무 것도 하지 않는다 — effect 본문에서 동기 setState 를
    // 하면 연쇄 렌더가 되고 `react-hooks/set-state-in-effect` 에도 걸린다.
    // 렌더 쪽이 이미 `!input` 을 조기반환으로 처리한다.
    if (!input) return;
    // 타자 한 글자마다 커맨드를 때리지 않도록 짧게 모은다.
    let cancelled = false;
    const timer = setTimeout(() => {
      invoke<GeometryResponse>('bb_compute_geometry', { input })
        .then(r => {
          if (cancelled) return;
          setResponse(r);
          setError(null);
        })
        .catch((e: unknown) => {
          if (cancelled) return;
          // Rust `validate()` / 기하 오류 메시지를 **그대로** 보여준다.
          // 프론트가 문구를 고쳐 쓰면 어느 쪽이 거부했는지 알 수 없게 된다 (§3.6.5.3).
          setError(String(e));
          setResponse(null);
        });
    }, 150);
    return () => {
      cancelled = true;
      clearTimeout(timer);
    };
  }, [input]);

  if (!input) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-text-canvas text-sm">프리셋을 불러오는 중…</p>
      </div>
    );
  }

  const g = input.geometry;

  return (
    <div className="h-full overflow-auto custom-scrollbar p-4 space-y-4">
      <div className="text-[11px] text-text-canvas/60">
        <code className="font-mono">bb_compute_geometry</code> — 하중과 무관한 기하 전처리
        (§3.6.4.7 ①). Solve 를 누르지 않아도 갱신된다. 유효숫자 9자리.
      </div>

      {error && (
        <div className="p-2 rounded border bg-red-500/10 border-red-400/30 text-red-200">
          <p className="text-[13px] font-medium">기하 계산 실패</p>
          <p className="text-xs opacity-80 whitespace-pre-wrap break-words">{error}</p>
        </div>
      )}

      {/* 입력 원본 — 화면의 파생값이 어느 입력에서 나온 것인지 대조하기 위한 것이다. */}
      <DetailTable
        title="Input Geometry (BallBearingGeometry)"
        rows={[
          ['Bore d', num(g.bore_mm), 'mm'],
          ['Outer diameter D', num(g.outer_diameter_mm), 'mm'],
          ['Width B', num(g.width_mm), 'mm'],
          ['Ball count Z', String(g.z), ''],
          ['Ball diameter D_w', num(g.d_w_mm), 'mm'],
          ['Pitch diameter D_pw', num(g.d_pw_mm), 'mm'],
          ['Inner groove radius r_i', num(g.r_i_mm), 'mm'],
          ['Outer groove radius r_e', num(g.r_e_mm), 'mm'],
          ['Nominal contact angle α', num(toDeg(g.alpha_nom_rad)), '°'],
          ['Nominal contact angle α', num(g.alpha_nom_rad), 'rad'],
          ['Clearance spec', Object.keys(g.clearance)[0], ''],
          ['Clearance value', num(Object.values(g.clearance)[0] as number), ''],
        ]}
      />

      {response && (
        <>
          {/* BbGeometryDerived — 전 9필드. 순서는 Rust struct 선언 순서와 동일. */}
          <DetailTable
            title="BbGeometryDerived (하중 무관 전처리 · 전 9필드)"
            rows={[
              ['A = r_i + r_e − D_w  (A.3)', num(response.derived.a_mm), 'mm'],
              ['α₀ initial contact angle  (A.1)', num(toDeg(response.derived.alpha_0_rad)), '°'],
              ['α₀ initial contact angle  (A.1)', num(response.derived.alpha_0_rad), 'rad'],
              ['R_i tilt moment arm  (A.4)', num(response.derived.r_i_center_mm), 'mm'],
              ['γ = D_w cos α / D_pw', num(response.derived.gamma), ''],
              ['Σρ_i inner curvature sum  (E.4)', num(response.derived.sum_rho_i_per_mm), '1/mm'],
              ['Σρ_e outer curvature sum  (E.5)', num(response.derived.sum_rho_e_per_mm), '1/mm'],
              ['F_i(ρ) inner curvature difference  (E.6)', num(response.derived.f_rho_i), ''],
              ['F_e(ρ) outer curvature difference  (E.7)', num(response.derived.f_rho_e), ''],
              ['G_r op equivalent radial clearance', num(response.derived.g_r_op_mm), 'mm'],
            ]}
          />

          {/* BbGeometrySummary — 전 13필드.
              앞 9개는 Derived 와 같은 값이어야 한다. 일부러 중복해 낸다 —
              두 구조체가 갈라지면 이 화면에서 바로 보인다. */}
          <DetailTable
            title="BbGeometrySummary (UI 요약 · 전 13필드)"
            rows={[
              ['A', num(response.summary.a_mm), 'mm'],
              ['α₀', num(toDeg(response.summary.alpha_0_rad)), '°'],
              ['α₀', num(response.summary.alpha_0_rad), 'rad'],
              ['R_i', num(response.summary.r_i_center_mm), 'mm'],
              ['γ', num(response.summary.gamma), ''],
              ['Σρ_i', num(response.summary.sum_rho_i_per_mm), '1/mm'],
              ['Σρ_e', num(response.summary.sum_rho_e_per_mm), '1/mm'],
              ['F_i(ρ)', num(response.summary.f_rho_i), ''],
              ['F_e(ρ)', num(response.summary.f_rho_e), ''],
              ['G_r op', num(response.summary.g_r_op_mm), 'mm'],
              ['f_i osculation = r_i / D_w', num(response.summary.osculation_inner), ''],
              ['f_e osculation = r_e / D_w', num(response.summary.osculation_outer), ''],
              ['Ball mass (single)', num(response.summary.ball_mass_g), 'g'],
              ['n·D_pw (ISO 16281 A.4, D-3)', num(response.summary.n_dpw_mm_per_min), 'mm/min'],
            ]}
          />

          {/* alerts — `AlertPanel` 과 같은 표현 (§3.6.5.2 S3) */}
          <div>
            <h4 className="text-sm font-semibold text-text-light mb-2 uppercase tracking-wider">
              Geometry Alerts
            </h4>
            {response.alerts.length === 0 ? (
              <p className="text-[13px] text-text-canvas/60">경고 없음</p>
            ) : (
              response.alerts.map((alert, i) => (
                <div
                  key={i}
                  className={`flex items-start gap-2 p-2 rounded border mb-1 ${ALERT_COLORS[alert.level]}`}
                >
                  <span className="text-[13px] font-bold mt-0.5">{ALERT_ICONS[alert.level]}</span>
                  <div className="min-w-0">
                    <p className="text-[13px] font-medium">{alert.code}</p>
                    <p className="text-xs opacity-80">{alert.message}</p>
                  </div>
                </div>
              ))
            )}
          </div>
        </>
      )}
    </div>
  );
}
