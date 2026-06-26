![](_page_0_Picture_0.jpeg)

![](_page_0_Picture_1.jpeg)

*Article*

# **Effects of Roller End/Rib Curvature Ratio on Friction and Accuracy in Tapered Roller Bearings**

**Wenhu Zhang \* and Gang Li**

School of Mechatronics Engineering, Henan University of Science and Technology, Luoyang 471000, China **\*** Correspondence: zwh@haust.edu.cn

# **Abstract**

To address the uncertainty in selecting the optimal spherical base curvature radius (*SR*) for tapered roller bearings, this study develops dynamic and friction torque models under combined loading conditions to evaluate three *SR* configurations—0.85*ρ*p, 0.90*ρ*p, and 0.95*ρ*p, where *ρ*p represents the curvature radius of the inner rib—in terms of load capacity, friction losses, and operational precision. The results indicate that (1) the 0.85*ρ*p configuration minimizes friction by optimizing the contact zone's *f<sup>V</sup>* parameter under combined loads, making it ideal for low-friction applications; (2) the 0.95*ρ*<sup>p</sup> design achieves superior operational accuracy; (3) the intermediate value of 0.90*ρ*<sup>p</sup> offers an optimal compromise, balancing friction torque reduction with operational precision. These findings establish quantitative guidelines for *SR* selection based on specific bearing performance requirements.

**Keywords:** tapered roller bearing; curvature radius of roller spherical reference surface; friction torque; running accuracy

# **1. Introduction**

Tapered roller bearings (TRBs) are extensively employed in transmission systems owing to their exceptional load-carrying capacity, demountability, and adjustable clearance. Significant research efforts have focused on optimizing their mechanical performance, including load capacity, fatigue resistance, stiffness, and load distribution [\[1](#page-16-0)[–4\]](#page-16-1). With growing emphasis on energy efficiency, secondary contact interfaces—particularly the roller end/rib interaction—have emerged as critical determinants of tribological performance. Under high axial loads, this contact dominates frictional losses and power dissipation [\[5](#page-16-2)[,6\]](#page-16-3), necessitating detailed analysis of its unique geometric and kinematic behavior.

The roller end/rib interface exhibits fundamentally distinct behavior compared to classical elasto-hydrodynamic lubrication (EHL) point contacts. Its operation under combined moderate loads and high sliding velocities requires specialized consideration of geometric pairing effects and spinning friction [\[7\]](#page-16-4). Research has established that curvature radii and geometric configurations profoundly influence tribological performance, affecting both load capacity and friction losses. Quasi-static and dynamic modeling approaches have been widely adopted to quantify contact loads and sliding velocities. Gupta's foundational work [\[8\]](#page-16-5) derived roller motion equations demonstrating the direct relationship between frictional behavior and roller skew, later refined by Majdoub et al. [\[9\]](#page-16-6), who identified tangential flange forces as the primary skew mechanism. Subsequent studies [\[10](#page-16-7)[–12\]](#page-16-8) developed nonlinear dynamic models revealing three key trends: (i) friction power scales positively with rim angle, (ii) larger roller end radii reduce friction, and (iii) flange contact friction governs

![](_page_0_Picture_12.jpeg)

Academic Editor: Walter D'Ambrogio

Received: 6 August 2025 Revised: 9 September 2025 Accepted: 16 September 2025 Published: 2 October 2025

**Citation:** Zhang, W.; Li, G. Effects of Roller End/Rib Curvature Ratio on Friction and Accuracy in Tapered Roller Bearings. *Machines* **2025**, *13*, 910. [https://doi.org/10.3390/](https://doi.org/10.3390/machines13100910) [machines13100910](https://doi.org/10.3390/machines13100910)

**Copyright:** © 2025 by the authors. Licensee MDPI, Basel, Switzerland. This article is an open access article distributed under the terms and conditions of the Creative Commons Attribution (CC BY) license [\(https://creativecommons.org/](https://creativecommons.org/licenses/by/4.0/) [licenses/by/4.0/\)](https://creativecommons.org/licenses/by/4.0/).

Machines 2025, 13, 910 2 of 18

skew generation. Wingertszahn et al. [13] advanced these insights through multibody simulations, confirming the pivotal role of curvature matching in contact mechanics.

At low speeds, mixed lubrication conditions prevail, with sliding friction dominating roller/rib interactions [14]. EHL analyses by Ibryaeva et al. [15] and Fujiwara et al. [16] converged on optimal radius ratios (0.6~0.8 and 0.6~0.85, respectively), balancing fluid film retention and skew mitigation. Recent methodological innovations include Li et al.'s [17] unified friction coefficient model spanning dry-to-EHL regimes, and Liu et al.'s [18] thermal EHL framework incorporating Carreau rheology, which identified an optimal curvature ratio minimizing friction. Experimental techniques have similarly progressed, with Cai et al. [19] quantifying rib/roller friction dominance in heavy-load conditions, while Majdoub et al. [20] enabled precise skew measurement via inductive sensors. Collectively, these works demonstrate that spherical/toroidal roller ends with medium radii (on toroidal ribs) optimize film thickness and load capacity; a fundamental trade-off exists between these objectives.

Despite these advances, no systematic methodology exists to reconcile operational precision with friction optimization through SR selection. This study addresses this gap by developing a principled framework for matching macro-geometric parameters in roller end/rib contacts, accounting for complex kinematics and mating geometry constraints. The proposed framework integrates advanced numerical simulation tools with foundation tests, offering a comprehensive approach to enhancing the energy efficiency and operational stability of TRBs in diverse applications.

#### 2. Materials and Methods

This study adopts a high-fidelity modeling approach where all bearing components are represented with their true geometries. The present work describes the most detailed model configuration, explicitly defining (i) component bodies and reference frames, (ii) force interactions, and (iii) system boundary conditions. All geometric parameters, material properties, and lubricant characteristics are obtained directly from the computational core.

#### 2.1. Geometric Relationship

Figure 1 shows the geometric relationship of tapered roller bearings: P is the cone top,  $FO_p$  is the axis, and PF is a round table surface of the bus bar.

<span id="page-1-0"></span>![](_page_1_Picture_8.jpeg)

Figure 1. Geometric relationship of tapered roller bearings.

The formula for calculating the angle  $\lambda$  between the cone and the end face of the large rib of the inner circle is as follows [21]:

$$\lambda = \sin^{-1} \left[ \frac{d_i' + d_2}{4SR} - \frac{h \sin(\alpha - \varphi)}{SR} \right] \tag{1}$$

$$h = \rho_{\rm p}\cos\varphi - \sqrt{SR^2 - D_w^2/4} \tag{2}$$

*Machines* **2025**, *13*, 910 3 of 18

$$d_i' = 2\rho_{\rm p}\sin\beta \tag{3}$$

$$\rho_{\rm p} = \left(\frac{E}{2\tan\alpha} + T - a_0\right) / \cos(\alpha - 2\varphi) \tag{4}$$

In this formula, *E* is the nominal small inner diameter of the outer circle; *a*<sup>0</sup> is the thickness of the inner circle large flange; *φ* is the roller half cone angle; *β* is the angle between the inner raceway bus bar and its center line; *T* is the bearing assembly height; *d* ′ *i* is the maximum diameter of the raceway in the initial inner circle; *d*<sup>2</sup> is the diameter of the large flange of the inner circle; *α* is the nominal contact angle of the bearing; *D<sup>w</sup>* is the diameter of the big end of the roller.

Angle *ψ* between the cone of the inner circle and the raceway is

$$\psi = 90 - \lambda + \beta \tag{5}$$

The position of the contact point between the inner rim and the base surface of the roller ball is

$$H = \frac{SR}{\tan \psi - \arctan \zeta} \tag{6}$$

$$\zeta = \frac{\left[\frac{D_w}{2\tan\varphi} - \sqrt{SR^2 - D_w^2/4}\right]\sin\varphi}{\frac{d_i}{2\sin\beta}\left[\frac{D_w}{2\tan\varphi} - \sqrt{SR^2 - D_w^2/4}\right]\cos\varphi}$$
(7)

In this formula, *d<sup>i</sup>* is the maximum diameter of the inner ring raceway.

## *2.2. Contact Modeling*

The contact area requires proper discretization, for roller/raceway line contact; therefore, this study employs the roller slicing technique (RST) [\[11\]](#page-16-18), which accounts for edge loading and angular misalignment effects. In contrast, face-to-rib contact deviates significantly from ideal point/line conditions, necessitating a cell-based approach.

Figure [2a](#page-2-0) presents the fundamental model for bearing load transmission and rotational functionality, which determines the load-carrying capacity, fatigue life, stiffness, and frictional losses of the bearing. Figure [2b](#page-2-0) illustrates the secondary interactions among rollers, guiding ribs, and the cage within the bearing assembly. These interactions become particularly critical under high-speed operation, light loading conditions, or inadequate lubrication, and may serve as primary contributors to bearing failure. These two models represent core concepts in the dynamic analysis of rolling bearings, characterizing the complex force and kinematic relationships among internal components during bearing operation.

<span id="page-2-0"></span>![](_page_2_Picture_13.jpeg)

(**a**) Roller/race interaction model (**b**) Roller/flange/cage interaction model

**Figure 2.** Force analysis model.

Machines 2025, 13, 910 4 of 18

### 2.2.1. Roller/Race Contact Model

According to Hertz's theory and Palmgren's work, normal force between the mth lamina of the jth roller and raceway can be calculated. The specific lamina's contact load per unit length is given by

$$\overline{q}_{jm}^{k} = \begin{cases} \frac{\left(\delta_{jm}^{k}\right)^{1.11}}{A^{1.11}L^{0.11}}, & \delta_{jm}^{k} > 0\\ 0, & \delta_{jm}^{k} \le 0 \end{cases}$$
(8)

where k = o(i) represents the outer (inner) raceway; A denotes the deformation constant; L signifies the effective length of the roller; and  $\delta^k_{jm}$  symbolizes the elastic deformation between the mth lamina of the jth roller and raceway.

The normal force of the jth roller and raceway is

$$Q_j^{\mathbf{k}} = \sum_{m=1}^{N_{\mathbf{P}}} \left(\delta_{jm}^{\mathbf{k}}\right)^{1.11} \frac{w}{L^{0.11} A^{1.11}} j = 1, \cdots, N_{\mathbf{Z}}$$
(9)

The integrated friction force per lamina yields the magnitude of the friction force as follows:

$$F_{sj}^{k} = \sum_{m=1}^{N_{\rm P}} \mu_{k} \overline{q}_{jm}^{k} \tag{10}$$

where  $N_{\rm P}$  represents the total number of laminas,  $N_{\rm Z}$  the total number of rollers, and w the width of each lamina, calculated as  $q = L/N_{\rm P}$ . Additionally,  $\mu_{\rm k}$  denotes the traction coefficient, as detailed in Section 2.3.

### 2.2.2. Roller Large End/Inner Ring Back Face Rib Contact Model

Hertz's elliptical contact theory is employed to ascertain the contact force arising from the interaction, with the normal force [22] as specified in Equation (11):

$$Q_{\rm fj} = \frac{\pi E' k}{3} \left[ 2\varepsilon R_{\rm eff} \left( \frac{\delta_{\rm fj}}{F} \right)^3 \right]^{\frac{1}{2}} \tag{11}$$

The meaning of the variables in the formula is shown in Appendix A.

The friction force of the roller end and flange is

$$F_{fi} = \mu_f Q_{fi} \tag{12}$$

where  $\mu_f$  is the friction coefficient (see Section 2.3).

### 2.2.3. Roller/Cage Contact Model

The normal force is

$$Q_{cj} = \begin{cases} 0 & (\delta_{cjm} \le 0) \\ \sum_{m=1}^{N_{P}} (\delta_{cjm})^{1.11} \frac{w}{L^{0.11} A^{1.11}} & (\delta_{cjm} > 0) \end{cases}$$
(13)

where  $\delta_{cjm}$  is the contact deformation between the mth lamina of the jth roller and cage pocket.

The friction force is given by Equation (14):

$$F_{ci} = \mu_c Q_{ci} \tag{14}$$

Machines 2025, 13, 910 5 of 18

 $\mu_{\rm c}$  is the friction coefficient (see Section 2.3).

# <span id="page-4-0"></span>2.3. Traction Coefficient

In the elasto-hydrodynamic lubrication framework, the traction coefficient is a time-dependent variable that depends on the slide/roll ratio and other parameters. It is determined using an experimental regression formula provided in Ref. [23]. The traction coefficient is given by Equation (15):

$$\mu = (A_u + B_u s)e^{-sC_u} + D_u \tag{15}$$

where  $A_u$ ,  $B_u$ ,  $C_u$  and  $D_u$  are the functions dependent on the normal load, the lubricant's inlet temperature, and the velocity of the two contacting objects, respectively; s represents the slide/roll ratio.

The traction coefficient under the boundary lubrication model is given by Equation (16):

$$\mu = (-0.1 + 22.28s)e^{-181.46s} + 0.1 \tag{16}$$

As shown in Ref. [24], the traction coefficient within the mixed lubrication model is determined through a smooth interpolation, utilizing coefficients from both the boundary and hydrodynamic lubrication models.

# 2.4. Damping Considerations

Due to complex influencing factors, parametric damping models are adopted. Material damping arises from elastic deformations, while lubricant film damping occurs in the elastohydrodynamic lubrication (EHL) contact zone [25].

This study employs a parametric damping model for interactions between raceways, ribs, and cage pockets. The damping force is governed by two key parameters: the maximum damping coefficient ( $d_{max}$ ) and the critical penetration depth ( $\delta_{max}$ ), beyond which damping saturates. The model is applied locally to each disc or cell, depending on the contact geometry. Equations (17) and (18) employ a widely-used model that incorporates maximum damping saturation, based on the hyperbolic tangent (tanh) function. This model represents the ratio of the maximum damping force achievable at the contact interface to the relative velocity. At high relative velocities, the damping force asymptotically approaches a saturation limit, preventing unbounded increase.  $F_D$ : Damping force,  $d_{max}$ : Maximum damping coefficient.

$$\overrightarrow{F_D}\left(\overrightarrow{v_N}, \delta_{max}, d_{max}\right) = -\overrightarrow{v_N} \cdot d \tag{17}$$

$$d = \begin{cases} 0 & \text{for } \delta < 0\\ \frac{-2 \cdot d_{max}}{\delta_{max}^3} \cdot \delta^3 + \frac{3 \cdot d_{max}}{\delta_{max}^2} \cdot \delta^2 & \text{for } 0 \le \delta < \delta_{max}\\ d_{max} & \text{for } \delta \ge \delta_{max} \end{cases}$$
(18)

### 2.5. Bearing Friction Torque

Friction torque mainly includes the contact friction torque between the roller and the raceway, the elastic hysteresis friction torque, the contact friction torque between the base surface of the big end ball of the roller and the big guard edge, the collision friction torque of the cage, and the viscous resistance moment of the lubricating oil [10,26].

There are both friction resistance and viscous resistance moments of the lubricant on rollers between rollers and raceways. The friction power consumption generated by contact between rollers and raceways is as follows:

$$H_{\mathbf{r}}^{k} = \int_{0}^{2\pi} \left[ \left( \int_{-L_{s}/2}^{L_{s}/2} F_{sj}^{k} \Delta V_{jm}^{k} dx_{r} \right) + M_{j}^{k} \omega_{bj} \right] d\phi$$
 (19)

Machines 2025, 13, 910 6 of 18

$$M_j^k = 14.2E_0L_sR_0^2U^{0.75}G^{-0.04}W^{0.08}$$
(20)

where  $L_s$  is the effective contact length of the roller;  $\Delta V_{jm}^k$  represents the relative sliding speed between the roller and the raceway;  $\omega_{bj}$  is the angular rotation speed of the roller;  $E_0$  and  $R_0$  are the equivalent elastic modulus and radius of curvature between the contacting bodies, respectively; U, G, and W are the velocity, material, and load parameters, respectively.

Due to the elastic hysteresis property of the material, the energy loss generated by tapered roller bearings is

$$H_{\rm e}^{\rm k} = \int_0^{2\pi} \left[ \int_{-L_s/2}^{L_s/2} \xi \sqrt{\frac{\pi \overline{q}_{jm}^{\rm k}}{\eta D_{wjm}}} \delta_{jm}^{\rm k} |\omega_{\rm i} - \omega_{\rm c}| dx_r \right] d\phi \tag{21}$$

where  $\xi$  is the elastic hysteresis coefficient of the material, which is desirable for steel  $\xi = 0.007$ ;  $\eta$  represents the combined elastic coefficient of the two elastomers;  $D_{wjm}$  is the diameter of the k-th segment of the roller;  $\omega_i$  is the inner circle's angular velocity;  $\omega_c$  is the angular velocity of the cage.

Relative sliding speed occurs when the base surface of the roller ball contacts with the inner circle's large retaining edge, generating the following energy loss:

$$H_{\rm f} = \int_0^{2\pi} \left( \mu_{\rm fj} Q_{\rm fj} \Delta V_{\rm fj} \right) d\phi \tag{22}$$

where  $\Delta V_{fj}$  is the relative sliding speed at the contact point between the big end face of the roller and the big rib of the inner circle (see Appendix B).

There are a collision and relative sliding between the roller and the cage pocket hole, and the resulting friction power consumption is

$$H_{c} = \int_{0}^{2\pi} \left[ \int_{-L_{s}/2}^{L_{s}/2} F_{cjm} \Delta V_{cjm} dx_{r} \right] d\phi$$
 (23)

where  $F_{cjm}$  is the tangential friction force between the roller and the hole;  $\Delta V_{cjm}$  is the relative sliding speed between the roller and cage hole.

The friction power consumption caused by the oil/gas resistance of the roller is

$$H_{\text{oil}} = \int_0^{2\pi} \frac{1}{8} C_d \rho_m D_{\text{w}} l(d_m \omega_{\text{c}})^2 d\phi$$
 (24)

where  $\rho_m$  is the density of oil and gas mixture;  $C_d$  is the resistance coefficient around the flow.

According to the law of conservation of energy, the total friction torque of tapered roller bearing is

$$M_{\rm all} = \frac{H_{\rm r}^k + H_{\rm e}^k + H_{\rm f} + H_{\rm c} + H_{\rm oil}}{\omega} \tag{25}$$

In this formula,  $\omega$  is the bearing speed.

#### 2.6. Dynamic Differential Equations

Figure 3 shows the forces and moments acting on the *j*th tapered roller.

Machines **2025**, 13, 910 7 of 18

<span id="page-6-0"></span>![](_page_6_Picture_1.jpeg)

Figure 3. Forces and moments acting on a tapered roller.

Based on Newton's law of motion, the dynamic differential equations of the translational motion of the tapered roller can be expressed in the Cartesian coordinates as follows:

$$m_{b}\ddot{x}_{bj} = -(Q_{j}^{i} + Q_{j}^{o})\sin\varphi + Q_{fj}\cos(\alpha_{m} - \lambda) - F_{cj}\sin\alpha_{m} - Q_{cj}\sin\varphi$$

$$m_{b}\ddot{y}_{bj} = (Q_{j}^{i} - Q_{j}^{o})\cos\varphi + Q_{fj}\sin(\alpha_{m} - \lambda) + F_{cj}\cos\alpha_{m} - Q_{cj}\cos\varphi\cos\alpha_{c} - \mu_{c}Q_{cj}\cos\varphi\sin\alpha_{c}$$

$$m_{b}\ddot{z}_{bj} = F_{sj}^{o} - F_{sj}^{i} + F_{fj} + F_{dj} + Q_{cj}\cos\varphi\sin\alpha_{c} - \mu_{c}Q_{cj}\cos\varphi\cos\alpha_{c}$$
(26)

where  $m_b$  is the mass of tapered roller;  $\ddot{x}_{bj}$ ,  $\ddot{y}_{bj}$ ,  $\ddot{z}_{bj}$  refer to the acceleration of the jth roller mass center;  $\omega$  is the inner circle's angular velocity.

The dynamic differential equations of the rotational motion of the jth tapered roller around its mass center and cage equilibrium can be seen in Appendix C.

# 3. Result Analysis

This paper takes 32,208 tapered roller bearing as the research object. The main parameters are shown in Table 1. The axial load is 12 kN, the radial load is 24 kN, and the inner circle speed is 6000 r/min. SARB rolling bearing dynamic simulation analysis software v.2.1 was used to conduct the bearing performance simulation analysis [27]. The simulation calculation model was referred to in Ref. [28].

<span id="page-6-1"></span>Table 1. Major parameters of bearing.

| Argument                                                             | Numerical Value |
|----------------------------------------------------------------------|-----------------|
| Inside diameter/Outside diameter (mm)                                | 40.0/80.0       |
| Bearing width (mm)                                                   | 24.75           |
| Inner circle width (mm)                                              | 23.0            |
| Outer circle width (mm)                                              | 19.0            |
| Radius of the sphere base (mm)                                       | 144.0           |
| Large flange Angle                                                   | 89°28′55″       |
| Large rib height (mm)                                                | 2.706           |
| Roller small/big end diameter (mm)                                   | 9.46/10.63      |
| Contact angle between roller and inner circle/outer circle/large rib | 10/14/87        |
| Number of rollers                                                    | 17              |
| Rated dynamic load                                                   | 81.1 kN         |

#### 3.1. Contact State Analysis

The bearing is in the stable running stage, and the results are repeatable in each running cycle. In order to show the results of the stress and sliding speed of the bearing in the circular direction, the calculation results in a single cycle are described in this paper. In this paper, the load of a single roller is at maximum at the circular direction of  $0.50\pi$ .

*Machines* **2025**, *13*, 910 8 of 18

Figure [4](#page-7-0) shows the contact height between the base surface of the roller ball and the large flange of the inner circle, and its value is jointly affected by geometric parameters, load, and azimuth angle. The existing research shows that the contact height is between 1/3 and 1/2 of the effective height of the big gear side. The contact height of the roller with the largest load is the smallest and has a large difference with the change of *SR*, and the value of the non-contact area is large and has a small change with *SR*. The azimuth angle is defined as the clockwise angular measurement in a horizontal plane from a reference direction to the line of sight of a target.

<span id="page-7-0"></span>![](_page_7_Figure_2.jpeg)

**Figure 4.** Contact area height.

Figure [5](#page-7-1) shows the length of the contact area between the base surface of the roller ball and the large flange of the inner circle. The contact length of the maximum loaded part of the flange is 5.52 mm and the contact length of 0.95*ρ*p is 5.89 mm, with an increase of about 7%. The shorter contact length makes the sliding speed in the circular direction smaller and low friction torque easier to achieve.

<span id="page-7-1"></span>![](_page_7_Figure_5.jpeg)

**Figure 5.** Contact area length.

*Machines* **2025**, *13*, 910 9 of 18

Figure [6](#page-8-0) shows the contact stress between the roller and the large flange, and its value is non-uniform distribution along the circumference direction, and the contact stress between the roller with the largest load and the large flange is also the largest. With the increase of *SR*, the contact stress between the roller and the large flange gradually decreases in the circular direction. In the range of 1.0π~2.0π, there is a small contact stress, which is because the bearing bears axial and radial loads at the same time, some rollers are not in the main bearing area, and the load is small.

<span id="page-8-0"></span>![](_page_8_Figure_2.jpeg)

**Figure 6.** Contact stress.

Figure [7](#page-8-1) shows the relative sliding speed at the contact point between the roller and the large flange. In the range of 0.0π~1.0π, the relative sliding speed increases with the increase of *SR*, and the maximum loading increase is about 6%. The relative sliding speed in the circumference direction is not fixed, and the sliding speed is the lowest in the area with a large load, and the sliding speed will change greatly when entering and leaving the bearing area, which will lead to the instability of the roller force for a short time.

<span id="page-8-1"></span>![](_page_8_Figure_5.jpeg)

**Figure 7.** Relative sliding speed of the contact area.

*Machines* **2025**, *13*, 910 10 of 18

Figure [8](#page-9-0) shows the *PV* value of the contact area between the large flange of the inner circle and the base surface of the roller ball, which is defined as the product of contact stress *P* between the roller and the large flange and sliding speed *V* at the contact point. The *PV* value is usually used to describe the wear degree and heat degree of the two contact points. In the range of 0~π and π~2π, the dominant values are the *P* value and *V* value, and the increase of *SR* can reduce the *PV* value of the roller and the large flange.

Figure [9](#page-9-1) shows the *fV* value of the contact area between the large flange of the inner circle and the base surface of the roller ball, which is defined as the product of the friction force *f* between the base surface of the roller ball and the large flange of the inner circle and the sliding speed *V* at the contact point. With the increase of SR, the *fV* value increases significantly in the bearing area, increasing by about 17% at 0.5π, which is the main reason for the difference in bearing friction torque. The smaller *SR* increases the wedge clearance between the roller and the large flange, and the relative sliding speed of the contact between the roller ball base and the large flange decreases (as shown in Figure [7\)](#page-8-1), and the friction coefficient decreases (as shown in Figure [9\)](#page-9-1), further reducing the friction torque of the bearing.

<span id="page-9-0"></span>![](_page_9_Figure_3.jpeg)

**Figure 8.** *PV* value of the contact area.

<span id="page-9-1"></span>![](_page_9_Figure_5.jpeg)

**Figure 9.** *fV* value and friction coefficient of contact area.

*Machines* **2025**, *13*, 910 11 of 18

# *3.2. Analysis of Running Accuracy*

Under the conditions of given load and speed, the running accuracy of tapered roller bearings can be judged by the inner circle centroid trajectory and cage centroid trajectory. The inner circle speed is selected as 6000 r/min, the axial load is 12 kN, and the radial load is changed. Figures [10](#page-10-0) and [11](#page-11-0) show the centroid trajectory of the bearing inner circle and cage.

<span id="page-10-0"></span>![](_page_10_Figure_3.jpeg)

**Figure 10.** Inner circle centroid trajectory.

Machines 2025, 13, 910 12 of 18

<span id="page-11-0"></span>![](_page_11_Figure_1.jpeg)

Figure 11. Cage centroid trajectory.

Figure 10 shows the inner circle centroid trajectory, and it defines the axial and radial load ratio c = Fa/Fr to describe the load state of the bearing. Figure 10a–f show the orbit of the centroid of the inner circle when c = 0.35, 0.7, 1.0, 1.35, 1.7, 2.1, respectively. In these figures, the x-axis represents the Y displacement of inner circle, while the y-axis represents the Z displacement of inner circle. As shown in Figure 10, the absolute value of Y displacement gradually increases due to the increase of radial load, while the orbit radius of the inner circle centroid increases and becomes irregular, indicating that the running accuracy of the bearing decreases.

Figure 11 shows the centroid trajectory of the cage when c = 1.0. At this time, the centroid trajectory of the cage is close to a circle, and the centroid trajectory of the cage is much larger than the change of the centroid trajectory of the inner circle. Due to the complexity of the description of the bearing component centroid trajectory, the RMS value of the inner circle/cage centroid trajectory radius  $R_{st}$  in the time domain is used to describe the running accuracy of the bearing. Among them,  $R_{St} = \sqrt{SY_t^2 + SZ_t^2}$ ; S represents the inner circle, cage, and roller components;  $SY_t$ ,  $SZ_t$  represent the Y-direction and Z-direction displacement of the component at time t.

Figure 12 shows the root mean square (RMS) values of the centroid locus radius, respectively. The variation rule of the RMS value of the inner circle centroid trajectory and the roller centroid trajectory is consistent, and the RMS value of the centroid trajectory radius increases with the increase of *SR* and *c*. The RMS value of the cage centroid trajectory radius shows the opposite trend to that of the inner circle and roller centroid trajectory. The increase of the cage centroid trajectory radius is mainly due to the influence of the collision force between the roller and the cage, as the cage is in a suspended state during operation.

Figure 13 shows the friction torque of the bearing under different loads when the inner circle speed is 6000 r/min. Under the same main structural parameters and loading conditions of the bearing, the force between the rollers and the raceway is basically the same. The contact state between the large flange and the spherical base surface of the roller is the main reason for the difference in friction torque. When the axial load/rated dynamic load is 0.32 and c = 1.70, the total friction torque of the bearing decreases by approximately 9.7%. Moreover, the reduction is more significant under larger loads. As SR increases, the friction torque of the bearing gradually increases, mainly due to the changes in the contact force, contact position, sliding speed, and friction coefficient between the spherical base surface of the roller and the large flange, which causes the friction torque of the bearing to gradually increase with the increase of SR.

*Machines* **2025**, *13*, 910 13 of 18

<span id="page-12-0"></span>![](_page_12_Figure_1.jpeg)

**Figure 12.** Centroid trajectory radius RMS value.

<span id="page-12-1"></span>![](_page_12_Figure_3.jpeg)

**Figure 13.** Bearing frictional torque.

# **4. Test Verification**

The 32,208 bearings were tested using a rolling bearing performance testing machine. The spindle structure of the testing machine is shown in Figure [14.](#page-13-0) The bearings are installed in type X mode and lubricated by oil injection. The inner circle speed is 1800 r/min and the axial load Fa is 2.16 kN. The torque sensor is used to measure the total friction torque of the two sets of bearings. Bearings *SR*= 0.85*ρ*<sup>p</sup> and *SR*= 0.95*ρ*<sup>p</sup> were selected for grouping

*Machines* **2025**, *13*, 910 14 of 18

measurement, and the friction torque results during stable operation for 10 min were captured, as shown in Figure [15.](#page-13-1)

<span id="page-13-0"></span>![](_page_13_Figure_2.jpeg)

**Figure 14.** Spindle structure of the test machine.

<span id="page-13-1"></span>![](_page_13_Figure_4.jpeg)

**Figure 15.** Comparison of bearing friction torque test.

Figure [15](#page-13-1) shows the measurement results of the friction torque of a single set of tapered roller bearings when *SR* = 0.85*ρ*<sup>p</sup> and *SR* = 0.95*ρ*p. The average friction torques are 0.56 N·m and 0.61 N·m, respectively. When *SR* = 0.85*ρ*p, the friction torque decreases by 8.2%. However, the friction torque fluctuates more significantly with a smaller *SR* value, with the maximum fluctuation being approximately 0.075 N·m (fluctuation value/maximum value = 14.1%), which is higher than 0.06 N·m (8.9%) when *SR* = 0.95*ρ*p. This indicates that the bearing stability is lower with a smaller *SR* value, verifying the accuracy of the proposed model and the correctness of the analysis.

# **5. Conclusions**

In this paper, the contact state, bearing friction torque, and running accuracy between the ball base and the large rib of the roller ball were analyzed when the curvature radius *SR* of the roller ball base was 0.85*ρ*p, 0.90*ρ*p, and 0.95*ρ*p, and the basis for the value of *SR* under different performance design requirements was studied. The results are as follows:

(1) The larger *SR* makes the contact stress, contact height, and *PV* value of the contact area between the base surface of the roller ball and the large flange smaller, but the relative sliding speed and *fV* value between the roller ball base and the contact area of the large flange is higher, and the final bearing friction torque increases with the increase of *SR*.

*Machines* **2025**, *13*, 910 15 of 18

(2) The increase of the *SR* value can increase the running accuracy of the bearing to a certain extent; it is recommended to select a larger value under high radial load conditions.

(3) When *SR* is 0.85*ρ*p, the friction torque is small; when *SR* is 0.95*ρ*p, the bearing running accuracy is high. When *SR* is 0.90*ρ*p, the bearing friction torque and running accuracy can be balanced.

**Author Contributions:** Methodology, W.Z.; Writing—original draft, G.L.; Writing—review & editing, G.L.; Supervision, W.Z. All authors have read and agreed to the published version of the manuscript.

**Funding:** This research received no external funding.

**Data Availability Statement:** The original contributions presented in this study are included in the article. Further inquiries can be directed to the corresponding author.

**Conflicts of Interest:** The authors declare no conflicts of interest.

# <span id="page-14-0"></span>**Appendix A. Roller Large End/Inner Ring Back Face Rib Contact Model**

Hertz's elliptical contact theory is applied in order to calculate the contact force by the interaction. The normal force [\[15\]](#page-16-11) is given by Equation (A1):

$$Q_{fj} = \frac{\pi E' k}{3} \left[ 2\varepsilon R_{\text{eff}} \left( \frac{\delta_{fj}}{F} \right)^3 \right]^{\frac{1}{2}}$$
 (A1)

$$\varepsilon = \frac{0.5968}{R_{\eta}/R_{\xi}} + 1.0003 \tag{A2}$$

$$\frac{1}{R_{\text{eff}}} = \frac{1}{R_{\eta}} + \frac{1}{R_{\xi}} \tag{A3}$$

$$k = 1.0339 \left(\frac{R_{\eta}}{R_{\xi}}\right)^{0.636}$$
 (A4)

$$F = 0.6023 \ln \frac{R_{\eta}}{R_{\xi}} + 1.5277 \tag{A5}$$

$$\frac{1}{E'} = \frac{1}{2} \left( \frac{1 - u_1^2}{E_1} + \frac{1 - u_2^2}{E_2} \right) \tag{A6}$$

where *R<sup>η</sup>* and *R<sup>ξ</sup>* are the equivalent curvature radii of roller and flange on direction *ξ* and *η*; *E*<sup>1</sup> and *E*<sup>2</sup> are the elastic modulus of roller and flange; *u*<sup>1</sup> and *u*<sup>2</sup> are Poisson's ratio of roller and flange; *ε* is the elliptic integral of the first kind; *F* is the elliptic integral of the second kind.

# <span id="page-14-1"></span>**Appendix B. Sliding Speed of Large Flange**

According to the kinematic relationship of tapered roller bearings, the linear rotation velocity of the base surface of the roller ball at the contact point is as follows:

$$V_{rj} = S_1 \omega_{rj} = [0.5D_w - H\cos(\alpha_{av} - \lambda)]\omega_{rj}$$
(A7)

In this formula, *S*<sup>1</sup> is the distance between the contact point and the center of the roller; *ωrj* is the rotation angular speed of the *j*th roller.

Machines **2025**, 13, 910

![](_page_15_Picture_1.jpeg)

**Figure A1.** Contact state between the roller's spherical reference surface and the rib.

The linear velocity of revolution  $V_{bj}$  of the base surface of the roller ball at the contact point is

$$V_{bj} = R_f \omega_{bj} = [r_m - S_1 \cos \gamma] \omega_{bj}$$
 (A8)

In this formula,  $\omega_{bj}$  is the angular velocity of the *j*th roller.

The linear speed  $V_{fj}$  of the large flange of the inner circle at the contact point is

$$V_{fj} = R_f \omega_i = (r_m - S_1 \cos \gamma) \omega_i$$
 (A9)

In this formula,  $\omega_i$  is the angular velocity of the inner circle.

Then, the relative sliding speed V at the contact point between the base surface of the roller ball and the large flange of the inner circle is

$$V = \left| V_{fj} - V_{rj} - V_{bj} \right| \tag{A10}$$

### <span id="page-15-0"></span>Appendix C

Appendix C.1. Dynamic Differential Equations of Roller

The dynamic differential equations of rotational motion of the *j*th tapered roller around its mass center can be described by the classical Euler equations of motion as follows:

$$I_{1}\dot{\omega}_{bxj} - (I_{2} - I_{3})\dot{\omega}_{byj}\dot{\omega}_{bzj} = -(M_{sj}^{i} + M_{sj}^{o})\sin\varphi - M_{dj} + \frac{1}{2}(F_{sj}^{o} + F_{sj}^{i} + \mu_{c}Q_{cj})(D_{w} - 2L_{1}\tan\varphi) - F_{fj}S_{R}\sin(\alpha_{m} - \lambda) - J_{x}\frac{d\omega_{bxj}}{dt}$$
(A11)

$$I_{2}\dot{\omega}_{\mathrm{byj}} - (I_{3} - I_{1})\omega_{\mathrm{bzj}}\omega_{\mathrm{bxj}} =$$

$$(M_{\mathrm{sj}}^{\mathrm{o}} - M_{\mathrm{sj}}^{\mathrm{o}})\cos\varphi + F_{\mathrm{fj}}[S_{\mathrm{R}}\cos(\alpha_{\mathrm{m}} - \lambda) - S_{\mathrm{R}}\cos\lambda + l_{\mathrm{c}}]$$

$$+ \frac{1}{2}\mu_{\mathrm{c}}Q_{\mathrm{cj}}(D_{\mathrm{w}} - 2L_{1}\tan\varphi)\cos\alpha_{\mathrm{c}}\sin\lambda - M_{\mathrm{fj}} - J_{y}\frac{d\omega_{\mathrm{byj}}}{dt}$$
(A12)

$$I_{3}\dot{\omega}_{bzj} - (I_{1} - I_{2})\omega_{bxj}\omega_{byj} =$$

$$-\frac{1}{2}\mu_{c}Q_{cj}(D_{w} - 2L_{1}\tan\varphi)\sin\alpha_{c}\sin\varphi + M_{j}^{i} - M_{j}^{o} - M_{gj}$$

$$+Q_{fj}(S_{R}\cos\lambda - l_{c})\sin(\alpha_{m} - \lambda) - J_{z}\frac{d\omega_{bzj}}{dt}$$
(A13)

where  $I_1$ ,  $I_2$ ,  $I_3$  refer to the roller's principal moments of inertia;  $\omega_{bxj}$ ,  $\omega_{bxj}$ ,  $\omega_{bxj}$ ,  $\dot{\omega}_{bxj}$ ,  $\dot{\omega}_{bxj}$ ,  $\dot{\omega}_{bxj}$ ,  $\dot{\omega}_{bxj}$ ,  $\dot{\omega}_{bxj}$ ,  $\dot{\omega}_{bxj}$ ,  $\dot{\omega}_{bxj}$  are the jth roller;  $M_{gj}$  is the gyroscopic moment acting on the jth roller;  $F_{dj}$  and  $F_{dj}$  are the fluid drag force and moment acting on the jth roller, respectively.

Machines 2025, 13, 910 17 of 18

Appendix C.2. Cage Equilibrium

The cage equilibrium is obtained by summation of all fluid film forces and roller to cage acting on the cage. The differential equation of the cage motion is

$$J_{c}\frac{d\omega_{c}}{dt} = -\frac{1}{2}\mu_{c}\sum_{j=1}^{N_{Z}}Q_{cj}(d_{c}\cos\alpha_{c} + D_{w} - 2L_{1}\tan\varphi) + \frac{1}{2}\sum_{j=1}^{N_{Z}}Q_{cj}d_{c}\sin\alpha_{c} - M_{c}$$
(A14)

where  $J_c$  is polar moment of inertia of the cage and  $\omega_c$  is the angle velocity of the cage.

## References

- <span id="page-16-0"></span>1. Eugenio, D. Optimal design of tapered roller bearings for maximum rating life under combined loads. Mech. Ind. 2017, 18, 112.
- 2. Lostado-Lorza, R.; Escribano-Garcia, R.; Fernandez-Martinez, R.; Illera-Cueva, M.; Mac Donald, B.J. Using the finite element method and data mining techniques as an alternative method to determine the maximum load capacity in tapered roller bearings. *J. Appl. Log.* **2017**, 24, 4–14. [CrossRef]
- 3. Kalyan, M.; Tiwari, R.; Ahmad, M.S. Multi-objective optimization in geometric design of tapered roller bearings based on fatigue, wear and thermal considerations through genetic algorithms. *Sādhanā* **2020**, *45*, 142. [CrossRef]
- <span id="page-16-1"></span>4. Tiwari, R.; Sunil, K.K.; Reddy, R.S. An Optimal Design Methodology of Tapered Roller Bearings Using Genetic Algorithms. *Int. J. Comput. Methods Eng. Sci. Mech.* **2012**, *13*, 108–127. [CrossRef]
- <span id="page-16-2"></span>5. Alberto, P.; Nicolas, F.; Philippe, V.; Morales-Espejel, G.E. Influence of spin on film thickness in elastohydrodynamic starved point contacts. *Tribol. Int.* **2021**, *156*, 106825. [CrossRef]
- <span id="page-16-3"></span>6. Pan, C.; Shao, R.; Cao, Y. Recent Patents on Rolling Bearing Lubrication Device. Recent Pat. Eng. 2024, 18, 74–91. [CrossRef]
- <span id="page-16-4"></span>7. Sven, W.; Max, M.; Marcel, B.; Stahl, T.; Wartzack, S. Geometrical Optimization of the EHL Roller Face/Rib Contact for Energy Efficiency in Tapered Roller Bearings. *Lubricants* **2021**, *9*, 67. [CrossRef]
- <span id="page-16-5"></span>8. Pan, C.; Zhang, J.; Li, S. Review of Research on Dynamic Characteristics of Rolling Bearing Cages. *Recent Pat. Eng.* **2024**, *19*, 91–111. [CrossRef]
- <span id="page-16-6"></span>9. Majdoub, F.; Mevel, B. Kinematic equilibrium of rollers in tapered roller bearings. *Tribol. Trans.* **2019**, 62, 567–579. [CrossRef]
- <span id="page-16-7"></span>10. Deng, S.E.; Hu, G.C.; Dong, X. Research on Power Consumption Characteristics of Double-row Tapered Roller Bearings. *Acta Armamentarii* **2014**, *35*, 1888–1907.
- <span id="page-16-18"></span>11. Deng, S.; Gu, J.; Cui, Y.; Zhang, W. Dynamic analysis of a tapered roller bearing. Ind. Lubr. Tribol. 2018, 70, 191–200. [CrossRef]
- <span id="page-16-8"></span>12. Wu, Z.H.; Xu, Y.Q.; Deng, S.E. Analysis of Dynamic Characteristics of Grease-Lubricated Tapered Roller Bearings. *Shock. Vib.* **2018**, 2018, 7183042. [CrossRef]
- <span id="page-16-9"></span>13. Wingertszahn, P.; Koch, O.; Maccioni, L.; Concli, F.; Sauer, B. Predicting Friction of Tapered Roller Bearings with Detailed Multi-Body Simulation Models. *Lubricants* **2023**, *11*, 369. [CrossRef]
- <span id="page-16-10"></span>14. Manjunath, M.; Fauconnier, D.; Ost, W.; De Baets, P. Experimental Analysis of Rolling Torque and Thermal Inlet Shear Heating in Tapered Roller Bearings. *Machines* **2023**, *11*, 801. [CrossRef]
- <span id="page-16-11"></span>15. Ibryaeva, O.; Sinitsin, V.; Sakovskaya, V.; Eremeeva, V. A novel hybrid method for fault diagnosis of two rolling bearings mounted on the same shaft. *Meas. Sens.* **2021**, *18*, 100210. [CrossRef]
- <span id="page-16-12"></span>16. Fujiwara, H.; Tsujimoto, T.; Yamauchi, K. Optimized Radius of Roller Large End Face in Tapered Roller Bearings (Machine Elements, Design and Manufacturing). *JSMET* **2009**, *75*, 2319–2326. [CrossRef]
- <span id="page-16-13"></span>17. Li, X.; Liu, J.; Huang, S.; Pan, G. Friction moment calculation method for tapered roller bearings under combined loads. *Sci. China Technol. Sci.* **2024**, *67*, 2565–2578. [CrossRef]
- <span id="page-16-14"></span>18. Liu, X.; Long, T.; Li, X.; Guo, F. Thermal EHL analysis of the inner ring rib and roller end in tapered roller bearings with the Carreau model. *Front. Manuf. Technol.* **2023**, *2*, 1029860. [CrossRef]
- <span id="page-16-15"></span>19. Cai, G.; Hou, Y.; Wang, X.; Sun, S.; Zhang, Y.; Wang, N. A measurement method for friction torque between rollers and raceways of tapered roller bearings under radial heavy load conditions. *Tribol. Int.* **2024**, *200*, 110071. [CrossRef]
- <span id="page-16-16"></span>20. Majdoub, F.; Saunier, L.; Sidoroff-Coicaud, C.; Mevel, B. Experimental and numerical roller skew in tapered roller bearings. *Tribol. Int.* **2020**, *145*, 106142. [CrossRef]
- <span id="page-16-17"></span>21. Lostado, R.; García, E.R.; Martinez, F.R. Optimization of operating conditions for a double-row tapered roller bearing. *Int. J. Mech. Mater. Des.* **2016**, *12*, 353–373. [CrossRef]
- <span id="page-16-19"></span>22. Klecher, R.J. *High Speed Cylindrical Roller Bearing Analysis, SKF Program "CYBEAN"*; NASA-CR-159460; NASA: Washington, DC, USA, 1978; Volume I–II.
- <span id="page-16-20"></span>23. Wang, Y.S.; Yang, B.Y.; Wang, L.Q. Investigation into the traction coefficient in elastohydrodynamic lubrication. *TriboTest* **2004**, *11*, 113–124. [CrossRef]

*Machines* **2025**, *13*, 910 18 of 18

<span id="page-17-0"></span>24. Friskney, B.; Mohammadpour, M.; Theodossiades, S.; Craig, C.; Rapson, G. Effects of transmission shaft flexibility on rolling element bearing tribodynamics in a high-performance transmission. *Mech. Mach. Theory* **2021**, *165*, 104440. [\[CrossRef\]](https://doi.org/10.1016/j.mechmachtheory.2021.104440)

- <span id="page-17-1"></span>25. Vigliani, A.; Cavallaro, P.S.; Venturini, S. Modelling and Experimental Testing of Passive Magnetic Bearings for Power Loss Reduction. *Appl. Sci.* **2025**, *15*, 4149. [\[CrossRef\]](https://doi.org/10.3390/app15084149)
- <span id="page-17-2"></span>26. Deng, S.E.; Jia, Q.Y.; Xue, J.X. *Design Principle of Rolling Bearings*, 2nd ed.; China Standard Press: Beijing, China, 2014.
- <span id="page-17-3"></span>27. Wang, Y.; Wang, H.; Cui, Y.; Li, C.; Deng, S. Research on the thermal characteristics of large span double row tapered roller bearing. *Acta Armamentarii* **2024**, *45*, 1285. [\[CrossRef\]](https://doi.org/10.12382/bgxb.2022.1056)
- <span id="page-17-4"></span>28. Korolev, V.A.; Korolev, A.A.; Krehel, R. Mathematical simulation and analysis of rolling contact fatigue damage in rolling bearings. *Int. J. Adv. Manuf. Technol.* **2017**, *89*, 661–664. [\[CrossRef\]](https://doi.org/10.1007/s00170-016-9136-9)

**Disclaimer/Publisher's Note:** The statements, opinions and data contained in all publications are solely those of the individual author(s) and contributor(s) and not of MDPI and/or the editor(s). MDPI and/or the editor(s) disclaim responsibility for any injury to people or property resulting from any ideas, methods, instructions or products referred to in the content.