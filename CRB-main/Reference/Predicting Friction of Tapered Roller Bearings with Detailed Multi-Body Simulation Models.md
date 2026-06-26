![](_page_0_Picture_0.jpeg)

*Article*

# **Predicting Friction of Tapered Roller Bearings with Detailed Multi-Body Simulation Models**

**Patrick Wingertszahn 1,[\\*](https://orcid.org/0009-0001-8375-1605) , Oliver Koch <sup>1</sup> [,](https://orcid.org/0000-0001-5967-0242) Lorenzo Maccioni <sup>2</sup> [,](https://orcid.org/0000-0002-2368-6821) Franco Concli [2](https://orcid.org/0000-0002-1237-5542) and Bernd Sauer <sup>1</sup>**

- <sup>1</sup> Chair of Machine Elements, Gears and Tribology, RPTU Kaiserslautern-Landau, 67663 Kaiserslautern, Germany; oliver.koch@rptu.de (O.K.); bernd.sauer@rptu.de (B.S.)
- <sup>2</sup> Faculty of Engineering, Free University of Bozen-Bolzano, 39100 Bolzano, Italy; lorenzo.maccioni@unibz.it (L.M.); franco.concli@unibz.it (F.C.)
- **\*** Correspondence: patrick.wingertszahn@mv.rptu.de; Tel.: +49-631-205-4723

**Abstract:** In the presented work, a parametric multibody simulation model is presented that is capable of predicting the friction torque and kinematics of tapered roller bearings. For a highly accurate prediction of bearing friction, consideration of solid and lubricant friction is mandatory. For tapered roller bearings in particular, the friction in the contact between the rolling element and raceway is of importance. Friction forces in the contact between the rolling element end face and inner ring rib as well as roller cage pocket contacts are also considered in the model. A large number of tests were carried out to validate the model in terms of the simulated frictional torque. Influencing variables such as speed, axial load, radial load, and temperature were investigated. The simulation results show good agreement with the measured friction torque, which confirms that the model is well suited to predict frictional torques and therefore the kinematics of tapered roller bearings.

**Keywords:** rolling bearing; multi body simulation; tapered roller bearings; friction; tribology; dynamic simulation; modelling; power losses; experimental validation

![](_page_0_Picture_10.jpeg)

**Citation:** Wingertszahn, P.; Koch, O.; Maccioni, L.; Concli, F.; Sauer, B. Predicting Friction of Tapered Roller Bearings with Detailed Multi-Body Simulation Models. *Lubricants* **2023**, *11*, 369. [https://doi.org/10.3390/](https://doi.org/10.3390/lubricants11090369) [lubricants11090369](https://doi.org/10.3390/lubricants11090369)

Received: 21 July 2023 Revised: 28 August 2023 Accepted: 30 August 2023 Published: 1 September 2023

![](_page_0_Picture_13.jpeg)

**Copyright:** © 2023 by the authors. Licensee MDPI, Basel, Switzerland. This article is an open access article distributed under the terms and conditions of the Creative Commons Attribution (CC BY) license [\(https://](https://creativecommons.org/licenses/by/4.0/) [creativecommons.org/licenses/by/](https://creativecommons.org/licenses/by/4.0/) 4.0/).

### <span id="page-0-0"></span>**1. Introduction**

As part of the ongoing efforts to increase drive train efficiency, there is a general trend to minimize frictional losses in gearboxes. Besides gear teeth, one of the main aspects is the rolling contact of bearings [\[1,](#page-18-0)[2\]](#page-18-1). By reducing preload forces in adjusted bearing arrangements, friction losses are often reduced. A disadvantage of this approach is that when it comes to axial operating clearance, the bearing arrangement is subjected to onesided loads, or in the case of an O arrangement, the overall system experiences heating. The clearance has a significant impact on the operational behavior of one or both bearings. It is particularly noticeable in applications where the bearing is subjected to low loads and insufficient lubrication throughout its life cycle and where the rollers are relatively large and heavy. Under these conditions, there is an increased sliding component in the motion of the rolling elements. Combined with rapid speed and load changes, this increases the risk of bearing damage caused by slippage [\[3–](#page-18-2)[5\]](#page-18-3).

For the calculation of bearing losses, there are many approaches available. Over the past years, numerous studies have been carried out and precise contact models developed in order to make the friction losses in highly loaded contacts calculable [\[6](#page-18-4)[–16\]](#page-18-5). The first empirical equations for bearing friction calculation were formulated by Stribeck [\[17\]](#page-18-6), Sjovall [\[18\]](#page-18-7), Lundberg and Palmgren [\[19](#page-18-8)[–22\]](#page-19-0). Soon, more complex computer-based models had been developed. Jones [\[23\]](#page-19-1) and Harris [\[24\]](#page-19-2) were two of the drivers of the growth in bearing modeling theory. With the continuous increase in demand for higher efficiency and lower power loss, the optimization of rolling bearings has become more and more important. Analytical studies on bearings were often accompanied by experimental investigations to validate the analysis, as is also performed in this work.

*Lubricants* **2023**, *11*, 369 2 of 22

The most accurate friction calculation can be carried out with dynamic simulation models of roller bearings. A brief overview of the development of dynamics models can be found in [\[25](#page-19-3)[–27\]](#page-19-4). Nowadays, the most accurate models are the dedicated calculation tools developed by the bearing manufacturers. These detailed dynamic simulation models are also capable of predicting and analyzing conditions in which damages caused by the dynamics of the bearing components, as mentioned above, can occur. Among the best-known tools are BEAST (BEAring Simulation Tool) of the SKF company (Goteborg, Sweden) [\[28](#page-19-5)[–31\]](#page-19-6), BRAIN (BeaRing Analysis In NSK) of the NSK company (Maidenhead, England) [\[32](#page-19-7)[,33\]](#page-19-8), Caba3D (Computer Aided Bearing Analyzer 3D) of the Schaeffler company (Herzogenaurach, Germany) [\[34\]](#page-19-9), CAGEDYN of the Timken company (North Canton, OH, USA) [\[35–](#page-19-10)[37\]](#page-19-11) and IBDAS used by NTN (Osaka, Japan) [\[38\]](#page-19-12). Since these models are part of the company's know-how, the information regarding them is scarce or lacking in detail. Moreover, these models are not accessible for general research. In addition to the above-mentioned programs, there are a large number of models for the dynamic simulation of rolling bearings capable of taking into account both dynamic and contact in a single program. Most of these models have been developed for specific problems [\[39](#page-19-13)[–45\]](#page-19-14). Other studies using dynamic simulation models use simplified friction calculations and focus on structural deformation or vibration analysis [\[45](#page-19-14)[–57\]](#page-20-0). Therefore, their use in general problems could lead to unreliable results.

### **2. Materials and Methods**

As described in Section [1,](#page-0-0) the reduction of preload forces in adjusted bearing arrangements can be used to minimize frictional losses in gearboxes. In order to predict the frictional losses of Tapered Roller Bearings (TRBs) and prevent critical operational conditions, a Multi-Body Simulation (MBS) model is needed. Due to the fact that there are no publicly available models that are generally applicable, sufficiently accurate, and validated, a highly detailed MBS model for TRB has been developed in the present paper. The MBS model runs under the program name LaMBDA (Lager MehrkörperBerechnung und DynamikAnalyse). It is described in Section [2.1.](#page-1-0)

To compare the simulated results with experimentally measured ones and validate them under different operating conditions, friction torque measurements have been performed. The experimental setup used is described in Section [2.2.](#page-8-0) Simulation results and measurements are presented in Section [3](#page-11-0) and discussed in Section [4.](#page-13-0)

### <span id="page-1-0"></span>*2.1. Multi Body Simulation Model*

The MBS model was developed based on an approach that has been established for model development at MEGT (Chair of Machine Elements, Gears and Tribology). It is parameterized and modular and uses self-developed routines for calculation. Routines for such calculations to be used in MBS models have been developed and improved over the past 20 years at MEGT [\[58](#page-20-1)[–61\]](#page-20-2).

Within the model, a combination of commercial software and self-programmed calculation routines is used. Depending on the level of detail, all bodies of a rolling bearing are modeled with their real geometries. In this work, the most detailed model structure available is described. In order to define the bodies, markers, forces, and boundary conditions, a graphical user interface is used. It was also developed in this work and can access a database of selected bearings and lubricants. This allows for user-friendly model generation. The force elements of the model use the self-written routines to calculate the individual force components in the contact (normal force, damping, and friction). The flow chart in Figure [1](#page-2-0) shows a simplified diagram of the procedure followed within the MBS model to calculate contact forces between the single elements by means of roller and ring raceway contact.

Lubricants 2023, 11, 369 3 of 22

<span id="page-2-0"></span>![](_page_2_Figure_1.jpeg)

Figure 1. Flow chart of contact force calculation between rolling elements and ring raceways [26].

In the MBS model geometry, material data and lubricant properties are defined. Together with the state parameters, this data is retrieved from the calculation core. In this case, state parameters mean, for example, the distance s'(t) between the coordinate systems of the individual bodies and their velocities  $\dot{s}(t)$ . Based on these, the contact between the rolling element and raceway is determined (see Section 2.1.1). The result values of the contact routine are contact point p, contact normal vector n and penetration  $\delta$ . With a load-deformation relationship, the effective contact normal force  $F_N$ . is determined from the penetration. In the next step, the time-dependent contact state parameters are calculated. These are the relative velocities  $\vec{u}_{rel}$  and sum velocities  $\vec{u}_{sum}$  in the contact point. The relative velocity is further used to calculate the damping forces  $F_D$  as described in Section 2.1.2. The normal forces, velocities, and lubricant data are used to determine the lubricant film height h in contact, the specific lubricant film height  $\Lambda$  and the solid load-bearing ratio  $\phi$ . On their basis frictional forces  $F_T$  and resulting torques  $M_T$  are calculated. The calculation of frictional forces for each contact in the model is presented in detail in Section 2.1.3. In the last step, the three components of the contact force  $F_{\Sigma}$  are summed up and given back to the MBS solver for iteration of the force equilibrium.

# <span id="page-2-1"></span>2.1.1. Contact Calculation

In order to correctly represent the contact between two bodies, the area of contact must be discretized. Depending on the type of contact, the discretization used in this model can be one- or two-dimensional. In the case of a roller ring raceway contact, where there is line contact, disc models are the state of the art and are also used here. Depending on the geometry pairing, the contact between the rolling element face and the ring rib deviates greatly from the idealized point or line contact. Therefore, a cell model is used to calculate this contact.

#### Slice Model

A conventional slice model is described in [62]. In this work, the Alternative Slicing Technique (AST) is used. It was first presented by Teutsch in [63] and subsequently implemented by Kiekbusch for different types of bearings in [26]. The AST model allows radial deformation of the slices toward each other. However, the discs cannot twist against each other. Thus, the deformation of each slice can be determined by its penetration into the contour of the opposite slice of the raceway. Conventional slice models neglect the influence that neighboring slices have on each other. As a result, they do not adequately represent the pressure distribution under bearing loads, especially with tilting. The disk model (AST) used here allows the calculation of excess pressure in the case of edge bearing or strong angular misalignments.

Lubricants **2023**, 11, 369 4 of 22

#### Cell Model

Contact geometries that cannot be reduced to a point or a line are calculated with cell models. One example is the contact between the rolling element's end face and the inner ring rib. Depending on the profile at the rolling element end face and the ring rib, the contact area can take a crescent shape. For contact calculation, a limited area of the ring and the rolling element end face is divided into squares. Based on the body geometry, the penetration between the contact partners is calculated. For this purpose, the contact problem as described in POLONSKY and KEER is solved [64]. The surface deformation is described by an influence matrix. It can be determined according to the Boussinesq equation [65]. Kiekbusch shows in [60] that the problem can be solved most effectively in dynamics simulation with a combination of FFT (Fast Fourier Transformation) and CG (Conjugate Gradient) solver methods. Accordingly, the method described there is used in this work.

### <span id="page-3-0"></span>2.1.2. Damping

In dynamics simulation, parametric damping models are often used since a variety of influencing factors make it difficult to determine the exact damping values of the contact points. Material damping occurs as a result of deformations and the non-linear elastic properties of contact partners. An additional damping effect is lubricant film damping, which occurs in the run-in zone of the Elasto-Hydrodynamic Lubrication (EHL) contact between the rolling element and raceway [66]. In this study, a parametric model is also used to model the damping force between the rolling element and the other bearing components—the ring raceway, ring ribs, and cage pocket. It is determined by two parameters, the maximum damping coefficient  $d_{max}$  and the penetration  $\delta_{max}$ , above which maximum damping is reached. Thus, it represents an easily adjustable modeling variant of the damping.

$$\vec{F}_D(\vec{v}_N, \delta_{max}, d_{max}) = -\vec{v}_N \cdot d \tag{1}$$

The direction of the damping force is opposite to the impact velocity, which can be understood as the relative velocity of the two touching bodies in the normal direction  $\overrightarrow{v}_N$ . The damping value d is calculated as a function of the penetration depth  $\delta$ .

$$d = \begin{cases} 0 \text{ for } \delta < 0\\ f_d(\delta_{max}, d_{max}, \delta) \text{ for } 0 \le \delta < \delta_{max}\\ d_{max} \text{ for } \delta \ge \delta_{max} \end{cases}$$
 (2)

It is described in the range  $0 \le \delta < \delta_{max}$  with a continuous cubic function.

$$d = \frac{-2 \cdot d_{max}}{\delta_{max}^3} \cdot \delta^3 + \frac{3 \cdot d_{max}}{\delta_{max}^2} \cdot \delta^2$$
 (3)

The damping model is applied to each disc or cell of the contact, depending on the contact point in the bearing.

#### <span id="page-3-1"></span>2.1.3. Friction

The mathematical and physical principles for describing friction at the contact points in rolling bearings are complex since different friction phenomena occur depending on the contact geometry and relative motion. Rolling and sliding friction are essential for the contact between the rolling element and raceway. Pure rolling occurs when the surface velocities of the contact partners are equal in terms of magnitude and direction. If there is a tangential relative movement between the two contact partners in the contact area, sliding occurs. Both forms of friction are present in the developed model. Losses resulting from non-linear elastic material behavior are here shown as hysteresis moments. Figure 2 gives an overview of the friction components considered. These are calculated for each slice in the discretized contact.

Lubricants 2023, 11, 369 5 of 22

<span id="page-4-0"></span>![](_page_4_Figure_1.jpeg)

Figure 2. Friction calculation in roller raceway contact of the TRB MBS model [26].

In the following, the implemented approaches to describing the friction components are explained.

## Lubricant Friction (Sliding)

In EHL contact, a distinction can be made between a rolling and a sliding component of the frictional force [67]. The sliding component results from the shear of the lubricant. The relative movement (sliding) between the rolling element and the ring raceway shears the lubricant. The resulting shear stresses  $\tau_{EHL}$  work against the movement of the rolling element. The force acting on the roler can be written as the integral of the shear stresses over A, which is the contact area between the rolling element and ring raceway.

$$F_{T,L,sl} = \pm \int \tau_{EHL} dA \tag{4}$$

If a constant equivalent shear stress  $\tau_{EHL}$  is taken as a basis, the relationship can be formulated as follows [67].

$$F_{T.L,sl} = \pm \tau_{EHL} \cdot A_{Hertz} \tag{5}$$

For the contact area A, the Hertzian contact area  $A_{Hertz}$  is used as a basis. This approach can also be formulated in discrete time if  $\tau_{EHL}$  and  $A_{Hertz}$  are known at each time step. The two models described below are available in LaMBDA for estimating the equivalent shear stress.

Bair and Winer assume that the shear stresses that a lubricant can carry are limited to a certain value, a limiting shear stress  $\tau_L$ . This value is a characteristic of the lubricant. The viscous component of the shear gradient  $\dot{\gamma}$  can be calculated from the quotient of the lubricants limiting shear stress  $\tau_L$ . and  $\eta$  as a logarithmic function [68].

$$\dot{\gamma} = \frac{\tau_L}{\eta} \cdot \ln \left( 1 - \frac{\tau_{EHL}}{\tau_L} \right) \tag{6}$$

The dynamic viscosity  $\eta$  is described by the modulus equation according to Dicke [69].

$$\eta = \eta_0 \cdot e^{\frac{p}{a_1 + a_2 \cdot \theta + (b_1 + b_2 \cdot \theta) \cdot p}} \tag{7}$$

The parameters  $a_1$ ,  $a_2$ ,  $b_1$  and  $b_2$  of the equation must be determined for each oil from viscosity measurements. The lubricant viscosity  $\eta_0$  at ambient pressure is calculated according to Vogel with the lubricant-dependent parameters K, B, and C [69]. The parameters

Lubricants 2023, 11, 369 6 of 22

describe the temperature dependence of the viscosity. The contact pressure is assumed to be p.

$$\eta_0 = K \cdot e^{\left(\frac{B}{\vartheta + C}\right)} \tag{8}$$

The temperature  $\vartheta$  is given in °C in this equation. For the calculation of the heat of compression in the contact area, Gold et al. [70] propose the following relation:

$$T = T_0 - \frac{1}{2} \cdot \frac{C_2}{C_1} + \sqrt{\left(\frac{C_2}{2 \cdot C_1}\right)^2 + \frac{p}{C_1}} \tag{9}$$

The constants  $C_1$  and  $C_2$  are lubricant dependent. The relative velocity in contact  $u_{rel}$  and the lubricant film height  $h_0$  are used to calculate the shear rate  $\dot{\gamma}$  [71].

$$\dot{\gamma} = \frac{u_{rel}}{h_0} \tag{10}$$

The relative velocity is calculated from the surface velocities of the two bodies, the roller and the ring. The equations presented by Moes are used to calculate the lubricant film height. The numerical implementation of these equations is presented in [14] and is also used here. In addition, lubricant viscosity is considerably influenced by thermal effects [44]. To take these into account, correction factors  $\phi_{\theta}$  are used.

$$h_{th} = \phi_{\vartheta} \cdot h_0 \tag{11}$$

The relationship according to Zhu and Cheng takes into account not only the relative motion in the contact but also the influence of the maximum Hertzian pressure [72].

$$\phi_{\theta} = \frac{1 - 13.2 \cdot \frac{p_0}{E'} \cdot \Gamma_{ZC}^{0.42}}{1 + 0.213 \cdot (1 + 2.23 \cdot s_{ZC}^{0.83}) \cdot \Gamma_{ZC}^{0.64}}$$
(12)

E' is the reduced Young's modulus of both contacting bodies and  $p_0$  is relative pressure. Deviations from perfect rolling, slippage is included as follows.

$$s_{ZC} = 2 \cdot \frac{u_{rel}}{u_{com}} \tag{13}$$

For the thermal load parameter  $\Gamma_{ZC}$ , Zhu and Cheng [73] take the temperature gradient of the viscosity as a basis in addition to the average conveying velocity  $u_{av}$  and the thermal conductivity  $\lambda_{\theta}$ .

$$\Gamma_{ZC} = \left(-\frac{\partial \eta}{\partial \theta}\right) \cdot \frac{u_{av}^2}{\lambda_{\theta}} \tag{14}$$

This can be calculated by differentiating the equation according to Vogel [74].

$$\frac{\partial \eta}{\partial \vartheta} = -\frac{B_V}{(C_V + \vartheta)^2} \cdot A_V \cdot e^{\frac{B_V}{C_V + \vartheta}} \tag{15}$$

The parameters  $A_V$ ,  $B_V$  and  $C_V$  are pressure- and temperature-dependent quantities that must be determined for each lubricant from viscosity measurements [74].

Lubricant Friction (Rolling)

Rolling resistance results from compressing and overrolling the lubricant in EHL contact. It is calculated according to Biboulet and Houpert [75]

$$F_{T,L,r} = \frac{E' \cdot R' \cdot L \cdot 1.4 \cdot (2 \cdot U)^{0.5} \cdot W^{0.5}}{0.985 \cdot \left(1 + \left(\frac{1.4}{1.45} \cdot \sqrt{\frac{W}{2 \cdot U}}\right)^{10}\right)^{\frac{1}{10}}}$$
(16)

Lubricants 2023, 11, 369 7 of 22

E' and R' are the reduced Young's modulus and the reduced radii of the contacting bodies and L the effective contacting length. The dimensionless parameters of these approaches are defined as follows:

Velocity parameter 
$$U = \frac{\eta_0 \cdot u_{av}}{E' \cdot R'}$$
 (17)

Load parameter 
$$W = \frac{Q}{L \cdot E' \cdot R'}$$
 (18)

The load parameter *W* is calculated on the basis of the load *Q* imposed on one slice of the rolling element.

### Solid Rolling Friction

Rolling friction losses due to contact point displacement in the rolling direction caused by elastic deformation of the contact partners are taken into account as a function of normal force, as in Scheuermann [76].

$$M_{TSr} = c_r \cdot F_N^{e_r} \tag{19}$$

The rolling resistance exponent  $e_r$  and the rolling friction coefficient  $c_r$  are material-dependent and can be taken from the literature on the subject or experiment.

### Material Hysteresis

Hysteresis refers to a contact that does not take place ideally elastically. The deformation energy is not completely returned. Part of the deformation energy dissipates into heat [61,77]. For the rolling contact between the rolling element and raceway, this means that the contact pressure is asymmetrical. When rolling under load, the contact elements deflect on the run-in side. However, due to the processes described, the deflection does not occur to the same extent. The force Q resulting from the compression is offset relative to the axis of rotation. It therefore causes a moment  $M_{T,Hys}$ , which is directed in the opposite direction to the rotary motion of the two bodies. The resulting moment is independent of the velocity. According to Johnson, it can be described as a function of a hysteresis loss factor  $a_v$  and is only proportional to the contact load and half of the Hertzian contact width b, which corresponds to the disk width in the discretized contact [78].

$$M_{T,Hys} = Q \cdot a_v \cdot \frac{2b}{3\pi} \tag{20}$$

### Solid Sliding Friction

In order to represent the proportion of solid friction in the contact between rolling elements and the raceway, a section-wise defined function is used. It is divided into a cubic part and a constant part. The distinction is made on the basis of the relative velocities in the contact. Using the input parameters  $v_s$  and  $v_d$  the friction coefficient-relative velocity curve can be adapted to experimentally determined values. Above the limit of  $v_d$ , a constant value  $\mu_d$  is assumed for the solid friction.

$$\mu = \begin{cases} h_{cubic} + \delta_{cubic}^2 \cdot (3 - 2\delta_{cubic}) \text{ for } |v_{sl}| < v_s \text{ or } v_s \le |v_{sl}| < v_d \\ \mu_d \text{ for } |v_{sl}| \ge v_d \end{cases}$$
 (21)

The parameters  $a_{cubic}$ ,  $h_{cubic}$ , and  $\delta_{cubic}$  are chosen for the ranges such that the value  $\mu_i$  decreases to the constant value  $\mu_d$  after a degressive slope to  $v_s$ .

$$a_{cubic} = 2 \cdot \mu_s \tag{22}$$

$$for |v_{sl}| < v_s \quad h_{cubic} = -\mu_s \tag{23}$$

*Lubricants* **2023**, *11*, 369 8 of 22

$$\delta_{cubic} = \frac{v_{sl} + v_s}{2v_s} \tag{24}$$

$$a_{cubic} = \mu_d - \mu_s \tag{25}$$

$$for v_s \le |v_{sl}| < v_d \quad h_{cubic} = \mu_s \tag{26}$$

$$\delta_{cubic} = \frac{v_{sl} - v_s}{v_d - v_s} \tag{27}$$

The resulting frictional force is calculated by multiplying the contact normal force with the described friction coefficient.

$$F_{T,S,sl} = F_N \cdot \mu \tag{28}$$

### Mixed Friction

Mixed friction is the transitional area between solid friction and friction due to EHL. Both types of friction are significantly influenced by the surface roughness of the contact bodies. Between the roughness peaks, small micro-EHL structures form, which can only be captured iteratively with great effort. In order to take these effects into account in the dynamics simulation, a simple model according to Zhou and Hoepprich is used [\[61,](#page-20-2)[73\]](#page-20-15). It is used to approximate the proportion of solid friction in the mixed friction regime *φ*. This portion can be determined as a first approximation from the asperity load ratio of the surfaces.

$$\phi = \frac{Q_s}{Q} = e^{-B_{ZH}\Lambda^C_{ZH}} \tag{29}$$

The proportion of the normal force *Q<sup>s</sup>* transmitted at solid contacts is set in relation to the total load *Q* here. The parameters *BZH* and *CZH* describing the roughness of the surfaces and are determined according to [\[73\]](#page-20-15). The lubricant film thickness parameter Λ is described as the quotient of the film thickness *h*<sup>0</sup> and the combined standard derivation of surface roughness *σ*.

$$\Lambda = \frac{h_0}{\sigma} \tag{30}$$

### Friction in Roller Rib Contact

Friction occurring between the rolling element end face and ring rib is called drilling friction due to the rotational movement the roller is performing. This drilling movement can also be understood as sliding when looking at the relative velocity vector of each discretized point in the contact. The closer the discretized point is to the axis of rotation of the roller, the smaller the magnitude of the velocity. The further away it is, the greater its magnitude becomes. In order to describe the friction that occurs during a sliding movement, suitable approaches exist. Here, a distinction is made between sliding friction in solid contact (rolling element and rib) and the resistance of the lubricant due to its shear.

# Solid Sliding Friction in Roller Rib Contact

The coefficient of friction for sliding in solid contact between the rolling element end face and the ring ribs is approximated via a cubic function, as it is used in the roller raceway contact (Equation (21)). In order to define the continuous function necessary for the MBS simulation, the pole point at *vsl* = 0 is approximated with a cubic section. The parameters *vs* , *v<sup>d</sup> µ<sup>s</sup>* and *µ<sup>d</sup>* can be used to set the range until a constant friction value is reached.

# Lubricant Friction in Roller Rib Contact

The friction component due to sliding in the lubricated contact results from the shear of the lubricant. As in the contact between the rolling element and the ring raceway, Lubricants 2023, 11, 369 9 of 22

Equation (4) is therefore used. The shear stresses in the lubricant are represented by the Bair-Winer model. In this case, in contrast to the line contact, the following equation according to [79] is used to calculate the lubricant film height.

$$h_{0,i,j} = 2.69 \cdot R_x \cdot U_{i,j}^{0.67} \cdot G_{i,j}^{0.53} \cdot W_{i,j}^{-0.069} \cdot \left(1 - 0.61 \cdot e^{-0.73 \cdot k_e}\right)$$
(31)

The equation was originally derived for an elliptical contact surface and takes into account the velocity, material, and load using the parameters  $U_{i,j}$ , (Equation (33))  $G_{i,j}$  (Equation (34)) and  $W_{i,j}$  (Equation (35)). It can be used in good approximation also for this kind of contact, even if the contact area deviates strongly from an ellipse in some cases. The elliptic parameter  $k_e$  is the ratio of the two contact axes a and b. It can be estimated from the reduced radii in x direction  $R_x$  and y direction  $R_y$ .

$$k = \frac{a}{b} \approx 1.0339 \cdot \left(\frac{R_y}{R_x}\right)^{0.636}$$
 (32)

The dimensionless parameters for film height calculation result from the following variables:

Velocity parameter 
$$U_{i,j} = \frac{\eta_0 \cdot u_{av,i,j}}{E' \cdot R_x}$$
 (33)

Material parameter 
$$G_{i,j} = \alpha_{p,i,j} \cdot E'$$
 (34)

Load parameter 
$$W_{i,j} = \frac{Q_{i,j}}{E' \cdot R_z^2}$$
 (35)

For calculation of the material parameter G, the pressure-viscosity coefficient  $\alpha_p$  is used. The correction of the lubricant film height for a thermal influence and the description of the dynamic viscosity are carried out according to Equations (6)–(15).

### Mixed Friction

The resulting friction force is calculated by summing the two friction force components (solid sliding friction and EHL sliding friction, or lubricant shear). Following Steinert, they are weighted with a dimensionless key figure for the solid body friction component [80].

$$F_T = \phi \cdot F_{T,S} + (1 - \phi) \cdot F_{T,L} \tag{36}$$

The asperity load ratio  $\phi$  is determined according to the approach of Zhou and Hoepprich [61,73] (Equation (30)).

### Friction in Roller Cage Contact

The contact between the rolling element and cage pocket contributes only slightly to the total frictional torque of the bearing. There are high relative speeds or predominantly sliding between the two surfaces. Coulomb's friction law is therefore used to calculate the frictional force.

$$F_T = F_N \cdot \mu \tag{37}$$

The friction value  $\mu$  used as a basis is approximated as in Equation (21). The polarity of the coefficient of friction at zero velocity is thus avoided. The cage is guided by the rolling elements. Therefore, no further frictional contacts are created. The cage cannot touch the bearing rings.

### <span id="page-8-0"></span>2.2. Friction Torque Measurement

The measurements of the frictional torque were carried out on the friction torque test rig of the MEGT (see Figure 3). The test rig was developed at the MEGT in the framework of Aul's work [58]. It has been used in many research projects over the past years and has

Lubricants 2023, 11, 369 10 of 22

proven itself reliable [26,58–60,81]. The test rig allows the frictional torque of a single test bearing to be measured. The outer ring of the test bearing is mounted on a hydrostatic bearing. A beam load cell is used to hold the rotational degree of freedom of the hydrostatic bearing in position while measuring the force required. With the distance between the axis of rotation of the test bearing and the beam load cell, the total frictional torque can be calculated from the measured force.

<span id="page-9-0"></span>![](_page_9_Picture_2.jpeg)

Figure 3. Scematic view and CAD model of the friction torque test bench at MEGT [42].

Furthermore, the test bearing is accessible for additional measurements such as cage or rolling element speed. The bearing can be loaded in the radial and axial directions. The tilting module also allows tilting and/or skewing of the test bearing, which was not used in the investigations presented here.

Three sets of measurements were performed. First, the TRB was loaded axially with 6 kN. The bearing was lubricated with an oil bath. The oil level was initially set to half the roller height. The lubricant used was an ISO VG 100 mineral oil without additives (reference oil FVA No. 3—for lubricant data, see Table A1 in Appendix A). The speed was kept constant until the temperature on the outer ring of the bearing reached the desired level. The speed ramp was then run through within a few minutes so that there was no change in temperature during the measurement. The measurement was taken at two temperatures, according to the setup shown in Table 1.

| Parameter   | Variable                    | Value     | Unit |
|-------------|-----------------------------|-----------|------|
| Axial load  | Fa                          | 6         | kN   |
| Radial load | $F_{\mathbf{r}}$            | 0         | kN   |
| Temperature | $\vartheta$                 | 42 and 50 | °C   |
| Shaft speed | N                           | 500-4000  | rpm  |
| Lubrication | Oil bath half roller height |           |      |
| Lubricant   | Reference oil FVA3          |           |      |

<span id="page-9-1"></span>**Table 1.** Test setup for friction torque measurement of a TRB type 32216 at pure constant axial load.

Second, a constant combined load was then set. The test sequence is the same as for purely axial loading. Then, the speed was varied at two constant temperatures, according to the setup shown in Table 2. The second series of measurements was carried out under combined radial and axial loads. The radial load was applied in a downward direction, meaning the load zone is located in the lower part of the rolling bearing that is well supplied with lubricant. The other boundary conditions were adopted.

For the last series of measurements, a constant preload of 6.5 kN of the bearing was applied. At constant speed, the radial load was increased from 1 to 15 kN. These measurements have been performed under a steady temperature. The setup parameters are listed in Table 3.

A TRB of type 32216 was used for all tests. The geometry of the bearing is shown in Table 4.

Lubricants 2023, 11, 369 11 of 22

<span id="page-10-0"></span>**Table 2.** Test setup for friction torque measurement of a TRB type 32216 at combined, constant axial and radial load.

| Parameter   | Variable                    | Value              | Unit |
|-------------|-----------------------------|--------------------|------|
| Axial load  | $F_{a}$                     | 6                  | kN   |
| Radial load | $F_{\mathbf{r}}$            | 6.5                | kN   |
| Temperature | $\vartheta$                 | 42 and 50          | °C   |
| Shaft speed | N                           | 500-4000           | rpm  |
| Lubrication | Oil bath half roller height |                    |      |
| Lubricant   |                             | Reference oil FVA3 |      |

<span id="page-10-1"></span>**Table 3.** Test setup for friction torque measurement of a TRB type 32216 at combined load.

| Parameter   | Variable                    | Value              | Unit |
|-------------|-----------------------------|--------------------|------|
| Axial load  | $F_{a}$                     | 6.5                | kN   |
| Radial load | $F_{\mathbf{r}}$            | 1–15               | kN   |
| Temperature | $\vartheta$                 | 50                 | °C   |
| Shaft speed | N                           | 2000               | rpm  |
| Lubrication | Oil bath half roller height |                    |      |
| Lubricant   |                             | Reference oil FVA3 |      |

<span id="page-10-2"></span>Table 4. Geometrical data of a TRB type 32216.

| Parameter                                     | Variable              | Value   | Unit |
|-----------------------------------------------|-----------------------|---------|------|
| Basic static load rating, radial              | $C_{0r}$              | 260,000 | N    |
| Inner diameter                                | $d_{\mathrm{i}}$      | 80      | mm   |
| Outer diameter                                | $D_{a}$               | 140     | mm   |
| Pitch diameter                                | $d_{\mathrm{Pd}}$     | 108.5   | mm   |
| Roller diameter                               | $d_{\mathrm{RB}}$     | 17      | mm   |
| Roller length                                 | $l_{ m RB}$           | 22.7    | mm   |
| Number of roller                              | $n_{\mathrm{RB}}$     | 16      | -    |
| Profile parameter                             | $a_{p}$               | 0.0005  | -    |
| Profile parameter                             | $c_{p}$               | 20.7    | mm   |
| Profile parameter                             | $d_{p}$               | 0.0     | mm   |
| Profile parameter                             | $k_{\rm p}^{^{1}}$    | 2.0     | mm   |
| Edge radius                                   | $r_{\rm e}$           | 1.0     | mm   |
| Combined standard derivation of roughness     | $\sigma_{ m Raceway}$ | 0.16    | μm   |
| Combined standard derivation of roughness     | $\sigma_{ m Rib}$     | 0.24    | μm   |
| Mixed friction parameters for raceway contact | $B_{ZH}$              | 2.32    |      |
| according to Zhou and Hoeprich [61,73]        | $C_{\mathrm{ZH}}$     | 0.97    |      |
| Mixed friction parameters for rib contact     | $B_{ZH}$              | 1.90    |      |
| according to Zhou and Hoeprich [61,73]        | $C_{\mathrm{ZH}}$     | 0.99    |      |

Figure 4 serves as an illustration of the geometric values. The profile retraction of the rolling elements is described by the parameters  $a_p$ ,  $c_p$ ,  $d_p$ ,  $k_p$  and  $r_e$ . They are defined in Teutsch [44] and allow the different standardized profiles of rollers to be described mathematically.

<span id="page-10-3"></span>![](_page_10_Picture_8.jpeg)

**Figure 4.** Graphical illustration of the geometry parameters of the TRB used in this study. On the left the sectional view of the TRB and on the right a rolling element is shown. The dotted lines represent the symmetry lines of the bearing and the rolling element.

Lubricants 2023, 11, 369 12 of 22

#### <span id="page-11-0"></span>3. Results

For model validation, the comparison of the frictional torque as an integral parameter of the contact modeling of all internal contacts in the bearing with the measured frictional torque has proved useful. The fictional torque is highly dependent on the load distribution inside the bearing and its dynamics. For this purpose, the total frictional torque is being used as the main outcome for this comparison.

As shown in Figure 5, the frictional torque obtained in the simulation (LaMBDA  $50\,^{\circ}$ C) when submitted to an axial load of  $6.5\,\mathrm{kN}$  and a shaft speed of  $500\,\mathrm{rpm}$  is about  $1.200\,\mathrm{Nmm}$ . The value is rising with an increasing shaft speed up to  $3800\,\mathrm{Nmm}$ . It can be observed that the simulated friction curve follows the measured one (measurement at  $50\,^{\circ}$ C) with a small offset. The simulated frictional torque values are slightly higher. The second measurement shown in Figure 5 was taken at  $42\,^{\circ}$ C. Since the load is the same, the changed frictional torque can be attributed exclusively to the temperature dependence of the lubricant properties. This concerns especially temperature-dependent changes in viscosity. In the simulation, the lubricant properties are included in the calculation of the rolling resistance as well as the losses due to shear of the lubricant. The fact that the simulated frictional torque follows the measured curve for both temperatures very well shows that the modeling of the lubricant properties is quite suitable.

<span id="page-11-1"></span>![](_page_11_Figure_4.jpeg)

**Figure 5.** Comparison of the measured and LaMBDA-calculated frictional torque of a TRB of type 32216 under purely axial load of 6 kN and oil bath lubrication with reference oil FVA No. 3.

The same experiments were performed with combined axially and radially loaded bearings. This load situation produces significant contact forces in the roller-raceway contact and between roller and rib. Therefore, it is suitable to validate the contact and friction models in the roller-rib contact as well. Both curves, simulation and experiment, show an increase in frictional torque with higher shaft speed and a decrease with lower temperature (Figure 6).

<span id="page-11-2"></span>![](_page_11_Figure_7.jpeg)

**Figure 6.** Comparison of the measured and LaMBDA-calculated frictional torque of a TRB of type 32216 under combined axial load of 6.5 kN and radial load of 6 kN with oil bath lubrication with reference oil FVA No. 3.

Lubricants 2023, 11, 369 13 of 22

Contrary to expectation, the measured frictional torque is noticeably lower than it is under pure axial load. This results from the changed load distribution in the bearing. Whereas with purely axial loads, the load zone extends over the entire bearing circumference, with additional radial loads, it is located only in the lower part of the bearing. Consequently, fewer rolling elements and thus a smaller contact length contribute to the friction. To investigate this influence further, the radial bearing load is varied at a constant preload. The results from experiment and simulation are presented in Figure 7.

<span id="page-12-0"></span>![](_page_12_Figure_2.jpeg)

**Figure 7.** Comparison of the measured and LaMBDA-calculated frictional torque of a TRB of type 32216 under combined load and oil bath lubrication with reference oil FVA No. 3 under variation of the radial load at  $50\,^{\circ}$ C.

As previously assumed, the measurements carried out (in light blue) show that the total frictional torque of the test bearing decreases with increasing radial load. Again, the comparison between experiment and model shows a very good agreement. The simulation model reproduces all the internal contacts of the bearing individually and in detail, which means that the lower frictional torque is displayed with a radial load component. With the high-resolution contacts and the detailed friction calculation described in Section 2.1, the model is very good at predicting the frictional torque. Furthermore, the model provides an explanation for the falling friction torque curve as it allows an insight into the inside of the bearing and shows the load distribution.

As a second proof of the assumption, the load distribution in the rolling bearing is used. The load distribution for the load cases—pure axial load and combined load with a high radial portion—is shown in Figure 8.

<span id="page-12-1"></span>![](_page_12_Figure_6.jpeg)

**Figure 8.** Load distribution of the TRB type 32216 used. Left: with purely axial load of 6 kN, right: with combined load of 6 kN axial and 16 kN radial.

It shows that under pure axial load (left), all rolling elements in the bearing are in the load zone. Thus, all contacts also contribute to the friction. In the load case with an additional radial load of 16 kN, the load zone shifts. It is now located in the lower area of the rolling bearing, which means that only 11 rolling elements are still carrying. This can be

Lubricants 2023, 11, 369 14 of 22

seen from the fact that only 12 points are still visible in the diagram. 11 represents the load-carrying rolling elements; all other points are at 0 N since the remaining rolling elements do not experience any load. Accordingly, fewer contacts (effective length) contribute to the friction.

#### <span id="page-13-0"></span>4. Discussion

The preliminary work shows that the dynamics simulations with LaMBDA are well suited to simulate the frictional torque of TRBs at low oil levels or minimum quantities of lubrication. Hydraulic losses due to a higher oil level are not calculated by default with LaMBDA. However, equations exist to take the losses into account. The method, according to Liebrecht, among others, can be regarded as the state of the art [82].

First comparative calculations between LaMBDA simulations and measurements that can be found in the literature, i.e., performed by Liebrecht [82] and Gonda [83], are shown in Figure 9. The measurements had been taken at a test rig where the test bearing and the supporting bearings have separate oil reservoirs to allow the measurement of torques due to the latter bearings independently from the losses of the support bearing [82,83]. While the friction torque simulated with LaMBDA (gray) with minimum quantity lubrication agrees well with the experimentally determined friction torques (light blue), the simulation with a fully flooded test volume (yellow) deviates strongly from the measured friction torque (orange). This means that the state-of-the-art approach (LaMBDA with Liebrecht) greatly underestimates the hydraulic losses in the bearing.

<span id="page-13-1"></span>![](_page_13_Figure_5.jpeg)

**Figure 9.** Comparison of the simulated frictional torque of a TRB of type 32208 under oil sump lubrication with reference oil FVA No. 3 at 50 °C and an axial load of 1 kN with the measured frictional torque on a single-bearing test rig. Simulated frictional torque with LaMBDA and experiment (left), experiment and CFD simulation [84] (right). Bearing data is provided in Table A2 in Appendix B) and lubricant data in Table A1 in Appendix A).

The equations formulated by Liebrecht are derived from experiments with a tapered roller bearing of type 32208 with vertical shaft alignment. They take into account, in addition to the operating conditions, the flow characteristics and simplify the internal bearing geometry. For the operating points investigated, they are simplified as follows:

$$M_{drag} = 0.86 \cdot \rho \cdot l_{RB} \cdot d_{RB}^{0.164} \cdot v^{0.836} \cdot d_m^{2.164} \cdot n_C^{1.164}$$
(38)

$$M_{churning,OR} = 0.32 \cdot \rho \cdot A_{ORL} \cdot d_{ORL}^{0.287} \cdot v^{0.713} \cdot (d_m \cdot n_C)^{1.287}$$
(39)

$$M_{churning,IR} = 0.52 \cdot \rho \cdot A_{IRL} \cdot d_{IRL}^{0.265} \cdot v^{0.735} \cdot (d_{IRL} \cdot n_{IR} - d_m \cdot n_C)^{1.265}$$
(40)

While length  $l_{RB}$  and diameter  $d_{RB}$  of the rolling elements and as well as raceway diameter  $d_{ORL}$  resp.  $d_{IRL}$ , effective surface  $A_{ORL}$  resp.  $A_{IRL}$  and  $d_m$  the mean rolling bearing diameter are geometrical values, the oil density  $\rho$ , the dynamic viscosity  $\eta$  and the kinematic viscosity v are lubricant properties. The velocities of the single components are

*Lubricants* **2023**, *11*, 369 15 of 22

taken into account via the cage speed *nC*. The oil level is taken into account indirectly via the effective surfaces.

Better results for hydraulic losses can be obtained from the studies in [\[84\]](#page-21-4) using a Computational Fluid Dynamics (CFD) model developed in an OpenFOAM® environment. The highly specialized CFD simulation obtained agrees with the experiment with only insignificant deviations (see Figure [9](#page-13-1) on the right side). In addition, the aforementioned simulations with OpenFOAM® have been validated with respect to lubricant flows in a 32312-A TRB [\[85–](#page-21-5)[87\]](#page-21-6). Combining the calculated losses from both LaMBDA and CFD simulations, the total bearing losses can be predicted very well (dark blue curve, Figure [9,](#page-13-1) left). These results show good agreement with the measured total bearing losses (orange). The comparison of the listed calculation methods shows that the state-of-the-art approach predicts the hydraulic losses in a bearing only to a certain extent. With highly specialized CFD simulations, it is possible to determine these losses very precisely [\[88\]](#page-21-7).

However, there is a need for further research to combine the two methods in order to determine the influence of hydraulic losses on the bearing kinematics. For this purpose, generally applicable equations would have to be derived from the CFD simulation, with which the flow-dependent forces of the lubricant on the bodies in the rolling bearing can be calculated. The forces can then be taken into account in the dynamics simulation and included in the force equilibrium so that they influence the movement—kinematics—of the roller bodies and cage.

### **5. Conclusions**

The developed dynamic model LaMBDA for tapered roller bearings provides very good results with respect to the simulated frictional torque. This has been shown by comparison with friction torque measurements on a single-bearing test rig. Lubricant properties depending on temperature, load, and stresses are correctly represented in the model. The highly accurate contact models and friction descriptions are the basis for the high expressiveness of the model.

Nevertheless, not all losses that occur in a rolling bearing are taken into account. If rolling bearings in a gearbox are to be simulated, the losses due to the oil outside the immediate contact must also be included. Further research is needed in this regard. As shown in the discussion, CFD simulations can make an important contribution to the description of these losses. CFD simulations can also provide information on the lubricant pumping mechanism of the TRB and the lubrication supply, which are not considered in the model yet. A mathematical-physical description of the losses can be derived from the complex flow simulations. In multi-body simulation, these approaches can contribute to the holistic system optimization of gearboxes. Studies on the implementation of these losses in the MBS require further investigation.

**Author Contributions:** Conceptualization, P.W. and O.K.; methodology, P.W.; software, P.W. and L.M.; validation, P.W.; resources, B.S. and F.C.; data curation, P.W.; writing—original draft preparation, P.W.; writing—review and editing, O.K., F.C.; visualization, P.W.; supervision, B.S. and F.C.; project administration, B.S. All authors have read and agreed to the published version of the manuscript.

**Funding:** This research was funded by Arbeitsgemeinschaft industrieller Forschungsvereinigungen (AiF) e.V. and Forschungsvereinigung Antriebstechnik (FVA) e.V., FVA project number 625 III, grant number 20764 N.

**Data Availability Statement:** Data are available on request from the authors.

**Acknowledgments:** The authors would like to thank the German Research Foundation (DFG) for its support in the context of the experimental work within the framework of the project 430 "Einfluss der hydraulischen Verluste auf die Reibung von Wälzlagern", grant numbers SCHW 826/12-1 and SA 898/23-1. The experiments of the two research projects were re-evaluated for this work and included in the discussion of the results obtained here.

**Conflicts of Interest:** The authors declare no conflict of interest.

Lubricants 2023, 11, 369 16 of 22

#### Nomenclature

| D .     |      |
|---------|------|
| Kearing | data |
| Bearing | uata |

 $egin{aligned} a_{
m p} & & {
m Profile\ parameter} \ c_{
m p} & & {
m Profile\ parameter} \ d_{
m i} & & {
m Inner\ diameter} \ d_{
m p} & & {
m Profile\ parameter} \ \end{array}$ 

 $d_{\rm m}$  mean rolling bearing diameter  $d_{\rm IRL}$  Inner ring raceway diameter  $d_{\rm ORL}$  Outer ring raceway diameter

 $d_{\mathrm{Pd}}$  Pitch diameter  $d_{\mathrm{RB}}$  Roller diameter  $k_{\mathrm{p}}$  Profile parameter  $l_{\mathrm{RB}}$  Roller length  $n_{\mathrm{RB}}$  Number of roller  $r_{\mathrm{e}}$  Edge radius

 $A_{\text{IRL}}$  effective surface of the inner ring  $A_{\text{ORL}}$  effective surface of the outer ring

B<sub>ZH</sub> Surface roughness parameter according to ZHOU and HOEPPRICH

C<sub>0r</sub> Basic static load rating, radial

C<sub>ZH</sub> Surface roughness parameter according to ZHOU and HOEPPRICH

D<sub>a</sub> Outer diameter

combined standard derivation of surface roughness

#### Lubricant parameters

a<sub>1</sub> Lubricant dependent parameter according to DICKE
 a<sub>2</sub> Lubricant dependent parameter according to DICKE
 b<sub>1</sub> Lubricant dependent parameter according to DICKE
 b<sub>2</sub> Lubricant dependent parameter according to DICKE

 $h_{\rm th}$  thermal corrected lubricant film height

 $h_0$  lubricant film height

 $A_{\rm V}$ Lubricant dependent parameter according to DICKE Lubricant dependent parameter according to VOGEL В  $B_{V}$ Lubricant dependent parameter according to DICKE CLubricant dependent parameter according to VOGEL  $C_{V}$ Lubricant dependent parameter according to DICKE  $C_1$ Lubricant dependent parameter according to GOLD ET AL.  $C_2$ Lubricant dependent parameter according to GOLD ET AL. K Lubricant dependent parameter according to VOGEL

 $\begin{array}{lll} \alpha & & \text{Temperature density coefficient} \\ \alpha_p & & \text{Pressure-viscosity coefficient} \\ \eta & & \text{Dynamic viscosity of a lubricant} \\ \eta_0 & & \text{Lubricant viscosity at ambient pressure} \end{array}$ 

 $\lambda_{\vartheta}$  thermal conductivity v kinematic viscosity  $\rho$  Lubricant density

 $au_L$  Limiting shear stress according to Bair and Winer

 $\phi_{\vartheta}$  thermal correction factors

#### State variables/states

 $F_{\rm a}$  Axial load  $F_{\rm r}$  Radial load  $n_{\rm C}$  cage speed  $n_{\rm IR}$  Inner ring speed N Shaft speed

 $\stackrel{
ightarrow}{s}$  Displacement between two coordinate systems

Relative velocity between two coordinate systems

Acceleration between two coordinate systems

Lubricants 2023, 11, 369 17 of 22

| T                                          | Absolute temperature                                                          |
|--------------------------------------------|-------------------------------------------------------------------------------|
| $T_0$                                      | Ambient temperature (20 °C)                                                   |
| $\vartheta$                                | Temperature in °C                                                             |
| $\overset{ ightarrow}{\omega}$             | Angle between two coordinate systems                                          |
| $\overset{\cdot}{\omega}$                  | Angular velocity between two coordinate systems                               |
| $\overset{\cdot \cdot }{\omega }$          | Angular acceleration between two coordinate systems                           |
| Model input p                              |                                                                               |
| $a_{\mathrm{cubic}}$                       | Parameter defining the coefficient of friction                                |
| $a_{ m V}$                                 | hysteresis loss factor                                                        |
| $c_{\mathbf{r}}$                           | rolling friction coefficient                                                  |
| $e_{\rm r}$                                | rolling resistance exponent                                                   |
| L                                          | effective contacting length                                                   |
| $v_{\rm s}$                                | Limit of relative velocity for static coefficient of friction                 |
| $v_{\rm d}$                                | Limit of relative velocity for dynamic coefficient of friction                |
| $\mu_{\mathrm{S}}$                         | Static coefficient of friction                                                |
| $\mu_{\mathrm{d}}$                         | Dynamic coefficient of friction                                               |
| Contact state v                            | variables                                                                     |
| а                                          | Axis of the contact ellipse                                                   |
| b                                          | HERTZIAN contact width/ axis of the contact ellipse                           |
| d                                          | damping coefficient                                                           |
| $d_{\max}$                                 | maximum damping coefficient                                                   |
| $f_{\rm d}$                                | Function describing the coefficient of damping depending on penetration depth |
| h                                          | lubricant film height                                                         |
| $h_{	ext{cubic}} \rightarrow$              | Parameter defining the coefficient of friction                                |
| n                                          | Contact normal vector                                                         |
| $\frac{p}{\overrightarrow{p}}$             | Contact pressure                                                              |
| p΄                                         | Contact point vector                                                          |
| $p_0$                                      | Relative pressure                                                             |
| $s_{ZC}$                                   | Slippage                                                                      |
| $u_{\rm av}$                               | Average conveying velocity of the lubricant                                   |
| $\stackrel{\rightarrow}{u}_{\mathrm{rel}}$ | Relative velocity vector in contact point                                     |
| $\overset{u_{\mathrm{rel}}}{\to}$          | Magnitude of relative velocity in contact point                               |
| $\stackrel{ ightarrow}{u}_{ m sum}$        | Sum velocity vector in contact point                                          |
| $u_{\text{sum}}$                           | Magnitude of sum velocity in contact point                                    |
| $\overrightarrow{v}_{ m N}$                | Velocity vector in contact normal direction                                   |
| $v_{ m sl}$                                | Effective sliding velocity                                                    |
| $A_{ m Hertz}$                             | HERTZIAN contact area                                                         |

 $A_{\text{Hertz}}$ HERTZIAN contact area

E'Reduced YOUNG's modulus of both contacting bodies

Magnitude of contact normal force

 $F_{\mathbf{N}}$   $\overrightarrow{F}_{\mathbf{N}}$ Contact normal force  $\overrightarrow{F}_{\mathbf{D}}$ Damping force

 $\overrightarrow{F}_{T,L,sl}$ Force resulting from sliding friction in lubricant  $\vec{F}_{T,S,sl}$ Force resulting from sliding friction in solid contact

 $\overrightarrow{F}_{\mathbf{T}}$ Traction force  $\overrightarrow{F}_{\Sigma}$ Contact force GMaterial parameter  $M_{\rm churning,IR}$ Churning losses inner ring  $M_{\rm churning,OR}$ Churning losses outer ring

 $M_{\rm drag}$ Drag losses

 $\vec{M}_{\mathrm{D}}$ Torque from damping force  $\vec{M}_{N}$ Torque from contact normal force

 $\stackrel{
ightarrow}{M}_{
m T}$ Torque from traction force

 $M_{T,Hys}$ Torque resulting from material hysteresis Lubricants 2023, 11, 369 18 of 22

| $\stackrel{ ightarrow}{M}_{ m T,L,r}$                                                                                                                                             | Torque resulting from rolling friction in lubricant            |
|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------|
| $\stackrel{ ightarrow}{M_{ m T,L,sl}}$                                                                                                                                            | Torque resulting from sliding friction in lubricant            |
| $\stackrel{ ightarrow}{M}_{T,L,r}$ $\stackrel{ ightarrow}{M}_{T,L,sl}$ $\stackrel{ ightarrow}{M}_{T,S,r}$ $\stackrel{ ightarrow}{M}_{T,S,sl}$ $\stackrel{ ightarrow}{M}_{\Sigma}$ | Torque resulting from rolling friction in solid contact        |
| $\stackrel{ ightarrow}{M}_{ m T,S,sl}$                                                                                                                                            | Torque resulting from sliding friction in solid contact        |
| $\stackrel{\rightarrow}{M}_{\Sigma}$                                                                                                                                              | Torque from contact force                                      |
| Q                                                                                                                                                                                 | load imposed on one slice/cell of the rolling element          |
| $Q_{\rm S}$                                                                                                                                                                       | proportion of the normal force transmitted at solid contacts   |
| R'                                                                                                                                                                                | Reduced radii of the contacting bodies                         |
| $R_{x}$                                                                                                                                                                           | Reduced radius in x direction                                  |
| $R_{ m y}$                                                                                                                                                                        | Reduced radius in y direction                                  |
| Ú                                                                                                                                                                                 | Velocity parameter                                             |
| W                                                                                                                                                                                 | Load parameter                                                 |
| $\dot{\gamma}$                                                                                                                                                                    | Shear gradient                                                 |
| δ                                                                                                                                                                                 | Penetration depth                                              |
| $\delta_{ m cubic}$                                                                                                                                                               | Parameter defining the coefficient of friction                 |
| $\delta_{max}$                                                                                                                                                                    | Penetration above which maximum damping coefficient is reached |
| μ                                                                                                                                                                                 | Coefficient of friction                                        |
| $	au_{ m EHL}$                                                                                                                                                                    | Shear stresses of the lubricant                                |
| φ                                                                                                                                                                                 | Solid load-bearing ratio                                       |
| $\Gamma_{ZC}$                                                                                                                                                                     | Thermal load parameter                                         |
| $\Delta t$                                                                                                                                                                        | Duration of a time step                                        |
| Λ                                                                                                                                                                                 | Lubricant film thickness parameter                             |
|                                                                                                                                                                                   |                                                                |

# <span id="page-17-1"></span>Appendix A

The lubricant viscosity as a function of pressure and temperature was determined with the aid of the modulus equation according to Dicke et al. [69] (Equation (7)) and Vogel [74] (Equation (15)). The parameters used are listed in Table A1.

<span id="page-17-0"></span>**Table A1.** Parameters for viscosity according to Dicke et al. [69] and Vogel [74] for reference oil FVA No. 3.

| Parameter                          | Variable | Value                  | Unit              |
|------------------------------------|----------|------------------------|-------------------|
| Temperature parameter <sup>1</sup> | K        | 0.062                  | mPa s             |
| Temperature parameter <sup>1</sup> | В        | 1021.7                 | °C                |
| Temperature parameter <sup>1</sup> | С        | 101.5517               | °C                |
| Pressure parameter <sup>1</sup>    | $a_1$    | 327.7918               | bar               |
| Pressure parameter <sup>1</sup>    | $a_2$    | 2.9862                 | bar/°C            |
| Pressure parameter <sup>1</sup>    | $b_1$    | $4.419 \cdot 10^{-3}$  | -                 |
| Pressure parameter <sup>1</sup>    | $b_2$    | $3.0115 \cdot 10^{-4}$ | 1/°C              |
| Density at 15 °C                   | ρ        | 887.6                  | kg/m <sup>3</sup> |
| Thermal conductivity               | λ        | 0.134                  | W/(m K)           |
| Temperature density coefficient    | α        | $-6.10^{-4}$           | g/(ml K)          |

<sup>&</sup>lt;sup>1</sup> The lubricant data use in the MBS model in this work have been measured by [89].

# <span id="page-17-2"></span>Appendix B

For the simulation of the bearing losses at the tapered roller bearing of type 32208 the following geometry was used.

*Lubricants* **2023**, *11*, 369 19 of 22

| Parameter                                     | Variable | Value  | Unit |
|-----------------------------------------------|----------|--------|------|
| Basic static load rating, radial              | C0r      | 94,000 | N    |
| Inner diameter                                | di       | 40     | mm   |
| Outer diameter                                | Da       | 80     | mm   |
| Pitch diameter                                | dPd      | 60     | mm   |
| Roller diameter                               | dRB      | 10     | mm   |
| Roller length                                 | lRB      | 17     | mm   |
| Number of rollers                             | nRB      | 17     | -    |
| Profile parameter                             | ap       | 0.0005 | -    |
| Profile parameter                             | cp       | 16.2   | mm   |
| Profile parameter                             | dp       | 0.0    | mm   |
| Profile parameter                             | kp       | 1.0    | mm   |
| Edge radius                                   | re       | 0.7    | mm   |
|                                               | σRaceway | 0.1    | µm   |
| Combined standard derivation of roughness     | σRib     | 0.1    | µm   |
| Mixed friction parameters for raceway contact | BZH      | 2.1    |      |
| according to Zhou and Hoeprich [61,73]        | CZH      | 0.85   |      |
| Mixed friction parameters for rib contact     | BZH      | 2.1    |      |
|                                               |          |        |      |

*C*ZH 0.85

<span id="page-18-10"></span>**Table A2.** Geometrical data of a TRB type 32208.

# **References**

<span id="page-18-0"></span>1. Woydt, M. The importance of tribology for reducing CO<sup>2</sup> emissions and for sustainability. *Wear* **2021**, *474–475*, 203768. [\[CrossRef\]](https://doi.org/10.1016/j.wear.2021.203768)

according to Zhou and Hoeprich [\[61](#page-20-2)[,73\]](#page-20-15)

- <span id="page-18-1"></span>2. Arora, A.; Jha, S.; Saini, V. Aspects of green-sustainable tribology and its impacts on future product development: A review. *Ecol. Environ. Conserv.* **2019**, *25*, S146–S157.
- <span id="page-18-2"></span>3. Takahashi, K.; Suzuki, D.; Nagatomo, T. Effect of Axial Clearance on Rolling Element Load of Double Row Tapered Roller Bearings. *Q. Rep. RTRI* **2019**, *60*, 196–201. [\[CrossRef\]](https://doi.org/10.2219/rtriqr.60.3_196)
- 4. Xu, T.; Yang, L.; Wu, Y. Friction torque study on double-row tapered roller bearing. In Proceedings of the 2019 IEEE International Instrumentation and Measurement Technology Conference (I2MTC), Auckland, New Zealand, 20–23 May 2019; pp. 1–6. [\[CrossRef\]](https://doi.org/10.1109/I2MTC.2019.8827169)
- <span id="page-18-3"></span>5. Jiang, Z.; Huang, X.; Zhu, H.; Jiang, R.; Du, S. A new method for contact characteristic analysis of the tapered roller bearing in wind turbine main shaft. *Eng. Fail. Anal.* **2022**, *141*, 106729. [\[CrossRef\]](https://doi.org/10.1016/j.engfailanal.2022.106729)
- <span id="page-18-4"></span>6. Venner, C.H. Multilevel Solution of the EHL Line and Point Contact Problems. Ph.D. Thesis, Faculty of Mechanical Engineering, Universiteit Twente, Enschede, The Netherland, 1991.
- 7. Pan, P.; Hamrock, B.J. Simple Formulas for Performance Parameters Used in Elastohydrodynamically Lubricated Line Contacts. *J. Tribol.* **1989**, *111*, 246–251. [\[CrossRef\]](https://doi.org/10.1115/1.3261900)
- 8. Kragelskii, I.V.; Dobychin, M.N.; Kombalov, V.S. *Friction and Wear: Calculation Methods*; Pergamon Press: Oxford, UK, 1982; pp. 156–207.
- 9. Muraki, M.; Kimura, Y. Traction Characteristics of Lubricating Oils. 2. A Simplified Thermal Theory of Traction with a Non-Linear Viscoelastic Model. *J. Jpn. Soc. Lubr. Eng.* **1983**, *28*, 753–760. (In Japanese)
- 10. Forster, N.H.; Schrand, J.B.; Gupta, P.K. Viscoelastic Effects in MIL-L-7808-Type Lubricant, Part II: Experimental Data Correlations. *Tribol. Trans.* **1992**, *35*, 275–280. [\[CrossRef\]](https://doi.org/10.1080/10402009208982118)
- 11. Gupta, P.K.; Cheng, H.S.; Zhu, D.; Forster, N.H.; Schrand, J.B. Viscoelastic Effects in MIL-L-7808-Type Lubricant, Part I: Analytical Formulation. *Tribol. Trans.* **1992**, *35*, 269–274. [\[CrossRef\]](https://doi.org/10.1080/10402009208982117)
- 12. Houpert, L. Piezoviscous-Rigid Rolling and Sliding Traction Forces, Application: The Rolling Element—Cage Pocket Contact. *J. Tribol.* **1987**, *109*, 363–370. [\[CrossRef\]](https://doi.org/10.1115/1.3261367)
- 13. Kannel, J.W.; Bell, J.C. Interpretations of the Thickness of Lubricant Films in Rolling Contact. 1. Examination of Measurements Obtained by X-Rays. *J. Lubr. Technol.* **1971**, *93*, 478–484. [\[CrossRef\]](https://doi.org/10.1115/1.3451620)
- <span id="page-18-9"></span>14. Kannel, J.W.; Walowit, J.A. Simplified Analysis for Tractions between Rolling-Sliding Elastohydrodynamic Contacts. *J. Lubr. Technol.* **1971**, *93*, 39–44. [\[CrossRef\]](https://doi.org/10.1115/1.3451526)
- 15. Goksem, P.G.; Hargreaves, R.A. The Effect of Viscous Shear Heating on Both Film Thickness and Rolling Traction in an EHL Line Contact—Part I: Fully Flooded Conditions. *J. Lubr. Technol.* **1978**, *100*, 346–352. [\[CrossRef\]](https://doi.org/10.1115/1.3453183)
- <span id="page-18-5"></span>16. Dowson, D.; Higginson, G.R. *Elasto-Hydrodynamic Lubrication*; Pergamon Press: Oxford, UK, 1977; pp. 161–181.
- <span id="page-18-6"></span>17. Stribeck, R. Ball Bearing of Various Loads. *Trans. ASME* **1907**, *29*, 420–463.
- <span id="page-18-7"></span>18. Sjovall, H. The Load Distribution within Ball and roller Bearings under Given External Radial and Axial Loads. *Tek. Tidskr. Mek* **1933**, *9*, 97–102.
- <span id="page-18-8"></span>19. Lundberg, G.; Palmgren, A. Dynamic Capacity of Roller Bearings. *Acta Polytech. Mech. Eng. Ser. R. Swed. Acad. Eng. Sci.* **1952**, *2*, 96–127.

*Lubricants* **2023**, *11*, 369 20 of 22

- 20. Palmgren, A. *Ball and Roller Bearing Engineering*; SKF Industries Inc.: Philadelphia, PA, USA, 1959.
- 21. Lundberg, G. Elastische Berührung zweier Halbräume. *Forsch. Auf Dem Geb. Des Ingenieurwesens* **1939**, *10*, 201–211. [\[CrossRef\]](https://doi.org/10.1007/BF02584950)
- <span id="page-19-0"></span>22. Palmgren, A.G. Die Lebensdauer von Kugellagern (Life Length of Roller Bearings or Durability of Ball Bearings). *Z. Des Vereines Dtsch. Ingenieure* **1924**, *14*, 339–341.
- <span id="page-19-1"></span>23. Jones, A.B. A General Theory for Elastically Constrained Ball and Radial Roller Bearings under Arbitrary Load and Speed Conditions. *J. Basic Eng.* **1960**, *82*, 309–320. [\[CrossRef\]](https://doi.org/10.1115/1.3662587)
- <span id="page-19-2"></span>24. Harris, T.A. *Rolling Bearing Analysis*; Wiley: New York, NY, USA, 1966.
- <span id="page-19-3"></span>25. Qian, W. Dynamic Simulation of Cylindrical Roller Bearings—Dynamische Simulation von Zylinderrollenlagern. Ph.D. Thesis, RWTH Aachen, Aachen, Germany, 2014. Available online: <https://publications.rwth-aachen.de/record/229010/files/4903.pdf> (accessed on 23 August 2023).
- <span id="page-19-15"></span>26. Kiekbusch, T. Strategien zur Dynamischen Simulation von Wälzlagern. Ph.D. Thesis, TU Kaiserslautern, Kaiserslautern, Germany, 2017. Maschinenelemente und Getriebetechnik Berichte BD. 23/2017.
- <span id="page-19-4"></span>27. Hong, S.-H.; Tong, V.-C. Rolling-element bearing modeling: A review. *Int. J. Precis. Eng. Manuf.* **2016**, *17*, 1729–1749. [\[CrossRef\]](https://doi.org/10.1007/s12541-016-0200-z)
- <span id="page-19-5"></span>28. Stacke, L.-E.; Fritzson, D.; Nordling, P. BEAST—A rolling bearing simulation tool. *Proc. Inst. Mech. Eng. Part K J. Multi-Body Dyn.* **1999**, *213*, 63–71. [\[CrossRef\]](https://doi.org/10.1243/1464419991544063)
- 29. Stacke, L.-E.; Fritzson, D. Dynamic behavior of rolling bearings: Simulations and experiments. *Proc. Inst. Mech. Eng. Part J J. Eng. Tribol.* **2001**, *215*, 499–508. [\[CrossRef\]](https://doi.org/10.1243/1350650011543754)
- 30. Ioannides, E.; Stacke, L.-E.; Fritzson, D.; Nakhimovski, I. Multibody Rolling Bearing Calculations: Computer Programm BEAST. In Proceedings of the World Tribologie Congress III (WTC 2005), Washington, DC, USA, 12–16 September 2005; pp. 903–904. [\[CrossRef\]](https://doi.org/10.1115/WTC2005-64337)
- <span id="page-19-6"></span>31. Stacke, L.E.; Fritzson, D. *Simulation of Rolling Element Bearings*. SKF Nova AB S-412 88, SKF, Sweden. 1999. Available online: [https://www.researchgate.net/publication/253752292\\_Simulation\\_of\\_Rolling\\_Element\\_Bearings](https://www.researchgate.net/publication/253752292_Simulation_of_Rolling_Element_Bearings) (accessed on 28 June 2023).
- <span id="page-19-7"></span>32. Aramaki, H. Rolling Bearing Analysis Program Package BRAIN. *Motion Control* **1997**, *3*, 15–24.
- <span id="page-19-8"></span>33. Aramaki, H.; Nakano, Y.; Shoda, Y. Rolling Bearing Analysis Codes BRAIN—The Estimation of Rolling Bearing Performance for an Automotive Application. SAE International Congress and Exposition. 1997. Available online: [https://www.jstor.org/stable/](https://www.jstor.org/stable/44731255) [44731255](https://www.jstor.org/stable/44731255) (accessed on 23 June 2023).
- <span id="page-19-9"></span>34. Hahn, B.; Smolenski, M.; Neukirchner, J. Investigations of New Cage Designs for the Main Bearings in Multi-Megawatt Wind Power Plants. In Proceedings of the 2nd Conference for Wind Power Drives (CWD), Aachen, Germany, 3–4 March 2015; pp. 321–333.
- <span id="page-19-10"></span>35. Houpert, L. CAGEDYN: A Contribution to Roller Bearing Dynamic Calculations Part I: Basic Tribology Concepts. *Tribol. Trans.* **2009**, *53*, 1–9. [\[CrossRef\]](https://doi.org/10.1080/10402000903132093)
- 36. Houpert, L. CAGEDYN: A Contribution to Roller Bearing Dynamic Calculations Part II: Description of the Numerical Tool and Its Outputs. *Tribol. Trans.* **2009**, *53*, 10–21. [\[CrossRef\]](https://doi.org/10.1080/10402000903132101)
- <span id="page-19-11"></span>37. Houpert, L. CAGEDYN: A Contribution to Roller Bearing Dynamic Calculations. Part III: Experimental Validation. *Tribol. Trans.* **2010**, *53*, 848–859. [\[CrossRef\]](https://doi.org/10.1080/10402004.2010.496069)
- <span id="page-19-12"></span>38. Development of the Industry's Highest Precision and Fastest Integrated Bearing Dynamic Analysis System (IBDAS). Available online: [https://www.ntnglobal.com/en/news/new\\_products/news201100013.html](https://www.ntnglobal.com/en/news/new_products/news201100013.html) (accessed on 23 August 2023).
- <span id="page-19-13"></span>39. Liu, X.; Deng, S.; Teng, H. Dynamic stability analysis of cages in high-speed oil-lubricated angular contact ball bearings. *Trans. Tianjin Univ.* **2011**, *17*, 20–27. [\[CrossRef\]](https://doi.org/10.1007/s12209-011-1487-6)
- 40. Jin, K.F.; Yao, T.Q. Multi-Body Contact Dynamics Analysis of Angular Contact Ball Bearing. *Appl. Mech. Mater.* **2013**, *444–445*, 45–49. [\[CrossRef\]](https://doi.org/10.4028/www.scientific.net/AMM.444-445.45)
- 41. Wang, Y.; Wang, W.; Zhang, S.; Zhao, Z. Investigation of skidding in angular contact ball bearings under high speed. *Tribol. Int.* **2015**, *92*, 404–417. [\[CrossRef\]](https://doi.org/10.1016/j.triboint.2015.07.021)
- <span id="page-19-17"></span>42. Lacroix, S.; Nélias, D.; Leblanc, A. Four-Point Contact Ball Bearing Model with Deformable Rings. *J. Tribol.* **2013**, *135*, 031402. [\[CrossRef\]](https://doi.org/10.1115/1.4024103)
- 43. Qi, Z.; Wang, G.; Zhang, Z. Contact analysis of deep groove ball bearings in multibody systems. *Multibody Syst. Dyn.* **2015**, *33*, 115–141. [\[CrossRef\]](https://doi.org/10.1007/s11044-014-9412-0)
- <span id="page-19-16"></span>44. Sopanen, J.; Mikkola, A. Dynamic model of a deep-groove ball bearing including localized and distributed defects. Part 1—Theory. *Proc. Inst. Mech. Eng. Part K J. Multi-Body Dyn.* **2003**, *217*, 201–211. [\[CrossRef\]](https://doi.org/10.1243/14644190360713551)
- <span id="page-19-14"></span>45. Sopanen, J.; Mikkola, A. Dynamic model of a deep-groove ball bearing including localized and distributed defects. Part 2—Implementation and results. *Proc. Inst. Mech. Eng. Part K J. Multi-Body Dyn.* **2003**, *217*, 213–223. [\[CrossRef\]](https://doi.org/10.1243/14644190360713560)
- 46. Shi, Z.; Liu, J. An improved planar dynamic model for vibration analysis of a cylindrical roller bearing. *Mech. Mach. Theory* **2020**, *153*, 103994. [\[CrossRef\]](https://doi.org/10.1016/j.mechmachtheory.2020.103994)
- 47. Liu, J.; Ni, H.; Zhou, R.; Li, X.; Xing, Q.; Pan, G. A Simulation Analysis of Ball Bearing Lubrication Characteristics Considering the Cage Clearance. *J. Tribol.* **2023**, *145*, 1128–1146. [\[CrossRef\]](https://doi.org/10.1115/1.4056358)
- 48. Russell, T.; Sadeghi, F. The effects of lubricant starvation on ball bearing cage pocket friction. *Tribol. Int.* **2022**, *173*, 107630. [\[CrossRef\]](https://doi.org/10.1016/j.triboint.2022.107630)
- 49. Deng, S.; Zhao, G.; Qian, D.; Jiang, S.; Hua, L. Investigation of Oil–Air Flow and Temperature for High-Speed Ball Bearings by Combining Nonlinear Dynamic and Computational Fluid Dynamics Models. *J. Tribol.* **2022**, *144*, 071204. [\[CrossRef\]](https://doi.org/10.1115/1.4052965)

*Lubricants* **2023**, *11*, 369 21 of 22

50. Weinzapfel, N.; Sadeghi, F. A Discrete Element Approach for Modeling Cage Flexibility in Ball Bearing Dynamics Simulations. *J. Tribol.* **2009**, *131*, 021102. [\[CrossRef\]](https://doi.org/10.1115/1.3063817)

- 51. Nakhaeinejad, M.; Bryant, M.D. Dynamic Modeling of Rolling Element Bearings with Surface Contact Defects using Bond Graphs. *J. Tribol.* **2011**, *133*, 011102. [\[CrossRef\]](https://doi.org/10.1115/1.4003088)
- 52. Ashtekar, A.; Sadeghi, F. A New Approach for Including Cage Flexibility in Dynamic Bearing Models by using Combined Explicit Finite and Discrete Element Methods. *J. Tribol.* **2012**, *134*, 041502. [\[CrossRef\]](https://doi.org/10.1115/1.4007348)
- 53. Brouwer, M.D.; Sadeghi, F.; Ashtekar, A.; Archer, J.; Lancaster, C. Combined Explicit Finite and Discrete Element Methods for Rotor Bearing Dynamic Modeling. *Tribol. Trans.* **2015**, *58*, 300–315. [\[CrossRef\]](https://doi.org/10.1080/10402004.2014.968699)
- 54. Ashtekar, A.; Sadeghi, F. Experimental and Analytical Investigation of High Speed Turbocharger Ball Bearings. *ASME J. Eng. Gas Turbines Power* **2011**, *133*, 122501. [\[CrossRef\]](https://doi.org/10.1115/1.4004004)
- 55. Tong, V.-C.; Kwon, S.-W.; Hong, S.-W. Fatigue Life of Cylindrical Roller Bearings. *Proc. Inst. Mech. Eng. Part J J. Eng. Tribol.* **2016**, *231*, 623–636. [\[CrossRef\]](https://doi.org/10.1177/1350650116668767)
- 56. Ghaisas, N.; Wassgren, C.R.; Sadeghi, F. Cage Instabilities in Cylindrical Roller Bearings. *J. Tribol.* **2004**, *126*, 681–689. [\[CrossRef\]](https://doi.org/10.1115/1.1792674)
- <span id="page-20-0"></span>57. Singh, S.; Köpke, U.G.; Howard, C.Q.; Petersen, D. Analyses of Contact Forces and Vibration Response for a Defective Rolling Element Bearing using an Explicit Dynamics Finite Element Model. *J. Sound Vib.* **2014**, *333*, 5356–5377. [\[CrossRef\]](https://doi.org/10.1016/j.jsv.2014.05.011)
- <span id="page-20-1"></span>58. Aul, E. Analyse von Relativbewegungen in Wälzlagersitzen. Ph.D. Thesis, TU Kaiserslautern, Kaiserslautern, Germany, 2008. Maschinenelemente und Getriebetechnik Berichte BD. 05/2008.
- 59. Mármol Fernández, M. Development of a New Bearing Geometry to Reduce Friction Losses. Ph.D. Thesis, TU Kaiserslautern, Kaiserslautern, Germany, 2022. Maschinenelemente und Getriebetechnik Berichte BD. 43/2022.
- <span id="page-20-7"></span>60. Kiekbusch, T.; Sauer, B. Calculation of the dynamic behavior of rolling bearings with detailed contact calculations. In Proceedings of the EUROMECH Colloquium 578, Rolling Contact Mechanics for Multibody System Dynamics, Funchal, Portugal, 10–13 April 2017; ISBN 978-989-99424-3-1.
- <span id="page-20-2"></span>61. Teutsch, R. Kontaktmodelle und Strategien zur Simulation von Wälzlagern und Wälzführungen. Ph.D. Thesis, TU Kaiserslautern, Kaiserslautern, Germany, 2005. Maschinenelemente und Getriebetechnik Berichte BD. 01/2005.
- <span id="page-20-3"></span>62. *DIN ISO 26281*; Wälzlager—Dynamische Tragzahlen und Nominelle Lebensdauer-Berechnung der Modifizierten Nominellen Referenz-Lebensdauer für Wälzlager. European Committee for Standardization: Brussels, Belgium, 2010.
- <span id="page-20-4"></span>63. Teutsch, R.; Sauer, B. An Alternative Slicing Technique to Consider Pressure Concentrations in Non-Hertzian Line Contacts. *ASME J. Tribol.* **2004**, *126*, 436–442. [\[CrossRef\]](https://doi.org/10.1115/1.1739244)
- <span id="page-20-5"></span>64. Polonsky, I.A.; Keer, L.M. A numerical method for solving rough contact problems based on the multi-level multi-summation and conjugate gradient techniques. *Wear* **1999**, *231*, 206–219. [\[CrossRef\]](https://doi.org/10.1016/S0043-1648(99)00113-1)
- <span id="page-20-6"></span>65. Boussinesq, J. *Application des Potentiels à l'étude de l'équilibre et du Mouvement des Solides élastiques: Principalement au Calcul des Déformations et des Pressions que Produisent, dans ces Solides, des Efforts Quelconques Exercés sur une Petite Partie de leur Surface ou de leur Intérieur: Mémoire Suivi de Notes Etendues sur Divers Points de Physique, Mathématique et d'analyse*; Gauthier-Villars: Paris, France, 1885.
- <span id="page-20-8"></span>66. Dietl, P. Damping and Stiffness Characteristics of Rolling Element Bearings: Theory and Experiment. Ph.D. Thesis, Technische Universität Wien, Vienna, Austria, 1997. [\[CrossRef\]](https://doi.org/10.13140/RG.2.2.18506.29122)
- <span id="page-20-9"></span>67. Wisniewsky, M. Elastohydrodynamische Schmierung. In *Handbuch der Tribologie und Schmierungstechnik*, 9th ed.; Expert-Verlag: Renningen, Germany, 2000.
- <span id="page-20-10"></span>68. Bair, S.; Winer, W.O. A Rheological Model for Elastohydrodynamic Contacts Based on Primary Laboratory Data. *ASME J. Lubr. Technol.* **1979**, *101*, 258–265. [\[CrossRef\]](https://doi.org/10.1115/1.3453342)
- <span id="page-20-11"></span>69. Peeken, H.; Dicke, H.; Welsch, G. Viscosity-Pressure-Temperature Characteristics of Different Types of Lubricants and their Possible Influence on Fatigue. In Proceedings of the 3rd World Congress on Gearing and Power Transmission, Paris, France, 12–14.February 1992; pp. 527–536.
- <span id="page-20-12"></span>70. Gold, P.W.; Schmidt, A.; Loos, J.; Aßmann, C. Viskositäts-Druck-Koeffizienten von mineralischen und synthetischen Schmierölen. *Tribol. Schmier.* **2001**, *48*, 40–48.
- <span id="page-20-13"></span>71. Lubenow, K. Axialtragfähigkeit und Bordreibung von Zylinderrollenlagern. Ph.D. Thesis, Universität Bochum, Bochum, Germany, 2002.
- <span id="page-20-14"></span>72. Aramaki, H.; Cheng, H.S.; Zhu, D. Film Thickness, Friction, and Scuffing Failure of Rib/Roller End Contacts in Cylindrical Roller Bearings. *J. Tribol.* **1992**, *114*, 311–316. [\[CrossRef\]](https://doi.org/10.1115/1.2920889)
- <span id="page-20-15"></span>73. Zhou, R.S.; Hoeprich, M.R. Torque of Tapered Roller Bearings. *ASME J. Tribol.* **1991**, *113*, 590–597. [\[CrossRef\]](https://doi.org/10.1115/1.2920664)
- <span id="page-20-16"></span>74. Engel, S. Reibungs-und Ermüdungsverhalten des Rad-Schiene-Systems mit und ohne Schmierung. Ph.D. Thesis, Universität Magdeburg, Magdeburg, Germany, 2002.
- <span id="page-20-17"></span>75. Biboulet, N.; Houpert, L. Hydrodynamic force and moment in pure rolling lubricated contacts. Part 1—Line contacts. *Proc. Inst. Mech. Eng. Part J J. Eng. Tribol.* **2010**, *224*, 765–775. [\[CrossRef\]](https://doi.org/10.1243/13506501JET790)
- <span id="page-20-18"></span>76. Scheuermann, M. Dynamiksimulation zur Virtuellen Produktentwicklung von Rollenschienenführungen. Ph.D. Thesis, TU Kaiserslautern, Kaiserslautern, Germany, 2010. Maschinenelemente und Getriebetechnik Berichte BD. 07/2021.
- <span id="page-20-20"></span><span id="page-20-19"></span>77. Stolarski, T.A.; Tobe, S. *Rolling Contacts*; Professional Engineering Publishing Limited: London, UK, 2000; pp. 55–73.
- 78. Johnson, K. *Contact Mechanics*; Cambridge University Press: Cambridge, UK, 1985.
- <span id="page-20-21"></span>79. Wang, Q.; Chung, Y.-W. *Encyclopedia of Tribology*; Springer: New York, NY, USA, 2013.

*Lubricants* **2023**, *11*, 369 22 of 22

<span id="page-21-0"></span>80. Steinert, T. Das Reibmoment von Kugellagern mit Bordgeführtem Käfig. Ph.D. Thesis, RWTH Aachen, Shaker Verlag, Aachen, Germany, 1996.

- <span id="page-21-1"></span>81. Sebteini, S.; Hudak, R. *Mindestlast von Wälzlagern. Abschlussbericht zum Forschungsvorhaben Nr. 830 I (Heft 1504)*; FVA Forschungsvereinigung Antriebstechnik e.V.: Frankfurt am Main, Germany, 2022.
- <span id="page-21-2"></span>82. Liebrecht, J. Technisch-Mathematischer Ansatz zur Berechnung der Hydraulischen Verluste in Wälzlagern. Ph.D. Thesis, RPTU Rheinland-Pfälzische Technische Universität Kaiserslautern Landau, Kaiserslautern, Germany, 2018. Maschinenelemente und Getriebetechnik Berichte Bd. 30/2018.
- <span id="page-21-3"></span>83. Gonda, A.; Sauer, B.; Großberndt, D.; Schwarze, H. Experimentelle und numerische Untersuchungen der hydraulischen Verluste in voll-und teilgefluteten kegelrollenlagern. *VDI Berichte* **2019**, *2348*, 97–106. [\[CrossRef\]](https://doi.org/10.51202/9783181023488)
- <span id="page-21-4"></span>84. Maccioni, L.; Ruth, L.; Koch, O.; Concli, F. Load-independent power losses of full-flooded lubricated tapered roller bearings: Numerical and experimental investigation of the effect of operating temperature and housing walls distances. *Tribol. Trans.* 2023, *in press*.
- <span id="page-21-5"></span>85. Maccioni, L.; Chernoray, V.G.; Mastrone, M.N.; Bohnert, C.; Concli, F. Study of the impact of aeration on the lubricant behavior in a tapered roller bearing: Innovative numerical modelling and validation via particle image velocimetry. *Tribol. Int.* **2022**, *165*, 107301. [\[CrossRef\]](https://doi.org/10.1016/j.triboint.2021.107301)
- 86. Maccioni, L.; Chernoray, V.G.; Bohnert, C.; Concli, F. Particle Image Velocimetry measurements inside a tapered roller bearing with an outer ring made of sapphire: Design and operation of an innovative test rig. *Tribol. Int.* **2022**, *165*, 107313. [\[CrossRef\]](https://doi.org/10.1016/j.triboint.2021.107313)
- <span id="page-21-6"></span>87. Maccioni, L.; Chernoray, V.G.; Concli, F. Fluxes in a full-flooded lubricated Tapered Roller Bearing: Particle Image Velocimetry measurements and Computational Fluid Dynamics simulations. *Tribol. Int.* **2023**, *188*, 108824. [\[CrossRef\]](https://doi.org/10.1016/j.triboint.2023.108824)
- <span id="page-21-7"></span>88. Maccioni, L.; Concli, F. Computational fluid dynamics applied to lubricated mechanical components: Review of the approaches to simulate gears, bearings, and pumps. *Appl. Sci.* **2020**, *10*, 8810. [\[CrossRef\]](https://doi.org/10.3390/app10248810)
- <span id="page-21-8"></span>89. Brouwer, L.; Bader, N.; Beilicke, R.; Schwarze, H.; Poll, G.; Deters, L. *Tribologische Fluidmodelle Nebenaggregate—Tribologische Fluidmodelle für Nebenantriebsaggregate in Hybrid- und Elektrofahrzeugen—Abschlussbericht zum FVV-Forschungsvorhaben 597 (Heft 1092)*; Forschungsvereinigung Verbrennungskraftmaschinen e. V.: Frankfurt am Main, Germany, 2016.

**Disclaimer/Publisher's Note:** The statements, opinions and data contained in all publications are solely those of the individual author(s) and contributor(s) and not of MDPI and/or the editor(s). MDPI and/or the editor(s) disclaim responsibility for any injury to people or property resulting from any ideas, methods, instructions or products referred to in the content.