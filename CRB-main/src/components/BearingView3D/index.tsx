// @ts-nocheck
// CRB Phase 1.4 stub: 이 컴포넌트는 TRB 데이터 모델을 참조 중 → Phase 6 (Frontend UI 변경) 에서 CRB 로 정식 재작성 예정
import { Canvas } from '@react-three/fiber';
import { OrbitControls, Text, Html } from '@react-three/drei';
import { useMemo, useState } from 'react';
import { useAppState } from '../../store';
import { useActiveResult } from '../../hooks/useActiveResult';
import * as THREE from 'three';
import type { MacroGeometry } from '../../types/bearing';

/** Radial force arrow: shaft (cylinder) + head (cone), pointing outward along ψ */
function ForceArrow({ psiRad, origin, length, color }: {
  psiRad: number; origin: [number, number, number]; length: number; color: THREE.Color;
}) {
  const shaftLen = length * 0.75;
  const headLen = length * 0.25;
  const shaftR = length * 0.04;
  const headR = length * 0.1;
  // direction unit vector in XY plane
  const dx = Math.cos(psiRad);
  const dy = Math.sin(psiRad);
  // rotation: default cylinder is along Y → rotate to lie in XY along (dx,dy)
  // angle from +Y to direction vector (dx,dy): atan2(dx, dy) rotated around Z
  const rotZ = -Math.atan2(dx, dy);
  return (
    <group position={origin}>
      {/* shaft: centered at half-shaft along direction */}
      <mesh
        position={[dx * shaftLen / 2, dy * shaftLen / 2, 0]}
        rotation={[0, 0, rotZ]}
      >
        <cylinderGeometry args={[shaftR, shaftR, shaftLen, 6]} />
        <meshStandardMaterial color={color} />
      </mesh>
      {/* head: cone at end of shaft */}
      <mesh
        position={[dx * (shaftLen + headLen / 2), dy * (shaftLen + headLen / 2), 0]}
        rotation={[0, 0, rotZ]}
      >
        <coneGeometry args={[headR, headLen, 8]} />
        <meshStandardMaterial color={color} />
      </mesh>
    </group>
  );
}

function AxisLabels({ size }: { size: number }) {
  const labelOffset = size * 1.15;
  const fontSize = size * 0.15;
  const labelProps = { fontSize, anchorX: 'center' as const, anchorY: 'middle' as const };
  return (
    <group>
      <axesHelper args={[size]} />
      <Text position={[labelOffset, 0, 0]} color="red" {...labelProps}>X</Text>
      <Text position={[0, labelOffset, 0]} color="green" {...labelProps}>Y</Text>
      <Text position={[0, 0, labelOffset]} color="#4488ff" {...labelProps}>Z</Text>
    </group>
  );
}

export default function BearingView3D() {
  const { state } = useAppState();
  const result = useActiveResult();
  const mg = state.input.macro_geom;
  const [showLoads, setShowLoads] = useState(false);

  if (!result) return null;

  return (
    <div className="w-full h-full relative">
      <button
        onClick={() => setShowLoads(v => !v)}
        style={{
          position: 'absolute',
          top: 8,
          right: 8,
          zIndex: 10,
          background: showLoads ? 'rgba(59,130,246,0.8)' : 'rgba(30,30,30,0.7)',
          color: '#fff',
          border: 'none',
          borderRadius: 6,
          padding: '4px 10px',
          fontSize: 12,
          cursor: 'pointer',
          backdropFilter: 'blur(4px)',
        }}
      >
        하중 표시 {showLoads ? 'ON' : 'OFF'}
      </button>
      <Canvas camera={{ position: [80, 80, 60], fov: 50 }}>
        <ambientLight intensity={0.4} />
        <directionalLight position={[10, 10, 20]} intensity={0.8} />
        <BearingScene
          rollerResults={result.equilibrium.roller_results}
          mg={mg}
          showLoads={showLoads}
        />
        <OrbitControls enableDamping dampingFactor={0.1} />
        <AxisLabels size={mg.outer_diameter * 0.1} />
      </Canvas>
    </div>
  );
}

function BearingScene({
  rollerResults,
  mg,
  showLoads,
}: {
  rollerResults: { psi_deg: number; q_normal: number; rib_result?: { f_rib: number } | null }[];
  mg: MacroGeometry;
  showLoads: boolean;
}) {
  const qMax = Math.max(...rollerResults.map(r => r.q_normal), 1);
  const rPitch = mg.d_pw / 2;
  const rRoller = mg.d_we_max / 2;
  const rRollerMin = mg.d_we_min / 2;
  const alphaRad = (mg.alpha * Math.PI) / 180;
  const lWe = mg.l_we;
  const halfL = lWe / 2;

  const colorScale = useMemo(() => {
    // Rainbow: blue(0) → cyan → green → yellow → red(1)
    return (q: number) => {
      const t = Math.min(q / qMax, 1);
      const hue = (1 - t) * 0.667; // 0.667(blue) → 0(red)
      return new THREE.Color().setHSL(hue, 1, 0.5);
    };
  }, [qMax]);

  // Ring cross-section profiles for LatheGeometry
  const { outerRingGeo, innerRingGeo } = useMemo(() => {
    const cosA = Math.cos(alphaRad);
    const sinA = Math.sin(alphaRad);
    const clr = 0.3; // clearance

    const rBore = mg.d / 2;
    const rOuter = mg.outer_diameter / 2;
    const halfT = mg.t / 2;

    // Roller envelope (inner/outer edges along bearing axis)
    const rollerInnerLargeR = rPitch + halfL * sinA - rRoller * cosA;
    const rollerInnerSmallR = rPitch - halfL * sinA - rRollerMin * cosA;
    const rollerOuterLargeR = rPitch + halfL * sinA + rRoller * cosA;
    const rollerOuterSmallR = rPitch - halfL * sinA + rRollerMin * cosA;

    const rollerLargeY = halfL * cosA;
    const rollerSmallY = -halfL * cosA;

    // Inner ring: bore to raceway
    const irRacewayLargeR = rollerInnerLargeR - clr;
    const irRacewaySmallR = rollerInnerSmallR - clr;
    const ribTopR = irRacewayLargeR + mg.h_rib;

    const innerPts: THREE.Vector2[] = [
      new THREE.Vector2(rBore, -halfT),           // bore, small end
      new THREE.Vector2(irRacewaySmallR, -halfT),  // small end face, raceway level
      new THREE.Vector2(irRacewaySmallR, rollerSmallY), // raceway small end
      new THREE.Vector2(irRacewayLargeR, rollerLargeY), // raceway large end
      new THREE.Vector2(ribTopR, rollerLargeY + 0.5),   // rib tip (angled face)
      new THREE.Vector2(ribTopR, halfT),           // rib back edge at ring face
      new THREE.Vector2(rBore, halfT),             // bore, large end
    ];

    const innerRingGeo = new THREE.LatheGeometry(
      innerPts, 64, 0, Math.PI * 2,
    );

    // Outer ring: raceway to outer surface
    const orRacewayLargeR = rollerOuterLargeR + clr;
    const orRacewaySmallR = rollerOuterSmallR + clr;

    const outerPts: THREE.Vector2[] = [
      new THREE.Vector2(orRacewaySmallR, -halfT),
      new THREE.Vector2(orRacewaySmallR, rollerSmallY),
      new THREE.Vector2(orRacewayLargeR, rollerLargeY),
      new THREE.Vector2(orRacewayLargeR, halfT),
      new THREE.Vector2(rOuter, halfT),
      new THREE.Vector2(rOuter, -halfT),
    ];

    const outerRingGeo = new THREE.LatheGeometry(
      outerPts, 64, 0, Math.PI * 2,
    );

    return { outerRingGeo, innerRingGeo };
  }, [mg, alphaRad, rPitch, rRoller, rRollerMin, halfL]);

  return (
    <group>
      {/* Outer ring — rotate LatheGeometry (Y-axis) to Z-axis */}
      <mesh geometry={outerRingGeo} rotation={[-Math.PI / 2, 0, 0]}>
        <meshStandardMaterial color="#475569" transparent opacity={0.3} side={THREE.DoubleSide} depthWrite={false} />
      </mesh>

      {/* Inner ring — rotate LatheGeometry (Y-axis) to Z-axis */}
      <mesh geometry={innerRingGeo} rotation={[-Math.PI / 2, 0, 0]}>
        <meshStandardMaterial color="#64748b" transparent opacity={0.35} side={THREE.DoubleSide} depthWrite={false} />
      </mesh>

      {/* Rollers - tapered cylinders positioned on pitch circle */}
      {rollerResults.map((r, i) => {
        const psiRad = (r.psi_deg * Math.PI) / 180;
        const px = rPitch * Math.cos(psiRad);
        const py = rPitch * Math.sin(psiRad);
        const color = colorScale(r.q_normal);

        return (
          <group key={i}>
            {/* Cylinder along Z (rotated from Y-default) */}
            <mesh position={[px, py, 0]} rotation={[-Math.PI / 2, 0, 0]}>
              <cylinderGeometry args={[rRollerMin, rRoller, lWe, 12]} />
              <meshStandardMaterial color={color} />
            </mesh>
            {showLoads && r.q_normal > 0 && (() => {
              const arrowLen = (r.q_normal / qMax) * rPitch * 0.4 + rRoller * 0.5;
              const arrowStart: [number, number, number] = [
                px + Math.cos(psiRad) * rRoller * 1.2,
                py + Math.sin(psiRad) * rRoller * 1.2,
                0,
              ];
              const tipX = arrowStart[0] + Math.cos(psiRad) * arrowLen;
              const tipY = arrowStart[1] + Math.sin(psiRad) * arrowLen;
              return (
                <>
                  <ForceArrow
                    psiRad={psiRad}
                    origin={arrowStart}
                    length={arrowLen}
                    color={color}
                  />
                  <Html
                    position={[tipX, tipY, 0]}
                    center
                    style={{ pointerEvents: 'none' }}
                  >
                    <div style={{
                      background: 'rgba(0,0,0,0.75)',
                      color: '#fff',
                      padding: '2px 5px',
                      borderRadius: 4,
                      fontSize: 10,
                      textAlign: 'center',
                      whiteSpace: 'nowrap',
                      lineHeight: 1.3,
                    }}>
                      <div style={{ fontWeight: 600 }}>#{i + 1}</div>
                      <div>Q: {(r.q_normal / 1000).toFixed(1)} kN</div>
                      {r.rib_result && r.rib_result.f_rib > 0 && (
                        <div style={{ color: '#fbbf24' }}>Rib: {(r.rib_result.f_rib / 1000).toFixed(2)} kN</div>
                      )}
                    </div>
                  </Html>
                </>
              );
            })()}
          </group>
        );
      })}
    </group>
  );
}
