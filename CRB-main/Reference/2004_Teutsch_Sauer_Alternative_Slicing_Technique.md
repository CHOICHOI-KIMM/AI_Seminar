# An Alternative Slicing Technique to Consider Pressure Concentrations in Non-Hertzian Line Contacts

**Authors:** Roman Teutsch, Bernd Sauer  
**Affiliation:** The University of Kaiserslautern, Institute of Machine Elements, Gears and Transmissions, Gottlieb-Daimler-Strasse, 67663 Kaiserslautern, Germany  
**Published:** ASME Journal of Tribology, Vol. 126, July 2004  
**DOI:** 10.1115/1.1739244

---

## Abstract

A new, fast method is presented for the analysis of roller-race contact in roller bearings. Based on a theoretical and implicit load-deflection relationship, an improved slicing technique is developed which accounts for a more accurate representation of the pressure distribution along the line of contact. For validation, the method is compared to literature data and results obtained by FEM analysis. In the course of this, roller profiling and misalignment are considered. Due to its fastness and accuracy, the method has its particular advantages when many contacts have to be evaluated several times, e.g., in static load distribution calculations and dynamic simulations.

---

## Introduction

After Heinrich Hertz had published his work "On the Contact of Elastic Solids" [1], contact problems were only divided in Hertzian and non-Hertzian cases. In the point contact case, Hertz gave an analytical solution for the load-deflection relationship. In the line contact case, no such relationship was proposed by him. Modern rolling bearing calculations, however, require that the compression of the rolling elements and raceways can be calculated from known loading conditions and vice versa.

Over the years, many authors have addressed non-Hertzian contact problems involving cylindrically shaped bodies of finite length. Half-space theory, experiments, and more recently, numerical schemes have been applied to evaluate load, deflection, pressure and stresses arising in a single contact. Only few authors, however, have developed explicit load-deflection relationships that can be used without time-consuming iteration.

In order to consider misalignment in line contact problems, so-called slicing techniques have frequently been used. The roller is sliced up into several laminae, each of which contributes to the roller stiffness according to its length. Each slice is usually treated separately, having no effect on the contact load or deflection of others. Therefore, pressure concentrations cannot be taken into account. If a more accurate pressure distribution is required, time-consuming calculations in a more involved single contact model have to be performed.

---

## Non-Hertzian Line Contact

### Theoretical and Experimental Studies

From the work of Hertz, relationships are available to calculate the maximum contact pressure $p_0$ and the semi-width $b$ of the contact area in a line contact [2]:

$$b = \sqrt{\frac{8 \cdot (1 - \nu^2) \cdot Q}{\pi \cdot E \cdot \Sigma\rho \cdot L}}$$

$$p_0 = \frac{2}{\pi} \cdot \frac{Q}{b \cdot L}$$

$$\Sigma\rho = \frac{1}{R_1} \pm \frac{1}{R_2} \tag{1}$$

In the latter expression the positive sign is used for convex-convex, the negative sign is used for convex-concave contacts, respectively. No exact and explicit relationship is available for expressing the load $Q$ as a function of the deflection $\delta$. Many authors describe, however, how the deflection may be calculated from known loading conditions. At least some give approximations for calculating $Q$ from a known deflection $\delta$, often taken as the theoretical interpenetration of the rigid bodies.

In 1939, **Lundberg** [3] published an analytical solution for calculating the deflection of two cylinders in contact. He assumed that their cylindrical shapes merge into half-spaces outside the contact region and that the contact pressure is constant along the cylinder's axes. To hold the latter assumption, he found that the contours of the cylinders need to follow a certain profile to avoid pressure concentrations towards the ends. The relationship he gave for the approach of the cylinder axes under a certain load was based on this profile.

In 1940, **Kowalsky** considered the deformation of a cylinder of finite length loaded from two sides by an elliptically distributed pressure [4].

Earlier than Kowalsky, **Dinnik** had found a similar expression but for a parabolic distribution of pressure along the width of contact [4]. His proposal is also given in [5] for the calculation of the deflection in a cylindrical contact.

In 1959, **Palmgren** [6] published his book, "Ball and Roller Bearing Engineering." From experiments with an un-profiled roller compressed between two plates, he found a simple expression relating the deflection $\delta$ to the load $Q$. He mentioned that the deflection was not dependent on the roller diameter and that his results could be used with only small error for the load calculation in roller bearings.

In 1961, **Kunert** [7] proved Lundberg's equation when he was concerned with the stress distribution in a half-space loaded by an elliptically distributed pressure over a rectangular contact area. He extended the theory to the case of a cylinder contacting a plane half-space. He stated that an evenly distributed pressure along the cylinder axis could not be achieved under these conditions. His corrected roller profile took stress concentrations below the surface into account. In the end he gave an approximate expression for the load-deflection relationship in a line contact. Since a unique optimum profile is assumed for every loading condition, it is, however, of restricted practical use.

In 2001, **Houpert** [8] compared expressions due to Palmgren, Kunert, and Zantopolus with equations given by Tripp in 1985, the best reference from his point of view. After introducing the outer-race-plus-housing-section-thickness as the appropriate depth of reference at which the deformation can be assumed to be zero, Houpert made approximate curve fits to the relationships given by Tripp. According to Houpert, his two simple equations are valid for roller-race type contacts in roller bearings.

### Table 1: Deflection-Load Relationships for Inner and Outer-Race-to-Roller-Contact

**Lundberg [3]:**

$$\delta_i = \delta_o = \frac{Q}{\pi} \cdot \frac{2}{L} \cdot \left(\frac{1-\nu_1^2}{E_1} + \frac{1-\nu_2^2}{E_2}\right) \cdot \left(1.8864 + \ln\frac{L}{2 \cdot b}\right) \tag{2}$$

**Dinnik [4]:**

$$\delta_i = \left(\frac{2 \cdot Q}{\pi \cdot L}\right) \cdot \left[\frac{(1-\nu_1^2)}{E_1}\left(\ln\frac{2 \cdot R_1}{b} + \frac{1}{3}\right) + \frac{(1-\nu_2^2)}{E_2}\left(\ln\frac{2 \cdot R_2}{b} + \frac{1}{3}\right)\right] \tag{3}$$

$$\delta_o = \left(\frac{2 \cdot Q}{\pi \cdot L}\right) \cdot \left[\frac{(1-\nu_1^2)}{E_1}\left(\ln\frac{2 \cdot R_1}{b} + \frac{1}{3}\right) + \frac{(1-\nu_2^2)}{E_2}\left(\ln\frac{t}{b} + \frac{1}{3}\right)\right]$$

**Kowalsky [4]:**

$$\delta_i = \left(\frac{2 \cdot Q}{\pi \cdot L}\right) \cdot \left[\frac{(1-\nu_1^2)}{E_1}\left(\ln\frac{2 \cdot R_1}{b} + 0.407\right) + \frac{(1-\nu_2^2)}{E_2}\left(\ln\frac{2 \cdot R_2}{b} + 0.407\right)\right] \tag{4}$$

$$\delta_o = \left(\frac{2 \cdot Q}{\pi \cdot L}\right) \cdot \left[\frac{(1-\nu_1^2)}{E_1}\left(\ln\frac{2 \cdot R_1}{b} + 0.407\right) + \frac{(1-\nu_2^2)}{E_2}\left(\ln\frac{t}{b} + 0.407\right)\right]$$

**Palmgren [6]:**

$$\delta_i = \delta_o = 3.84 \times 10^{-5} \cdot \frac{Q^{0.9}}{L^{0.8}} \tag{5}$$

**Kunert [7]:**

$$\delta_i = \delta_o = 4.05 \times 10^{-5} \cdot \frac{Q^{0.925}}{L^{0.85}} \tag{6}$$

**Tripp [8]:**

$$\delta_i = \left(\frac{2 \cdot Q}{\pi \cdot L}\right) \cdot \left[\frac{(1-\nu_1^2)}{E_1}\left(\ln\frac{4 \cdot R_1}{b} - \frac{1}{2}\right) + \frac{(1-\nu_2^2)}{E_2}\left(\ln\frac{4 \cdot R_2}{b} - \frac{1}{2}\right)\right] \tag{7}$$

$$\delta_o = \left(\frac{2 \cdot Q}{\pi \cdot L}\right) \cdot \left[\frac{(1-\nu_1^2)}{E_1}\left(\ln\frac{4 \cdot R_1}{b} - \frac{1}{2}\right) + \frac{(1-\nu_2^2)}{E_2}\left(\ln\frac{2 \cdot t}{b} - \frac{\nu_2}{2 \cdot (1-\nu_2)}\right)\right]$$

**Houpert [8]:**

$$\delta_i = \left[\frac{Q \cdot (1-\nu^2)}{0.2723 \cdot E \cdot L} \cdot d_m^{0.074}\right]^{1/1.074} \tag{8}$$

$$\delta_o = \left[\frac{Q \cdot (1-\nu^2)}{0.27835 \cdot E \cdot L} \cdot \left(\frac{t}{1 + D/d_m}\right)^{0.078}\right]^{1/1.078}$$

### Case Study Parameters (from [8])

- $D = 20$ mm, $L = 30$ mm, $d_m = 60$ mm, $t = 6$ mm
- $E = 2.031 \times 10^5$ MPa, $\nu = 0.3$

In case of the inner race contact, most of the load-deflection relationships compare fairly well among each other. Only Lundberg's and Kunert's results deviate noticeably. Since both authors assume an optimum roller profile, this is not surprising though.

In case of the outer race contact, Palmgren's results, too, deviate greatly from the ones obtained using the expressions of others. The load-deflection relationships due to Tripp, Dinnik, and Kowalsky are nearly identical. Houpert's curve fit deviates slightly in the highly loaded region.

It is important to note, that only the equations given by Palmgren, Kunert, and Houpert can be explicitly solved for $Q$ from a known deflection $\delta$. In the equations due to Lundberg, Tripp, Dinnik, and Kowalsky, the deflection is dependent on the load $Q$ and the semi-width $b$, which, according to Eqs. (1), also depends on $Q$. Consequently, the latter expressions can only be solved for $Q$ by time-consuming iterative techniques; a great disadvantage as far as the analysis of rolling bearings is concerned.

It is also important to note, that Palmgren's and Kunert's expressions are independent from the roller radius. The implicit equations of Tripp, Dinnik, and Kowalsky, however, show a weak dependency of the load-deflection relationship on the curvature of the contacting bodies. Only the curve-fit due to Houpert incorporates such geometrically based adjustment terms and is explicitly solvable for $Q$ without costly iteration at the same time.

### Numerical Approaches

As for some of the theories mentioned above, Boussinesq's half-space theory has frequently been adopted in numerical approaches. In general, the condition of contact reads as follows:

$$\delta_1 + \delta_2 + f_1 + f_2 - \alpha \geq 0 \tag{9}$$

where $\delta_1$ and $\delta_2$ are the elastic deformations and $f_1$ and $f_2$ are functions describing the geometry, i.e., the initial gap to an assumed reference plane for body 1 and 2, respectively. $\alpha$ is taken as the rigid body approach of the two bodies. To satisfy the contact condition, Eq. (9) must be equal to zero within the contact area and greater than zero outside. The contact pressure must be zero outside the contact region and positive within. With reference to aforementioned half-space theory, the elastic deformations can be expressed by the following equation:

$$\delta_1 + \delta_2 = \left(\frac{1-\nu_1^2}{\pi \cdot E_1} + \frac{1-\nu_2^2}{\pi \cdot E_2}\right) \cdot \iint_{\Omega} \frac{p(x', y') \cdot dx' \cdot dy'}{\sqrt{(x-x')^2 + (y-y')^2}} \tag{10}$$

By subdividing the contact area in a number of discrete cells, the integral in (10) can be approximated by the sum of the pressure acting on each cell times the cell area, divided by the distance of one cell reference point to another. The resulting set of equations can be solved by some numerical means.

In 1971, **Conry and Seirig** [10] used a simplex algorithm to solve the problem. Instead of pressures, they adopted forces acting at discrete points within the contact area.

When solving the problem iteratively, **Singh and Paul** [11] (1974) encountered that the solutions accuracy was very much dependent on the discretization. The application of a finer grid with smaller cells partly produced physically meaningless results. In an effort of canceling out random errors, they applied an averaging technique, which they called the *Method of Redundant Field Points*.

In 1977, **Reusner** [12] extended the half-space theory to the case where the contacting bodies were both limited in the longitudinal direction. By mirroring the contact pressure about the end planes of the contacting bodies, he managed to cancel out shear stresses in these planes.

**Nayak and Johnson** [13] presented a paper in 1978, in which they assumed a piecewise linear pressure distribution over a number of strips subdividing a slender area of contact.

In 1980, **Hartnett** [14] partly integrated Eq. (10) by assuming a constant pressure distribution over each rectangular cell.

**Harris** [15] (1984) described a slicing technique to account for misalignment in cylindrical contacts. Each slice of the roller was given a linearized fraction of the total roller stiffness calculated from Palmgren's equation [6].

**De Mul, Kalker, and Fredriksson** [9] reported an improved half-space model in 1986. In this, they added correction terms due to Reusner [12] for the finite length and depth of the contacting bodies. Their results compared well to their own FEM evidence and to data published by others.

A common advantage of all reported half-space models is their fairly high accuracy and the fastness of the solution process when compared to analogous FE models. Compared to simpler models, similar to the one proposed by Harris [15], however, computation time is still excessive. Therefore, the use of some slicing technique can be advantageous when many contacts have to be solved simultaneously, even though the solution's accuracy may be reduced greatly. For that reason, the slicing method is improved in this paper to account for a more exact load and pressure distribution along the line of contact.

---

## Alternative Slicing Technique (AST)

### Deflection-Load Relationship

As shown above, the equations given by Tripp, Kowalsky, and Dinnik are nearly equivalent within a wide range. From all compared relationships, their approaches are considered best. The implicit nature of their expressions, however, is disadvantageous.

In order to calculate the load from a given deflection without the need of time-consuming iteration, simpler deflection-load relationships are required. Since Houpert's curve-fits deviate slightly from the original, new curves were fitted in a similar way as described by him in [8]. For this, Dinnik's instead of Tripp's equation was taken as a basis, leading to the following deflection-load relationships for inner and outer race, respectively:

$$\delta_i = 3.17 \cdot \left(\frac{d_m}{2}\right)^{0.08} \cdot \left(\frac{Q \cdot (1-\nu^2)}{E \cdot L}\right)^{0.92} \tag{11}$$

$$\delta_o = 2.66 \cdot \left(\frac{t}{1 + D/d_m}\right)^{0.09} \cdot \left(\frac{Q \cdot (1-\nu^2)}{E \cdot L}\right)^{0.91}$$

In case of a roller contacting a flat plate, the latter expression in (11) simplifies to:

$$\delta_{rp} = 2.66 \cdot (t)^{0.09} \cdot \left(\frac{Q \cdot (1-\nu^2)}{E \cdot L}\right)^{0.91} \tag{12}$$

It should be noted that this equation is independent of the roller diameter, a fact that confirms Palmgren's results. It is, however, still dependent on the thickness of the plate, which Palmgren did not consider.

### FE Validation

An FE model of an un-profiled roller contacting a flat plate was set up. Due to symmetry, only 1/8 of the roller and 1/4 of the plate had to be modeled. Parameters:
- $D = L = 10$ mm
- $E = 2.08 \times 10^5$ MPa, $\nu = 0.3$
- $t/D = 4 \ldots 0.5$
- $\alpha = \delta = 0.01$ mm

Figure 5 indicates that the deflection calculated from Eq. (12) is closest to the FE results, with a maximum relative error of less than 1.5 percent.

### Influence Coefficient Matrix

In our alternative model, the roller is sliced up into $n$ slices of equal length $l$. For each slice $j$, Eqs. (11) can be written as:

$$\delta_{i,j}^{1/0.92} = \frac{c_i^{1/0.92}}{l} \cdot q_j \tag{13}$$

$$\delta_{o,j}^{1/0.91} = \frac{c_o^{1/0.91}}{l} \cdot q_j$$

in which:

$$c_i = 3.17 \cdot \left(\frac{d_m}{2}\right)^{0.08} \cdot \left(\frac{(1-\nu^2)}{E}\right)^{0.92} \tag{14}$$

$$c_o = 2.66 \cdot \left(\frac{t}{1 + D/d_m}\right)^{0.09} \cdot \left(\frac{(1-\nu^2)}{E}\right)^{0.91}$$

and:

$$l = \frac{L}{n} \tag{15}$$

The expressions:

$$\frac{c_i^{1/0.92}}{l} = s_i \tag{16}$$

$$\frac{c_o^{1/0.91}}{l} = s_o$$

are the elastic compliances associated with each slice in contact with either inner or outer race.

So far, we have followed the common process applied in simpler slicing techniques, where all slices are treated independently. Making a similar assumption as in the half-space theory, however, a force at slice $j$ also contributes to the deformation in a distant slice $k$. For each slice, different weighting functions can be defined to account for this influence. The results are two systems of linear equations for inner and outer race, respectively, which can be written in matrix form as follows:

$$[S_w]_i \cdot \{q\}_i = \{\Delta\}_i \tag{17}$$

$$[S_w]_o \cdot \{q\}_o = \{\Delta\}_o$$

In each of them, $[S_w]$ is the matrix of weighted influence coefficients, normalized by the mean value of all weighting functions:

$$[S_w] = \frac{n}{\sum_{j,k} w_{j,k}} \begin{bmatrix} s \cdot w_{j,k} & \cdots & s \cdot w_{j,n} \\ \vdots & \ddots & \vdots \\ s \cdot w_{n,k} & \cdots & s \cdot w_{n,n} \end{bmatrix} \quad \text{for } j = 1 \ldots n,\ k = 1 \ldots n \tag{18}$$

Note that the influence coefficient matrix is symmetric, meaning that:

$$w_{j,k} = w_{k,j} \tag{19}$$

$\{\Delta\}$ is the vector of deflections measured for each slice. The deflection is taken as the theoretical interpenetration of the rigid contact partners:

$$\{\Delta\} = \begin{Bmatrix} \delta_j^{1/ex} \\ \vdots \\ \delta_n^{1/ex} \end{Bmatrix}, \quad \text{for } j = 1 \ldots n \tag{20}$$

where $ex = 0.92$ for inner and $ex = 0.91$ for outer race contact, respectively.

$\{q\}$ is the vector of unknown forces:

$$\{q\} = \begin{Bmatrix} q_j \\ \vdots \\ q_n \end{Bmatrix}, \quad \text{for } j = 1 \ldots n \tag{21}$$

Singh and Paul [11] suggested, that to a first approximation, the influence of a unit pressure at one cell of their model on the deflection of a distant cell decreases by $1/r$, where $r$ is taken as the distance between the two cell reference points. Taking this idea, the influence coefficients might be calculated from the following expression:

$$w_{j,k} = \left(\frac{1}{r_{j,k}}\right)^{1/ex} \quad \text{for } j \neq k \tag{22}$$

where $r_{j,k}$ is the distance between slice $j$ and slice $k$ with respect to respective mid-planes. Influence coefficients for cases where $j = k$ are approximated by the following expression, which might be interpreted as the influence of two half slices on the mid-plane of the original slice:

$$w_{j,k} = \left(\frac{4}{l}\right)^{1/ex} \quad \text{for } j = k \tag{23}$$

The sets of linear equations may be solved by Gaussian elimination or other standard numerical means. Note that iterations are only necessary if one slice is shifted out of contact by the action of forces in neighboring slices. Misalignment and profiling of rollers and/or raceways are taken into account by a different initial interpenetration calculated for each slice.

---

## Discussion of Results

### Comparison With FEM

Figure 6 shows a comparison of total contact force between our alternative slicing technique and aforementioned FE model for a rigid body approach of $\alpha = \delta = 0.01$ mm and $t/D = 1$. The no. of slices used for the AST approach is varied from 10 to 1000.

It can be observed that the contact forces calculated with only few slices slightly over-estimate values calculated from the simple curve-fit to Dinnik's deflection-load relationship. They are in very good agreement, however, with the values calculated from the FE model. If more slices are used, the values obtained from the alternative model slowly converge towards the Dinnik curve-fit.

In Fig. 7 a comparison of the peak pressures along the line of contact for $t/D = 2$ is presented. Both the FE model and our alternative slicing technique show the typical pressure concentrations towards the ends of the unprofiled roller. With a number of 100 slices, both methods agree fairly well, but with a number of only 30 slices, the estimated pressure values are already reasonable.

It should be noted that our model predicts a more "barreled" pressure distribution; a fact that was also observed with the model of de Mul, Kalker, and Fredriksson in [9].

Even on a low-performance PC (Intel Pentium 200 mmx, 96 MB RAM), only fractions of a second were needed for the calculation of a single contact with applied numbers of slices ranging between 30 and 100.

### Application to Profiled Rollers

The roller was profiled according to Lundberg's suggestion [3]:

$$f_1(x) + f_2(x) = \left(\frac{1-\nu_1^2}{E_1} + \frac{1-\nu_2^2}{E_2}\right) \cdot \frac{Q}{L} \cdot f(2x/L) \tag{24}$$

where:

$$f(2x/L) = \frac{1}{\pi} \cdot \ln\frac{1}{1 - \left(\frac{2 \cdot x}{L}\right)^2} \tag{25}$$

Obviously, the latter equation is only valid in the range $0 \leq |2x/L| < 1$. Our model, however, uses slices of a certain length (thickness), referenced in the mid plane of the slice. Therefore, the case that $|2x/L| = 1$ cannot occur by definition, i.e., the equation is unconditionally valid in our case.

Figure 8 compares the pressure distribution along the roller axis of a profiled and an un-profiled roller ($t/D = 500$). The peak pressures at both ends of the roller are much reduced for the profiled case. Even though the overall pressure is more evenly distributed, small pressure peaks are still existent. This is in accordance with the findings of Kunert [7], who stated that for the case of a cylindrically shaped body contacting a flat half-space, no optimum profile can be found.

Figure 9 shows the pressure distribution for a reduced plate thickness ($t/D = 10$). The profile was more tapered so that the contact pressure was reduced towards the ends of the roller. Therefore, it can be concluded that the calculation of an optimum roller profile should involve both the loading conditions and some reference thicknesses of the contacting parts.

### Application to Misaligned Rollers

The technique was applied to a case study discussed in reference [9]. De Mul et al. compared their half-space model to the experimental work of Kannel and Hartnett carried out on a two-disk test rig. For comparison, the same contact load of $Q = 2224$ N was assumed.

Figure 10 shows the pressure distribution for the case when no misalignment is present. The slightly "barreled" shape in the middle region, mentioned by de Mul et al. in [9], can also be observed.

In Fig. 11, the pressure distribution for a tilting angle of $\theta = 0.86$ mrad $\approx 0.05°$ is compared with the results of de Mul et al. [9]. The results of our model agree well with the values taken from the reference. The very high pressure peak at the end of the disk, however, could not completely be confirmed.

---

## Conclusions

1. An alternative slicing technique has been proposed which accounts for a more exact pressure distribution in a line contact.

2. Two simple deflection-load relationships were developed by curve-fitting a well-known implicit expression. One can be applied to inner race, the other to outer race type contacts in a roller bearing. Results compared well to values obtained from a simplified FE model.

3. The introduction of weighting functions to consider the influence of a force in one slice on the deflection of another lead to linear systems of equations which were solved by a standard Gaussian elimination procedure.

4. The comparison with the FE model showed very good agreement but a slightly more "barreled" shape of the pressure along the line of contact.

5. The profiling of the roller confirmed the suitability of the technique. The optimum profile was found to be dependent on the contact load and the thicknesses of the contacting bodies.

6. The method can also be adopted in cases where misalignment is present.

7. Due to its fastness the proposed method has its particular advantages when several contacts have to be evaluated simultaneously and where a great number of iterations are needed (e.g., in dynamic applications).

---

## Nomenclature

| Symbol | Description |
|--------|-------------|
| $b$ | Semi-width of contact area |
| $c$ | Constant |
| $d_m$ | Bearing pitch diameter |
| $ex$ | Exponent |
| $f$ | Function |
| $l$ | Length of one slice [mm] |
| $n$ | Number of slices |
| $p_0$ | Maximum contact pressure |
| $q$ | Load per slice |
| $\{q\}$ | Vector of slice loads |
| $s$ | Elastic compliance per slice |
| $t$ | Raceway + housing/plate thickness |
| $w$ | Weighting function |
| $x$ | Contact area longitudinal direction |
| $y$ | Contact area transverse direction |
| $D$ | Roller diameter |
| $E$ | Modulus of elasticity |
| $L$ | Total length of contact area |
| $Q$ | Total contact load |
| $R$ | Radius of curvature |
| $[S_w]$ | Matrix of weighted influence coefficients |
| $\alpha$ | Rigid body approach |
| $\delta$ | Contact deflection/interpenetration |
| $\nu$ | Poisson's ratio |
| $\theta$ | Misalignment angle |
| $\Sigma\rho$ | Sum of curvature of contacting bodies |
| $\{\Delta\}$ | Vector of contact deflections |
| $\Omega$ | Contact area |

### Indices

| Index | Description |
|-------|-------------|
| 1 | Rolling element |
| 2 | Raceway/plate |
| $i$ | Inner race |
| $j, k$ | Row, column index |
| $o$ | Outer race |
| $r$ | Distance between slices |
| $rp$ | Roller-plate |

---

## References

1. Hertz, H., 1882, "Über die Berührung fester elastischer Körper," *Journal für reine und angewandte Mathematik*, 92, pp. 156–171.
2. Brändlein, J., Eschmann, P., Hasbargen, L., and Weigand, K., 1995, *Die Wälzlagerpraxis*, 3rd ed., Vereinigte Fachverlage GmbH.
3. Lundberg, G., 1939, "Elastische Berührung zweier Halbräume," *Forschung auf dem Gebiete des Ingenieurwesens*, 10(5), pp. 201–211.
4. Rothbart, H. A., 1985, *Mechanical Design and Systems Handbook*, 2nd ed., McGraw-Hill.
5. Young, W. C., and Roark, J. R., 1989, *Roark's Formulas for Stress and Strain*, 6th ed., McGraw-Hill Professional.
6. Palmgren, A., 1959, *Grundlagen der Wälzlagertechnik*, 2nd ed., Franckh'sche Verlagshandlung, W. Keller & Co.
7. Kunert, K., 1961, "Spannungsverteilung im Halbraum bei elliptischer Flächenpressungsverteilung über einer rechteckigen Druckfläche," *Forschung auf dem Gebiete des Ingenieurwesens*, 27(6), pp. 165–174.
8. Houpert, L., 2001, "An Engineering Approach to Hertzian Contact Elasticity—Part I," *ASME J. Tribol.*, 123, pp. 582–588.
9. De Mul, J. M., Kalker, J. J., and Fredriksson, B., 1986, "The Contact Between Arbitrarily Curved Bodies of Finite Dimensions," *ASME J. Tribol.*, 108, pp. 140–148.
10. Conry, T. F., and Seirig, A., 1971, "A Mathematical Programming Method for Design of Elastic Bodies in Contact," *ASME J. Appl. Mech.*, pp. 387–392.
11. Singh, K. P., and Paul, B., 1974, "Numerical Solution of Non-Hertzian Elastic Contact Problems," *ASME J. Appl. Mech.*, pp. 484–490.
12. Reusner, H., 1977, "Druckflächenbelastung und Oberflächenverschiebung im Wälzkontakt von Rotationskörpern," Dissertation, University of Karlsruhe, Germany.
13. Nayak, L., and Johnson, K. L., 1979, "Pressure Between Elastic Bodies Having a Slender Area of Contact and Arbitrary Profiles," *Int. J. Mech. Sci.*, 21, pp. 237–247.
14. Hartnett, M. J., 1980, "A General Numerical Solution For Elastic Body Contact Problems," *ASME J. Appl. Mech.*, 39, pp. 51–66.
15. Harris, T. A., 1984, *Rolling Bearing Analysis*, 2nd ed., John Wiley & Sons, Inc.
