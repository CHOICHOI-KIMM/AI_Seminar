# Hydrodynamic force and moment in pure rolling lubricated contacts. Part 2: point contacts

N Biboulet\* and L Houpert

TIMKEN Europe, Colmar, France

The manuscript was received on 25 January 2010 and was accepted after revision for publication on 31 March 2010.

DOI: 10.1243/13506501JET791

**Abstract:** Hydrodynamic rolling force and moments in point contact have been studied in detail using isoviscousrigid (IVR) and elastohydrodynamic (EHL) models. Using fully flooded assumptions, curve-fitted relationships are given for calculating the IVR and EHL hydrodynamic rolling forces. Both are proportional (or almost proportional in the IVR case) to 2a, the Hertzian contact length being perpendicular to the rolling direction, and are also functions of the dimensionless speed parameter. A single curve-fitted relationship has been derived to cover the full range of operating conditions with a smooth transition from IVR to EHL regime of lubrication. The results obtained are slightly higher than those previously published (the ratio being of the order of 1.5 for usual operating conditions).

Point contact and line contact (with a contact length  $\mathcal{L}$  being equal to the point contact length 2a) hydrodynamic rolling forces have also been compared. The point contact forces are about 26 per cent larger than those obtained using line contact relationship (published in part 1) because of a larger domain of integration in the lateral direction.

By limiting the width of the integration domain to  $\mathcal{L}$  (roller length or ball diameter), the effect of  $2a/\mathcal{L}$  on the hydrodynamic rolling force has been studied, leading to the derivation of a truncation factor  $\mathcal{C}$ . As the load increases, 2a increases and the truncation factor decreases until reaching a limit when ellipse truncation starts because  $2a/\mathcal{L}$  is equal to or larger than one. Using the truncation factor and limiting the  $2a/\mathcal{L}$  ratio to one, it was found that point contact and line contact hydrodynamic forces are the same within a few per cent. A single point contact relationship can therefore be suggested, covering the IVR to EHL operating conditions with a smooth transition between these lubrication regimes, and also a smooth transition from point contact to line contact as the load increases and contact ellipse truncation occurs.

Finally, calculations of power losses due to the Poiseuille flow in the rolling direction x and in the perpendicular direction z show that the power loss in the z direction is usually very small for wide elliptical contacts and that most of the power is dissipated in the inlet and outlet, with a 26 per cent contribution of the integration domain defined out the range -a < z < a. This result is in line with the truncation factor defined previously.

**Keywords:** rolling resistance, hydrodynamic rolling resistance, power losses, hydrodynamic rolling power losses, bearing torque, bearing power losses, race torque, point contact, elliptical contact, elastohydrodynamic, isoviscousrigid

#### 1 INTRODUCTION

This paper follows a previous study (part 1) that presents results regarding the hydrodynamic rolling

To the authors' knowledge, little has been published regarding calculating hydrodynamic rolling force in point contact. isoviscousridid (IVR) and piezoviscousrigid (PVR) relationships given by Houpert [1] have

force and moment, using line contact assumption.

email: nans.biboulet@yahoo.com

However, it is known that crown radii are used in roller bearings (on races and on rollers), so that roller bearings are subjected to elongated elliptical contacts especially before truncation occurs.

<sup>\*</sup>Corresponding author: TIMKEN Europe, 2 Rue Timken, Colmar 68000. France.

been obtained by curve-fitting Dalmaz's results [2] in a simplified manner. elastohydrodynamic (EHL) results have also been derived indirectly in reference [1] using a curve-fitting conducted by Tevaarwerk and Johnson [3] of the pressure centre shift (pressure shift calculated by Hamrock and Dowson [4]). An assumption was made in reference [1] that the product, hydrodynamic rolling force times the equivalent radius, is equal to the product, load times pressure shift. This assumption was confirmed for line contact in part 1.

Despite the question about the accuracy of previous relationships, many problems still remain – for example, the difficulty of ensuring a smooth transition (without discontinuity) between PVR and EHL results, as well as the problem of using point contact relationship when considering very elongated elliptical contact, the ratio between the two equivalent radii  $(R_z/R_x)$  being often of the order of 1000 to 10000 in roller bearings. Should then a point contact or a line contact relationship be used?

As the load increases, the elliptical contact length increases until reaching and exceeding the race width or effective roller length (causing ellipse length truncation). Are truncated point contact rolling forces then converging towards the line contact ones calculated in part 1? In ball bearings, contact ellipse truncations seldom occurs, but the race width limits the domain of integration of the Reynolds equation. Is it possible to account for a kind of truncation factor defined by the roller length?

The objectives of this study are, therefore, to revisit the calculation of the hydrodynamic rolling forces in point contact and to answer to all previously described questions.

# 2 EQUATIONS AND DEFINITIONS

## 2.1 Reynolds equation

The reduced radii of curvature are defined by

$$\frac{1}{R_x} = \frac{1}{R_{x1}} + \frac{1}{R_{x2}}$$

$$\frac{1}{R_z} = \frac{1}{R_{z1}} + \frac{1}{R_{z2}}$$
(1)

The reduced Young's modulus reads

$$\frac{2}{E'} = \frac{1 - \nu_1^2}{E_1} + \frac{1 - \nu_2^2}{E_2} \tag{2}$$

Two types of dimensionless parameters are used. The Hertzian dimensionless parameters are used for EHL calculations (large elastic deformation), see equation (3); the classical dimensionless parameters

are used for IVR calculations, see equation (4)

$$H = \frac{hR_x}{b^2}$$

$$X = \frac{x}{b}$$

$$Z = \frac{z}{b} \quad \text{for EHL}$$

$$P = \frac{p}{p_h}$$

$$\overline{R} = \frac{R_x}{b}$$

$$\tilde{H} = \frac{h}{R_x}$$

$$\tilde{X} = \frac{x}{R_x} \quad \text{for IVR}$$

$$\tilde{Z} = \frac{z}{\sqrt{R_x R_z}}$$

$$\tilde{P} = \frac{p}{E'}$$
(4)

The gap height between the two surfaces is defined by the summation of the solid approach  $h_0$ , the undeformed geometry  $h_{\rm S}(x,z)$ , and the elastic deformations. The undeformed film thickness can be derived analytically using a circular shape in the x and z directions. However, it is clear that the ratio  $z/R_z$  is very small in bearings, so that a parabolic approximation can be used along the z direction leading to

$$h(x,z) = h_0 + h_S(x,z) + \frac{2}{\pi E'} \iint_{\Omega} \frac{p(x',z') \, dx' \, dz'}{\sqrt{(x-x')^2 + (z-z')^2}} dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx' \, dx'$$

For EHL calculations, the parabolic approximation along x is also reasonable and allows us to reduce the number of dimensionless parameters. Equations (6) and (7) define the dimensionless gap height. The parameter  $\mathcal{E}$ , corresponding to an elliptical integral, is defined in Appendix 2

$$H(X,Z) = H_0 + H_S(X,Z) + \frac{1 + R_x/R_z}{2\pi\mathcal{E}} \iint_{\Omega} \frac{P(X',Z') dX' dZ'}{\sqrt{(X - X')^2 + (Z - Z')^2}}$$

$$(6)$$

$$H_S(X,Z) = \frac{X^2}{2} + \frac{R_x}{R_z} \frac{Z^2}{2}$$

$$\tilde{H}\left(\tilde{X},\tilde{Z}\right) = \tilde{H}_0 + \tilde{H}_S\left(\tilde{X},\tilde{Z}\right)$$

$$\tilde{H}_S\left(\tilde{X},\tilde{Z}\right) = 1 - \sqrt{1 - \tilde{X}^2 - \tilde{Z}^2}$$
(7)

The Reynolds equation is used

$$\frac{\partial}{\partial x} \left( \frac{\rho h^3}{\eta} \frac{\partial p}{\partial x} \right) + \frac{\partial}{\partial z} \left( \frac{\rho h^3}{\eta} \frac{\partial p}{\partial z} \right) = 12 \, u_{\rm m} \frac{\partial \rho h}{\partial x} \tag{8}$$

The dimensionless Reynolds equations read

$$\frac{\partial}{X} \left( \frac{\overline{\rho} H^3}{\overline{\eta} \overline{\lambda}} \frac{\partial P}{\partial X} \right) + \frac{\partial}{\partial Z} \left( \frac{\overline{\rho} H^3}{\overline{\eta} \overline{\lambda}} \frac{\partial P}{\partial Z} \right) = \frac{\partial \overline{\rho} H}{\partial X}$$
(9)

$$\frac{\partial}{\partial \tilde{X}} \left( \tilde{H}^3 \frac{\partial \tilde{P}}{\partial \tilde{X}} \right) + \frac{R_x}{R_z} \frac{\partial}{\partial \tilde{Z}} \left( \tilde{H}^3 \frac{\partial \tilde{P}}{\partial \tilde{Z}} \right) = 6 U \frac{\partial \tilde{H}}{\partial \tilde{X}}$$
(10)

For the EHL contacts, a Barus viscosity and a Dowson and Higginson [5] compressibility are used

$$\overline{\eta} = \frac{\eta}{\eta_0} = e^{\alpha p_h P} = e^{\overline{\alpha} P} \tag{11}$$

$$\overline{\rho} = \frac{\rho}{\rho_0} = \frac{0.59 \cdot 10^9 + 1.34 \cdot Pp_h}{0.59 \cdot 10^9 + Pp_h}$$
 (12)

The different dimensionless parameters defining the operating conditions in the Reynolds equation and the force balance equation are detailed below

$$\overline{\lambda} = \frac{12 u_{\rm m} \eta_0 R_{\rm x}^2}{b^3 p_{\rm h}} = \left(\frac{128}{3}\right)^{1/3} \frac{\pi}{M^{4/3}}$$

$$\overline{\alpha} = \alpha p_{\rm h} = \left(\frac{3 M}{2}\right)^{1/3} \frac{L}{\pi}$$

$$\frac{bE'}{\pi p_{\rm h} R_{\rm x}} = \frac{4}{\pi} \frac{\mathcal{E}}{1 + (R_{\rm x}/R_{\rm z})} = \beta$$

$$W = \frac{w}{E' R_{\rm x}^2} = \frac{2\pi p_{\rm h} b^2}{3 \kappa E' R_{\rm x}^2}$$

$$U = \frac{2\eta_0 u_m}{E' R_{\rm x}}$$

$$G = \alpha E'$$

$$M = \beta^{1/4} \frac{\kappa W}{U^{3/4}}$$

$$L = \beta^{-3/4} G U^{1/4}$$

Note that  $\beta$  varies in a limited range from 1 for circular contacts to  $4/\pi$  for wide elliptical contacts as a function of the ellipticity  $\kappa=b/a$ . When this parameter appears with a low exponent, it can be neglected or replaced by its elliptical asymptotic value without significant error. Note also that a factor 2 is present in

the definition of *U*. The vertical force balance reads

$$\iint_{\Omega} p(x, z) \, \mathrm{d}x \, \mathrm{d}z = w \tag{15}$$

$$\iint_{\Omega} P(X, Z) \, dX \, dZ = \frac{2\pi}{3\kappa} \tag{16}$$

$$\iint_{\Omega} \tilde{P}\left(\tilde{X}, \tilde{Z}\right) \, d\tilde{X} \, d\tilde{Z} = \sqrt{\frac{R_x}{R_z}} \, W \tag{17}$$

Multigrid techniques, see Venner and Lubrecht [6], are used to solve the presented sets of equations. Some curve-fitted equations for the elliptical integral and the ellipticity calculation are presented in Appendix 2.

## 2.2 Forces and moments

Similar types of dimensionless parameters as in part 1 for hydrodynamic forces F and moments T are defined below

$$F = \frac{f}{p_{\rm h}b^2} \tag{18}$$

$$T = \frac{t}{p_{\rm b}b^3} \tag{19}$$

$$\tilde{F} = \frac{f}{E'R_r^2} \tag{20}$$

$$\tilde{T} = \frac{t}{E'R_x^3} \tag{21}$$

The pressure generated in the lubricant film is normal to the surfaces. The pressure distribution is not symmetric along the rolling direction  $\boldsymbol{x}$  because of the cavitation. A pressure is generated in the contact inlet, whereas the outlet pressure drops to zero. Moreover, the viscous flow also creates a tangential stress along the surfaces. These two efforts (normal and tangential) lead to hydrodynamic forces and moments. These forces and moments are functions of the operating conditions. Perpendicularly to the rolling direction, along  $\boldsymbol{z}$ , the geometry and the pressure distribution is symmetric. Thus, no force or moment is generated. However, the lubricant shear due to the Poiseuille flow along  $\boldsymbol{z}$  leads to an additional power loss.

The normal force and moment component can be integrated knowing the pressure distribution and the gap height along the rolling direction. The tangential stress due to the viscous flow is defined in equation (22). Only pure-rolling hydrodynamic forces and moments are studied here, thus  $\Delta u$  is zero

$$\tau_{xy} = \eta \frac{\partial u}{\partial y} = \frac{\partial p}{\partial x} \left( y - \frac{h}{2} \right) + \eta \frac{\Delta u}{h}$$

$$\tau_{xy}(y = h) = \frac{\partial p}{\partial x} \frac{h}{2}$$
(22)

By projecting the tangential stresses on x axis, one obtains for the Hertzian dimensionless parameters the

(14)

resistant moment due to the Poiseuille term (as shown in part 1, this integral is the only one worth to be calculated because other force and moment integrals can, most of the time, be neglected or deduced)

$$T_{t}^{x} = -\frac{1}{2} \iint_{\Omega} \left( 1 + \frac{H_{0} - H(X, Z)}{\overline{R}^{2}} \right) \times H(X, Z) \frac{\partial P(X, Z)}{\partial X} dX dZ$$
(23)

#### 2.3 Power losses

Power losses due to viscous shearing are defined below for pure-rolling conditions

$$\dot{\gamma} = \frac{\sqrt{\tau_{xy}^2 + \tau_{yz}^2}}{\eta} = \frac{1}{\eta} \sqrt{\left(\frac{\partial p}{\partial x}\right)^2 + \left(\frac{\partial p}{\partial z}\right)^2} \frac{h}{2}$$
 (24)

The power losses per unit area  $\theta$  read

$$\theta = \int_{0}^{h} \tau \dot{\gamma} \, dy$$

$$= \int_{0}^{h} \frac{\tau^{2}}{\eta} \, dy$$

$$= \frac{1}{12 \eta} \left( \left( \frac{\partial p}{\partial x} \right)^{2} + \left( \frac{\partial p}{\partial z} \right)^{2} \right) h^{3}$$
(25)

Using dimensionless parameters, one obtains

$$\theta = \frac{64 \, b \, p_{\rm H}^5}{E^{\prime 3} \eta_0} \frac{1}{12 \, \bar{\eta}} \left( \left( \frac{\partial P}{\partial X} \right)^2 + \left( \frac{\partial P}{\partial Z} \right)^2 \right) H^3$$

$$= \frac{64 \, b \, p_{\rm H}^5}{E^{\prime 3} n_0} \, (\Theta_X + \Theta_Z) = \frac{64 \, b \, p_{\rm H}^5}{E^{\prime 3} n_0} \, \Theta \tag{26}$$

Thus, the global power losses are

$$\iint_{\Omega} \theta \, dx = \frac{64 \, b^3 \, p_{H}^5}{E^{\prime 3} \eta_0} \iint_{\Omega} \Theta \, dX \, dZ \tag{27}$$

# 3 RESULTS AND DISCUSSION

### 3.1 Published results

The hydrodynamic rolling force for IVR contacts was studied in references [1] and [2] and defined as a function of a curve-fitting of the IVR film thickness proposed by Brewe and Hamrock [7]. If Brewe's film thickness equation is approximated on a wide range of U/W and for radius ratio  $R_z/R_x$  larger than 10, using reference [8] one can suggest a new hydrodynamic

rolling force equation using U and W parameters

$$\tilde{F}_{\text{t Houpert IVR}}^{x} = 0.835 \left(\frac{R_{x}}{R_{z}}\right)^{-0.358} U^{0.636} W^{0.364}$$

$$= 0.34 \,\beta^{-1/3} \left(\frac{R_{x}}{R_{z}}\right)^{-0.358}$$

$$\times \kappa^{0.636} \frac{2a}{R_{x}} U^{0.636} (\kappa W)^{0.031}$$

$$\approx 0.32 \,\beta^{-1/3} \kappa^{0.046} \frac{2a}{R_{x}} U^{0.636} (\kappa W)^{0.031}$$
(28)

After introducing the contact length 2a explicitly, the IVR hydrodynamic force is almost load-independent. The contact length is introduced in the equation by analogy with the line contact. Practically, this contact length is a function of the load raised to the power one-third. The exponent on the ellipticity almost vanishes as well. Its value depends strongly on the choice made during curve-fittings, especially the range of parameters, but nevertheless remains small.

For EHL point contacts, the shift of the pressure centre was studied and curve fitted in references [3] and [4]. Then, the hydrodynamic rolling resistance was deduced from the shift of pressure in reference [1]. Equation (29) represents the hydrodynamic rolling force for EHL point contacts. The dimensionless speed is the main parameter. The piezoviscosity has almost no influence with a power 0.022 (part 1 showed G to a power between 0 and -0.36). The load to power 0.133 has a limited influence; however, the contact length is introduced in the equation by analogy with the line contact. Practically, this contact length is function of the load power one-third. The contact ellipticity has a very limited influence after the contact length term appears explicitly

$$\begin{split} \tilde{F}_{\rm t \; Houpert \; EHL}^x &= 1.544 \; \beta^{1/3} \kappa^{-0.577} U^{0.656} W^{0.466} G^{0.022} \\ &= 0.674 \; \kappa^{-0.043} \frac{2a}{R_x} \; U^{0.656} \left(\kappa \; W\right)^{0.133} G^{0.022} \end{split} \tag{29}$$

## 3.2 Moment for fully flooded contacts

Hydrodynamic forces and moments are sensitive to the integration domain in both x and z directions. Numerical starvation has a more important impact on the hydrodynamic forces than on the central or minimum film thickness. For some operating conditions (low load, high speed), very large integration domains were used. A parametric study with varying operating conditions was conducted: for the EHL code, from L=4 to L=16 and M=0.2 to M=800. For readers who are not familiar with the Moes parameters, it represents for steel and for a fixed G=4520: U=6e-13 to U=1.6e-10 and  $\kappa W=5e-10$  to

 $\kappa W=1.8e-5$ . It represents for pressures a range between  $p_{\rm h}=60e6$  Pa to  $p_{\rm h}=4e9$  Pa. Almost no deformation occurs for the lowest M values; so EHL results should normally reach an IVR asymptote. However, numerical results at very low M are not easy to obtain with the EHL dimensionless parameters (especially because of the required domain size). One can nevertheless see an increase in the hydrodynamic rolling moment for low M towards the IVR results. A wide range of ellipticity was covered from circular to wide elliptical contacts,  $\kappa=1/60$  for EHL contacts and even lower for IVR

$$T_{\text{IVR}}^* = \beta^{-1/12} \frac{3.2}{\kappa^{0.88} M U^{1/12}}$$

$$\tilde{T}_{\text{IVR}}^* = 0.77 \, \beta^{-1/3} \kappa^{0.12} \frac{2a}{R_x} \, U^{2/3}$$

$$T_{\text{EHL }\infty}^* = \frac{7.5}{\kappa \, M}$$

$$\tilde{T}_{\text{EHL }\infty}^* = 1.8 \, \beta^{-1/4} \frac{2a}{R_x} \, U^{3/4}$$
(31)

Similar to part 1, two asymptotic behaviours were found in equations (30) and (31). However, the EHL asymptote is reached slower than for line contact. The curve-fitting for IVR shows a very satisfying correlation with exponents on the different parameters quite different from the line contact IVR ones. The transition from IVR to EHL is smooth in relation to the line contact. Using the EHL asymptote at low M may lead to a significant underestimation of the rolling moment. The main advantage of the EHL curve-fitting is to show a close similarity with the EHL line contact curvefitting. The exponent on the dimensionless speed is the same; moreover, the different constants (22 per cent larger for point contact than for line contact) will be partially explained later using the integral on the hydrodynamic force or using the power dissipation integral. Figure 1 shows some numerical results and the asymptotical curve-fittings.

The transition between IVR and EHL regimes is less clear than for line contacts. A curve-fitting of the entire IVR–EHL domain is proposed in equation (32) to fit the two asymptotic behaviours

$$\tilde{T}^* = \frac{2a}{R_x} \left( (0.77 \,\beta^{-1/3} \kappa^{0.12} U^{-1/12} - 1.8 \,\beta^{-1/4}) \right.$$

$$\times \frac{1}{1 + (M/6.6)} + 1.8 \,\beta^{-1/4} \right) U^{3/4}$$
(32)

An example for few EHL operating conditions is given below. Figure 2 represents at the top the simple integral of the tangential stresses along the rolling direction times M as a function of Z (at each Z location, a contribution to the total moment is calculated

![](_page_4_Figure_8.jpeg)

Fig. 1 Fully flooded moment as a function of M for IVR (top) and EHL (bottom) regimes

![](_page_4_Figure_10.jpeg)

**Fig. 2** Integrals  $IT_t^x$  and  $IIT_t^x$  perpendicularly to the rolling direction

similarly as the line contact integral), at the bottom, the double integral of the tangential stresses along the rolling direction times  $\kappa M$  as a function of Z. It corresponds to the cumulated integral along Z of this tangential component. The value in  $Z_b$  corresponds to the moment of the complete contact. If the curvefittings were perfect, all the curves should end at the same level. However, one can notice that the contribution to the total moment from  $-Z_b$  to  $-1/\kappa$  (Z = $-1/\kappa$  corresponding to z = -a) is about 13 per cent. Because of the symmetry, the same contribution is seen between  $1/\kappa$  and  $Z_b$ . This means that about 26 per cent of the total moment is built up outside the Hertzian domain and this is a first explanation why the constant in the torque curve-fitting for EHL conditions for point contact is about 22 per cent larger than the constant for line contact (1.8 in comparison with 1.47). Figure 2 at the top is plotted only between 0 and 2 for clarity, all the curves being symmetric

$$IT_{t}^{x}(Z) = -\frac{M}{2} \int_{-\infty}^{\infty} H\left(X', Z\right) \frac{dP\left(X', Z\right)}{dX'} dX'$$

$$IIT_{t}^{x}(Z) = -\frac{\kappa M}{2}$$

$$\times \int_{-\infty}^{Z} \int_{-\infty}^{\infty} H\left(X', Z'\right) \frac{\partial P\left(X', Z'\right)}{\partial X'} dX' dZ'$$
(34)

$$\operatorname{IIT}_{t}^{x}(Z_{b}) \approx \kappa M T_{t}^{x}$$

# 3.3 Limited contact width

The previous calculations were performed using very large domain integration to limit the numerical starvation. However, contact width is limited in roller bearing by the race width or the roller length. This has been simulated by limiting the integration domain in the direction perpendicular to the rolling direction. The contact behaviour, in terms of hydrodynamic rolling moment, is function of the operating conditions and the contact ellipticity. Many calculations were performed varying the ratio  $2a/\mathcal{L}$  ( $\mathcal{L}$  being the roller length or the ball diameter), the operating conditions and the contact ellipticity in order to propose a curvefitting of a truncation coefficient C to be applied on the fully flooded moment. Only EHL conditions were studied because the domain limitation in Z direction will really be effective when the Hertzian contact width 2a will be of the same order of magnitude as the roller length  $\mathcal{L}$ , which is the case at moderate to high loads

$$C = \frac{1}{1 + ((1/c) - 1)(2a/\mathcal{L})^{(2-\kappa)(0.0018M+1.33)}}$$

$$c = \frac{0.85}{1 + 4.72M^{-0.39}\kappa^{0.484M^{0.165}}}$$
(35)

$$c = \frac{0.85}{1 + 4.72M^{-0.39}\kappa^{0.484M^{0.165}}} \tag{36}$$

![](_page_5_Figure_9.jpeg)

Truncation coefficient as a function of the contact ellipticity  $\kappa$ , the operating conditions M, and contact width ratio  $2a/\mathcal{L}$ 

In equations above, c is the value of the truncation coefficient when the contact width is equal to the race width. This value is maximum when M is high (high load, low speed, low viscosity) and  $\kappa$  is low (wide elliptical contacts). Inversely, the truncation coefficient drops for circular, low load, and high-speed contacts. The truncation coefficient logically tends to 1 when 2a becomes small relative to  $\mathcal{L}$ . Figure 3 represents at the top the value of c as a function of the contact ellipticity for different operating conditions with  $2a/\mathcal{L}$ equals to 1. At the bottom, the truncation coefficient is plotted as a function of  $2a/\mathcal{L}$  for different operating conditions and ellipticities. Numerical results are the points, curve-fitting are the solid lines. When  $2a/\mathcal{L} > 1$ , the Hertzian ellipse is truncated and line contact equations developed in part 1 should be used. An alternative is to keep the point contact equation but with  $2a/\mathcal{L}$  being limited to one so as to calculate the truncation coefficient C. Results are almost identical to those obtained for line contacts (part 1) within a few percent.

#### **Comparisons**

The curve fittings found in the literature (black dotted lines for IVR and black dashed-dotted lines for EHL) and the results presented in this paper (black dashed lines for IVR and black thin solid lines for EHL) are plotted for two ellipticities in Figs 4 to 7. Figures 6 and 7 are for a wide elliptical contact, whereas Figs 4 and 5

![](_page_6_Figure_2.jpeg)

**Fig. 4** Comparison of the hydrodynamic rolling moment as function of the dimensionless speed κ = 0.223 (vertical axis lettering defined in the legend)

are for a narrower elliptical contact. The red thick solid lines represent the model switching between the two asymptotic behaviours: IVR (dashed lines) and EHL (thin solid lines).

Figures 4 and 6 represent the hydrodynamic torque or force as a function of the speed *U* for two loads *W* . Figures 5 and 7 represent the hydrodynamic torque or force as a function of the load *W* for two speeds *U*. The goal is to compare model trends for various operating conditions.

For a low load (top plot in Fig. 4), the IVR regime is reached faster by increasing the speed than for a higher load at the bottom. The different curves are relatively parallel, which shows that the different models show a more or less similar dependence on the speed. On Fig. 5, the EHL regime is reached sooner by increasing the load at low speed (top plot) than at high speed (bottom plot). Also note the difference with line contacts where two very distinct slopes were found for IVR and EHL regimes when plotted versus the load. Equations below represent the ratio between curve fittings found in the literature and the equations suggested in this work. For usual operating conditions, this ratio is of

![](_page_6_Figure_7.jpeg)

**Fig. 5** Comparison of the hydrodynamic rolling moment as function of the dimensionless load κ = 0.223 (vertical axis lettering defined in the legend)

the order of 0.6 for IVR and 0.7 for EHL contacts

$$\begin{split} \frac{\tilde{F}_{\text{t Houpert IVR}}^{x}}{\tilde{T}_{\text{IVR}}^{*}} &= 0.624 \, \left(\frac{\kappa}{0.01}\right)^{-0.043} \left(\frac{U}{10^{-10}}\right)^{-0.031} \\ &\times \left(\frac{W}{10^{-7}}\right)^{0.031} \\ &\times \left(\frac{W}{10^{-7}}\right)^{0.031} \\ &\times \left(\frac{W}{10^{-10}}\right)^{0.031} \\ &\times \left(\frac{W}{10^{-7}}\right)^{0.333} \\ &\frac{\tilde{F}_{\text{t Houpert EHL}}^{x}}{\tilde{T}_{\text{EHL }\infty}^{*}} &= 0.734 \, \left(\frac{\kappa}{0.01}\right)^{0.09} \left(\frac{U}{10^{-11}}\right)^{-0.094} \\ &\times \left(\frac{W}{10^{-5}}\right)^{0.133} \left(\frac{G}{4000}\right)^{0.022} \\ &\tilde{T}_{\text{EHL }\infty}^{*} &= 10.9 \, 10^{-9} \, \left(\frac{\kappa}{0.01}\right)^{-0.667} \left(\frac{U}{10^{-11}}\right)^{0.75} \\ &\times \left(\frac{W}{10^{-5}}\right)^{0.333} \end{split}$$

![](_page_7_Figure_2.jpeg)

**Fig. 6** Comparison of the hydrodynamic rolling moment as function of the dimensionless speed κ = 0.0146 (vertical axis lettering defined in the legend)

## **3.5 Power losses**

The analytical demonstration presented in part 1 concerning the equivalence between the power loss and the moment cannot been made for point contacts. However, if one assumes a similar relation to be valid, the following equation can be written

$$\left(\frac{3}{2}\right)^{4/3} \frac{M^{4/3}}{\pi} \iint_{\Omega} \Theta \, dX dZ \approx T_{t}^{x} \tag{37}$$

Using the equation (31), one obtains for EHL fully flooded conditions

$$\iint_{\Omega} \Theta_{\text{EHL }\infty} \, dX \, dZ = \frac{13.7}{\kappa M^{7/3}} \tag{38}$$

Figure 8 represents the cumulated power losses perpendicularly to the rolling direction defined in equation (39). The predicted final value of 13.7 in equation (38) is reached, which validates numerically the present authors' assumption. Similarly as previously shown in Fig. 2, the power losses between −*Z*<sup>b</sup>

![](_page_7_Figure_10.jpeg)

**Fig. 7** Comparison of the hydrodynamic rolling moment as function of the dimensionless load κ = 0.0146 (vertical axis lettering defined in the legend)

![](_page_7_Figure_12.jpeg)

**Fig. 8** Dimensionless cumulated power losses perpendicularly to the rolling direction

and −1/κ is about 13 per cent of the total power losses

$$II\Theta(Z) = \kappa M^{7/3} \int_{-\infty}^{Z} \int_{-\infty}^{\infty} \Theta(X', Z') \, dX' \, dZ'$$
 (39)

Figure 9 represents the power losses per unit area concerning the Poiseuille term along *X* at the top, along *Z* at the middle and total at the bottom (the

![](_page_8_Figure_2.jpeg)

**Fig. 9** Power losses due to the Poiseuille term along *X* direction at the top, along *Z* direction at the middle and total power losses at the bottom, *M* = 10, *L* = 8 and κ = 0.223

colour scale is very different at the middle). However, the contribution of *<sup>Z</sup>* to is negligible for wide elliptical contacts (less than 2 per cent). For circular contacts, this is no longer verified and *<sup>Z</sup>* may represent up to 30 per cent of for some operating conditions. However, equations (37) and (38) remain correct: the dimensionless total power losses due to the Poiseuille terms remains proportional to the hydrodynamic rolling moment

$$\int_{\Omega} \Theta_{Z \text{ EHL } \infty} dX dZ \ll \int_{\Omega} \Theta_{\text{EHL } \infty} dX dZ$$
 for  $\kappa \ll 1$  (40)

Concerning the build up of the power losses along the rolling direction *X*, similar results as part 1 would have shown that almost all the power losses are generated outside the Hertzian region (inlet and outlet).

# **4 CONCLUSION**

The point contact Reynolds equation and elasticity equation have been used to calculate the film thickness, the pressure distribution, and the resultant hydrodynamic rolling force. Two specific tools have been written: one using IVR assumptions and another one using EHL assumptions with different appropriate dimensionless variables.

Using, in a first step, fully flooded conditions, a single equation has been developed for calculating the hydrodynamic rolling force in a point contact as a function of the ellipticity ratio κ and the dimensionless load and speed parameters *W* and *U*, respectively. The calculated force tends asymptotically towards an IVR equation at low load and high speed, and tends asymptotically towards an EHL equation at large load and low speed. The IVR hydrodynamic rolling force is almost proportional to the contact length 2*a*. The EHL hydrodynamic rolling force is proportional to the contact length.

These new hydrodynamic rolling force relationships have been compared with those previously published, leading to new forces about 1.6 times larger (for IVR) or 1.4 times larger (for EHL) than those published for usual operating conditions. The point contact hydrodynamic rolling forces have also been compared with those calculated using the line contact of length L being equal to 2*a* (line contact relationships are given in a previous paper, part 1). It has been found that the point contact hydrodynamic rolling forces are larger than the line contact ones, about 26 per cent larger, for example when studying EHL forces. An explanation for this peculiar result has been found: It is due to the larger domain of integration used when studying point contact.

In rolling bearings, the contact width (hence width of the domain of integration) is limited by the race width, roller length, or ball diameter L. As the load increases, the contact length 2*a* increases too (2*a* is proportional to the load raised to the power one-third) until reaching the roller length L where ellipse truncation occurs. This triggered a specific study of the truncation effect in which a truncation factor C has been calculated and curve-fitted as a function of the ratio 2*a*/L and the operating conditions.

A remarkable, but meaningful, result has been found: when using the truncation factor, the EHL point contact hydrodynamic rolling force is equal to the one calculated in part 1 using line contact models when 2*a*/L is equal to one. When 2*a*/L becomes larger than one, it should be limited to one when calculating the truncation factor; the point contact and line contact hydrodynamic rolling forces will then be the same within a few per cent.

Finally, a single point contact relationship can be suggested, covering the IVR to EHL operating conditions with a smooth transition between these lubrication regimes, and also a smooth transition from point contact to line contact as the load increases and contact ellipse truncation occurs. Interesting power loss calculations have also been done for differentiating the power loss due to the Poiseuille flow in the rolling direction X and lateral direction Z. It has been found that the power loss in the Z direction is usually very small for wide elliptical contacts and that most of the power is dissipated in the inlet and outlet, with about 26 per cent contribution of the integration domain defined out the range -a < z < a. This result is in line with the truncation factor defined previously.

Not studied is the starvation effect on the hydrodynamic rolling force, but a starvation factor has been given in part 1 (for line contact) for both IVR and EHL regime so that it can be suggested to keep, as a first estimate, the same starvation factor when studying point contact.

#### **ACKNOWLEDGEMENT**

The authors thank The Timken Company for the permission to publish this work.

© Authors 2010

#### REFERENCES

- **1 Houpert, L.** Piezoviscous-rigid rolling and sliding traction forces; application: the rolling element cage pocket contact. *ASME I. Tribol.*, 1987, **109**, 363–371.
- **2 Dalmaz, G.** *Le film mince visqueux dans les contacts hertziens en régimes hydrodynamique et élastohydrodynamique.* Docteur d'Etat Es Science thesis, no. I-DE-7907, INSA-Lyon, 1979.
- **3 Tevaarwerk, J. L.** and **Johnson, K. L.** The influence of fluid rheology on the performance of traction drives. *ASME J. Lubr. Technol.*, 1979, **101**, 266.
- **4 Hamrock, B. J.** and **Dowson, D.** Isothermal elastohydrodynamic lubrication of point contact, part III fully flooded results. *ASME J. Lubr. Technol.*, 1977, **99**, 264–276.
- 5 Dowson, D. and Higginson, G. R. Elastohydrodynamic lubrication, the fundamentals of roller and gear lubrication, 1966 (Pergamon Press, Oxford, UK).
- **6 Venner, C. H.** and **Lubrecht, A. A.** *Multilevel methods in lubrication*, vol. 37, 2000 (Elsevier Tribology Series, Amsterdam).
- **7 Brewe, D. E.** and **Hamrock, B.** Analysis of starvation on hydrodynamic lubrication in non-conforming contact. *ASME J. Lubr. Technol.*, 1982, **104**, 410–411.
- **8 Houpert, L.** An engineering approach to Hertzian contact ellipticity. *ASME J. Tribol.*, 2001, **123**, 582–588.
- 9 Hamrock, B. J. and Brewe, D. Simplified solution for stresses and deformations. *ASME J. Lubr. Technol.*, 1983, 105. 171–177.
- **10 Moes, H.** Lubrication and beyond. Technical report, code 115531, University of Twente, Enschede, The Netherlands, 2000.

#### APPENDIX 1

#### Notation

| a                                    | Hertzian contact half width along $z$ (m)             |
|--------------------------------------|-------------------------------------------------------|
|                                      |                                                       |
| b                                    | Hertzian contact half width along $x$ (m)             |
| $\mathcal{C}$                        | truncation coefficient                                |
| $E_1$ , $E_2$                        | Young's modulus for body 1 and 2 (Pa)                 |
| E'                                   | equivalent Young's modulus (Pa)                       |
|                                      |                                                       |
| ${\cal E}$                           | elliptical integral                                   |
| $f \ F, 	ilde{F}$                    | force (N)                                             |
| $\tilde{F}$ $\tilde{F}$              | dimensionless force = $f/p_h b^2$ , = $f/E'R_x^2$     |
| $\sigma$                             |                                                       |
| ${\mathcal F}$                       | elliptical integral                                   |
| G                                    | dimensionless viscosity                               |
|                                      | $parameter = \alpha E'$                               |
| h                                    | gap height (m)                                        |
|                                      | 01 0                                                  |
| $H,	ilde{H}$                         | dimensionless gap                                     |
|                                      | height = $hR_x/b^2$ , = $h/R_x$                       |
| $H_0, 	ilde{H_0}$                    | dimensionless mutual approach                         |
| L                                    | The Moes parameter $= \beta^{-3/4}GU^{1/4}$           |
|                                      |                                                       |
| ${\cal L}$                           | roller length along $z$ (m)                           |
| M                                    | The Moes parameter = $\beta^{1/4} (\kappa W/U^{3/4})$ |
| p                                    | pressure (Pa)                                         |
|                                      | -                                                     |
| $p_{ m h}$ ~                         | maximum Hertzian pressure (Pa)                        |
| $P, \tilde{P}$                       | dimensionless pressure = $p/p_h$ , = $p/E'$           |
| $R_x$ , $R_z$                        | reduced radii of curvature (m)                        |
| $R_{x1}, R_{x2}$                     | radii of curvature along X for body 1                 |
| $n_{x1}, n_{x2}$                     | •                                                     |
|                                      | and 2 (m)                                             |
| $R_{z1}$ , $R_{z2}$                  | radii of curvature along $Z$ for body 1               |
|                                      | and 2 (m)                                             |
| $\overline{R}$                       | dimensionless reduced radius of                       |
| Λ                                    |                                                       |
|                                      | $curvature = R_x/b$                                   |
| t                                    | moment (N m)                                          |
| $T$ , $	ilde{T}$                     | dimensionless                                         |
| 1,1                                  |                                                       |
|                                      | $moment = t/p_h b^3, = t/E'R_x^3$                     |
| $u_{ m m}$                           | mean velocity (m/s)                                   |
| U                                    | dimensionless velocity = $2\eta_0 u_{\rm m}/E'R_x$    |
| w                                    | load (N)                                              |
|                                      |                                                       |
| W                                    | dimensionless load = $w/E'R_x^2$                      |
| x, y, z                              | coordinates (m)                                       |
| $X, \tilde{X}$                       | dimensionless                                         |
| ,                                    | coordinates = $x/b$ , = $x/R_x$                       |
| V V                                  |                                                       |
| $X_{\rm a}$ , $X_{\rm b}$            | dimensionless domain boundaries                       |
|                                      | $\operatorname{along} X$                              |
| $Z, 	ilde{Z}$                        | dimensionless                                         |
| _,_                                  | coordinates = $z/b$ , = $z/\sqrt{R_x R_z}$            |
| 7 7                                  |                                                       |
| $-Z_{\mathrm{b}}$ , $Z_{\mathrm{b}}$ | dimensionless domain boundaries                       |
|                                      | along $Z$                                             |
|                                      | -                                                     |
| 01                                   | pressure viscosity index (Pa <sup>-1</sup> )          |
| <u>α</u>                             |                                                       |
| $\overline{lpha}$                    | dimensionless viscosity index = $\alpha p_h$          |
| $\dot{\gamma}$                       | shear rate $(s^{-1})$                                 |
| $\eta$                               | viscosity (Pas)                                       |
|                                      | · · · · · · · · · · · · · · · · · · ·                 |
| $\overline{\eta}$                    | dimensionless viscosity = $\eta/\eta_0$               |
| $\eta_0$                             | viscosity at ambient pressure (Pas)                   |
| $\theta$                             | power losses per unit area (W/m <sup>2</sup> )        |
| Θ                                    | dimensionless power losses                            |
|                                      | -                                                     |
| κ                                    | ellipticity                                           |
|                                      |                                                       |

| $\overline{\lambda}$ | dimensionless parameter of Reynolds equation = $12 u_m \eta_0 R_x^2 / b^3 p_h$ |
|----------------------|--------------------------------------------------------------------------------|
| $v_1, v_2$           | the Poisson coefficient for body 1 and 2                                       |
| ho                   | density (kg/m³)                                                                |
| $ ho_0$              | density at ambient pressure (kg/m³)                                            |
| $\overline{ ho}$     | dimensionless density = $\rho/\rho_0$                                          |
| τ                    | lubricant shear stress (Pa)                                                    |
|                      |                                                                                |

# Superscripts

| $o^x$ , $o^z$ | along $x$ , along $z$ |
|---------------|-----------------------|
| <b>o</b> *    | fully flooded limit   |

# Subscripts

| $o_{\rm a}, o_{\rm b}$ | domain bound | dary (inlet), domain |
|------------------------|--------------|----------------------|

boundary (outlet)

 $o_{\rm c}, o_{\rm m}$  central, minimum

 $o_{ ext{IVR}}, o_{ ext{EHL}}$  isoviscousrigid, elastohydrodynamic

 $o_{\rm n}, o_{\rm t}$  normal component, tangential component

 $o_{\rm S}$  undeformed

## **APPENDIX 2**

# **Elliptical parameters**

The elliptical integrals are from Hamrock and Brewe [9]

$$\mathcal{E} = \int_0^{\pi/2} \sqrt{1 - (1 - \kappa^2) \sin^2 \psi} \, d\psi$$
 (41)

$$\mathcal{F} = \int_0^{\frac{\pi}{2}} \frac{1}{\sqrt{1 - (1 - \kappa^2)\sin^2 \psi}} \, d\psi \tag{42}$$

Equation (43) is the relation between the contact ellipticity  $\kappa$  and the radius ratio. The approximated equation (44) is from Moes [10]

$$\frac{R_x}{R_z} = \kappa^2 \frac{\mathcal{F} - \mathcal{E}}{\mathcal{E} - \kappa^2 \mathcal{F}} \tag{43}$$

$$R_{z} = \frac{\mathcal{E} - \kappa^{2} \mathcal{F}}{1 + \sqrt{\ln(16/(R_{x}/R_{z}))/2(R_{x}/R_{z})} - \sqrt{\ln 4}} + 0.16 \ln(R_{x}/R_{z})}$$
(44)

The elliptical integral approximation  $\mathcal{E}$  is from Moes [10]. For circular contacts,  $R_x/R_z=1$ , the elliptical integral  $\mathcal{E}$  tends asymptotically to  $\pi/2$  while the ellipticity  $\kappa$  becomes 1 (circular contact) and to 1 when  $\kappa$  tends to 0 (line contact)

$$\mathcal{E} \approx \frac{\pi}{2} \kappa^2 \left( 1 + \frac{2 \left( 1 - \kappa^2 \right)}{\pi \kappa^2} - 0.125 \ln \kappa^2 \right) \tag{45}$$

## **IVR film thickness**

An IVR film thickness curve fitting of the numerical results is proposed below for radius ratio  $R_z/R_x$  larger than 10

$$\tilde{H}_{\text{IVR}}^* = 37 \left( \frac{U}{\sqrt{(R_x/R_z)}W} \right)^{1.86}$$
 (46)