// BB 3D 뷰 (Plan §4 Phase 4-3 「P4-3b」 · 「P4-3b 표현 확정 — 심플 노선」)
//
// ─────────────────────────────────────────────────────────────────────
//  이 화면의 검증 임무 — §3.6.4.2
// ─────────────────────────────────────────────────────────────────────
//  3D 는 미세 수치를 못 읽는다. 대신 **공간 직관**은 3D 만 준다.
//
//  | 구성 | 확인하는 것 | 대응 Level |
//  |---|---|---|
//  | 볼 세트 + `Q_j` 컬러맵 | 볼 배치와 **하중구간의 공간 위치** | C-7 · D-2b/2c |
//  | 접촉선 세그먼트(`α_j` 방향) | 접촉각이 **벌어지는 방향** | D-1 · C-2 |
//  | 틸트축 ↔ 모멘트 벡터 | **틸트축이 모멘트와 맞는가** | **D-2d** ← 3D 의 핵심 존재 이유 |
//
//  깨졌을 때의 징후: 「하중구간이 하중 화살표와 어긋남」·「틸트축이 모멘트와 반대」.
//
// ─────────────────────────────────────────────────────────────────────
//  🔴 좌표계 — D-7 / D-8 을 씬에 **그대로** 옮긴다
// ─────────────────────────────────────────────────────────────────────
//  · **X = 회전축**(D-7, ISO 규약). 반경평면은 **YZ**.
//  · **`φ = 0` 이 +Y**(D-8, `BallResult.phi_rad` 주석 · `solver/bb/bearing.rs` 의
//    `R_j = A cos α₀ + δ_y cos φ_j + δ_z sin φ_j` → 반경 단위벡터가 `(cos φ, sin φ)` in (Y, Z)).
//    → 씬에서 볼 중심은 **`(0, r cos φ_j, r sin φ_j)`**.
//    → 반경 단위벡터 `e_r(φ) = (0, cos φ, sin φ)`. 우수좌표계 `Z = X × Y` 가 그대로 만족된다.
//  · `LatheGeometry` 는 three 규약상 **Y축 회전체**이고 프로파일 점은 `Vector2(반경, 축위치)` 다.
//    회전축을 씬 **X**(= 회전축)로 눕히려면 **`rotation = [0, 0, +π/2]`**.
//  ⚠ 기존 `components/BearingView3D`(TRB 잔존)는 **+Z 를 회전축**으로 쓴다.
//    볼 중심 `(r cos ψ, r sin ψ, 0)` · `rotation=[-π/2,0,0]` — **BB 와 정면으로 어긋난다.**
//    그 파일에서 가져온 것은 `colorScale`(HSL 레인보우) · `AxisLabels` · `Html` 툴팁뿐이고,
//    `ForceArrow` 는 **Z축 회전 1개로 XY 평면 가정이 박혀 있어** 방향벡터로 일반화했다
//    (BB 는 5-DOF 라 축하중 `F_x` 화살표도 같은 함수로 그려야 한다).
//
//  접촉선의 방향 — 축단면(P4-3a)과 **같은 식**이다:
//    축단면: `n = (sin α, cos α)` in (축 X, 반경 r), `P_i = C − (D_w/2)·n`, `P_e = C + (D_w/2)·n`
//    3D 로 올리면 `n₃(φ, α) = (sin α, cos α cos φ, cos α sin φ)`.
//    → 반경 **안쪽**이 내륜 접촉, **바깥쪽**이 외륜 접촉. 부호가 뒤집히면 내·외륜이 바뀐다.
//
// ─────────────────────────────────────────────────────────────────────
//  표현 — 「심플 노선」 (Plan 「P4-3b 표현 확정」, 사용자 참조 그림 지시 2026-08-24)
// ─────────────────────────────────────────────────────────────────────
//  어두운 배경 · **죽인 반투명 링 실루엣** · **하중 컬러맵 전동체** · 중심의 작은 축 마커.
//  링은 `bore–OD–B` 사각 프로파일의 **단순 실루엣**이다 — 홈 반경 `r_i`·`r_e` 의 정밀
//  모델링은 **축단면(P4-3a)의 몫**이고 3D 에서는 시야만 가린다.
//  **불채택**: 정밀 홈 프로파일 · 1/4 절개(clipping plane).
//
// ─────────────────────────────────────────────────────────────────────
//  단위 정책 (S2 확정 — 바꾸지 말 것)
// ─────────────────────────────────────────────────────────────────────
//  길이 **mm** · 하중 **N** · 모멘트 **N·mm** · **접촉각만 °**(내부 rad) · **틸트 γ 는 rad**.
//  ⚠ 기존 3D 의 `q/1000` **kN** 표기는 **D-10 위반**이다 — 따라하지 않는다.
//
// ─────────────────────────────────────────────────────────────────────
//  ⚠ 지어내지 않는 것
// ─────────────────────────────────────────────────────────────────────
//  · 화살표의 **길이는 물리량이 아니다** — 씬 크기에 고정하고 크기는 **숫자로만** 낸다.
//    (하중·모멘트·틸트는 단위가 서로 달라 한 화면에서 길이로 비교할 수 없다.)
//  · **결과가 없으면 기하만 그린다** — `φ_j = 2πj/Z`(D-8) 에 볼을 놓고 접촉각은 `α₀`,
//    하중은 전부 0(무하중 색)이다. 이것은 솔버 출력이 아니라 **기하의 정의**이며
//    화면에 「무하중 기하」라고 명시한다. 결과를 기다리면 Level A 기하를 눈으로 보는
//    경로가 막힌다 (축단면이 무하중 단면을 그리는 것과 같은 이유).

import { useEffect, useMemo, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Canvas } from '@react-three/fiber';
import { OrbitControls, Text, Html } from '@react-three/drei';
import * as THREE from 'three';
import { useAppState } from '../store';
import type { Alert } from './generated/Alert';
import type { BallBearingGeometry } from './generated/BallBearingGeometry';
import type { BallResult } from './generated/BallResult';
import type { BbGeometryDerived } from './generated/BbGeometryDerived';
import type { BbOperatingConditions } from './generated/BbOperatingConditions';
import type { Displacement } from './generated/Displacement';

/** `commands::GeometryResponse` 대응 — `BbGeometryView`·`BbAxialSectionView` 와 같은 형태다. */
interface GeometryResponse {
  derived: BbGeometryDerived;
  alerts: Alert[];
}

/** 유효숫자 9자리 (또는 지수표기) — 검증 대조가 목적이라 자릿수를 줄이지 않는다. */
function num(v: number): string {
  if (!Number.isFinite(v)) return String(v);
  if (v === 0) return '0';
  const a = Math.abs(v);
  if (a < 1e-4 || a >= 1e9) return v.toExponential(8);
  return v.toPrecision(9);
}

const toDeg = (rad: number) => (rad * 180) / Math.PI;

/** (−180, 180] 로 접는다 — 방위차 Δ 표시용 (축단면과 같은 함수). */
function wrapDeg(d: number): number {
  let x = ((((d + 180) % 360) + 360) % 360) - 180;
  if (x === -180) x = 180;
  return x;
}

// ─── 색 팔레트 — `BbLoadDistView`·`BbAxialSectionView` 와 같은 값 ───────
const C_FORCE = '#ef4444'; // 외부 하중
const C_MOMENT = '#a3e635'; // 외부 모멘트
const C_TILT = '#38bdf8'; // 틸트축 (γ_y, γ_z)
const C_RING_INNER = '#94a3b8';
const C_RING_OUTER = '#64748b';

/** three 의 기본 원기둥·원뿔 축. `setFromUnitVectors` 의 시작 벡터다. */
const CYL_AXIS = new THREE.Vector3(0, 1, 0);

/**
 * 레인보우 컬러맵 — 무하중 = 청색(hue 0.667), `Q_max` = 적색(hue 0).
 * 기존 `components/BearingView3D` 의 `colorScale` 을 그대로 가져왔다.
 */
function loadColor(q: number, qMax: number): THREE.Color {
  const t = qMax > 0 ? Math.min(Math.max(q / qMax, 0), 1) : 0;
  return new THREE.Color().setHSL((1 - t) * 0.667, 1, 0.5);
}

/**
 * 방향 화살표 — **방향벡터 일반화** (Plan 확정: `setFromUnitVectors(Vector3(0,1,0), dir)`).
 *
 * 원본(`components/BearingView3D` 의 `ForceArrow`)은 `rotZ = -atan2(dx, dy)` 회전 하나로
 * **XY 평면 전용**이었다. BB 는 5-DOF 라 축방향(`F_x`)·모멘트·틸트축이 모두 임의 방향이므로
 * 회전을 쿼터니언으로 일반화한다. 그룹을 회전시키므로 자식은 **로컬 +Y 가 곧 `dir`** 인
 * 좌표에서 배치된다.
 */
function DirArrow({
  dir,
  origin,
  length,
  color,
}: {
  dir: THREE.Vector3;
  origin: THREE.Vector3;
  length: number;
  color: string;
}) {
  const quat = useMemo(() => {
    const d = dir.clone();
    if (d.lengthSq() === 0) return new THREE.Quaternion();
    return new THREE.Quaternion().setFromUnitVectors(CYL_AXIS, d.normalize());
  }, [dir]);

  const shaftLen = length * 0.78;
  const headLen = length * 0.22;
  const shaftR = length * 0.022;
  const headR = length * 0.06;

  return (
    <group position={origin} quaternion={quat}>
      <mesh position={[0, shaftLen / 2, 0]}>
        <cylinderGeometry args={[shaftR, shaftR, shaftLen, 10]} />
        <meshStandardMaterial color={color} />
      </mesh>
      <mesh position={[0, shaftLen + headLen / 2, 0]}>
        <coneGeometry args={[headR, headLen, 14]} />
        <meshStandardMaterial color={color} />
      </mesh>
    </group>
  );
}

/**
 * 중심의 작은 축 마커 — `axesHelper` + `X/Y/Z` 라벨 (원본 `AxisLabels` 그대로).
 * ⚠ **X 가 회전축**(D-7)이고 **+Y 가 φ=0**(D-8)이라는 것을 눈으로 확인하는 장치다.
 */
function AxisMarker({ size }: { size: number }) {
  const off = size * 1.18;
  const fontSize = size * 0.22;
  const p = { fontSize, anchorX: 'center' as const, anchorY: 'middle' as const };
  return (
    <group>
      <axesHelper args={[size]} />
      <Text position={[off, 0, 0]} color="#f87171" {...p}>
        X (회전축)
      </Text>
      <Text position={[0, off, 0]} color="#4ade80" {...p}>
        Y (φ=0)
      </Text>
      <Text position={[0, 0, off]} color="#60a5fa" {...p}>
        Z
      </Text>
    </group>
  );
}

/**
 * 내·외륜 실루엣. `bore–OD–B` **사각 프로파일**의 `LatheGeometry` 다.
 *
 * 🔴 `rotation = [0, 0, +π/2]` — three 의 Y축 회전체를 씬 **X**(= 회전축)로 눕힌다.
 *   프로파일 점은 `Vector2(반경, 축위치)`.
 * 반경 경계는 **볼 궤도를 비운다**: 내륜 바깥 = `D_pw/2 − D_w/2`,
 * 외륜 안쪽 = `D_pw/2 + D_w/2`. (홈 형상은 그리지 않는다 — 위 「표현」 참조.)
 */
function Rings({ g }: { g: BallBearingGeometry }) {
  const { inner, outer } = useMemo(() => {
    const halfB = g.width_mm / 2;
    const rBore = g.bore_mm / 2;
    const rOD = g.outer_diameter_mm / 2;
    const rPitch = g.d_pw_mm / 2;
    const rBall = g.d_w_mm / 2;
    // 입력이 비정상이어도(볼이 링을 뚫는 등) 렌더가 죽지 않도록 순서만 보장한다.
    const rInnerTop = Math.max(rBore * 1.001, rPitch - rBall);
    const rOuterBot = Math.min(rOD * 0.999, rPitch + rBall);
    const rect = (r0: number, r1: number) => [
      new THREE.Vector2(r0, -halfB),
      new THREE.Vector2(r1, -halfB),
      new THREE.Vector2(r1, halfB),
      new THREE.Vector2(r0, halfB),
      new THREE.Vector2(r0, -halfB),
    ];
    return {
      inner: new THREE.LatheGeometry(rect(rBore, rInnerTop), 96, 0, Math.PI * 2),
      outer: new THREE.LatheGeometry(rect(rOuterBot, rOD), 96, 0, Math.PI * 2),
    };
  }, [g.width_mm, g.bore_mm, g.outer_diameter_mm, g.d_pw_mm, g.d_w_mm]);

  useEffect(() => {
    return () => {
      inner.dispose();
      outer.dispose();
    };
  }, [inner, outer]);

  return (
    <group rotation={[0, 0, Math.PI / 2]}>
      <mesh geometry={inner}>
        <meshStandardMaterial
          color={C_RING_INNER}
          transparent
          opacity={0.2}
          side={THREE.DoubleSide}
          depthWrite={false}
        />
      </mesh>
      <mesh geometry={outer}>
        <meshStandardMaterial
          color={C_RING_OUTER}
          transparent
          opacity={0.2}
          side={THREE.DoubleSide}
          depthWrite={false}
        />
      </mesh>
    </group>
  );
}

/**
 * 볼 1개 + 접촉선 세그먼트.
 *
 * 중심 `C = (0, r cos φ, r sin φ)` (D-8).
 * 접촉 법선 `n₃ = (sin α, cos α cos φ, cos α sin φ)` — 축단면(P4-3a)과 같은 식.
 * 세그먼트는 `C ± 0.72 D_w · n₃` 로 그린다 — **내·외륜 접촉점(±D_w/2)을 지나 양끝이
 * 볼 밖으로 조금 나온다.** 구는 자세가 없어 이렇게 하지 않으면 `α_j` 가 볼 안에 묻힌다.
 * 굵기는 `Q_j / Q_max` 로 준다 (하중구간과 접촉각을 동시에 보이기 위함).
 */
function Ball({
  br,
  rPitch,
  dW,
  qMax,
  index,
  hovered,
  onHover,
}: {
  br: BallResult;
  rPitch: number;
  dW: number;
  qMax: number;
  index: number;
  hovered: boolean;
  onHover: (i: number | null) => void;
}) {
  const cosP = Math.cos(br.phi_rad);
  const sinP = Math.sin(br.phi_rad);
  const center = useMemo(
    () => new THREE.Vector3(0, rPitch * cosP, rPitch * sinP),
    [rPitch, cosP, sinP],
  );
  const quat = useMemo(() => {
    const n = new THREE.Vector3(
      Math.sin(br.alpha_rad),
      Math.cos(br.alpha_rad) * cosP,
      Math.cos(br.alpha_rad) * sinP,
    );
    return new THREE.Quaternion().setFromUnitVectors(CYL_AXIS, n.normalize());
  }, [br.alpha_rad, cosP, sinP]);

  const t = qMax > 0 ? Math.min(Math.max(br.q_n / qMax, 0), 1) : 0;
  const color = loadColor(br.q_n, qMax);
  const segLen = dW * 1.44;
  const segR = dW * (0.028 + 0.075 * t);

  return (
    <group>
      <mesh
        position={center}
        onPointerOver={e => {
          e.stopPropagation();
          onHover(index);
        }}
        onPointerOut={() => onHover(null)}
      >
        <sphereGeometry args={[dW / 2, 28, 20]} />
        <meshStandardMaterial color={color} roughness={0.35} metalness={0.15} />
      </mesh>

      {/* 접촉선 세그먼트 — 로컬 +Y 가 n₃ 이므로 원기둥은 볼 중심에 그대로 둔다 */}
      <mesh position={center} quaternion={quat}>
        <cylinderGeometry args={[segR, segR, segLen, 10]} />
        <meshStandardMaterial
          color={color}
          emissive={color}
          emissiveIntensity={0.45}
          roughness={0.3}
        />
      </mesh>

      {hovered && (
        <Html position={center} center style={{ pointerEvents: 'none' }}>
          <div
            style={{
              background: 'rgba(0,0,0,0.82)',
              color: '#fff',
              padding: '4px 7px',
              borderRadius: 4,
              fontSize: 10,
              lineHeight: 1.35,
              whiteSpace: 'nowrap',
              fontFamily: 'ui-monospace, monospace',
            }}
          >
            <div style={{ fontWeight: 700 }}>볼 #{index + 1}</div>
            <div>φ = {num(toDeg(br.phi_rad))} °</div>
            <div>Q = {num(br.q_n)} N</div>
            <div>α = {num(toDeg(br.alpha_rad))} °</div>
            <div>δ = {num(br.delta_mm)} mm</div>
            <div style={{ color: br.loaded ? '#f59e0b' : '#94a3b8' }}>
              {br.loaded ? '접촉' : '비접촉'}
            </div>
          </div>
        </Html>
      )}
    </group>
  );
}

/**
 * 외부 하중 · 모멘트 · 틸트축.
 *
 * 🔴 **D-2d 는 여기서 본다** — 틸트축 `(0, γ_y, γ_z)` 와 모멘트 벡터 `(0, M_y, M_z)` 를
 *   같은 길이로 겹쳐 그린다. 정합하면 두 화살표가 포개진다.
 *   기대 부호의 근거(Theory §4.4 확정형): `M_y > 0 ⟹ γ_y > 0`, `M_z > 0 ⟹ γ_z > 0`.
 *   ⚠ 볼이 `Z` 개로 **이산**이라 순수 모멘트라도 작은 편차는 정상이다. 화면은 두 벡터를
 *     보일 뿐 정합 여부를 **단정하지 않는다** (4단 수치 판정은 축단면 P4-3a 의 나침반이 맡는다).
 *
 * ⚠ 화살표 **길이는 물리량이 아니다**. 씬 크기에 고정한다 (단위가 서로 달라 비교 불가).
 * ⚠ 하중은 **`resultInput.operating`**(Solve 시점 스냅샷)에서 읽는다 — `bbInput` 은
 *   편집 중이라 시점이 어긋난다 (축단면과 같은 근거).
 */
function LoadVectors({
  op,
  disp,
  scale,
}: {
  op: BbOperatingConditions | null;
  disp: Displacement | null;
  scale: number;
}) {
  const origin = useMemo(() => new THREE.Vector3(0, 0, 0), []);

  const items = useMemo(() => {
    const out: { dir: THREE.Vector3; color: string; label: string }[] = [];
    if (op) {
      if (op.f_y_n !== 0 || op.f_z_n !== 0) {
        out.push({
          dir: new THREE.Vector3(0, op.f_y_n, op.f_z_n),
          color: C_FORCE,
          label: `F_r = ${num(Math.hypot(op.f_y_n, op.f_z_n))} N`,
        });
      }
      if (op.f_x_n !== 0) {
        out.push({
          dir: new THREE.Vector3(Math.sign(op.f_x_n), 0, 0),
          color: C_FORCE,
          label: `F_x = ${num(op.f_x_n)} N`,
        });
      }
      if (op.m_y_nmm !== 0 || op.m_z_nmm !== 0) {
        out.push({
          dir: new THREE.Vector3(0, op.m_y_nmm, op.m_z_nmm),
          color: C_MOMENT,
          label: `M = (${num(op.m_y_nmm)}, ${num(op.m_z_nmm)}) N·mm`,
        });
      }
    }
    if (disp && (disp.ry_rad !== 0 || disp.rz_rad !== 0)) {
      out.push({
        dir: new THREE.Vector3(0, disp.ry_rad, disp.rz_rad),
        color: C_TILT,
        label: `틸트축 (γ_y, γ_z) = (${num(disp.ry_rad)}, ${num(disp.rz_rad)}) rad`,
      });
    }
    return out;
  }, [op, disp]);

  return (
    <group>
      {items.map((it, i) => {
        const tip = it.dir.clone().normalize().multiplyScalar(scale * 1.08);
        return (
          <group key={i}>
            <DirArrow dir={it.dir} origin={origin} length={scale} color={it.color} />
            <Html position={tip} center style={{ pointerEvents: 'none' }}>
              <div
                style={{
                  background: 'rgba(0,0,0,0.7)',
                  color: it.color,
                  padding: '2px 5px',
                  borderRadius: 4,
                  fontSize: 10,
                  whiteSpace: 'nowrap',
                  fontFamily: 'ui-monospace, monospace',
                }}
              >
                {it.label}
              </div>
            </Html>
          </group>
        );
      })}
    </group>
  );
}

function Scene({
  g,
  balls,
  qMax,
  op,
  disp,
  showLoads,
  showRings,
}: {
  g: BallBearingGeometry;
  balls: BallResult[];
  qMax: number;
  op: BbOperatingConditions | null;
  disp: Displacement | null;
  showLoads: boolean;
  showRings: boolean;
}) {
  const [hover, setHover] = useState<number | null>(null);

  return (
    <group>
      {showRings && <Rings g={g} />}
      {balls.map((br, i) => (
        <Ball
          key={i}
          br={br}
          index={i}
          rPitch={g.d_pw_mm / 2}
          dW={g.d_w_mm}
          qMax={qMax}
          hovered={hover === i}
          onHover={setHover}
        />
      ))}
      {showLoads && (
        <LoadVectors op={op} disp={disp} scale={(g.outer_diameter_mm / 2) * 0.62} />
      )}
      <AxisMarker size={g.d_pw_mm * 0.28} />
    </group>
  );
}

export default function BbBearingView3D() {
  const { state } = useAppState();
  const { bbInput, result, resultInput } = state;

  const [derived, setDerived] = useState<BbGeometryDerived | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [showLoads, setShowLoads] = useState(true);
  const [showRings, setShowRings] = useState(true);

  // `BbAxialSectionView`·`BbGeometryView` 와 **같은 패턴** — 150 ms 디바운스 + `cancelled`.
  // ⚠ 구독 대상은 `bbInput` **전체**다. `geometry` 만 구독하면 낡은 값이 남는다.
  useEffect(() => {
    if (!bbInput) return;
    let cancelled = false;
    const timer = setTimeout(() => {
      invoke<GeometryResponse>('bb_compute_geometry', { input: bbInput })
        .then(r => {
          if (cancelled) return;
          setDerived(r.derived);
          setError(null);
        })
        .catch((e: unknown) => {
          if (cancelled) return;
          setError(String(e)); // Rust 메시지를 **그대로** 낸다 (§3.6.5.3)
          setDerived(null);
        });
    }, 150);
    return () => {
      cancelled = true;
      clearTimeout(timer);
    };
  }, [bbInput]);

  const g = bbInput?.geometry ?? null;
  const alpha0 = result?.geometry.alpha_0_rad ?? derived?.alpha_0_rad ?? null;

  // 결과가 없으면 **기하만** 그린다 — φ_j = 2πj/Z (D-8), α = α₀, Q = 0.
  // 솔버 출력이 아니라 기하의 정의다 (파일 헤더 「지어내지 않는 것」 참조).
  const balls: BallResult[] | null = useMemo(() => {
    if (result) return result.equilibrium.ball_results;
    if (!g || alpha0 === null) return null;
    const z = Math.max(0, Math.round(g.z));
    return Array.from({ length: z }, (_, j) => ({
      phi_rad: (2 * Math.PI * j) / z,
      delta_mm: 0,
      alpha_rad: alpha0,
      q_n: 0,
      loaded: false,
      a_inner_mm: 0,
      b_inner_mm: 0,
      p_max_inner_mpa: 0,
      a_outer_mm: 0,
      b_outer_mm: 0,
      p_max_outer_mpa: 0,
    }));
  }, [result, g, alpha0]);

  if (!bbInput || !g) {
    return (
      <div className="h-full flex items-center justify-center">
        <p className="text-text-canvas text-sm">프리셋을 불러오는 중…</p>
      </div>
    );
  }

  if (!balls) {
    return (
      <div className="h-full flex items-center justify-center">
        <p className="text-text-canvas text-sm">
          {error ? `기하 계산 실패: ${error}` : '기하 계산 중…'}
        </p>
      </div>
    );
  }

  const qMax = result?.equilibrium.q_max_n ?? 0;
  const op = resultInput?.operating ?? null;
  const disp = result?.equilibrium.displacement ?? null;
  // 입력이 Solve 이후 변경되었는가 — 축단면(P4-3a)과 같은 판정.
  const stale = result !== null && resultInput !== null && resultInput !== bbInput;

  const rOD = g.outer_diameter_mm / 2;
  const camPos: [number, number, number] = [rOD * 1.9, rOD * 1.25, rOD * 1.7];

  const tiltDeg =
    disp && (disp.ry_rad !== 0 || disp.rz_rad !== 0)
      ? wrapDeg(toDeg(Math.atan2(disp.rz_rad, disp.ry_rad)))
      : null;
  const momDeg =
    op && (op.m_y_nmm !== 0 || op.m_z_nmm !== 0)
      ? wrapDeg(toDeg(Math.atan2(op.m_z_nmm, op.m_y_nmm)))
      : null;

  return (
    <div className="w-full h-full relative">
      {/* ── 토글 ─────────────────────────────────────────── */}
      <div className="absolute top-2 right-2 z-10 flex gap-2">
        <button
          onClick={() => setShowLoads(v => !v)}
          className="px-2.5 py-1 rounded-md text-xs text-white border-0 cursor-pointer backdrop-blur-sm"
          style={{ background: showLoads ? 'rgba(59,130,246,0.8)' : 'rgba(30,30,30,0.7)' }}
        >
          하중·모멘트·틸트 {showLoads ? 'ON' : 'OFF'}
        </button>
        <button
          onClick={() => setShowRings(v => !v)}
          className="px-2.5 py-1 rounded-md text-xs text-white border-0 cursor-pointer backdrop-blur-sm"
          style={{ background: showRings ? 'rgba(59,130,246,0.8)' : 'rgba(30,30,30,0.7)' }}
        >
          링 {showRings ? 'ON' : 'OFF'}
        </button>
      </div>

      {/* ── 범례 ─────────────────────────────────────────── */}
      <div
        className="absolute bottom-2 left-2 z-10 rounded-md px-3 py-2 text-[11px] leading-relaxed font-mono text-white backdrop-blur-sm"
        style={{ background: 'rgba(0,0,0,0.55)' }}
      >
        <div className="flex items-center gap-2">
          <span>Q_j</span>
          <span
            className="inline-block h-2 w-28 rounded-sm"
            style={{
              background:
                'linear-gradient(to right, hsl(240,100%,50%), hsl(180,100%,50%), hsl(120,100%,50%), hsl(60,100%,50%), hsl(0,100%,50%))',
            }}
          />
          <span>0 … {num(qMax)} N</span>
        </div>
        <div className="mt-1 text-white/70">
          축: <span style={{ color: '#f87171' }}>X = 회전축</span> · φ = 0 은 +Y (D-7 · D-8)
        </div>
        <div className="text-white/70">
          접촉선 = 볼별 α_j 방향(내·외륜 접촉점 관통), 굵기 ∝ Q_j
        </div>
        {result ? (
          <div className="text-white/70">
            <span style={{ color: C_FORCE }}>━ 하중</span>{' '}
            <span style={{ color: C_MOMENT }}>━ 모멘트</span>{' '}
            <span style={{ color: C_TILT }}>━ 틸트축</span> (길이는 물리량이 아니다)
          </div>
        ) : (
          <div style={{ color: '#fbbf24' }}>
            무하중 기하 (Solve 전) — φ_j = 2πj/Z, α = α₀ = {num(toDeg(alpha0 ?? 0))} °, Q = 0
          </div>
        )}
        {tiltDeg !== null && (
          <div className="text-white/70">
            틸트 방위 {num(tiltDeg)} °
            {momDeg !== null && (
              <> · 모멘트 방위 {num(momDeg)} ° · Δ = {num(wrapDeg(tiltDeg - momDeg))} ° (D-2d)</>
            )}
          </div>
        )}
      </div>

      {stale && (
        <div
          className="absolute top-2 left-2 z-10 rounded-md px-3 py-1.5 text-[11px] text-amber-200 backdrop-blur-sm"
          style={{ background: 'rgba(120,53,15,0.75)' }}
        >
          입력이 Solve 이후 변경되었다. <b>형상은 현재 입력</b>, <b>하중·변위는 resultInput</b>{' '}
          기준이다.
        </div>
      )}

      {error && result === null && (
        <div
          className="absolute top-2 left-2 z-10 rounded-md px-3 py-1.5 text-[11px] text-red-200 backdrop-blur-sm"
          style={{ background: 'rgba(127,29,29,0.75)' }}
        >
          기하 계산 실패: {error}
        </div>
      )}

      <Canvas
        camera={{ position: camPos, fov: 45, near: rOD * 0.02, far: rOD * 60 }}
        style={{ background: '#0b1120' }}
      >
        <ambientLight intensity={0.5} />
        <directionalLight position={[rOD * 2, rOD * 2, rOD * 3]} intensity={0.9} />
        <directionalLight position={[-rOD * 2, -rOD * 1.5, -rOD * 2]} intensity={0.35} />
        <Scene
          g={g}
          balls={balls}
          qMax={qMax}
          op={op}
          disp={disp}
          showLoads={showLoads}
          showRings={showRings}
        />
        <OrbitControls enableDamping dampingFactor={0.12} />
      </Canvas>
    </div>
  );
}
