# TRB Contact Analysis — Theory Manual

## Table of Contents

| Chapter | Title | Description |
|---------|-------|-------------|
| [01](01_Introduction.md) | Introduction | System overview, dual-mode concept, analysis pipeline |
| [02](02_Geometry.md) | Bearing Geometry | Macro geometry, raceway geometry, coordinate system |
| [03](03_ProfileModel.md) | Profile Modification | Crown types, dub-off, raceway profiles, cubic spline interpolation |
| [04](04_SliceModel.md) | Slice Discretization | Slicing concept, equivalent radii, profile superposition |
| [05](05_HertzContact.md) | Hertz Line Contact | Combined elastic modulus, contact half-width, max pressure, Weber bulk deformation |
| [06](06_Gen1_Solver.md) | Gen1: Independent Slice | Independent nonlinear spring model, Newton-Raphson for target load |
| [07](07_Gen3_Solver.md) | Gen3: Beam-Coupled Slice | Timoshenko beam FE, Newton-Raphson with active set, rigid body projection |
| [08](08_RibContact.md) | Large-End Rib Contact | Elliptical Hertz point contact, Hamrock-Brewe approximation, spin moment |
| [09](09_BearingEquilibrium.md) | 5-DOF Bearing Equilibrium | Roller approach, force/moment residual, scaled Newton-Raphson |
| [10](10_FatigueLife.md) | Fatigue Life Calculation | ISO 16281 lamina-level life, lubrication, a_ISO, C_r |
| [11](11_DualMode.md) | Dual-Mode Comparison | Gen1/Gen3 comparison, edge stress rise, recommendation logic |
| [12](12_SingleRollerVisualization.md) | Single Roller Visualization | Roller selector, distributions view, contact patch heatmap, Gen1/Gen3 overlay |
| [13](13_StaticLoadRating.md) | Static Load Rating | ISO 76 C₀ᵣ/P₀ᵣ/S₀, ISO 17956 lamina-level S₀,eff |
| [14](14_Lubrication.md) | Lubrication & Film Thickness | EHL film thickness, Greenwood-Tripp mixed lubrication, TRB kinematics, traction & power loss |
| [15](15_TransientDynamics.md) | Transient Roller Dynamics | Roller inertia, SRR, per-slice geometric micro-slip, drag models, contour visualization |
| [16](16_LoadCalculationFlow.md) | Load Calculation Flow | 전체 하중 계산 플로우 (dual-raceway + rib contact), 방향/좌표계, 수렴 구조 |
| [17](17_SplitContactModel.md) | Split Contact Model | 내/외륜 독립 하중 분배, δ_o split, Gen1/Gen3 적용, 보간, UI |

---

**Notation Conventions:**
- SI units throughout: [mm], [N], [MPa], [μm], [rad], [°C]
- Bold for vectors/matrices: **F**, **K**
- Subscript `k` = slice index, `j` = roller index
- `α` = contact angle, `ψ` = roller angular position
