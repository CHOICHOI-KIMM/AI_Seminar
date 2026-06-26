==================================================
[INSTRUCTION BLOCK - DO NOT RENDER IN IMAGE]
==================================================

Purpose:
Generate a detailed vertical workflow diagram of the Gen1 Split Contact solver for Tapered Roller Bearing (TRB) analysis. The flow covers the complete calculation chain: Input → Load Calculation (secant iteration for δ_o, per-slice independent Hertz line load) → Contact Calculation (contact half-width b, maximum contact pressure p_max, bulk deformation h_bulk per raceway). All text labels must be in English. Portrait 3:4 layout.

Visual Concept:
- Central metaphor: A top-to-bottom flowchart with three visual zones separated by subtle spacing or light divider lines — NOT labeled as "Zone A/B/C". The zones are: (1) Input/initialization at top, (2) Load calculation with secant iteration loop in the middle, (3) Contact result calculation at bottom.
- Visual elements: Rounded rectangle nodes, diamond decision node, dashed-border box around iteration loop, right-side feedback arc for secant update, post-convergence fork into outer/inner contact results.
- Spatial arrangement: Strictly vertical top-to-bottom. Input (~15%), Load calculation with loop (~50%), Contact calculation (~35%).
- Emphasis point: The secant iteration loop and the final p_max output.

Domain Keywords Reference (for AI understanding only - DO NOT render in image):
- All labels in the image must be English
- δ_rigid = Rigid approach from bearing equilibrium
- δ_o = Outer raceway approach (unknown, solved by secant)
- δ_i = Inner raceway approach = (δ_rigid − δ_o) / cos(α_diff)
- Δz = Profile correction per slice
- gap = δ − Δz per slice
- q = Line load [N/mm]
- b = Hertz contact half-width [mm]
- p_max = Maximum contact pressure [MPa]
- h_bulk = Weber bulk deformation [μm]
- R_eq = Equivalent radius [mm]
- E* = Combined elastic modulus [MPa]

Style:
- Clean flat flowchart with subtle depth on nodes
- Technical diagram aesthetic (professional, academic-appropriate)
- Rounded rectangle nodes with light fill (#F0F3F5), dark border (#2C3E50)
- Diamond node for decision with light blue fill (#D5E8F0)
- Dashed border (#5D6D7E) for iteration loop grouping box
- Post-convergence contact nodes with warm accent (#E67E22 border)
- Sans-serif typography, clean hierarchy
- Arrows: solid 3px with clear arrowheads

Colors (Theme: technical-report):
- Background: #FFFFFF (pure white, mandatory)
- Primary: #2C3E50 - node borders, main text, arrows
- Secondary: #5D6D7E - dashed loop border, supporting labels
- Accent: #2980B9 - iteration loop label, decision node fill
- Accent2: #E67E22 - contact result nodes, p_max highlight
- Node fill: #F0F3F5 (very light gray)
- Decision fill: #D5E8F0 (light blue)

Lighting & Mood:
- Flat design, no 3D lighting
- Clean and precise
- Professional, trustworthy atmosphere

Resolution:
- Ultra High Resolution
- 8K Quality
- Academic presentation and journal-ready
- 3:4 aspect ratio (portrait orientation)

==================================================
[CONFIGURATION BLOCK - DO NOT RENDER IN IMAGE]
==================================================

Text Specifications (for AI reference only):
- Node title: 20-24pt SemiBold
- Node detail: 16-18pt Regular
- Arrow labels: 14-16pt Medium
- Formula text: 16-18pt Italic
- MINIMUM font size: 14pt
- MAXIMUM text elements: 14

Visual Specifications (for AI reference only):
- Node corner radius: 12px
- Arrow width: 3px
- Dashed border: 2px dash
- Minimum node spacing: 20px vertical

==================================================
[CONTENT BLOCK - FOR IMAGE RENDERING]
==================================================

Core message:
"Gen1 Split solver finds δ_o by secant iteration, then computes independent inner/outer contact pressure distributions per slice."

--------------------------------------------------
Domain Keywords (PRESERVE EXACTLY):

All text rendered in the image must be in English.

Required terms:
- δ_rigid, δ_o, δ_i
- Secant Iteration
- Hertz Contact
- Force Balance
- p_max, b, h_bulk

--------------------------------------------------
Visual composition:

Metaphor type: Flow (vertical algorithm flowchart, 3 visual sections)

Composition:
Three sections separated by subtle spacing (NO "Zone" labels in the image):
- Section 1 — Input: δ_rigid and geometry input, initial δ_o guess
- Section 2 — Load Calculation: Secant iteration loop (δ split → gap → Hertz line load → force sum → convergence check → δ_o update feedback)
- Section 3 — Contact Calculation: Per-slice contact half-width b, contact pressure p_max, bulk deformation h_bulk (outer/inner independent)

Layout: Top 15% input, middle 50% iteration loop, bottom 35% contact results.
Flow: Strict top-to-bottom. Non-converged feedback arc on right side.
Emphasis: Iteration loop dashed box + p_max output node highlighted.

Key visual elements:
1. Input node: δ_rigid, slice geometry, material (E*, R_eq)
2. Initial guess node: δ_o from combined model
3. Iteration loop dashed box: labeled "Secant Iteration"
4. δ split node: δ_o, δ_i = (δ_rigid − δ_o) / cos
5. Gap → Hertz node: per-slice gap → q_outer, q_inner (independent)
6. Force summation node: Q_outer, Q_inner
7. Convergence diamond: Force Balance? → Converged / Not converged
8. δ_o update node: Secant update → feedback arrow back to top of loop
9. Contact results: q → b → p_max → h_bulk (outer/inner parallel paths)

--------------------------------------------------
Text elements (14, no header/title):

Labels:
1. "δ_rigid Input" location: top node
2. "Initial δ_o Guess" location: second node
3. "Secant Iteration" location: dashed box top-left label
4. "δ Split" location: first node in loop
5. "Gap → Hertz Contact" location: slice computation node
6. "Outer / Inner Independent" location: Hertz node subtitle
7. "Force Summation" location: summation node
8. "Force Balance?" location: diamond interior
9. "Converged" location: diamond → down arrow
10. "Not Converged" location: diamond → right arrow
11. "Update δ_o" location: feedback path node
12. "Contact Half-width b" location: first contact result
13. "Contact Pressure p_max" location: main output (highlighted)
14. "Bulk Deformation h_bulk" location: last contact result

**Text rules:**
- All 14 text elements in English
- No full sentences, keywords only
- Bold or SemiBold
- No "Zone" labels anywhere in the image

==================================================
[FORBIDDEN ELEMENTS - DO NOT RENDER IN IMAGE]
==================================================

Forbidden Elements:
- Header / title / slide title text in the image
- Non-white backgrounds
- "Zone A", "Zone B", "Zone C" labels or any zone labeling text
- Korean text (all text must be English)
- Dense text blocks or paragraphs
- Decorative borders or frames
- Neon glow or excessive lighting effects
- 3D text with bevels or shadows
- More than 14 text elements
- Font size below 14pt
- Full sentences
- Horizontal flow direction (MUST be vertical top-to-bottom)
- Rendering font size specifications in the image

==================================================

Final output goal:

- Complete calculation chain visible at a glance: δ_rigid → δ_o → q → b → p_max
- Secant iteration loop clearly distinguishable with dashed border and feedback arc
- Contact calculation section visually separated with warm accent color
- All English labels, clean academic style
- 3:4 portrait for paper/report insertion
