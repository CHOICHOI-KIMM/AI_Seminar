==================================================
[INSTRUCTION BLOCK - DO NOT RENDER IN IMAGE]
==================================================

Purpose:
Generate a vertical workflow of the film-thickness-to-bearing-life algorithm chain. The flow traces: EHL film calculation → roughness interaction (Λ ratio, mixed lubrication regime) → life modification factor (κ determination, a_ISO) → ISO 16281 lamina-level fatigue life → optional Film Decay for grease. Also shows the micropitting safety factor S_λ as a parallel output branch.

Visual Concept:
- Central metaphor: A top-to-bottom flowchart with a main spine and one side branch. Main spine: h_eff → σ (composite roughness) → Λ = h_min/σ → regime classification → κ (two methods: ViscosityRatio or FilmThicknessRatio) → a_ISO → L_10m (modified life). Side branch from Λ: S_λ = Λ_min/Λ_perm → micropitting risk (Safe/Marginal/AtRisk). Optional Film Decay block inserting between h_eff and Λ when grease lubrication.
- Visual elements: Rounded rectangle nodes on main spine. Diamond for κ method selection. A dashed side-branch for S_λ. A dashed optional block for Film Decay.
- Emphasis: The Λ node as the central hub connecting film thickness to both life and micropitting.

Domain Keywords Reference (for AI understanding only - DO NOT render in image):
- h_eff = effective film thickness after thermal/starvation corrections
- σ = composite roughness = √(Rq₁² + Rq₂²)
- Λ = h_min / σ (lambda ratio)
- κ = viscosity ratio for ISO 281 (two methods)
- ViscosityRatio: κ = ν_actual / ν_ref (no film dependency)
- FilmThicknessRatio: κ = Λ^1.3 (film dependent)
- a_ISO = life modification factor from ISO 281 (function of κ, e_c, C_u/P)
- e_c = contamination factor
- L_10m = a_1 × a_ISO × L_10 (modified rating life)
- S_λ = Λ_min / Λ_perm (micropitting safety, adapted from ISO/TS 6336-22)
- Film Decay: Van Zoelen h(t) time-dependent starvation for grease

Style:
- Clean flat flowchart
- Main spine with optional/side branches clearly distinguished (dashed borders)
- Traffic-light colors for regime and micropitting risk classification
- Sans-serif typography

Colors (Theme: technical-report):
- Background: #FFFFFF
- Primary: #2C3E50
- Secondary: #5D6D7E
- Accent: #2980B9 - main pipeline highlight
- Regime: Full EHL=#27AE60, Mixed=#F39C12, Boundary=#E74C3C
- Micropitting: Safe=#27AE60, Marginal=#F39C12, AtRisk=#E74C3C
- Film Decay block: dashed #8E44AD border
- κ diamond: #D5E8F0 fill

Resolution:
- 8K Quality, 3:4 aspect ratio (portrait)

==================================================
[CONFIGURATION BLOCK - DO NOT RENDER IN IMAGE]
==================================================

Text Specifications (for AI reference only):
- Node title: 20-24pt SemiBold
- MINIMUM font size: 14pt, MAXIMUM text elements: 14

==================================================
[CONTENT BLOCK - FOR IMAGE RENDERING]
==================================================

Core message:
"Lambda ratio is the central hub: it feeds both the ISO life modification factor (κ → a_ISO → L_10m) and the micropitting safety assessment (S_λ)."

--------------------------------------------------
Domain Keywords (PRESERVE EXACTLY):
All English. Λ, κ, a_ISO, L_10m, S_λ, Film Decay, e_c

--------------------------------------------------
Visual composition:

Metaphor type: Flow (vertical) with side branch

Main spine (top to bottom):
1. h_eff (from EHL models)
2. [Optional: Film Decay h(t) — dashed block, grease only]
3. σ composite roughness
4. Λ = h_min / σ → Regime (Full EHL / Mixed / Boundary)
5. κ method selection diamond (ViscosityRatio vs FilmThicknessRatio)
6. a_ISO = f(κ, e_c, C_u/P)
7. L_10m = a_1 × a_ISO × L_10

Side branch from Λ node:
- S_λ = Λ_min / Λ_perm → Risk (Safe / Marginal / AtRisk)

Key visual elements:
1. h_eff input node
2. Film Decay optional block (dashed, purple)
3. Λ hub node (large, prominent)
4. Regime color bar (green/amber/red)
5. κ method diamond with two paths
6. a_ISO calculation node
7. L_10m output node (highlighted)
8. S_λ side branch with risk classification

--------------------------------------------------
Text elements (14):

1. "h_eff" location: top node
2. "Film Decay h(t)" location: optional dashed block
3. "σ roughness" location: roughness node
4. "Λ = h_min / σ" location: central hub
5. "Full EHL / Mixed / Boundary" location: regime bar
6. "κ method?" location: diamond
7. "ViscosityRatio" location: left path from diamond
8. "FilmThicknessRatio" location: right path from diamond
9. "a_ISO" location: life factor node
10. "e_c contamination" location: a_ISO input
11. "L_10m" location: final output (highlighted)
12. "S_λ = Λ_min / Λ_perm" location: side branch
13. "Safe / Marginal / AtRisk" location: micropitting risk
14. "Grease only" location: Film Decay annotation

==================================================
[FORBIDDEN ELEMENTS - DO NOT RENDER IN IMAGE]
==================================================

Forbidden Elements:
- Header/title, non-white background, Korean text, dense paragraphs
- More than 14 text elements, full sentences, font specs

==================================================

Final output goal:
- Λ as the central hub visible immediately
- Two output paths (life + micropitting) clearly distinguished
- Film Decay as an optional grease-specific insertion
- Traffic-light risk classification intuitive
- 3:4 portrait
