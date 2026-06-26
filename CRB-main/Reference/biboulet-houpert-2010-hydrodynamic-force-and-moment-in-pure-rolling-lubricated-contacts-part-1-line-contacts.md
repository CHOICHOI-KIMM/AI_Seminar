# **Hydrodynamic force and moment in pure rolling lubricated contacts. Part 1: line contacts**

**N Biboulet**<sup>∗</sup> and **L Houpert**

TIMKEN Europe, Colmar, France

*The manuscript was received on 25 January 2010 and was accepted after revision for publication on 30 March 2010.*

DOI: 10.1243/13506501JET790

**Abstract:** Hydrodynamic rolling force and moments in line contact have been studied in detail using isoviscousrigid (IVR) and elastohydrodynamic (EHL) models. Using fully flooded assumptions, curve-fitted relationships are suggested for calculating the IVR and EHL hydrodynamic rolling force per unit length. At high speed and light load, EHL numerical results converge towards IVR results so that a single curve-fitted relationship has been derived for covering the full range of operating conditions with a rapid transition from IVR to EHL regime of lubrication.

Results obtained are often close to published results (especially in the IVR regime). The EHL hydrodynamic rolling force per unit length is found to be load independent, while load exponents ranging from 0.01 to 0.37 can be found in the literature. A single relationship for both lubrication regimes (IVR and EHL) is given for deriving a starvation factor function of the ratio between the film thickness at the inlet meniscus and the fully flooded minimum film thickness.

Finally, the calculation of the total power loss per unit length has also been conducted by integrating through the film and along the rolling direction the power loss per unit volume (defined as the product shear stress time shear rate). Results obtained are consistent with the calculation of the rolling force per unit length.

**Keywords:** rolling resistance, hydrodynamic rolling resistance, power losses, hydrodynamic rolling power losses, bearing torque, bearing power losses, race torque, line contact, elastohydrodynamic, isoviscousrigid

## **1 INTRODUCTION**

There is a growing interest in accurately calculating power losses in rolling bearings and trying to reduce power losses for conserving energy or fuel. One important contribution to the moment in rolling bearings (among rib contact, drag, churning, cage slip, and so on) is the hydrodynamic rolling resistance.

Calculating the bearing power loss can be done by calculating the bearing torque times the shaft speed. Many bearing torque models are described in the literature, and among them, the pioneered work of Snare [**1**], and Palmgren [**2**] who introduced the hydrodynamic moment called *M*<sup>0</sup> proportional to the product (speed × viscosity)2/3. This moment accounts for the hydrodynamic rolling force generated at any rolling element – race contact, but the initial calculations only apply to a theoretical case with zero load.

Bearing manufacturers then developed their own relationships based on empirical experience, for example, Witte [**3**], where the hydrodynamic race moment is proportional to (speed × viscosity)2/<sup>3</sup> × load1/3. Several analytical bearing torque models have also been described [**4**–**8**] in which analytical relationships have been used for calculating the hydrodynamic rolling force as a function of the speed, viscosity, and load.

The problem is that the hydrodynamic rolling forces are not well known and authors will present further some of the published relationships. Several curve-fitted relationships are available for EHL line contact (leading to substantial variations) and EHL point contact relationships have not been studied specifically. Account should also be made of the miscellaneous lubrication regimes or assumptions: at low load and high speed, one should use isoviscousrigid (IVR) and piezoviscousrigid (PVR) relationships, and

*email: nans.biboulet@yahoo.com*

<sup>∗</sup>*Corresponding author: TIMKEN Europe, 2 Rue Timken, Colmar 68000, France.*

PiezoViscousElastic (PVE also called EHL) relationships should be used at high load or low speed.

An attempt has been made by Houpert [9] to derive EHL point contact relationships as well as PVR relationships, but some problems still remain, for example, the difficulty of ensuring a smooth transition from PVR to EHL relationships and a smooth transition from point contact to line contact when the contact ellipse length exceeds the roller length leading to ellipse truncation. Starvation effects are also important to understand since they contribute to reduce hydrodynamic power losses in pure rolling contacts.

Besides its contribution to power losses, an accurate knowledge of the hydrodynamic rolling force (referred also further as the tangential force) and associated hydrodynamic pressure force (referred also further as the normal force and described finally as a function of the hydrodynamic rolling force) is also essential when trying to predict the dynamic behaviour of bearings with the risk of skidding, smearing, and cage failure [10, 11]. The objectives of this study are therefore clear: derive a consistent set of equations for calculating the hydrodynamic rolling force applicable to line and point contact, with all lubrication regimes covered and starvation and truncation effects included.

Part 1 deals with line contact and part 2 describes the point contact.

## 2 EQUATIONS AND DEFINITIONS

#### 2.1 Reynolds equation

The reduced radius of curvature is defined by

$$\frac{1}{R} = \frac{1}{R_1} + \frac{1}{R_2} \tag{1}$$

The reduced Young modulus reads

$$\frac{2}{E'} = \frac{1 - \nu_1^2}{E_1} + \frac{1 - \nu_2^2}{E_2} \tag{2}$$

Two types of dimensionless parameters are used. The Hertzian dimensionless parameters for EHL contacts with large elastic deformations are defined in equation (3). The classical dimensionless parameters for IVR calculations are defined in equation (4). Both definitions will be used further and were used in two different calculation codes

$$H = \frac{hR}{b^2}$$

$$X = \frac{x}{b}$$

$$P = \frac{p}{p_h}$$

$$\overline{R} = \frac{R}{b}$$

$$\tilde{H} = \frac{h}{R}$$

$$\tilde{X} = \frac{x}{R}$$

$$\tilde{P} = \frac{p}{E'}$$
(4)

The gap height between the two surfaces is defined by the summation of the solid approach  $h_0$ , the undeformed geometry  $h_S(x)$ , and the elastic deformations

$$h(x) = h_0 + h_S(x)$$

$$-\frac{2}{\pi E'} \int_{-\infty}^{+\infty} p(x') \ln\left(\frac{x - x'}{x_0}\right)^2 dx'$$

$$h_S(x) = R\left(1 - \sqrt{1 - \left(\frac{x}{R}\right)^2}\right)$$
(5)

Equations (6) and (7) define the dimensionless gap height, respectively, for EHL and IVR calculations. For EHL calculations, the undeformed geometry can be approximated by a parabolic shape because a limited calculation domain is sufficient

$$H(X) = H_0 + H_S(X)$$

$$-\frac{1}{2\pi} \int_{-\infty}^{+\infty} P(X') \ln\left(\frac{X - X'}{X_0}\right)^2 dX' \qquad (6)$$

$$H_S(X) = \overline{R} \left(\overline{R} - \sqrt{\overline{R}^2 - X^2}\right) \approx \frac{X^2}{2}$$

$$\tilde{H}(\tilde{X}) = \tilde{H}_0 + \tilde{H}_S(\tilde{X})$$

$$\tilde{H}_S(\tilde{X}) = 1 - \sqrt{1 - \tilde{X}^2} \qquad (7)$$

The Reynolds equation is used

$$\frac{\mathrm{d}}{\mathrm{d}x} \left( \frac{\rho h^3}{\eta} \frac{\mathrm{d}p}{\mathrm{d}x} \right) = 12 \, u_{\mathrm{m}} \, \frac{\mathrm{d}\rho h}{\mathrm{d}x} \tag{8}$$

The dimensionless equations for EHL and IVR regimes are

$$\frac{\mathrm{d}}{\mathrm{d}X} \left( \frac{\overline{\rho} H^3}{\overline{n}\overline{\lambda}} \frac{\mathrm{d}P}{\mathrm{d}X} \right) = \frac{\mathrm{d}\overline{\rho}H}{\mathrm{d}X} \tag{9}$$

$$\frac{\mathrm{d}}{\mathrm{d}\tilde{X}} \left( \tilde{H}^3 \frac{\mathrm{d}\tilde{P}}{\mathrm{d}\tilde{X}} \right) = 6 U \frac{\mathrm{d}\tilde{H}}{\mathrm{d}\tilde{X}} \tag{10}$$

For the EHL contacts, a Barus viscosity and a Dowson–Higginson [12] compressibility are used

$$\overline{\eta} = \frac{\eta}{\eta_0} = e^{\alpha p_h P} = e^{\overline{\alpha} P} \tag{11}$$

$$\overline{\rho} = \frac{\rho}{\rho_0} = \frac{0.59 \cdot 10^9 + 1.34 \cdot Pp_h}{0.59 \cdot 10^9 + Pp_h} \tag{12}$$

The different dimensionless parameters defining the operating conditions in the Reynolds equation and

(3)

the force balance equation are detailed below (note the factor 2 in the definition of U)

$$\overline{\lambda} = \frac{12 u_{\rm m} \eta_0 R^2}{b^3 p_{\rm h}} = \frac{3\pi^2}{8M_{\rm l}^2}$$

$$\overline{\alpha} = \alpha p_{\rm h} = L \sqrt{\frac{M_{\rm l}}{2\pi}}$$

$$\frac{bE'}{4Rp_{\rm h}} = 1$$

$$W_{\rm l} = \frac{w_{\rm l}}{E'R} = \frac{\pi p_{\rm h} b}{2E'R}$$

$$U = \frac{2\eta_0 u_{\rm m}}{E'R}$$

$$G = \alpha E'$$

$$M_{\rm l} = \frac{W_{\rm l}}{\sqrt{U}}$$

$$L = GU^{1/4}$$
(14)

The vertical force balance reads

$$\int_{-\infty}^{+\infty} p(x) \, \mathrm{d}x = w_{\mathrm{l}} = \frac{w}{\mathcal{L}} \tag{15}$$

$$\int_{-\infty}^{+\infty} P(X) \, \mathrm{d}X = \frac{\pi}{2} \tag{16}$$

$$\int_{-\infty}^{+\infty} \tilde{P}\left(\tilde{X}\right) \, \mathrm{d}\tilde{X} = W_{\mathrm{l}} \tag{17}$$

Multigrid techniques [13] are used to solve the presented sets of equations.

#### 2.2 Forces and moments

Two types of dimensionless parameters for forces *F* and moments *T* are defined below

$$F = \frac{f_{\rm l}}{p_{\rm h}b} = \frac{f}{p_{\rm h}b\mathcal{L}} \tag{18}$$

$$T = \frac{t_{\rm l}}{p_{\rm h}b^2} = \frac{t}{p_{\rm h}b^2\mathcal{L}} \tag{19}$$

$$\tilde{F} = \frac{f_{\rm i}}{E'R} = \frac{f}{E'R\mathcal{L}} \tag{20}$$

$$\tilde{T} = \frac{t_{\rm l}}{E'R^2} = \frac{t}{E'R^2\mathcal{L}} \tag{21}$$

Figures 1 and 2 show a lubricated contact. The pressure generated in the lubricant film is normal to the surfaces. The pressure distribution is not symmetric along the rolling direction *X* because of the cavitation. A pressure is generated in the contact inlet, whereas the outlet pressure drops to zero and is responsible for the hydrodynamic pressure force. Moreover, the viscous (Poiseuille) flow also creates a tangential stress along the surfaces. These two efforts (normal and

![](_page_2_Picture_16.jpeg)

Fig. 1 Hydrodynamic force and moment

![](_page_2_Picture_18.jpeg)

Fig. 2 Hydrodynamic force and moment

tangential) lead to a hydrodynamic force and moment. Force and moment are functions of the operating conditions.

Knowing the pressure distribution and the surface slope along the rolling direction, the normal force and moment can be directly integrated. The projection of the normal force on the rolling direction axis corresponds to the hydrodynamic pressure force. The tangential stress due to the viscous flow is defined in equation (22). Only pure-rolling hydrodynamic forces and moments are studied here; thus,  $\Delta u$  is zero. The tangential stress at y=h is also integrated along the rolling direction X. The projection of the tangential force on the rolling direction axis corresponds to the hydrodynamic rolling force

$$\tau_{xy} = \eta \frac{\mathrm{d}u}{\mathrm{d}y} = \frac{\mathrm{d}p}{\mathrm{d}x} \left( y - \frac{h}{2} \right) + \eta \frac{\Delta u}{h}$$

$$\tau_{xy}(y = h) = \frac{\mathrm{d}p}{\mathrm{d}x} \frac{h}{2}$$
(22)

Using projections on X and Y axes, for normal (index n) and tangential (index t) stresses, six integrals can be written in equations (23). The hydrodynamic rolling force studied in the literature corresponds to  $F_t^x$  and the hydrodynamic pressure force corresponds to  $F_n^x$ . For the Hertzian dimensionless parameters, one has

$$F_n^y = \int_{-\infty}^{+\infty} P(X) \, dX$$

$$F_n^x = -\frac{1}{R} \int_{-\infty}^{+\infty} P(X) \, \frac{dH(X)}{dX} \, dX$$

$$F_t^y = -\frac{1}{2R^2} \int_{-\infty}^{+\infty} H(X) \, \frac{dP(X)}{dX} \, \frac{dH(X)}{dX} \, dX$$

$$F_{t}^{x} = -\frac{1}{2\overline{R}} \int_{-\infty}^{+\infty} H(X) \frac{dP(X)}{dX} dX$$

$$T_{n}^{y} = \int_{-\infty}^{+\infty} XP(X) dX$$

$$T_{n}^{x} = -\int_{-\infty}^{+\infty} \left(1 + \frac{H_{0} - H(X)}{\overline{R}^{2}}\right) P(X) \frac{dH(X)}{dX} dX$$

$$T_{t}^{y} = -\frac{1}{2\overline{R}^{2}} \int_{-\infty}^{+\infty} XH(X) \frac{dP(X)}{dX} \frac{dH(X)}{dX} dX$$

$$T_{t}^{x} = -\frac{1}{2} \int_{-\infty}^{+\infty} \left(1 + \frac{H_{0} - H(X)}{\overline{R}^{2}}\right) H(X) \frac{dP(X)}{dX} dX$$

$$(23)$$

The equivalent integrals for the classical dimensionless parameters are presented in equation (48) in Appendix 2. Finally, one is interested in the resultant force and moment plotted in Fig. 1  $(f_1^x, f_1^y, \text{and } t_1)$ 

$$\sum F^{x}$$

$$\sum F^{y}$$

$$\sum T$$

#### 2.3 Power losses

Power losses due to viscous shearing can be defined using equation (22) and the shear rate defined below

$$\dot{\gamma} = \frac{\tau}{\eta} = \frac{1}{\eta} \frac{\mathrm{d}p}{\mathrm{d}x} \left( y - \frac{h}{2} \right) + \frac{\Delta u}{h} \tag{24}$$

The power losses per unit area  $\theta$  read

$$\theta = \int_0^h \tau \dot{\gamma} \, dy$$

$$= \int_0^h \frac{\tau^2}{\eta} \, dy$$

$$= \frac{\eta \Delta u^2}{h} + \frac{1}{12 \eta} \left(\frac{dp}{dx}\right)^2 h^3$$
(25)

For pure-rolling conditions, one obtains

$$\theta = \frac{64 b p_H^5}{E'^3 \eta_0} \frac{1}{12 \bar{\eta}} \left(\frac{dP}{dX}\right)^2 H^3$$

$$= \frac{64 b p_H^5}{E'^3 \eta_0} \Theta$$
(26)

Thus, the power losses per unit length is

$$\int_{-\infty}^{+\infty} \theta \, dx = \frac{64 \, b^2 \, p_H^5}{E^3 \eta_0} \int_{-\infty}^{+\infty} \Theta \, dX \tag{27}$$

#### 3 RESULTS AND DISCUSSION

#### 3.1 Forces and moments

A parametric study varying operating conditions  $(M_1)$  and L, hence  $\overline{\lambda}$  and  $\overline{\alpha}$ ) has been conducted. A large amount of data was generated allowing to propose general conclusions and fitted equations. After the analysis of these numerical results, one can confirm that some terms can be neglected in equations (23) and (48) and only one integral is really of interest. Equations (28) to (36) define which integrals are negligible or trivial and which one is worth to calculate

$$F_t^y \ll F_n^y$$

$$T_t^y \ll T_t^x$$

$$\tilde{F}_t^y \ll \tilde{F}_n^y$$

$$\tilde{T}_t^y \ll \tilde{T}_t^x$$
(29)

However, these simplifications can become incorrect for high speed, high viscosity, and low load contacts. The second line of equations (28) and (29) are especially not correct for IVR contacts with  $\tilde{H}_{\rm m}>1e-3$ . Typically, for U=1e-9 and  $\tilde{H}_{\rm m}=1e-3$ ,  $\tilde{F}_t^y/\tilde{F}_n^y$  is smaller than 1 per cent but  $\tilde{T}_t^y/\tilde{T}_t^x$  reaches 10 per cent. These values decreases rapidly when the film thickness decreases. For example, for U=1e-9 and  $\tilde{H}_{\rm m}=1e-4$ ,  $\tilde{T}_t^y/\tilde{T}_t^x$  is 3 per cent.

Equations (28) and (29) mean that the vertical component of the tangential stress (viscous flow) can be neglected for the forces and the moments. Moreover, one also obtains the following coupling

$$T_t^x \approx \overline{R}F_t^x$$
 (30)

$$\tilde{T}_t^x \approx \tilde{F}_t^x \tag{31}$$

Equations (30) and (31) simply mean that the moment can be calculated as a concentrated tangential load (the hydodynamic rolling force) at a distance R from the roller centre with the same kind of restrictions mentioned previously for equations (28) and (29); however, error is very limited: for U = 1e - 9 and  $\tilde{H}_{\rm m} = 1e - 3$ , the difference between  $\tilde{T}_t^x$  and  $\tilde{F}_t^x$  is smaller than 2 per cent.

Moreover, manipulating the integrals one can find a direct relation between the horizontal forces due to normal and tangential stresses. Equation (32) demonstrates that the horizontal force due to the normal component is equal to twice the horizontal force due to the tangential component. Thus, there is no interest in calculating both integrals.

Equations (33) and (34) precise first that the classical vertical force balance equation (resultant of all the vertical forces) is retrieved because of equations (28) and (29) and second that the horizontal force resultant

can be calculated using only the tangential component because of equation (32)

$$\tilde{F}_{n}^{x} = -\int_{-\infty}^{+\infty} \tilde{P}\left(\tilde{X}\right) \frac{\mathrm{d}\tilde{H}\left(\tilde{X}\right)}{\mathrm{d}\tilde{X}} \, \mathrm{d}\tilde{X}$$

$$= -\left(\underbrace{\left[\tilde{P}\left(\tilde{X}\right)\tilde{H}\left(\tilde{X}\right)\right]}_{=0} - \int_{-\infty}^{+\infty} \tilde{H}\left(\tilde{X}\right) \frac{\mathrm{d}\tilde{P}\left(\tilde{X}\right)}{\mathrm{d}\tilde{X}} \, \mathrm{d}\tilde{X}\right)$$

$$= -2\tilde{F}_{t}^{x} \qquad (32)$$

$$\sum \tilde{F}^{y} \approx W_{l}$$

$$\sum \tilde{F}^{x} = -\tilde{F}_{t}^{x} = \frac{\tilde{F}_{n}^{x}}{2}$$

$$\sum F^{y} \approx \frac{\pi}{2}$$

$$\sum F^{y} \approx \frac{\pi}{2}$$

$$\sum F^{x} = -F_{t}^{x} = \frac{F_{n}^{x}}{2}$$

$$(33)$$

The moment due to the normal stress is null or negligible. For a rigid contact, the normal component direction passes through the roller centre; thus, the normal components induce no moment. This point is illustrated in the first line of equations (35) and (36). Concerning deformed contacts, calculations show that this remains correct and the moment due to the normal stress remains much smaller than 1 per cent of the moment due to tangential components. Thus, using all the equations from (28), only one term,  $T_x$ , is really worth to calculate

$$\sum T_n \approx 0$$

$$\sum T \approx T_t^x$$

$$\sum \tilde{T}_n \approx 0$$

$$\sum \tilde{T} \approx \tilde{T}_t^x$$
(35)

Finally, one can deduce from the above results the equation between the shift of the pressure centre (used sometimes in the literature) and the moment due to Poiseuille flow

$$W_1 \frac{\delta'}{R} = 2 \, \tilde{T}_t^x \tag{37}$$

## 3.2 Published results

Several curve fittings for  $\tilde{F}_t^x$  can be found in the literature. IVR results in equations (38) are consistent and the different authors obtained very similar equations. However, different trends in EHL equations (39) are observed. Note that equations indexed with  $\times$  are

deduced from curve fitting of  $\delta'$  using equation (37) in reference [9]

$$\begin{split} \tilde{F}_{t \; \mathrm{Dalmaz\; IVR}}^{x} &= 1.21 \; U^{0.491} W_{\mathrm{l}}^{0.509} \quad [\mathbf{14}] \\ \tilde{F}_{t \; \mathrm{Dowson\; IVR}}^{x} &= 1.46 \; U^{0.5} W_{\mathrm{l}}^{0.5} \quad [\mathbf{15}] \\ \tilde{F}_{t \; \mathrm{Hamrock\; EHL}}^{x} &= 0.0295 \; U^{0.44} W_{\mathrm{l}}^{0.37} \quad [\mathbf{16}] \\ \tilde{F}_{t \; \mathrm{Zhou\; EHL}}^{x} &= 18.62 \; U^{0.648} W_{\mathrm{l}}^{0.246} G^{-0.352} \quad [\mathbf{8}] \\ \tilde{F}_{t \; \mathrm{Goksem\; EHL}}^{x} &= 2.74 \; U^{0.658} W_{\mathrm{l}}^{0.0126} G^{-0.342} \quad [\mathbf{17}] \\ \tilde{F}_{t \; \mathrm{Pan\; EHL}}^{x} &= 1.16 \; U^{0.638} W_{\mathrm{l}}^{0.019} G^{-0.358} \quad [\mathbf{18}] \end{split} \tag{39}$$

Equations (39) show that dimensionless speed is the main parameter for EHL contacts. However, the exponent of G and especially the dimensionless load  $W_1$  differ significantly in the equations from almost 0 to 0.4. The authors think that equations with a dimensionless load  $W_1$  power  $\approx 0.3$  were not obtained from numerical simulations only. The experimental dependence of the torque on the load power one third was more or less imposed during curve fittings. A plausible explanation of the experimental dependence on the load will be proposed in part 2 of this study. These different expressions lead to very different predictions when scanning on a realistic range of operating conditions; a ratio of five between the minimum and the maximum predicted moment can be obtained. Note also that the sensitivity to numerical starvation of this kind of calculation may have been a problem in cited studies.

Moreover, the transition between IVR to EHL regime was not clearly defined in cited studies. PVR equations are proposed in reference [9] but a smooth transition from PVR to EHL is not always ensured.

## 3.3 Moment for fully flooded contacts

Hydrodynamic forces and moments are very sensitive to the integration domain. Numerical starvation has a more important impact on the hydrodynamic forces than on the central or minimum film thickness. When increasing the integration domain, the central film thickness tends more rapidly to a limit than the hydrodynamic forces.

IVR calculations are used to define an asymptotic behaviour at low load, high speed, and high viscosity contacts. The calculation domain starts at  $\tilde{X}_a = -1$ . For a large range of IVR operating conditions, this enables one to obtain the asymptotic fully flooded value of the hydrodynamic moment, even if from a practical point of view, such a large amount of oil in the inlet is not available. For very thick film  $(\tilde{H}_{\rm m}>1e-3)$ , the fully flooded moment asymptote is not reached. Note that such cases may be out of the scope of the Reynolds assumptions. The curve fitting of the numerical results is very close to  $\tilde{F}_{t\,{\rm Dowson\,IVR}}^x$  (difference is not significant

compared to numerical errors)

$$\begin{split} T_{\text{IVR}}^* &= \frac{1.4}{M_{\text{l}}} \\ \tilde{T}_{\text{IVR}}^* &= 1.42 \ U^{1/2} W_{\text{l}}^{1/2} \end{split} \tag{40}$$

Concerning EHL contacts, a wide range of operating conditions has been covered: from L=4 to L=16 and  $M_{\rm l}=0.2$  to  $M_{\rm l}=200$ . For readers who are not familiar with Moes parameters, it represents for steel and for a fixed G=4520: U=6.1e-13 to U=1.6e-10 and  $W_{\rm l}=1.6e-7$  to  $W_{\rm l}=2.5e-3$ . It represents for pressures a range between  $p_{\rm h}=36e6$  Pa and  $p_{\rm h}=4.5e9$  Pa. Almost no deformation occurs for the smallest  $M_{\rm l}$  values; and EHL results reach logically the IVR asymptote. However, the behaviour for large  $M_{\rm l}$  is surprising because the real moment is found almost independent of the load and the piezoviscosity index (few per cents not accounted for here). Equation (41) represents the asymptotic behaviour for large  $M_{\rm l}$ 

$$T^*_{\rm EHL~\infty} = {1.45\over M_{\rm l}^{3/2}}$$
 
$$\tilde{T}^*_{\rm EHL~\infty} = 1.47~U^{3/4} \eqno(41)$$

The entire IVR–EHL domain is correctly described by equation (42) which fits intrinsically the two asymptotic behaviours described above (Fig. 3). One can notice that IVR results are asymptotically obtained even using the EHL calculation

$$T^* = M_{\rm l}^{-3/2} \frac{1.4\sqrt{M_{\rm l}}}{(1 + ((1.4/1.45)\sqrt{M_{\rm l}})^{10})^{1/10}}$$
(42)

An example for few EHL operating conditions is given below (one set of curves to vary L, the other to vary  $M_l$ ). Figure 4 represents the dimensionless pressure along the rolling direction and shows different

![](_page_5_Figure_9.jpeg)

**Fig. 3** Fully flooded moment  $T^*$  as a function of  $M_l$ , IVR to EHL regimes

EHL operating conditions from relatively hydrodynamic conditions to quasi-Hertzian pressure profiles. Figure 5 represents the integral of the tangential stresses times  $M_{\rm l}^{3/2}$  as a function of the abscissa X (equation (43)). This integral represents the build up of the hydrodynamic rolling resistance along X

$$IT_{t}^{x}\left(X\right) = -\frac{M_{l}^{3/2}}{2} \int_{-\infty}^{X} H\left(X'\right) \frac{dP\left(X'\right)}{dX'} dX'$$

$$IT_{t}^{x}\left(X_{b}\right) \approx M_{l}^{3/2} T_{t}^{x}$$
(43)

Several points can be mentioned.

- 1. Even if the pressure and film thickness are quite different between different operating conditions, the integrals  $IT_t^x$  crosses all at the same value for the abscissa X = -1.
- 2. Even if the integral  $\operatorname{IT}_t^x$  shows very different values in the high-pressure region, the final value at X=1 is constant regardless of the operating conditions. A symmetric pressure and geometry distributions in the high-pressure region would lead to a zero contribution to  $\operatorname{IT}_t^x$ . But here, the contribution of the high-pressure region is limited and almost constant and leads to a constant final value of the integral  $\operatorname{IT}_t^x(X_b)$ .

#### 3.4 Moment for starved contacts

As mentioned, the hydrodynamic moment is relatively sensitive to starvation. In this study, starvation was obtained numerically by limiting the inlet calculation domain. The moment logically decreases when the inlet domain is restricted. The reduction is more severe for hydrodynamic moment than for film thickness as already found in reference [17]. The ratio  $T/T^*$  is studied; this ratio tends to 1 for a large inlet domain. The required inlet domain size depends on the operating conditions: low speed, low viscosity, and high load require a relatively small inlet size; inversely, high speed, high viscosity, and low load require a large inlet domain.

A simple parameter S, which represents a degree of starvation defined by the ratio of the film thickness available at the inlet boundary  $X_a$  over the minimum film thickness in fully flooded conditions, allows the ratio  $T/T^*$  to gather on a single curve. This remains correct for both IVR and EHL regimes; calculations were performed with a maximum starvation of S=2. The minimum film thickness is used in the S parameter instead of the central film thickness to easily ensure the continuity from IVR to EHL regimes. Equation (44) is the curve fitting of the hydrodynamic torque ratio between starved and fully flooded conditions using the EHL and IVR calculation results. Figure 6 represents

![](_page_6_Figure_2.jpeg)

**Fig. 4** Pressure along rolling direction L = 12

![](_page_6_Figure_4.jpeg)

**Fig. 5** Integral  $IT_t^x$  along the rolling direction

this curve fitting and the numerical results

$$\frac{T}{T^*} = \frac{S^2 + 162.8 \, S}{S^2 + 193.1 \, S + 1257}$$

$$S = \frac{H_a}{H_{\rm m}^*}$$
(44)

However,  $H_a$  can be difficult to evaluate especially in roller bearings. It depends on many parameters (lubricant supply, etc.).

## 3.5 Comparisons

Comparisons with published results can be made. The fully flooded IVR regime is asymptotically reached for high speed, high viscosity, and low load and corresponds to published equations. However, concerning the EHL regime, some significant differences exist. The piezoviscosity parameter G has no effect on the moment, whereas the dependence on the dimensionless speed U is stronger. The calculations also show a completely different dependence on the load (except references [17] and [18]). An EHL asymptote

![](_page_7_Figure_2.jpeg)

**Fig. 6** Influence of the starvation on the moment:  $T/T^*$  as a function of S

![](_page_7_Figure_4.jpeg)

**Fig. 7** Comparison of the hydrodynamic moment as function of the dimensionless speed (lettering of the vertical axis in the legend)

is relatively rapidly reached, and thus for high loads, the calculated moment does not depend on the load anymore.

Figure 7 represents several existing curve fittings  $F_{t \text{ Dowson IVR}}^x$ ,  $F_{t \text{ Zhou EHL}}^x$  and  $F_{t \text{ Goksem EHL}}^x$  as a function of U assuming G=4520. Three other curves have been considered representing curve fittings presented in this work: fully flooded, a moderate starvation represented by  $\tilde{H}_a=1e-2$ , and an important starvation with  $\tilde{H}_a=1e-3$ . Figure 8 rep-

![](_page_7_Figure_8.jpeg)

**Fig. 8** Comparison of the hydrodynamic moment as function of the dimensionless load (lettering of the vertical axis in the legend)

resents the hydrodynamic moment as a function of  $W_1$ . To summarize, the predicted moment is schematically larger than published results for moderate loads and becomes smaller for high loads. The crossing point is shifted towards low loads when the velocity decreases. Here, a constant  $\tilde{H}_a$  has been imposed; thus, the starvation influence becomes really significant for low-load and high-speed conditions.

For an easier comparison, equations (38) to (41) can be written using typical value of G, U, and  $W_{\rm l}$  for each regime

$$\begin{split} \frac{\tilde{F}_{t \, \mathrm{Dalmaz \, IVR}}^{x}}{\tilde{T}_{IVR}^{*}} &= 0.926 \quad \left(\frac{U}{10^{-10}}\right)^{-0.009} \quad \left(\frac{W_{\mathrm{l}}}{10^{-6}}\right)^{0.009} \\ \frac{\tilde{F}_{t \, \mathrm{Dowson \, IVR}}^{x}}{\tilde{T}_{IVR}^{*}} &= 1.03 \\ \tilde{T}_{IVR}^{*} &= 14.2 \, 10^{-9} \left(\frac{U}{10^{-10}}\right)^{0.5} \quad \left(\frac{W_{\mathrm{l}}}{10^{-6}}\right)^{0.5} \\ \frac{\tilde{F}_{t \, \mathrm{Hamrock \, EHL}}^{x \, \times}}{\tilde{T}_{EHL \, \infty}^{*}} &= 1.71 \quad \left(\frac{U}{10^{-11}}\right)^{-0.31} \quad \left(\frac{W_{\mathrm{l}}}{10^{-4}}\right)^{0.37} \\ \frac{\tilde{F}_{t \, \mathrm{Zhou \, EHL}}^{x}}{\tilde{T}_{EHL \, \infty}^{*}} &= 0.939 \quad \left(\frac{U}{10^{-11}}\right)^{-0.102} \quad \left(\frac{W_{\mathrm{l}}}{10^{-4}}\right)^{0.246} \\ &\times \left(\frac{G}{4000}\right)^{-0.352} \end{split}$$

$$\begin{split} \frac{\tilde{F}_{t \; \text{Goksem EHL}}^{x}}{\tilde{T}_{\text{EHL} \; \infty}^{*}} &= 1.00 \quad \left(\frac{U}{10^{-11}}\right)^{-0.092} \quad \left(\frac{W_{l}}{10^{-4}}\right)^{0.0126} \\ & \times \left(\frac{G}{4000}\right)^{-0.342} \\ & \times \left(\frac{\tilde{F}_{t \; \text{Pan EHL}}}{\tilde{T}_{\text{EHL} \; \infty}^{*}} = 0.580 \quad \left(\frac{U}{10^{-11}}\right)^{-0.112} \quad \left(\frac{W_{l}}{10^{-4}}\right)^{0.019} \\ & \times \left(\frac{G}{4000}\right)^{-0.358} \\ & \tilde{T}_{\text{EHL} \; \infty}^{*} = 8.26 \; 10^{-9} \quad \left(\frac{U}{10^{-11}}\right)^{0.75} \end{split}$$

#### 3.6 Power losses

Using equations (9), (25), and (27), one can easily link the moment to the power loss for an incompressible fluid

$$\int_{-\infty}^{+\infty} \Theta \, dX = \int_{-\infty}^{+\infty} \frac{\overline{\lambda}}{12} \frac{dP}{dX} \left( \frac{H^3}{\eta \overline{\lambda}} \frac{dP}{dX} \right) dX$$

$$= \frac{\overline{\lambda}}{12} \int_{-\infty}^{+\infty} \frac{dP}{dX} (H + \text{cst}) \, dX$$

$$= \frac{\pi^2}{16 M_l^2} \left( \frac{1}{2} \int_{-\infty}^{+\infty} \frac{dP}{dX} H \, dX + \frac{1}{2} \underbrace{[\text{cst } P]}_{=0} \right)$$

$$\frac{16}{\pi^2} M_l^2 \int_{-\infty}^{+\infty} \Theta \, dX \approx T_t^x$$
(45)

Numerical results show that this remains true even with a compressible assumption. Using equations (41) and (45), one can show for EHL fully flooded conditions

$$\int_{-\infty}^{+\infty} \Theta_{\text{EHL }\infty} \, dX = 0.9 M_1^{-7/2} \tag{46}$$

Finally, Fig. 9 represents the cumulated dimensionless power losses along the rolling direction X defined in equation (47) for different EHL operating conditions. The heat is generated in the inlet and the outlet only. The high-pressure region does not generate any heat for pure-rolling conditions because of the small film thickness (power 3) and the high viscosity. Moreover, similar to Fig. 5, all the curves in Fig. 9 (representing power losses for different operating conditions) have the same final value. The Newtonian fluid model was chosen because of its simplicity and to have an asymptotic trend corresponding to low shear rate or high shear limit lubricants. This is compatible with the pure-rolling assumptions. Modeling thermal non-Newtonian lubricants would be an interesting but challenging development. Indeed, at low

![](_page_8_Figure_9.jpeg)

**Fig. 9** Dimensionless cumulated power losses along the rolling direction

pressure, introducing appropriate limiting shear stress is not straightforward

$$I\Theta(X) = M_{\rm l}^{7/2} \int_{-\infty}^{X} \Theta(X') \, \mathrm{d}X' \tag{47}$$

#### 4 CONCLUSION

Hydrodynamic rolling force and moments (per unit length) in line contact have been studied in details using IVR and EHL models. The final hydrodynamic moments can be approximated by the product hydrodynamic rolling force times the equivalent radius, since it has been found that the hydrodynamic pressure force (due to the horizontal component of the pressure) does not contribute to the final moment calculated around the roller centre. This is obvious for rigid circular roller, but was not obvious when accounting for elastic surface deformation.

Using fully flooded assumptions, curve-fitted relationships are proposed for calculating the IVR and EHL hydrodynamic rolling force per unit length. At high speed and low load, EHL numerical results converge towards IVR results so that a single curve-fitted relationship has been derived for covering the full range of operating conditions with a rapid transition from IVR to EHL results.

The IVR relationship is close to what has been published by Dowson (the hydrodynamic force per unit length is proportional to the square root of  $U \times W$ ), while the EHL relationship shows a force per unit length proportional to  $U^{0.75}$  and load independent. The EHL fully flooded relationships have been compared with some published ones, showing similar order of magnitude for the rolling force, but also differences concerning the load exponent applied on W (ranging from 0.01 to 0.37).

Starvation has also been studied by varying the inlet meniscus (defining the corresponding film thickness height at the inlet meniscus abscissa). Starvation causes a reduction in the hydrodynamic rolling force per unit length, and this reduction has been found in both IVR and EHL regime to be function of a single starvation parameter.

Finally, the total power losses per unit length have also been calculated by integrating the product shear stress times shear rate (through the film thickness and along the rolling direction) to demonstrate analytically that the final power loss is consistent with the hydrodynamic rolling force; its value is equal to two times the product rolling force times rolling speed when pure rolling occurs. The cumulative power loss per unit length distribution along the rolling direction is shown for several operating conditions and is confirmed to be load independent in the EHL regime.

All previously described results have been obtained using line contact assumptions. However, it is known that crown radii are used in roller bearings (on races and on roller). Thus, roller bearings may behave as point contact with very elongated elliptical contact at low load.

To the authors' knowledge, little has been published in the literature for calculating EHL hydrodynamic rolling force in point contact, so a similar study should be conducted using point contact assumptions. Such a study will be done and published in a subsequent paper (part 2).

#### ACKNOWLEDGEMENT

The authors would like to thank The Timken Company for the permission to publish this work.

© Authors 2010

#### REFERENCES

- 1 Snare, B. Rolling resistance in bearing. *SKF Ball Bearing I.*, 1967, **152**, 3–8.
- **2 Palmgren, A.** *Ball and roller bearing engineering*, 3rd edition, 1959, pp. 34–41 (Burbank, Philadelphia).
- **3 Witte, D. C.** Operating torque of tapered roller bearing. *ASLE Trans.*, 1973, **16**(1), 61–67.
- **4 Houpert, L.** and **Leenders, P.** A study of mixed lubrication in modern deep groove ball bearings. In Proceedings of the 11th Leeds–Lyon Symposium on *Tribology*, The University of Leeds, Leeds, UK, 1984.
- **5 Houpert, L.** and **Leenders, P.** A theoretical and experimental investigation into rolling bearing friction. In Proceedings of the Eurotrib Conference, Lyon, 1985.
- **6 Houpert, L.** Numerical and analytical calculations in ball bearings. In Proceedings of the 8th European Space Mechanism and Tribology Symposium, Centre de Congrès Diagora, Labège, Toulouse, France, 1999.

- **7 Houpert, L.** Ball bearing and tapered roller bearing torque: analytical, numerical and experimental results. *STLE Tribol. Trans.*, 2002, **45**(3), 345–353.
- **8 Zhou, R. S.** and **Hoeprich, M. R.** Torque of tapered roller bearings. *ASME J. Tribol.*, 1991, **113**, 590–597.
- **9 Houpert, L.** Piezoviscous-rigid rolling and sliding traction forces; application: the rolling element–cage pocket contact. *ASME I. Tribol.*, 1987, **109**, 363–371.
- **10 Houpert, L.** CAGEDYN: a contribution to roller bearing dynamic calculations. Part I: basic tribology concepts. *STLE Tribol. Trans.*, 2010, **53**(1), 1–9.
- **11 Houpert, L.** CAGEDYN: a contribution to roller bearing dynamic calculations. Part II: description of the numerical tool and its outputs. *STLE Tribol. Trans.*, 2010, **53**(1), 10–21.
- 12 Dowson, D. and Higginson, G. R. Elastohydrodynamic lubrication, the fundamentals of roller and gear lubrication, 1966 (Pergamon Press, Oxford, UK).
- **13 Venner, C. H.** and **Lubrecht, A. A.** *Multilevel methods in lubrication*, Elsevier Tribology Series (Ed. D. Dowson), vol. 37, 2000 (Elsevier, Amsterdam).
- **14 Dalmaz, G.** *Le film mince visqueux dans les contacts hertziens en régimes hydrodynamique et élastohydrodynamique.* Docteur d'Etat Es Science thesis: INSA-Lyon, no. I-DE-7907, Lyon, 1979.
- **15 Dowson, D.** Session 3: Hertzian conditions, paper 10, elastohydrodynamics. In Proceedings of the Institution of Mechanical Engineers, International Conference on *Lubrication and wear: fundamentals and applications to design*, 1968, vol. 182, part 3A, pp. 151–167.
- **16 Hamrock, B. J.** and **Jacobson, B. O.** Elastohydrodynamic lubrication of line contact. *ASLE Trans.*, 1984, **24**(4), 275–287.
- 17 Goksem, P. G. and Hargreaves, R. A. The effect of viscous shear heating in both film thickness and rolling traction in an EHL line contact. *ASME J. Lubr. Technol.*, 1978, 100, 346–358
- **18 Pan, P.** and **Hamrock, B. J.** Simple formulae for performance parameters used in elastohydrodynamic lubricated line contact. *ASME Trans.*, 1989, **111**, 246–251.

## APPENDIX 1

#### Notation

| $b$ $E_1, E_2$ $E'$      | Hertzian contact half width along <i>x</i> (m)<br>Young modulus for bodies 1 and 2 (Pa)<br>equivalent Young's modulus (Pa) |
|--------------------------|----------------------------------------------------------------------------------------------------------------------------|
| J                        | force (N)                                                                                                                  |
| <i>J</i> 1 ~             | force per unit length (N/m)                                                                                                |
| $f_{ m i} \ F, 	ilde{F}$ | dimensionless force = $f_l/p_h b = f_l/E'R$                                                                                |
| G                        | dimensionless viscosity                                                                                                    |
|                          | $parameter = \alpha E'$                                                                                                    |
| h                        | gap height (m)                                                                                                             |
| $H,	ilde{H}$             | dimensionless gap                                                                                                          |
|                          | $height = hR/b^2 = h/R$                                                                                                    |
| $H_0, 	ilde{H_0}$        | dimensionless mutual approach                                                                                              |
| L                        | Moes parameter = $GU^{1/4}$                                                                                                |
| $\mathcal L$             | contact length (m)                                                                                                         |
| $M_{ m l}$               | Moes parameter = $W_1/\sqrt{U}$                                                                                            |
| p                        | pressure (Pa)                                                                                                              |
| Ρ                        | pressure (ru)                                                                                                              |

(48)

| $p_{ m h}$               | maximum Hertzian pressure (Pa)                   |
|--------------------------|--------------------------------------------------|
| $P, \tilde{P}$           | dimensionless pressure = $p/p_h = p/E'$          |
| $\frac{R}{R}$            | reduced radius of curvature (m)                  |
| $\overline{R}$           | dimensionless reduced radius of                  |
|                          | curvature = R/b                                  |
| $R_1$ , $R_2$            | radii of curvature along $X$ for body 1          |
|                          | and 2 (m)                                        |
| S                        | starvation coefficient = $H_a/H_{\rm m}^*$       |
| t                        | moment (Nm)                                      |
| $t_{ m l}$               | moment per unit length (N)                       |
| $T$ , $	ilde{T}$         | dimensionless                                    |
|                          | $moment = t_1/p_h b^2 = t_1/E'R^2$               |
| $u_{ m m}$               | mean surface velocity (m/s)                      |
| U                        | dimensionless velocity = $2\eta_0 u_{\rm m}/E'R$ |
| w                        | load (N)                                         |
| $w_1$                    | load per unit length (N/m)                       |
| $W_{\mathrm{l}}$         | dimensionless load = $w_1/E'R$                   |
| x, y                     | coordinates (m)                                  |
| $X, \tilde{X}$           | dimensionless coordinates = $x/b = x/R$          |
| $X_a, X_b$               | dimensionless domain boundaries                  |
|                          | along X                                          |
|                          | O                                                |
| α                        | pressure viscosity index (Pa <sup>-1</sup> )     |
| $\overline{\alpha}$      | dimensionless viscosity index = $\alpha p_h$     |
| γ̈́                      | shear rate ( $s^{-1}$ )                          |
| $\delta'$                | shift of the pressure center (m)                 |
| $\eta$                   | viscosity (Pa.s)                                 |
| $\frac{\eta}{\eta}$      | dimensionless viscosity = $\eta/\eta_0$          |
| $\eta_0$                 | viscosity at ambient pressure (Pas)              |
| $\theta$                 | power losses per unit area (W/m²)                |
|                          | dimensionless power losses                       |
| $\frac{\Theta}{\lambda}$ | dimensionless parameter of Reynolds              |
| ,,                       | equation = $12 u_m \eta_0 R^2 / b^3 p_h$         |
| $v_1, v_2$               | Poisson coefficient for body 1 and 2             |
| $\rho$                   | density (kg/m³)                                  |
| •                        | density at ambient pressure (kg/m <sup>3</sup> ) |
| $\frac{ ho_0}{ ho}$      | dimensionless density = $\rho/\rho_0$            |
| au                       | lubricant shear stress (Pa)                      |
| ı                        | idoffeditt siledi stress (1 d)                   |
| Superscripts             |                                                  |
| •                        |                                                  |

## Subscripts

| $o_a, o_b$                     | domain boundary (inlet), domain     |
|--------------------------------|-------------------------------------|
|                                | boundary (outlet)                   |
| $o_{\rm n}, o_{\rm t}$         | normal component, tangential        |
|                                | component                           |
| $o_{	ext{IVR}}, o_{	ext{EHL}}$ | isoviscousrigid, elastohydrodynamic |
| $o_c, o_{\mathrm{m}}$          | central, minimum                    |
| $o_S$                          | undeformed                          |
|                                |                                     |

## **APPENDIX 2**

The equivalent force and moment integrals for the classical dimensionless parameters

$$\begin{split} \tilde{F}_{n}^{y} &= \int_{-\infty}^{+\infty} \tilde{P}\left(\tilde{X}\right) \, \mathrm{d}\tilde{X} \\ \tilde{F}_{n}^{x} &= -\int_{-\infty}^{+\infty} \tilde{P}\left(\tilde{X}\right) \frac{\mathrm{d}\tilde{H}\left(\tilde{X}\right)}{\mathrm{d}\tilde{X}} \, \mathrm{d}\tilde{X} \\ \tilde{F}_{t}^{y} &= -\frac{1}{2} \int_{-\infty}^{+\infty} \tilde{H}\left(\tilde{X}\right) \frac{\mathrm{d}\tilde{P}\left(\tilde{X}\right)}{\mathrm{d}\tilde{X}} \, \mathrm{d}\tilde{H}\left(\tilde{X}\right)}{\mathrm{d}\tilde{X}} \, \mathrm{d}\tilde{X} \\ \tilde{F}_{t}^{x} &= -\frac{1}{2} \int_{-\infty}^{+\infty} \tilde{H}\left(\tilde{X}\right) \frac{\mathrm{d}\tilde{P}\left(\tilde{X}\right)}{\mathrm{d}\tilde{X}} \, \mathrm{d}\tilde{X} \\ \tilde{T}_{n}^{y} &= \int_{-\infty}^{+\infty} \tilde{X}\tilde{P}\left(\tilde{X}\right) \, \mathrm{d}\tilde{X} \\ \tilde{T}_{n}^{x} &= -\int_{-\infty}^{+\infty} \left(1 + \tilde{H}_{0} - \tilde{H}\left(\tilde{X}\right)\right) \tilde{P}\left(\tilde{X}\right) \frac{\mathrm{d}\tilde{H}}{\mathrm{d}\tilde{X}} \, \mathrm{d}\tilde{X} \\ \tilde{T}_{t}^{y} &= -\frac{1}{2} \int_{-\infty}^{+\infty} \tilde{X}\tilde{H}\left(\tilde{X}\right) \frac{\mathrm{d}\tilde{P}\left(\tilde{X}\right)}{\mathrm{d}\tilde{X}} \, \mathrm{d}\tilde{X} \\ \tilde{T}_{t}^{x} &= -\frac{1}{2} \int_{-\infty}^{+\infty} \left(1 + \tilde{H}_{0} - \tilde{H}\left(\tilde{X}\right)\right) \tilde{H}\left(\tilde{X}\right) \frac{\mathrm{d}\tilde{P}\left(\tilde{X}\right)}{\mathrm{d}\tilde{X}} \, \mathrm{d}\tilde{X} \end{split}$$

| $o^x$ , $o^y$ | along $x$ , along $y$ |
|---------------|-----------------------|
| $o^*$         | fully flooded limit   |