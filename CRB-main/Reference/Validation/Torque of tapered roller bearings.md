# Torque of Tapered Roller Bearings

R. S. Zhou

M. R. Hoeprich

The Timken Company, Canton, Ohio 44706-2798

An analytic tapered roller bearing torque model is presented along with laboratory test data. Initial results of this proposed model are favorable. An accurate general purpose torque prediction tool could be obtained by extending the concepts presented in conjunction with a more comprehensive analysis of actual bearing operating conditions. By using EHL (Elastohydrodynamic Lubrication) theory and micromacro contact analysis, the bearing torque can be determined by predicting each torque component for each roller due to raceway rolling, raceway moments due to EHL pressure distribution, and frictional force of rib-roller end contact. The roughness effect of contact surfaces, effect of EHL film thickness parameter (the ratio of film thickness to composite surface roughness), and thermal EHL effects are also included. A bearing torque test rig, which can measure the torque of cup race, cone race, and rib separately, was built and used to provide test data. Good agreement between the experimentally measured bearing torques and the predictions of the new torque model has been obtained. This torque model will provide a greater fundamental understanding and is more versatile over a wide range of operating conditions.

#### 1 Introduction

Bearing friction represents energy loss and heat generation. The study of bearing torque has been carried out for a number of years. Being capable of predicting the operating torque accurately allows the designer to correctly determine the energy loss and heat generation rate.

In 1959, Palmgren presented a bearing torque model [1]. A general expression for the operating torque of a tapered roller bearing was derived by Witte [2] based on determining a constant and exponents that came from experimental data, and the G factor obtained from the bearing geometry. With the development of elastodynamic lubrication (EHL) theory, a great deal of theoretical and experimental work has been published on both film thickness and traction in rolling and/or sliding contacts over the last couple of decades. The friction performance of the rib-roller end contact of tapered roller bearings was analyzed to optimize the geometric configuration [3]. A running torque formula for tapered roller bearings under axial load was derived by Aihara [4] by combining pure rolling resistance on the raceway and the friction force at the ribroller end contact. However, Aihara's running torque equation for tapered roller bearings under pure axial load was still based on modifying the analysis and experiments. Generally speaking, the formulas for predicting bearing torque were experimentally determined for given operating conditions. The sources of traction forces and moments of bearing torque were still not clearly determined. This model includes a different

Contributed by the Tribology Division of The American Society of Mechanical Engineers and presented at the Joint ASME/STLE Tribology Conference, Toronto, Canada, October 7-10. Manuscript received by the Tribology Division February 15, 1990; revised manuscript received June 23, 1990. Paper No. 90-Trib-62. Associate Editor: H. S. Cheng.

approach to the rib friction analysis and an improved raceway rolling resistance force analysis.

A new torque prediction model for tapered roller bearings is presented in this paper. By curve fitting the numerical data of isothermal EHL line contact, a formula was obtained that allows easy evaluation of the rolling resistance force at the raceway contacts. Based on EHL theory and micro-macro contact analysis, the torque of a single roller is theoretically determined by predicting each torque component due to raceway rolling resistances, and frictional force of rib-roller end rolling/sliding contact. The total bearing torque can be finally obtained by summing each roller's torque under either axial load and/or radial load. The effects of roughness of rib-roller contact surfaces, thermal EHL, Lambda ratio, and lubricant parameters are also considered in this torque model for tapered roller bearings.

## 2 Torque Prediction Model of a Tapered Roller Bearing

A section of a tapered roller bearing and the load reactions on a roller is shown in Fig. 1, where the concentrated loads are used to simplify the actual distributed loads on the raceways and rib-roller end contact.

The torque of tapered roller bearings mainly consists of the following parts:

- (a) Bearing torque due to raceway rolling and/or sliding
- (b) Bearing torque due to rib-roller end contact
- (c) Friction force between rollers and cage
- (d) Flinging forces of lubricant

Since (c) and (d) are relatively small, only (a) and (b) will be considered. Thus, to determine the friction forces and moments in a tapered roller bearing, the traction forces on raceways and

![](_page_1_Figure_1.jpeg)

Fig. 1 Forces acting on one tapered roller of a conventional tapered roller bearing

the rolling and sliding force in rib-roller end contacts will be analyzed.

#### (1) Forces and Moments in Contacts

(a) Shear Stress Forces. The internal friction or viscosity of the fluid gives rise to shear stresses,  $\tau$ , between the relative sliding layers. It can be shown that the rate of shear is equal to the viscosity  $(\eta)$  times the velocity gradient, du/dy, and hence

$$\tau = \eta \, \frac{\partial u}{\partial v} \tag{1}$$

For two rolling and sliding contact bodies, the viscous drag at the surfaces can be obtained by integrating the surface shear stress from inlet to outlet as derived by A. W. Crook [5],

$$F = \Sigma_{\tau} = F_{r0} + F_{s} = -\frac{1}{2} \int_{\text{inlet}}^{\text{outlet}} h \frac{\partial P}{\partial x} dx + (u_{2} - u_{1}) \int_{\text{inlet}}^{\text{outlet}} \frac{\eta}{h} dx$$

where  $F_{r_0}$  is the rolling resistance force (as shown in Fig. 2(a)),  $F_s$  is the friction force due to sliding, and h is the EHL film thickness. Only a very small amount of slip is required to generate the balancing force  $F_s$ .

(b) Pressure Forces. The EHL pressure distribution can be reduced to resultant forces in the X and Y directions ( $N_y$  and  $P_x$ , respectively)

$$N_{y} = \int P(x) \ dx \tag{3}$$

$$P_{x,i} = \int P(x) \, \frac{dy}{dx} \, dx \tag{4}$$

and

$$P_x = P_{x,1} + P_{x,2}$$

 $N_{\nu}$  will be offset from the Y-axis by an amount e,

$$e = \frac{1}{N_{\nu}} \int x P_i(x) dx$$
 (5)

The forces  $N_y$  and  $P_{x,i}$  result in equal and opposite moments on each body at each contact. Also, since  $P_x = 2 F_{r0}$ , we can write  $N_y e = P_x R_e = P_{x,1} R_1 = P_{x,2} R_2 = 2 F_{r0} R_e$ . The values due to above three forces for the contacts of roller-cup and roller-cone can be calculated numerically. Figure 2(b) shows an isothermal EHL numerical solution of those three equivalent values,  $N_0 e_0 = P_x R_{e,0} = 2 F_0 R_{e,0}$ , for the cup roller contact of a tapered roller bearing. Because the contact deflection is small,  $R_e$  can be used in calculating the moment  $P_x R_e$ . The equivalent moments  $(N_y e, P_x R_e, \text{ and } 2 F_{r0} R_e)$ , which were obtained by numerical solutions of isothermal elastohydrodynamic line contact of pure rolling, are shown in the Table 1.

#### (2) Analytic Torque Model of Tapered Roller Bearings

(a) Bearing Torque from Raceway Contacts,  $M_{\rm BRG,race}$ . The cone-roller and cup-roller raceway contacts of a tapered roller bearing are different with respect to pure rolling contact. The associated sliding and/or asperity contact reaction forces  $(F_{s,i}$  and  $F_{s,0})$  have to be determined in order to satisfy the force and moment equilibrium equations.

Figure 3(a) presents the forces and moments acting on a tapered roller surfaces, where  $F_{s,0}$  and  $F_{s,i}$  are the friction forces due to sliding and/or asperity contact on the raceways.  $F_{s,i1}$  and  $F_{s,01}$  are reaction force components associated with the rolling raceway forces,  $F_{s,i2}$  and  $F_{s,02}$  are reaction force components associated with the rib-roller end forces ( $F_{s,i} = F_{s,i1} + F_{s,i2}$ ), and  $F_{s,0} = F_{s,01} + F_{s,02}$ ). From the force and moment equilibrium, momentarily neglecting the rib-roller end contact, and recalling that the net moment due to the pressure distribution is zero, the following equations can be written

$$\Sigma F_x = F_{s,i1} - F_{s,01} + P_{r,i} - P_{r,0} + F_0 - F_i = 0$$
 (6)

#### Nomenclature

 a,b = semi-major and semiminor axis of an elliptical contact

B =exponent of asperity load

C =exponent of asperity load

 $D_r$  = mean roller diameter

 $D_{LE}$  = large end roller diameter

e = offset of centroid of pressure distribution from geometric center

E' = equivalent Young's modulus

 $F_{i, j}$  = rolling resistance of cone for No. j roller

 $F_{o,j}$  = rolling resistance of cup for No, j roller

 $F_{\text{rib, }j}$  = friction force of ribroller end contact for No. j roller

 $G_{i,j}$  = dimensionless material parameter of cone for No. j roller

 $G_{o,j}$  = dimensionless material parameter of cup for No. j roller

 $G_{\infty}$  = limiting elastic shear modulus

h = height of rib-roller end contact

 $h_c = \text{center EHL film thick-}$ 

 $h_0(x)$  = undeformed height of surface profile from its meanline

L = roller length

 $L_{Apex}$  = apex length of tapered roller bearing

 $M_{i, j}$  = moment due to cone EHL pressure distribution for No. j roller  $M_{o,j}$  = moment due to cup EHL pressure distribution for No. j roller

 $M_r =$ moment at rib-roller end contact

 $N_{i, j}$  = normal force of coneroller contact for No. j

 $N_{o, j}$  = normal force of cuproller contact for No. j

 $N_{\text{rib}, j}$  = normal force of ribroller end contact for No. j roller

 $P_i(\xi')$  = local asperity contact pressure

 $P_0$  = yield stress of material  $P_x$  = pressure force along rolling direction

 $R_{Apex}$  = radius of roller end curvature

![](_page_2_Figure_1.jpeg)

Fig. 2(a) Rolling resistance force  $F_{r0}$ , pressure forces  $P_{x1}$ ,  $P_{x2}$ , and normal load  $N_v$  for line contact pure rolling

![](_page_2_Figure_3.jpeg)

Fig. 2(b) An numerical isothermal EHL solution of three equivalent traction moments  $(N_0e_0, P_xR_e, and 2F_0R_e)$  for cup-roller contact of a tapered roller bearing

$$\Sigma M = (F_{s,01} + F_{s,i1} - F_0 - F_i) \frac{D_r}{2} = 0$$
 (7)

Solving for  $F_{s,i1}$  and  $F_{s,01}$ 

![](_page_2_Figure_7.jpeg)

Fig. 2(c) Nondimensional load parameter versus rolling resistance force for various nondimensional speed parameters by the rolling resistance formula (17). (Note: The solid points are the data from full numerical solution of isothermal EHD line contact.)

$$F_{s,i1} = F_i + \frac{1}{2} \left( P_{r,0} - P_{r,i} \right) \tag{8}$$

$$F_{s,01} = F_0 - \frac{1}{2} (P_{r,0} - P_{r,i})$$
 (9)

Equations for cup and cone torque may now be written and combined to obtain a convenient bearing moment equation (see Fig. 3(b)). The cup and cone moment equations shown below are identical with the appropriate substitutions. It is simply quicker to develop equation (10) in this manner.

$$\begin{split} M_{\rm BRG,race} &= M_{\rm cup,race} = F_0 R_0 + F_{s,01} R_0 \\ &= 2 F_0 R_0 - \frac{1}{2} \; R_0 (P_{r,0} - P_{r,i}) \\ M_{\rm BRG,race} &= M_{\rm cone,race} = F_i R_i + R_{s,i1} R_i \\ &= 2 F_i R_i + \frac{1}{2} \; R_i (P_{r,0} - P_{r,i}) \end{split}$$

## Nomenclature (cont.)

 $R_{e, i}$  = equivalent radius of cone-roller contact

 $R_{e, 0}$  = equivalent radius of cuproller contact

 $R_{\text{rib}}$  = radius of rib contact point to rotating center of bearing

 $S_0$  = critical shear stress of material

 $U_{i,j}$  = dimensionless speed parameter of cone for No. j roller

 $U_{o,j}$  = dimensionless speed parameter of cut for No. j

 $u_{\rm rib} = {\rm velocity\ of\ rib}$ 

 $u_{r0, \text{ end}} = \text{velocity of roller end}$  $v_s = \text{sliding speed of rib-}$ 

 $W_{i,j}$  = roller end contact dimensionless load parameter of cone for No. j roller

 $W_{0,j}$  = dimensionless load parameter of cut for No. j

 $z_r = \text{number of rollers}$ 

α = pressure-viscosity coefficient of lubricant

 $\beta = 1/2$  roller included angle

 $\eta_0$  = viscosity of lubricant at atmospheric pressure

 $\tau$  = shear stress

 $\tau_L$  = limiting shear stress

 $\mu_{\text{rib}}$  = friction coefficient of rib-roller end contact

 $\mu_{r,a}$  = rib friction coefficient due to asperity contact

 $\mu_{r, EHL}$  = rib friction coefficient due to EHL lubricant shearing

 $\phi_c$  = surface film constant  $\phi_T$  = EHL thermal reduction

 $\phi_{Ti,j}$  = EHL thermal reduction factor of cone contact for No. j roller

 $\phi_{T0, j}$  = EHL thermal reduction factor of cup content for i roller

 $\phi_{T, \text{ rib}, j} = \text{EHL}$  thermal reduction factor of rib contact for No. j roller

 $\xi$  = dummy variable for x $\lambda$  = dummy variable for y

 $\Lambda$  = ratio of film thickness to composite surface roughness

 $\omega_R = \text{angular velocity of roller}$ about its center relative
to rotating axes

 $\omega_{BRG}$  = cone angular velocity

 $\Omega'$  = contact region

 $\omega_c = ext{angular velocity of bearing roller center about bearing center line}$ 

Table 1 Numerical solutions of isothermal EHD line contact on the effects of offset distance and torque moments of dimensionless load, speed, and material parameters

|                                            |                                                        | , . ,                    |                                                                | · · · · · · · · · · · · · · · · · · ·                    |                                                          |                                                          |
|--------------------------------------------|--------------------------------------------------------|--------------------------|----------------------------------------------------------------|----------------------------------------------------------|----------------------------------------------------------|----------------------------------------------------------|
| Load $W = \frac{w}{E'R_e}$ $W \times 10^4$ | Speed $U = \frac{\eta_0 u}{E' R_e}$ $U \times 10^{11}$ | Material $G = \alpha E'$ | Offset(M) $e \times 10^{3}$                                    | Moments $\left(\frac{N}{M} \times M\right)$              |                                                          |                                                          |
| W X 10.                                    |                                                        |                          |                                                                | $N \times e$                                             | $P_x \times R_e$                                         | $2F_{r0}$ , $\times R_e$                                 |
| 1.0<br>1.5<br>2.0<br>2.5<br>3.0            | 35.0                                                   | 3745.                    | 0.00646<br>0.00484<br>0.00375<br>0.00318<br>0.00276<br>0.00220 | 0.5561<br>0.6248<br>0.6473<br>0.6849<br>0.7133<br>0.7616 | 0.5483<br>0.6133<br>0.6594<br>0.7023<br>0.6923<br>0.7278 | 0.5485<br>0.6135<br>0.6600<br>0.7031<br>0.6933<br>0.7282 |
| 2.0                                        | 1.0<br>3.0<br>10.<br>60.<br>100.                       | 3745.                    | 0.000397<br>0.000787<br>0.00177<br>0.00502<br>0.00647          | 0.0684<br>0.1358<br>0.3154<br>0.8649<br>1.1148           | 0.0612<br>0.1452<br>0.3169<br>0.8775<br>1.1277           | 0.0623<br>0.1455<br>0.3172<br>0.8781<br>1.1290           |

![](_page_3_Picture_3.jpeg)

Fig. 3(a) Traction forces and moments acting on the surfaces of a tapered roller

![](_page_3_Picture_5.jpeg)

Fig. 3(b) Traction forces and moments acting on the cone and cup, where  $\textit{M}_{\text{cone}} = \textit{M}_{\text{cup}} = \textit{M}_{\text{BRG}}$ 

where

$$R_0 - R_i = D_r \cos(\alpha_i + \beta)$$

By combining the above two equations, the bearing torque from raceway contacts becomes

$$M_{\text{BRG,race}} = F_0 R_0 + F_i R_i - \frac{1}{4} D_r \cos(\alpha_i + \beta) (P_{r,0} - P_{r,i})$$

$$= F_0 (R_0 - R_{e,0} \cos(\alpha_i + \beta))$$

$$+ F_i (R_i + R_{e,i} \cos(\alpha_i + \beta))$$
(10)

(b) Bearing Torque From Rib-Roller End Contact,  $M_{\rm BRG,rib}$ . From the moment equilibrium about the cone-roller contact line, the following equation can be written (see Fig. 3(a))

$$\Sigma M = -F_{\rm rib}h + F_{s,02}D_r - M_r = 0 \tag{11}$$

i.e., 
$$F_{s,02} = \frac{F_{\text{rib}} h}{D_r} + \frac{M_r}{D_r}$$
 (12)

The bearing torque due to rib-roller end contact becomes

$$M_{\text{BRG,rib}} = M_{\text{cup,rib}} = F_{s,02} R_0$$

$$= F_{\text{rib}} \frac{h}{D_r} R_0 + \frac{M_r}{D_r} R_0$$
(13)

where  $M_r$  is the friction moment due to the approximate

![](_page_3_Picture_17.jpeg)

Fig. 3(c) Diagram of velocity and friction force in the elliptic area of rib-roller end contact

elliptical area of the rib-roller end contact, as shown in Fig. 3(c), and it can be calculated as

$$M_r = \int_{-a}^{a} \int_{-b}^{b} \rho \times \mathbf{F}_{mr} \, dx \, dy$$

$$= \frac{3N_{\text{rib}}}{2a \, b \, \pi} \int_{-a}^{a} \int_{-b}^{b} \mu_{\text{rib}} \, \bar{P} \, \rho \times \mathbf{f}_{mr} dx \, dy \qquad (14)$$

where,

$$\tilde{P} = \left(1 - \frac{x^2}{a^2} - \frac{y^2}{b^2}\right)^{0.5}$$

and  $N_{\rm rib}$  is the contact load of rib-roller end contact, and  $f_{mr}$ is the unit vector of force  $F_{mr}$ .

(c) Total Bearing Torque,  $M_{{\rm BRG},t}$ . From equations (10) and (13), the total bearing torque  $M_{BRG,t}$  for a single roller of a unit contact length becomes

$$M_{\mathrm{BRG},i} = M_{\mathrm{BRG,race}} + M_{\mathrm{BRG,rib}}$$
  
=  $F_0(R_0 - R_{e,0} \cos(\alpha_i + \beta)) + F_i(R_i)$ 

+ 
$$R_{e,i}$$
cos  $(\alpha_i + \beta)$ ) +  $F_{rib} \frac{h}{D_r} R_0 + M_r \frac{R_0}{D_r}$  (15)

The rolling resistance forces,  $F_0$  and  $F_i$ , and friction force of rib-roller end contact,  $F_{\rm rib}$ , can be calculated as follows:

## (3) Rolling Resistance Forces, $F_0$ and $F_i$

The lubricant film shape (h) and the pressure distribution (p) of an isothermal EHL line contact can be determined by solving the Reynolds equation, elasticity equation of line contact, and satisfying the viscosity-pressure relationship simultaneously. As soon as the numerical results of contact pressure distribution, film shape, and elastic deformation are obtained, the rolling resistance force can be calculated. By integrating the pressure distribution with equation (5), the offset of the centroid of the pressure distribution from geometric contact center can be determined.  $e_0$  and the rolling resistance force  $F_0$  are shown in Fig. 2(b).

The method described above to obtain the rolling resistance solution is too cumbersome to be used by an engineer and involves too much computation time for a designer. By using a least-squares power fit, after curve fitting the load parameter W, material parameter G, and speed parameter U as a function of rolling resistance forces of pure rolling for several sets of numerical data of isothermal EHL line contact, it was found that

$$F_{r0,\rm iso} \propto W^{0.246}$$

and,

$$F_{r0,\rm iso} \propto (G\ U)^{0.648} \tag{16}$$

The above proportionality equations establish the rolling resistance force relationship with load, speed, and material parameters. By using the same format developed by Goksem and Hargreaves [6], a composite rolling resistance equation of isothermal EHD line contact can be expressed as

$$F_{r0,\text{iso}} = \frac{Ne}{2R_e} = 29.2 \frac{R_e}{\alpha} (GU)^{0.648} W^{0.246}$$
 (17)

where

$$G = \alpha E'$$
, nondimensional material parameter; (18)

$$U = \frac{\eta_0 \ u_r}{E' \ R_e}$$
, nondimensional speed parameter; (19)

$$W = \frac{w}{E'R_e}$$
, nondimensional load parameter; (20)

w = N/L, load per unit width of line contact;  $R_e$  is the equivalent radius of roller with cup or cone contact curvatures, and  $\alpha$  is the pressure-viscosity coefficient of the lubricant.

The relationship between load parameter and rolling resistance for various speed parameters are shown in Fig. 2(c) predicted by the above rolling resistance equation (17), and the solid points in Fig. 2(c) are the full numerical solutions of isothermal EHD line contact for the same operating data.

To account for the thermal effect of inlet shear heating, a thermal reduction factor  $\phi_T$  developed by Zhu and Cheng [7] is used to modify the isothermal film thickness formula,

$$\phi_T = \frac{1 - 13.2(P_{\rm HZ}/E')L_T^{0.42}}{1 + 0.213(1 + 2.23\ S^{0.83})L_T^{0.64}} \tag{21}$$

where.

 $P_{\rm HZ}=$  Hertzian contact pressure  $L_T=(-d\eta_0/dT)~u_r^2/K$  is the thermal loading parameter

 $S = V_s/V_r$  is the ratio of sliding to rolling velocity

K =thermal conductivity of lubricant

 $u_r = \text{rolling velocity.}$ 

Then, the rolling traction formula for a fully flooded EHL contact can be derived as,

$$F_{r0} = \phi_T F_{r0,iso} \tag{22}$$

Equation (17), (21), and (22) can be used to calculate both cup and cone rolling resistance forces  $F_0$  and  $F_i$ . Experimental results support the use of  $\phi_T$ .

When a limited quantity of oil is present and the inlet zone is not fully flooded, it is hypothetical that a starvation factor  $\phi_s$  [8] could be used to reduce the effective film thickness. This would have to be verified with further experimentation. The equation (22) could then be written as,

$$F_{r0} = \phi_s \phi_T F_{r0,iso} \tag{23}$$

In the case of a fully flooded contact, the starvation factor  $\phi_s$  equals 1.0, which is the value used in the data comparisons to be shown.

#### (4) Friction Force of Rib-Roller End Contacts, Frib

The rib speed and the roller end speed relative to the contact center of contact area are.

$$u_{\rm rib} = R_{\rm rib} (\omega_{\rm BRG} - \omega_c) \tag{24}$$

$$u_{r0,\text{end}} = r_{rc}\omega_R \tag{25}$$

where,  $R_{rib}$  and  $r_{rc}$  are the radii from the point of rib-roller end contact to the bearing center and roller rotating center, respectively. The friction force of rib-roller end contact  $F_{\rm rib}$ consists of two parts:

$$F_{\rm rib} = F_{r,a} + F_{r,\rm EHL} \tag{26}$$

where,  $F_{r,a}$  is the frictional force due to asperity contact for a roller, and  $F_{r,\mathrm{EHL}}$  is the traction force due to EHL lubricant

(a) Traction Force due to Asperity Contacts,  $F_{r,a}$ . When a bearing operates at a low Lambda condition at the rib-roller end contact (Lambda ( $\Lambda$ ) =  $h/\sigma_c$ , the ratio of EHL film thickness to composite surface roughness), the term  $F_{r,a}$  will be the important part of the total rib-roller end friction force  $F_{rib}$ .

A micro-macro contact model was developed to analyze micro-pitting [9]. This model can be used to determine the asperity load of contact and the friction due to asperity contact. The contact between a rough elastic-perfectly plastic surfaces with purely longitudinal asperities and a smooth rigid plane was numerically simulated [10]. The deflection of the surface profile and the deformed height of surface profile above the contacting rigid plane are given by

$$d(x) = -\frac{4}{\pi E'} \int_{\Omega'} \ln \left| \frac{\xi' - x}{\xi' - x_r} \right| P_i(\xi') d\xi'$$
 (27)

$$h(x) = d(x) + \tilde{h} + h_0(x) \tag{28}$$

where  $\xi'$  the dummy variable for x,  $\Omega'$  the contact region,  $x_r$ the fixed reference position on the surface profile,  $P_i$  the local asperity contact pressure,  $\tilde{h}$  the compliance, and  $h_0(x)$  the undeformed height of the surface profile from its meanline.

When a surface profile is taken by means of a profilometer connected with a data acquisition system, it is convenient to use a digital filtering method to filter out the fine structures of the profiles and find the major surface structure that includes both surface roughness and waviness as suggested by Hirst and Hollander [11]. Figure 4 shows two typical composite surface profiles in which the fine structures have been filtered. One is a standard ground surface (surface roughness  $R_a$  is about

![](_page_5_Figure_1.jpeg)

Fig. 4 The composite surface profiles (after run in and filtering fine structures of profiles) of standard grinding surface, and special finishing surface

![](_page_5_Figure_3.jpeg)

Fig. 5 The ratio of the asperity load to the total contact load versus the Lambda ratio for different surface finishes

 $0.25~\mu m$ ), and the other is a special finished surface ( $R_a$  is about 0.075  $\mu m$ ). Applying these two surface profiles in the rough-contact analysis, the calculated ratio of asperity load to total contact load versus the Lambda value is plotted in Fig. 5. It is interesting to observe that because of the contact of asperities, the rougher surface has a wider range of Lambda to effect the total asperity contact load than the smoother surface. If a function is used to fit the above curves, the ratio of asperity contact load to total applied load can be written in general as

$$\frac{N_a}{N} = \exp^{-B\Lambda^c} \tag{29}$$

where  $N_a$  is the load supported by asperity contacts, and N is the total load. The constants B and C are dependent upon the profiles of contact surfaces and the material parameters. Once the material and surface profiles are determined, the constants B and C can be obtained.

The Lambda value can be expressed as

$$\Lambda = \frac{h_{\rm rib}}{\sigma_c} = \frac{\phi_{T,\rm rib}h_{\rm rib,iso}}{\sigma_c}$$
 (30)

where,  $\phi_{T,\mathrm{rib}}$  is the EHL thermal reduction factor for the ribroller end contact, as in equation (21), and  $\sigma_c$  is the composite surface roughness of rib and roller end surfaces. The isothermal central film thickness formula for the most general case of elliptical contact is expressed by Chittenden, Dowson et al. [12],

$$h_{\text{rib,iso}} = 4.30 \ U^{0.68} G^{0.49} W^{-0.073} R_e (1 - e^{-1.28 \zeta})$$
 (31)

where,  $\zeta$  is the elliptical ratio, and  $R_e$  is the effective radius of curvature in the direction of lubricant entrainment.

From the adhesion theory of metals with contaminated films,

the friction coefficient due to asperity contact can be expressed as,

$$\mu_a = \frac{\text{critical shear stress of the interface}}{\text{yield pressure of the bulk metal}}$$
(32)

It can be written as,

$$\mu_a = \frac{\phi_c S_0}{P_a} \tag{33}$$

where,  $S_0$  is the critical shear stress for the metal,  $P_0$  is the yield stress, and  $\phi_c$  is the surface film constant. Both  $P_0$  and  $S_0$  must refer to the softer of the metals of contacting bodies. For most metals, the ratio of  $S_0/P_0$  is about 0.2.  $\phi_c$  is determined by the surface films of two contact surfaces, which varies with lubricant additives (chemical reaction, absorption, or desorption etc.) under operating condition, normally  $\phi_c \leq 1.0$ . For base oil,  $\phi_c$  can be simplified to equal 1.0.

The frictional force of the rib-roller end contact due to asperity contacts can be written as,

$$F_{r,a} = \mu_a N_a = \frac{\phi_c S_0}{P_0} N_{\text{rib}} \exp^{-B\Lambda^c}$$
 (34)

where, constant B and c can be obtained by interpolating the curves of  $N_a/N$  as shown in Fig. 5, or by calculating two given profiles of contact surfaces to find the exact  $N_a$  value for a given operating condition.

(b) Traction Force due to EHL Lubricant Shearing,  $F_{r,\text{EHL}}$ . To calculate traction in the rib-roller end contact which has a slide-to-rolling ratio of around 0.25 to 0.4, Bair and Winer's rheological model [13] can be adopted,

$$\dot{\gamma} = \left(\frac{1}{G_{\infty}}\right) \dot{\tau} - \frac{\tau_L}{\eta} \ln\left(1 - \frac{\tau}{\tau_L}\right) \tag{35}$$

There are three primary physical properties of the lubricant to be used in relating the shear stress  $\tau$  and the shear rate  $\dot{\gamma}$ . These are the limiting elastic shear modulus,  $G_{\infty}$ , the low shear stress viscosity,  $\eta$ , and the limiting shear stress,  $\tau_L$ . All of them are functions of pressure and temperature.

The limiting elastic shear modulus,  $G_{\infty}$ , and the limiting shear stress,  $\tau_L$ , can be obtained by Dyson's formulas [14],

$$G_{\infty} = \frac{1.2 P}{2.52 + 0.024 t} - 10^8 \tag{36}$$

$$\tau_L = 0.25 G_{\infty} \tag{37}$$

and let 
$$\dot{\gamma} \simeq (u_{\rm rib} - u_{\rm Ro, end})/h_c$$
 (38)

where  $h_c$  is the center film thickness of rib-roller end EHL contact, P is the contact pressure and t is the lubricant temperature.

By iterating on equations (35) to (38), the local shear stress  $\tau(x,y)$  can be solved at each point in the grid of Hertzian contact area of rib-roller end contact. The rolling/sliding traction due to lubricant shear stress between the rib and roller end can be calculated by the integration:

$$F_{r,\text{EHL}} = \phi_{T,\text{rib}} \int_{-a}^{a} \int_{-b}^{b} \tau(x,y)_{x} dx dy$$
 (39)

Adding  $F_{r,\mathrm{EHL}}$  (equation (39)) with  $F_{r,a}$  (equation 34)), the frictional force of rib-roller end contact,  $F_{\mathrm{rib}}$ , can be determined by

$$F_{\text{rib}} = F_{r,a} + F_{r,\text{EHL}} = \mu_{\text{rib}} N_{\text{rib}}$$
 (40)

where.

$$\mu_{\rm rib} = \mu_{r,a} + \mu_{r,\rm EHL} \tag{41}$$

## (5) Total Bearing Torque, $M_{\rm BRG,total}$

After calculating all the forces and moments for each roller,

![](_page_6_Picture_1.jpeg)

Fig. 6 Schematic diagram of bearing torque test rig

the total bearing torque,  $M_{BRG,total}$ , can be written as,

$$\begin{aligned} M_{\text{BRG,total}} &= \sum_{j=1}^{z_r} \left[ F_{0,j}(R_0 - R_{e,0} \cos(\alpha_i + \beta)) \right. \\ &\left. + F_{i,j}(R_i + R_{e,i} \cos(\alpha_i + \beta)) + F_{\text{rib},j} \frac{h}{D} R_0 + M_{r,j} \frac{R_e}{D} \right] \end{aligned}$$

where, z, is the number of rollers, and

$$F_{i,j} = \phi_{Ti,j}\phi_{si,j} \ 29.2 \frac{R_e}{\alpha} L \ (G_{i,j}U_{i,j})^{0.648} W_{i,j}^{0.246}$$
 (43)

$$F_{0,j} = \phi_{To,j}\phi_{so,j}29.2 \frac{R_e}{\alpha} L (G_{0,j}U_{0,j})^{0.648} W_{o,j}^{0.246}$$
 (44)

$$F_{\text{rib},j} = \frac{\phi_{c,j} S_0}{P_0} N_{\text{rib},j} \exp^{B\Lambda^c} j + \phi_{T,\text{rib},j} \iint \tau_j(x,y) dx dy \qquad (45)$$

$$M_{r,j} = \frac{3N_{\text{rib},j}}{2 \ a_{i}b_{i}\pi} \int_{-a_{j}}^{a_{j}} \int_{-b_{j}}^{b_{j}} \mu_{\text{rib},j} \ \bar{P}_{j} \ \rho_{j} \times \mathbf{f}_{mr} dx \ dy$$
 (46)

The constants and factors (such as  $S_0/P_0$ ,  $\phi_T$ ,  $\phi_c$ , B, C, etc.), equivalent radii ( $R_{e,0}$  and  $R_{e,i}$ ), and all nondimensional parameters (such as  $G_{0,j}$ ,  $G_{i,j}$ ,  $U_{0,j}$ ,  $U_{i,j}$ ,  $W_{0,j}$ , and  $W_{i,j}$ ) for each roller have been discussed previously, and can be determined by the given material, surface profiles, lubricant, and operating conditions of a given tapered roller bearing.

#### 3 Torque Test Rig

In order to understand the torque performance of tapered roller bearings, a bearing torque test rig that can measure the torques of the cup race, the cone race, and the cone rib separately was built.

The schematic diagram of the bearing torque test rig is shown in Fig. 6. The test shaft, which is used to mount the test bearing, is supported by two air journal bearings. Two air thrust bearings were designed to supply the bearing's inner race axial load and outer race reaction load. The cup housing is totally floating. The cup housing is supported by one air journal bearing and one air thrust bearing. Therefore, the bearing torque can be measured by the reaction force of the floating bearing housing. An air cylinder provides the axial bearing test load.

The rib of the test bearing is separated from the cone, and connected to the cone with three support arms. The bending strains of the arms, which represent the rib torque, are measured by strain gages. Thermisters are mounted on the test bearing near the contact surfaces to measure the temperatures of the rib, cone, and cup during the tests. All signal wires for strain gages and thermisters through the center hole of the rotary shaft, and are connected through a multi-channel rotary transformer. Thermal calibrations were made for all thermisters and strain gages after assembly in order to obtain the thermal correction factor during various operating conditions.

![](_page_6_Figure_15.jpeg)

Fig. 7(a) Predicted friction coefficients of rib-roller end contact for a typical test bearing

![](_page_6_Figure_17.jpeg)

Fig. 7(b) Predicted total friction coefficient of rib-roller end contact versus test results. (Test bearing and operating condition: as same as Fig. 8, SAE20 oil, oil operating temp. 49°C)

### 4 Torque Test Results and Torque Predictions

The bearing torque test rig has been run under various operating conditions and lubricants. Good agreement has been obtained by comparing the predictions of the new analytic torque model with the test results obtained with the bearing torque rig and other bearing test machines.

To determine the rib-roller end friction force  $F_{\rm rib}$ , both asperity contact and lubricant shearing between the rib and roller end are considered in this torque model. Figure 7(a) shows the predicted rib-roller end friction coefficients for a typical bearing. It is obvious that the friction due to asperity contact is important when the bearing was run in thin EHL film conditions. The test results of the friction coefficient due to rib-roller end contact versus the predicted value calculated by this torque model is shown in Fig. 7(b).

The test results of total bearing torque and the torque model's predictions for a typical tapered roller bearing under two kinds of oil and load conditions are plotted in Fig. 8. The contact stresses for this data were 0.85 GPa (SAE20 oil) and 1.47 GPa (Vactra oil), which are in the general range of real applications. The bearing torque curve turns flatter when the bearing speed increases, especially when using thick lubricant. The thermal EHL effect  $\phi_T$  will then play an important role in torque prediction. This is shown by the good fit of the calculated line which includes the thermal reduction factor. Without the thermal reduction factor the predicted torque is too high, especially in the case of higher speed and heavy viscous oil.

For a bearing operating under low Lambda values, the torque due to rib-roller end contact is important as is shown in Fig. 9. When the bearing speed increases to a certain range (above

![](_page_7_Figure_1.jpeg)

Fig. 8 Test results of total bearing torque versus torque model's predictions for a typical tapered roller bearing under 2 kind of oils, loads, and various speed conditions. (Test bearing: cup work point dia. 120 mm, cup raceway angle 20°49', roller length 11.7 mm, 24 rollers, oil operating temp. 49°C.)

1600 rpm), the raceway torque is the major part of total bearing torque. The agreement between test data and the analytic prediction is still observed.

Figure 10 shows the plots of test results of high speed bearing application versus the new torque model predictions. Even if a very thin lubricant (43 SUS at 210°F) was used in the tests, the new analytic model still shows good prediction. It is obvious that the thermal effect in EHL rolling contact is very important in torque prediction of high speed bearing applications.

#### 5 Conclusions

- 1. An analytic torque model for tapered roller bearings has been derived and tested for tapered roller bearing torque under various operating conditions. Good agreement was confirmed between the predictions of the torque model and test results.
- 2. From the analysis of rolling resistance force, it can be seen that in order to satisfy force and moment equilibrium, slight sliding and/or asperity contacts occur during bearing operation. Then, the amount of shear stresses due to sliding and/or friction due to asperity contacts are just enough to achieve equilibrium with the rolling resistance forces and EHL pressure moments.
- 3. The torque model analytically predicts each torque component quantitatively, and provides a basic understanding of the sources of bearing torque and heat generation for a given tapered roller bearing under given operating conditions.
- 4. Because the new model is based on the contacts of a single roller, it can be easily used for all load conditions, either pure axial load, or radial load, or mixed load applications. On the other hand, the new model has the potential for future developments of bearing torque analysis, such as chemical effects, misalignment, edge loading, and grease lubrication, etc.

## Acknowledgment

The authors wish to express their thanks to The Timken Company for support and permission to publish this work and also to their colleagues, P. K. Kropp, G. E. Kreider, D. K. Lawrentz, C. A. Moyer, and D. C. Witte, for their assistance. The authors want to express their gratitude for discussions and assistance from Prof. H. S. Cheng of Northwestern University and Dr. F. Sadeghi of Purdue University.

#### References

- 1 Palmgren, A., "Ball and Roller Bearing Eng.," 3rd ed., Burbank, Philadelphia, 1959, pp. 34-41.
  2 Witte, D. C., "Operating Torque of Tapered Roller Bearings," ASLE
- 2 Witte, D. C., "Operating Torque of Tapered Roller Bearings," ASLE Trans., Vol. 16, No. 1, 1973, pp. 61-67.

![](_page_7_Figure_15.jpeg)

Fig. 9 Predicted total bearing torque and the torques due to raceways and rib-roller end contacts versus test results of total bearing torque for a small bearing. (LM12700 bearing: cup work point dia. 41.5 mm, cup raceway angle 11°32′, roller length 10.8 mm, 17 rollers, SAE75 oil, oil operating temp. 80°C)

Predicted Total Bearing Torque vs. Test Results

![](_page_7_Figure_17.jpeg)

Fig. 10 Predicted torques versus test results for a high speed bearing. (L521900 bearing: cup work point dia. 138 mm, cup raceway angle 14°40′, roller length 15.2 mm, 39 rollers, Mil-L-23699 oil, oil operating temp. 94°C)

- 3 Dalmaz, G., Tessier, J. F., and Dudragne, G., "Friction Improvement in Cycloidal Motion Contact: Rib-Roller End Contact in Tapered Roller Bearings," *Proceedings of the 7th Leeds-Lyon Symp. on Tribology*, Sept., 1980, pp. 175–185.
- 4 Aihara, S., "A New Running Torque Formula for Tapered Roller Bearings Under Axial Load," ASME JOURNAL OF TRIBOLOGY, Vol. 109, July 1987, pp. 471-478.
- 5 Crook, A. W., "The Lubrication of Rollers IV. Measurements of Friction and Effective Viscosity," *Philosophical Trans. of the Royal Society of London*, Series A, Vol. 255, Jan. 1963, pp. 281-312.
- 6 Goksem, P. G. and Hargreaves, R. A., "The Effect of Viscous Shear Heating on Both Film Thickness and Rolling Traction in an EHL Line Contact," ASME JOURNAL OF LUBRICATION TECHNOLOGY, Vol. 100, July 1978, pp. 346-358.
  - 7 Zhu, D., and Cheng, H. S., Private Communication, 1988.
- 8 Hamrock, B. J., and Dowson, D., Ball Bearing Lubrication: The Elastohydrodynamics of Elliptical Contacts," Wiley, 1981.
- 9 Zhou, R. S., Cheng, H. S., and Mura, T., "Micropitting in Rolling and Sliding Contact Under Mixing Lubrication," ASME JOURNAL OF TRIBOLOGY, Vol. 111, Oct., 1989, pp. 605-613.
- 10 Lee, S. C., and Cheng, H. S., "On the Load-Average Gap Relation and Asperity Temperatures between Two Rough Sliding Contacts with Longitudinal Roughness," Presented in 42th ASLE Annual Meeting, May 1987.
- . 11 Hirst, W., and Hollander, A. E., "Surface Finish and Damage in Sliding," *Proc. Royal Soc. of London*, Series A. 337, 1974, pp. 379-394.
- 12 Chittenden, R. J., Dowson, D., Dunn, J. F., and Taylor, C. M., "A Theoretical Analysis of the Isothermal Elastohydrodynamic Lubrication of Concentrated Contacts," (Part I and Part II) *Proceedings of the Royal Soc. of London*, Series A, Vol. 397, 1985, pp. 255-294.
- 13 Bair, S., and Winer, W. O., "A Rheological Model for Elastohydrodynamic Contacts Based on Primary Laboratory Data," ASME JOURNAL OF LUBRICATION TECHNOLOGY, Vol. 101, July 1979, pp. 258–265.
- 14 Dyson, A., "Frictional Traction and Lubrication Rheology in Elastohydrodynamic Lubrication," *Philosophical Trans. of the Royal Society of London*, Series A, Vol. 266, 1970, p. 1-33.