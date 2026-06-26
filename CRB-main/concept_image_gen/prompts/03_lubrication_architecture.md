==================================================
[INSTRUCTION BLOCK - DO NOT RENDER IN IMAGE]
==================================================

Purpose:
Generate a technical comparison chart of four EHL models (M1/M2/M3 = empirical, HMEHL = numerical) for TRB analysis. The image must clearly show:
(1) The 3+1 structural split (Empirical vs Numerical)
(2) Qualitative feature comparison across 6 key dimensions — showing WHERE each model is strong or weak
(3) Why M2 is "Recommended" for design practice

Visual Concept:
- Layout: A comparison matrix / feature chart. Rows = 6 comparison dimensions. Columns = 4 models (M1 | M2 | M3 | HMEHL), with M1/M2/M3 under a shared "Empirical" header and HMEHL under "Numerical" header.
- Each cell contains a short keyword AND a visual indicator (filled circle ●, half ◐, or empty ○) showing capability level:
  ● = Full support / accurate
  ◐ = Partial / limited
  ○ = Not supported / inaccurate
- The 6 comparison rows:
  1. Roughness: ○ M1 | ● M2 | ○ M3 | ● HMEHL
  2. Viscosity: ◐ M1 (Barus) | ● M2 (Roelands) | ● M3 | ● HMEHL
  3. Thermal φ_T: ◐ M1 (speed only) | ● M2 (+SRR) | ● M3 | ● HMEHL (self-consistent)
  4. Starvation φ_s: ○ M1 (user input) | ● M2 (physics + user) | ● M3 | — HMEHL (N/A)
  5. Traction μ: ◐ M1 (τ₀/p const) | ● M2 (Eyring post-process) | ● M3 | ● HMEHL (Eyring in-PDE)
  6. Output: ◐ M1 (h_c scalar) | ◐ M2 (h_c scalar) | ◐ M3 (h_c scalar) | ● HMEHL (P(x),h(x),T(x))
- Below the matrix: a bottom bar showing the shared downstream pipeline "→ Λ → Regime → κ → a_ISO → L₁₀ₘ"
- Column headers show model name, year, and a use-case tag (Fast Review / Recommended / Research / Full Physics)
- M2 column highlighted with blue accent border. HMEHL column with orange accent border.

Domain Keywords Reference (for AI understanding only - DO NOT render in image):
- M1 (DH) = Dowson-Higginson 1977 — simplest empirical, Barus viscosity, no roughness, Gupta thermal, user-input starvation, constant μ. Fastest but least accurate.
- M2 (MK) = Masjedi-Khonsari 2012 — roughness correction rc(σ̄) integrated into film formula, Roelands high-pressure viscosity, Murch-Wilson SRR-dependent thermal, physics-based starvation, Eyring traction (post-process: film computed first with Newtonian viscosity, then friction computed with Eyring), flash temperature. Best balance of speed and accuracy for TRB design.
- M3 (NVM) = Nijenbanning-Venner-Moes 1994 — 4-regime asymptotic blend (covers low-speed to high-speed), Moes M/L/D parameters, elliptical contact capable (future ball bearing support), but no own roughness model (shares M2's). Used for paper validation and Van Zoelen Film Decay pipeline.
- HMEHL = Numerical solver — Reynolds PDE on 256-node grid, FAS V-cycle multigrid, DC-FFT elasticity, FBNS mass-conserving cavitation, Eyring non-Newtonian viscosity applied INSIDE the PDE (modifies effective viscosity at every grid point, so film shape itself changes), TEHL energy equation. Outputs spatially resolved P(x), h(x), T(x) distributions. Highest accuracy but slowest (~100× M1).

Style:
- Clean comparison matrix / feature chart
- Technical diagram aesthetic
- Filled/half/empty circle indicators for capability level
- Column headers with colored accent bars
- Sans-serif typography, clean grid lines

Colors (Theme: technical-report):
- Background: #FFFFFF
- Primary: #2C3E50 - text, grid lines
- Secondary: #5D6D7E - supporting labels
- M1 column header: #27AE60 (green)
- M2 column header: #2980B9 (blue) — highlighted as Recommended
- M3 column header: #8E44AD (purple)
- HMEHL column header: #E67E22 (orange)
- Full support ●: #2C3E50
- Partial ◐: #95A5A6
- Not supported ○: #D5D8DC
- Shared pipeline bar: #F0F3F5

Lighting & Mood:
- Flat design, clean and precise

Resolution:
- 8K Quality, 4:3 aspect ratio (landscape)

==================================================
[CONFIGURATION BLOCK - DO NOT RENDER IN IMAGE]
==================================================

Text Specifications (for AI reference only):
- Column header: 20-24pt Bold
- Row label: 16-18pt SemiBold
- Cell keyword: 14-16pt Regular
- Capability indicator: 18-20pt (● ◐ ○ symbols)
- MINIMUM font size: 14pt, MAXIMUM text elements: 14

Visual Specifications (for AI reference only):
- Grid line width: 1px
- Column header height: 60px
- Row height: 40px
- Circle indicator diameter: 16px

==================================================
[CONTENT BLOCK - FOR IMAGE RENDERING]
==================================================

Core message:
"M2 is recommended for TRB design because it uniquely integrates surface roughness into the film formula while maintaining closed-form speed — M1 ignores roughness, M3 lacks its own roughness model, and HMEHL is too slow for routine design."

--------------------------------------------------
Domain Keywords (PRESERVE EXACTLY):
All English. M1 (DH), M2 (MK), M3 (NVM), HMEHL, Barus, Roelands, Eyring, Roughness, Λ

--------------------------------------------------
Visual composition:

Metaphor type: Comparison Matrix (feature chart with capability indicators)

Structure:
- Top: Two group headers — "Empirical (Closed-form)" spanning M1/M2/M3 | "Numerical" spanning HMEHL
- Column headers: M1 (DH) 1977 / M2 (MK) 2012 / M3 (NVM) 1994 / HMEHL
- Sub-labels under headers: Fast Review / Recommended / Research / Full Physics
- 6 comparison rows with keyword + indicator per cell
- Bottom: shared pipeline bar

Row details:
1. "Roughness" — M1: ○ None | M2: ● Integrated rc(σ̄) | M3: ○ Shared from M2 | HMEHL: ● Patir-Cheng
2. "Viscosity" — M1: ◐ Barus | M2: ● Roelands | M3: ● Roelands | HMEHL: ● Roelands
3. "Thermal φ_T" — M1: ◐ Gupta | M2: ● Murch-Wilson | M3: ● Murch-Wilson | HMEHL: ● TEHL
4. "Starvation φ_s" — M1: ○ User input | M2: ● Physics+User | M3: ● Physics+User | HMEHL: ○ N/A
5. "Traction μ" — M1: ◐ τ₀/p const | M2: ● Eyring (post) | M3: ● Eyring (post) | HMEHL: ● Eyring (in-PDE)
6. "Output" — M1: ◐ h_c scalar | M2: ◐ h_c scalar | M3: ◐ h_c scalar | HMEHL: ● P(x),h(x),T(x)

--------------------------------------------------
Text elements (14):

1. "Empirical (Closed-form)" location: top group header spanning M1-M3
2. "Numerical" location: top group header for HMEHL
3. "M1 (DH) 1977" location: column 1 header
4. "M2 (MK) 2012" location: column 2 header
5. "M3 (NVM) 1994" location: column 3 header
6. "HMEHL" location: column 4 header
7. "Roughness" location: row 1 label
8. "Viscosity" location: row 2 label
9. "Thermal φ_T" location: row 3 label
10. "Starvation φ_s" location: row 4 label
11. "Traction μ" location: row 5 label
12. "Output" location: row 6 label
13. "Recommended" location: M2 column badge
14. "Λ → Regime → κ → a_ISO" location: shared bottom bar

**Text rules:**
- All English, keywords only
- Cell contents: keyword + indicator symbol
- No full sentences

==================================================
[FORBIDDEN ELEMENTS - DO NOT RENDER IN IMAGE]
==================================================

Forbidden Elements:
- Header/title/slide title
- Non-white backgrounds
- Korean text
- Dense paragraphs or full sentences
- More than 14 text elements
- Font size below 14pt
- 2×2 grid layout (MUST be 4-column comparison matrix)
- Placing M3 in "Numerical" category

==================================================

Final output goal:
- At a glance: M2 has the most filled circles (●) among empirical models
- Roughness row makes M2's advantage immediately clear (only ● in empirical group)
- HMEHL stands out in Output row (only ● for spatial distributions)
- 3+1 structural distinction (Empirical vs Numerical) clearly visible
- Shared Λ→life pipeline connects all four models
- 4:3 landscape
