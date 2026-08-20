# Chapter 2: Bearing Geometry

## 2.1 Macro Geometry (Bearing Level)

원추 롤러 베어링의 기본 치수를 정의한다.

```
          ┌─── outer_diameter (D) ───┐
          │                          │
    ┌─────┤     α ↗                  ├─────┐
    │     │    ↗  d_pw               │     │ T (width)
    │     │   ↗  ←────→              │     │
    │     │  Roller (L_we)           │     │
    │     │   D_we_max  D_we_min     │     │
    └─────┤                          ├─────┘
          │          d (bore)        │
          └──────────────────────────┘
```

| Symbol | Description | Unit |
|--------|-------------|------|
| `d` | Bore diameter | mm |
| `D` (outer_diameter) | Outer diameter | mm |
| `T` | Bearing width | mm |
| `α` (alpha) | Contact angle (half-taper angle) | deg |
| `Z` | Number of rollers | - |
| `D_we_max` | Roller large-end diameter | mm |
| `D_we_min` | Roller small-end diameter | mm |
| `L_we` | Roller effective contact length | mm |
| `d_pw` | Pitch circle diameter | mm |
| `h_rib` | Large-end rib height | mm |
| `α_rib` | Rib angle | deg |
| `G_r` | Radial internal clearance | μm |

### Roller Taper

롤러는 소단(small end)에서 대단(large end)으로 선형 테이퍼 형상이다:

$$
R_{roller}(x) = R_{small} + \frac{R_{large} - R_{small}}{L_{we}} \cdot x
$$

여기서 `x`는 소단으로부터의 축방향 거리 [mm], `R_small = D_we_min / 2`, `R_large = D_we_max / 2`.

### Taper Geometry Parameter (γ)

ISO 281에서 중요한 무차원 파라미터:

$$
\gamma = \frac{D_{we} \cos\alpha}{d_{pw}}
$$

여기서 `D_we = (D_we_max + D_we_min) / 2` (평균 롤러 직경). 일반적으로 γ ∈ [0.02, 0.25].

## 2.2 Raceway Geometry

내/외륜 레이스웨이의 기하학적 형상을 정의한다.

| Symbol | Description | Unit |
|--------|-------------|------|
| `α_i` | Inner raceway taper angle | deg |
| `α_o` | Outer raceway taper angle | deg |
| `R_i` | Inner raceway transverse curvature radius | mm |
| `R_o` | Outer raceway transverse curvature radius | mm |
| `r_rib` | Large-end rib fillet radius | mm |
| `d_uc` | Raceway undercut depth | mm |
| `L_uc` | Raceway undercut axial extent | mm |

### Transverse Curvature

레이스웨이 횡단면의 곡률 반경은 접촉 반폭과 응력에 직접 영향을 미친다. 평면(∞) 레이스웨이 대비 오목(concave) 레이스웨이는 등가 반경이 커져 접촉 응력이 감소한다.

### Raceway Undercut

열처리 또는 가공 상의 이유로 레이스웨이 단부에 언더컷(relief groove)이 존재할 수 있다. 언더컷 영역에서는 접촉이 발생하지 않으며, 유효 접촉 길이가 단축된다.

## 2.3 Coordinate System

- **베어링 좌표계**: 원점은 베어링 중심, z축은 베어링 축(축방향), x축은 반경 하중 방향
- **롤러 좌표계**: 원점은 롤러 소단, x축은 롤러 축(축방향), 소단→대단 방향
- **각도 ψ**: 롤러 원주 위치, x축(F_r 방향)으로부터 반시계 방향

$$
\psi_j = j \cdot \frac{2\pi}{Z}, \quad j = 0, 1, \ldots, Z-1
$$
