==================================================
[INSTRUCTION BLOCK - DO NOT RENDER IN IMAGE]
==================================================

Purpose:
Generate a side-by-side pipeline comparison of the four EHL film thickness calculation methods (M1/M2/M3/HMEHL). Each method is shown as a vertical column flowing top-to-bottom, with shared input at the top and shared output classification at the bottom. The columns highlight where the methods diverge: viscosity model, film formula, roughness handling, correction factors, and output resolution.

Visual Concept:
- Central metaphor: Four parallel vertical pipelines (swimlane style). Shared input row at top, shared output row at bottom. Each column represents one model (M1, M2, M3, HMEHL). Pipeline stages are shown as horizontally aligned nodes across columns, with differences highlighted where the methods diverge.
- Pipeline stages (top to bottom):
  Row 1 (shared): Input — ν₄₀, ν₁₀₀, α_pv, geometry, load
  Row 2: Viscosity model — Barus (M1) vs Roelands (M2/M3/HMEHL)
  Row 3: Film formula — D-H (M1) vs D-H+rc (M2) vs 4-regime (M3) vs Reynolds PDE (HMEHL)
  Row 4: Roughness — None (M1/M3) vs Integrated (M2) vs Patir-Cheng homogenized (HMEHL)
  Row 5: Corrections — Gupta+User φ_s (M1) vs Murch-Wilson+Auto φ_s (M2/M3) vs Self-consistent TEHL (HMEHL)
  Row 6 (shared): Output — h_c scalar (M1/M2/M3) vs h(x), P(x), T(x) distributions (HMEHL)
  Row 7 (shared): Λ → Regime → κ → a_ISO
- Emphasis: Row 3 (film formula) as the key differentiator. HMEHL column visually wider or accented.

Domain Keywords Reference (for AI understanding only - DO NOT render in image):
- Barus: η = η₀·exp(α·p), simple exponential
- Roelands: η = η₀·exp(S·((1+p/p_r)^Z−1)), nonlinear high-pressure
- D-H: Dowson-Higginson H = f(U,G,W) closed-form
- rc(σ̄): M-K roughness correction factor
- 4-regime: NVM asymptotic blending of IR/IE/RP/EP
- Reynolds PDE: numerical solution on 256-node grid with FAS multigrid
- Patir-Cheng: flow factor homogenization for rough surfaces
- TEHL: coupled thermal-EHL energy equation

Style:
- Swimlane flowchart, 4 columns
- Clean flat nodes with color-coded column headers
- Horizontal alignment shows comparable stages
- Sans-serif typography

Colors (Theme: technical-report):
- Background: #FFFFFF
- M1 column: #27AE60 header
- M2 column: #2980B9 header
- M3 column: #8E44AD header
- HMEHL column: #E67E22 header
- Shared rows: #F0F3F5 fill
- Node fill: white with colored left border matching column

Resolution:
- 8K Quality, 3:4 aspect ratio (portrait — tall to fit 7 rows × 4 columns)

==================================================
[CONFIGURATION BLOCK - DO NOT RENDER IN IMAGE]
==================================================

Text Specifications (for AI reference only):
- Column header: 20-24pt Bold
- Node text: 14-16pt Regular
- Row label: 14pt Medium italic
- MINIMUM font size: 14pt, MAXIMUM text elements: 14

==================================================
[CONTENT BLOCK - FOR IMAGE RENDERING]
==================================================

Core message:
"M1→M2→M3→HMEHL progressively upgrade viscosity model, roughness handling, and output resolution while sharing the same Λ→life pipeline."

--------------------------------------------------
Domain Keywords (PRESERVE EXACTLY):
All English. M1, M2, M3, HMEHL, Barus, Roelands, D-H, rc(σ̄), Reynolds PDE, Λ

--------------------------------------------------
Visual composition:

Metaphor type: Swimlane (4 parallel vertical columns)

Key visual elements:
1. Shared input row (top): "Input: ν, α, geometry, load"
2. Four column headers: M1 (DH) / M2 (MK) / M3 (NVM) / HMEHL
3. Viscosity row: Barus | Roelands | Roelands | Roelands
4. Film formula row: D-H closed | D-H + rc(σ̄) | 4-regime blend | Reynolds PDE
5. Roughness row: None | Integrated | None (M2 shared) | Patir-Cheng
6. Corrections row: Gupta φ_T | Murch-Wilson φ_T | Murch-Wilson φ_T | TEHL self-consistent
7. Output row: h_c scalar | h_c scalar | h_c scalar | P(x), h(x), T(x)
8. Shared bottom: Λ → Regime → κ → a_ISO

--------------------------------------------------
Text elements (14):

1. "M1 (DH)" location: column 1 header
2. "M2 (MK)" location: column 2 header
3. "M3 (NVM)" location: column 3 header
4. "HMEHL" location: column 4 header
5. "Input" location: shared top row
6. "Barus" location: M1 viscosity node
7. "Roelands" location: M2/M3/HMEHL viscosity (shared label)
8. "D-H" location: M1 film node
9. "D-H + rc(σ̄)" location: M2 film node
10. "4-regime" location: M3 film node
11. "Reynolds PDE" location: HMEHL film node
12. "h_c scalar" location: M1/M2/M3 output
13. "P(x), h(x), T(x)" location: HMEHL output
14. "Λ → Regime → κ → a_ISO" location: shared bottom

==================================================
[FORBIDDEN ELEMENTS - DO NOT RENDER IN IMAGE]
==================================================

Forbidden Elements:
- Header/title, non-white background, Korean text, dense paragraphs
- More than 14 text elements, full sentences, font specs in image

==================================================

Final output goal:
- Four methods compared stage-by-stage at a glance
- Key divergence points (film formula, roughness) visually prominent
- HMEHL column stands out as the only numerical/distributed output
- 3:4 portrait for paper insertion
