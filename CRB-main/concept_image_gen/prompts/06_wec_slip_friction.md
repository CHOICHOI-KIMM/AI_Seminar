==================================================
[INSTRUCTION BLOCK - DO NOT RENDER IN IMAGE]
==================================================

Purpose:
Generate a technical visualization of the WEC (White Etching Crack) risk assessment framework for TRB. The image shows three independent risk criteria evaluated in parallel — (1) Guo Slip-under-Load, (2) Cumulative Friction Energy, (3) Smearing/Flash Temperature — each fed by the TRB kinematics and traction calculation chain: cone-apex geometry → slice-by-slice sliding velocity → traction coefficient (Eyring) → friction force/energy accumulation. The three criteria converge into an overall WEC risk level.

Visual Concept:
- Central metaphor: A funnel structure. At the top, TRB kinematics computes slice sliding and traction. This feeds into three parallel evaluation branches in the middle. At the bottom, the three branches converge into a single overall risk assessment with recommendations.
- Top section (Kinematics): Cone-apex geometry → ω_cage, ω_roller → per-slice u_slide → SRR → μ_eff (Eyring traction)
- Middle section (3 parallel criteria):
  Branch 1: Guo (2021) — counts simultaneous slip + high-load events across time → risk_index = 0.6×slip_load + 0.4×load_severity
  Branch 2: Friction Energy (Argonne) — ΣF_traction × u_slide × dt / A_contact → energy density [J/mm²] vs E_crit=50
  Branch 3: Smearing — peak SRR, negative SRR (stricter threshold), flash temperature ΔT (Blok formula)
- Bottom: Three risk levels merge → Overall WEC Risk (Low/Medium/High/Critical)

Domain Keywords Reference (for AI understanding only - DO NOT render in image):
- WEC = White Etching Crack, hydrogen-driven subsurface transformation
- Cone apex = TRB geometry where inner/outer/roller cones converge to common apex
- ω_cage = cage angular velocity = ω_i × sin(α_i)/(sin(α_i)+sin(α_o))
- u_slide = sliding velocity per slice (zero in ideal apex-aligned geometry)
- SRR = Slide-Roll Ratio = u_slide / u_roll
- Negative SRR = roller slower than raceway (60% stricter threshold per WEC literature)
- Eyring traction: μ = (τ₀/p)×sinh⁻¹(η₀×SRR/(h_c×τ₀))
- Flash temperature: ΔT = C_Blok × μ × Q × |V_slide|
- E_crit = 50 J/mm² critical friction energy density
- Guo risk_index = 0.6×slip_load_fraction + 0.4×load_severity

Style:
- Clean flat diagram with three parallel branches
- Technical diagram aesthetic
- Each criterion branch has a distinct color
- Convergence funnel at bottom
- Sans-serif typography

Colors (Theme: technical-report):
- Background: #FFFFFF
- Primary: #2C3E50
- Guo branch: #2980B9 (blue)
- Energy branch: #E67E22 (orange)
- Smearing branch: #E74C3C (red)
- Risk levels: Low=#27AE60, Medium=#F39C12, High=#E67E22, Critical=#E74C3C
- Kinematics section: #F0F3F5 fill

Resolution:
- 8K Quality, 4:3 aspect ratio (landscape)

==================================================
[CONFIGURATION BLOCK - DO NOT RENDER IN IMAGE]
==================================================

Text Specifications (for AI reference only):
- Section title: 20-24pt SemiBold
- Branch labels: 16-18pt Medium
- MINIMUM font size: 14pt, MAXIMUM text elements: 14

==================================================
[CONTENT BLOCK - FOR IMAGE RENDERING]
==================================================

Core message:
"WEC risk is assessed through three independent criteria — slip-under-load, friction energy, and flash temperature — all derived from TRB kinematics and converging into a unified risk level."

--------------------------------------------------
Domain Keywords (PRESERVE EXACTLY):
All English. WEC, SRR, Guo, Eyring, Flash Temperature, E_crit

--------------------------------------------------
Visual composition:

Metaphor type: Funnel (top: kinematics → middle: 3 parallel criteria → bottom: overall risk)

Top section — TRB Kinematics:
- Cone-apex geometry → ω_cage, ω_roller → u_slide per slice → SRR → μ_eff

Three parallel branches:
1. Guo Slip-under-Load (blue): time-series scan → simultaneous slip + high load → risk index
2. Friction Energy (orange): Σ(μ×Q×u_slide×dt) / A_contact → J/mm² vs E_crit=50
3. Smearing (red): peak SRR, negative SRR (stricter), ΔT flash temperature

Bottom — Convergence:
- Three risk levels → worst-case → Overall WEC Risk → Recommendations

Key visual elements:
1. Kinematics block at top (cone geometry icon, angular velocities)
2. Three colored branch lines splitting from kinematics
3. Guo criterion card (blue)
4. Friction Energy criterion card (orange)
5. Smearing criterion card (red)
6. Convergence funnel merging three branches
7. Overall risk level bar (4-level: Low/Medium/High/Critical)

--------------------------------------------------
Text elements (14):

1. "TRB Kinematics" location: top section
2. "ω_cage, ω_roller" location: kinematics detail
3. "u_slide → SRR → μ_eff" location: kinematics output
4. "Guo Slip-under-Load" location: branch 1 title
5. "Slip + High Load events" location: branch 1 detail
6. "Friction Energy" location: branch 2 title
7. "Σ(μ·Q·u·dt) / A" location: branch 2 formula
8. "E_crit = 50 J/mm²" location: branch 2 threshold
9. "Smearing" location: branch 3 title
10. "SRR, −SRR, ΔT flash" location: branch 3 detail
11. "Overall WEC Risk" location: convergence output
12. "Low" location: risk bar left
13. "Critical" location: risk bar right
14. "Worst-case rule" location: convergence annotation

==================================================
[FORBIDDEN ELEMENTS - DO NOT RENDER IN IMAGE]
==================================================

Forbidden Elements:
- Header/title, non-white background, Korean text, dense paragraphs
- More than 14 text elements, full sentences, font specs

==================================================

Final output goal:
- Three independent WEC criteria clearly color-coded and parallel
- Kinematics chain (geometry → sliding → traction) feeding all three
- Convergence into single risk level visually intuitive
- 4:3 landscape for presentation
