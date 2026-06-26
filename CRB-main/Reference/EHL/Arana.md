![](_page_0_Picture_1.jpeg)

![](_page_0_Picture_2.jpeg)

(\$)SAGE

# Partial EHL friction coefficient model to predict power losses in cylindrical gears

Aitor Arana , Jon Larrañaga and Ibai Ulacia

Proc IMechE Part J:
J Engineering Tribology
2019, Vol. 233(2) 303–316
© IMechE 2018
Article reuse guidelines:
sagepub.com/journals-permissions
DOI: 10.1177/1350650118778655
journals.sagepub.com/home/pij

#### **Abstract**

The accurate prediction of friction coefficient and power losses in the gear mesh is a key subject to several gear-related fields of study. However, there is still not a unified method for large ranges of operating conditions, different gear geometries and lubricant types. The current paper meets this demand by modelling partial EHL friction with an asperity-fluid load sharing approach where fluid traction is calculated with the Ree-Eyring equation and the reference stress behaviour is predicted from piezoviscosity coefficient. It will be shown that only an accurate description of the lubricant's viscosity behaviour is required to compute friction in gears. Finally, mesh power losses are predicted considering thermal effects and numerical predictions are compared to experimental results showing good agreement.

### **Keywords**

Cylindrical gears, friction coefficient, power losses, partial EHL

Date received: 15 February 2018; accepted: 20 April 2018

### Introduction

The prediction of friction coefficient and power losses in geared transmissions is present in several fields of study, from failure mode analysis to efficiency prediction, thermal rating or dynamic behaviour. Although several models can be found in the scientific literature to predict friction and power losses with accuracy, there is still not a general method available that copes with the large range of operating conditions and gear geometries found in gear transmissions. Moreover, lubricants often need to be characterised in preliminary measurements through twin-disc, ball-on-disc or even FZG-tests and, as a result, power loss predictions and friction coefficient models are limited to the specifications of the tribometer and the selected oil.

Literature review stands out three main approaches of increasing complexity to predict friction coefficient and power losses in gears: (i) experimental methods based on power loss factors,  $H_{\nu}$ , (ii) empirical equations from twin disc measurements and (iii) physics-based models. The first group of models<sup>2,3</sup> are based on Ohlendorf's approach<sup>4</sup> who proposed the equation  $P_{VZP} = P_A \cdot \bar{\mu} \cdot H_{\nu}$  for standard spur gears depending exclusively on gear geometry,  $H_{\nu}$ , the mean coefficient of friction,  $\bar{\mu}$ , and the input power,  $P_A$ . Experimentally measured power loss from FZG tests is used to calculate the mean coefficient of friction for different oils, gear geometries, surface roughness and operating conditions. Then, the friction coefficient model is developed from regression analysis.

However, the power loss factor,  $H_{\nu}$ , was not originally developed to account for the influence of helix angle, high contact ratio or tooth modifications. In addition, Wimmer<sup>5</sup> found that these models largely simplify load sharing between teeth and substantial deviations were found when mesh stiffness was considered. Although several corrections are proposed for these variables, and similar power loss factor models have been developed to prevent these shortcomings,<sup>6</sup> the accuracy of the majority of these friction coefficient models is still limited to simple gear geometries and specific operating conditions. Furthermore, power loss predictions following Ohlendorf's approach neglect the variation of lubrication regime in the path of contact which may lead to significant errors when gears operate at high speeds and torques.

The second group of models<sup>7,8</sup> is based on regression analysis of tribometer measurements. Such tests allow to easily control the curvature radius, the contact width, disc/ball materials, lubricant type, oil jet temperature and slide to roll ratio. Therefore, friction behaviour is directly characterised under real contact

Faculty of Engineering, Mechanical and Industrial Production Department, Mondragon Unibertsitatea, Mondragon, Spain

### Corresponding author:

Aitor Arana Ostolaza, Faculty of Engineering, Mechanical and Industrial Production Department, Mondragon Unibertsitatea, Loramendi 4, Arrasate-Mondragon 20500, Spain.

Email: aarana@mondragon.edu

pressures, velocities and temperatures. When these models are used in the prediction of power losses in gears, the contact path must be discretised to compute instantaneous kinematic and load parameters which serve as inputs for the empirical friction models. The method accounts for the variation of friction coefficient along the line of action but, unfortunately, all empirical models are developed for specific lubricants and therefore they cannot be directly used to predict friction with a different oil without loss of accuracy.

Finally, the third group of models<sup>9,10</sup> analytically or numerically predict partial or full EHL friction coefficient based on the discretisation of the Hertzian contact region for each position in the line of action. For a given normal load and mean rolling speed in the contact region, lubricant film pressure and thickness distributions are computed by solving the transient Reynold's equation together with the energy, elasticity and force balance equations across the contact and taking into account the dependence of lubricant viscosity on temperature and pressure. Then, sliding friction coefficient is tipically computed from Eyring or limiting shear stress models and finally instantaneous power losses are predicted. Although the results of these models are accurate over a wide range of operating conditions, lubricants and lubrication regimes, the required computational time to compute the full length of the path of contact is high, even when efficient algorithms such as the multigrid method<sup>9</sup> are used. To solve this problem Xu et al.,<sup>11</sup> and later Li and Kahraman, 12 developed simple full and partial EHL friction coefficient models from the linear regression of thousands of numerical EHL simulations covering typical gear contact parameter ranges (i.e. pressure, temperature, slide-to-roll ratio, roughness, etc.). However, once again, both friction coefficient models were developed for specific lubricants and therefore they cannot be extended to other oils.

In this work, it will be shown that fast and sufficiently accurate predictions of power loss and friction coefficient in gears can be made for any lubricant without previous characterisation in a tribometer. Instantaneous friction coefficient is computed analytically following a simple asperity load sharing approach, where fluid traction is computed from the Eyring shear stress model incorporating the limiting shear stress behaviour both of which are evaluated at the mean temperature, pressure and sliding conditions in the contact. It is also shown that the characteristic shear stress values,  $\tau_E$  and  $\tau_L$ , can be related by a single base oil-dependent coefficient and that only an accurate viscosity-pressure-temperature description of the lubricant behaviour is required to compute traction. Finally, analytically predicted friction coefficients and power losses are compared to numerical simulations and experimental results from Ziegltrum et al. 10 showing good agreement.

### Partial EHL friction coefficient model

# Theoretical background

Tallian<sup>13</sup> suggested that the degree of asperity interaction is governed by the specific film thickness ratio  $\lambda = h_c/\sigma$  where  $h_c$  is the central EHD film thickness and  $\sigma$  is the composite root mean square roughness of the surfaces. According to Tallian, partial EHD exists when  $0.5 \div 1 < \lambda < 3 \div 4$ . In this regime, normal and tangential loads applied to the contacting bodies are shared between the oil film and the surface asperities according to the following equation

$$F = F_f + F_s \tag{1}$$

where F is the total load,  $F_f$  is the portion of load carried by the elastohydrodynamic film and  $F_s$  is the load carried by surface asperities. If equation (1) is stated in terms of traction

$$\bar{\tau} \cdot A_0 = \tau_s \cdot A_s + \tau_f \cdot A_f \tag{2}$$

$$\bar{\tau} = \tau_s \cdot \frac{A_s}{A_0} + \tau_f \cdot \left(1 - \frac{A_s}{A_0}\right) \tag{3}$$

$$\bar{\mu} = \frac{1}{\bar{p}} \cdot \left[ \tau_s \cdot \frac{A_s}{A_0} + \tau_f \cdot \left( 1 - \frac{A_s}{A_0} \right) \right] \tag{4}$$

where  $\bar{\mu}$  represents the mean coefficient of friction between the mating surfaces,  $\tau_s$  and  $\tau_f$  are the solid and fluid tractions, respectively, and  $A_s/A_0$  is the ratio of the real contact area (i.e. the summation of the individual contacting spots of radius  $r_i$  in Figure 1) over the apparent (i.e. Hertzian) contact area. Thus, the ratio  $A_s/A_0$  is defined in the [0,1] range where 0 represents total separation of the surfaces and 1 is used for full contact.

Rewriting  $\xi = A_s/A_0$  and introducing solid and fluid friction coefficients,  $\mu_s$  and  $\mu_f$ , equation (4) results in the following friction coefficient for partial EHL regimes, which is the basis for most empirical equations available in gear literature.<sup>14</sup>

$$\mu = \xi \cdot \mu_s + (1 - \xi) \cdot \mu_f \tag{5}$$

It is often assumed that the boundary friction coefficient,  $\mu_s$ , is constant and independent of the

![](_page_1_Figure_17.jpeg)

**Figure 1.** Representation of asperities in contact in partial EHL.

operating conditions.<sup>12</sup> This assumption is strongly supported by several studies: Robbe-Valloire<sup>15</sup> proposed the value 0.08 for a pin-on-disc type contact, Faraon and Shipper<sup>16</sup> reported values close to 0.13 for starved line contacts and recently Masjedi and Khonsary<sup>17</sup> have measured asperity friction coefficients ranging from 0.12 to 0.135 in roller contacts. Although solid friction coefficient is not very sensitive to sliding speed, it is slightly dependent on load, temperature and surface roughness.<sup>17</sup> These influencing factors have been included in some partial EHL friction coefficient models for gears<sup>7</sup> but very little variation should be expected with respect to a constant friction coefficient model as concluded by Diab.<sup>18</sup>

Contrary to boundary friction, the model for the fluid friction coefficient,  $\mu_f$ , has a strong influence on mixed lubrication. This is due to the extreme shear rates, temperature and pressure conditions found in gear type contacts where the lubricant does not behave as a Newtonian fluid. Thus, in order to calculate friction in a partially lubricated contact, the accurate description of the lubricant's non-Newtonian rheology is very important.

# Non-Newtonian rheological model

Johnson and Tevaarwerk<sup>19</sup> described the visco-elastic behaviour of the lubricant as the sum of an elastic term,  $\dot{\gamma}_e$ , and a viscous term,  $\dot{\gamma}_v$ , in equation (6)

$$\dot{\gamma} = \dot{\gamma}_e + \dot{\gamma}_v = \frac{1}{G} \cdot \frac{d\tau}{dt} + \frac{\tau_E}{\eta} \cdot \sinh\left(\frac{\tau}{\tau_E}\right) \tag{6}$$

where G is the elastic shear modulus,  $\eta$  is the low shear rate dynamic viscosity of the lubricant,  $\tau$  is the shear stress and  $\tau_E$  is the reference stress or Eyring stress which is the threshold value above which the fluid starts to behave in a non-linear manner.

In addition, at the high pressures present in gear contacts, the lubricant may show a plastic behaviour with a limiting shear stress,  $\tau_L$ , which is proportional to pressure, p, and independent of the shear rate,  $\dot{\gamma}$ . Incorporating this term in the previous model and solving for shear stress, one obtains

$$\tau = \min \left[ \tau_E \cdot \sinh^{-1} \left( \frac{\eta}{\tau_E} \cdot \left( \dot{\gamma} - \frac{1}{G} \cdot \frac{d\tau}{dt} \right) \right), \ \tau_L \right]$$
 (7)

The average shear stress,  $\bar{\tau}$ , is computed integrating the local shear stress,  $\tau$ , over the contact area and finally fluid friction coefficient results from  $\mu_f = \bar{\tau}/\bar{p}$ , where  $\bar{p}$  is the mean contact pressure. For the sake of simplicity, in this study, film pressure is assumed to follow the Hertz solution and the local shear stress is computed from mean conditions in the contact  $(\bar{\Theta}, \bar{p})$ .

According to equation (7), four fluid properties  $(\tau_E, \tau_L, \eta, G)$  are required to fully define lubricant behaviour in a concentrated contact, all of which depend on temperature and pressure. However, in

the case of gears, it is possible to reduce the number of parameters needed to compute traction when subject to the following two assumptions:

- 1. The elastic term,  $\dot{\gamma}_e$ , in equation (6) can be neglected in most gear transmissions.
- 2. It is possible to predict the Eyring and limiting shear stresses from a single base oil-dependent coefficient.

Writing  $d\tau/dt = (V_e/l) \cdot d\tau/d(x/l)$  in equation (7), the Deborah number  $D_e = (\eta \cdot V_e)/(G \cdot l)$  is introduced, where  $V_e$  is the entrainment velocity and lthe contact width  $(l = 2 \cdot b_H)$ . If  $D_e \ll 1$  elastic effects can be neglected and therefore the characterisation of the elastic shear modulus is not necessary. This is the case in most gear transmissions but it is especially suited to automotive and aeronautical applications where lubricant viscosities are low and they usually operate at high input torques and speeds involving high film temperatures. At the contact temperature and pressure of these applications, the elastic shear modulus is of the order 10<sup>9</sup> Pa. Considering that the entraining velocity exceeds 1 m/s and the contact width is around 10<sup>-4</sup> m, the low shear rate dynamic viscosity must exceed 10<sup>5</sup> Pa · s for the elastic effects to become significant. In order to validate this assumption, the authors have analysed more than  $1 \cdot 10^6$  test cases with a typical aeronautical oil complying MIL-L-23699 requirements.<sup>20</sup> The test matrix covers different geometries (modules, pressure angles, aspect ratios, etc.) with film temperatures above 60 °C, tangential speeds in the range [5, 50] m/s and specific line loads from 100 to 2000 N/mm. The results are presented in Figure 2 where the variation of the Deborah number along the standardised line of action is shown. Only results in the full EHL regime have been retained for the analysis and a contour plot has been added to stress the influence of the single and double teeth contact regions.

![](_page_2_Figure_15.jpeg)

**Figure 2.** Density plot of the Deborah number along the path of contact for MIL-L-23699 oil<sup>20</sup> subject to aeronautical/automotive operating conditions.

As it can be noticed in Figure 2, the Deborah number remains below one in almost all the cases; with 97% of the results below 1.0 and 68% of them under 0.1. These results support the assumption that in the case of gear transmissions with low shear rate viscosities and subject to high torques and speeds, it is possible to neglect elastic effects. In the single tooth contact region, sudden pressure increase causes an instantaneous increase in both viscosity and elastic shear modulus, which necessarily modifies the Deborah number. However, the order of magnitude of the ratio  $\eta/G$  is not changed significantly and therefore values remain below one. Furthermore, for any given combination of geometry and operating conditions, the smallest values are always found in the double teeth contact region where most of the power losses occur due to the high sliding velocities; therefore, it is possible to use only the viscous term in equation (7) to predict power losses in gears. This assumption can be further extended to other lubricants producing similar results when the viscosity grade according to ISO is under 220.

On the other hand, one of the main drawbacks when calculating friction coefficient with the Eyring shear stress model is the fact that the value of the reference stress,  $\tau_E$ , needs to be curve-fitted from experimental measurements in tribometers, and consequently, the application of the model without preliminary characterisation of the lubricant's traction behaviour becomes difficult. However, the limiting shear stress can be characterised in high pressure rheometers and it is known to be linearly dependent on pressure  $\tau_L = \Lambda \cdot p$ , with slope  $\Lambda$ known as the limiting-stress pressure coefficient.<sup>21</sup> The latter is strongly dependent on the base oil type and varies slightly with temperature. Therefore, if a simple relation involving a material-dependent coefficient is found between  $\tau_E$  and  $\tau_L$ , it is possible to use equation (7) as a prediction model for different lubricants.

Several authors<sup>22,23</sup> report constant ratios between the reference stress and the limiting shear stress obtained experimentally, while others such as Bair and Winer<sup>24</sup> conclude analytically that  $\tau_E \approx 2 \cdot \Lambda/\alpha$ which agrees well with the experimental values from Johnson and Tevaarwerk.<sup>19</sup> Their result explained the sigmoid shape of the friction curve by the growth of the plastic region within the contact ruled by the limiting shear stress. Jacod et al.<sup>25</sup> also found an analytical relationship by applying Eyring and limiting shear stress models, to the same traction data such that predicted values are equal in a wide range of operating conditions and lubricants. Their results were validated with a different set of friction coefficient data and both authors lead to approximately the same solution at the pressure and temperature levels present in EHD contacts (see Figure 3) which suggests that the value of  $\tau_E$  can be related to the piezoviscosity coefficient in such conditions.

![](_page_3_Figure_5.jpeg)

**Figure 3.** Eyring stress predicted by literature <sup>24,25</sup> as a function of film temperature and pressure for a ISO VG 100 mineral oil with  $\Lambda=0.047$  in line contact with  $R_{\rm e}=10$  mm,  $V_{\rm e}=5$  m/s, and SRR=0.1.

This conclusion can also be addressed following a physical explanation in Eyring's theory of fluid flow. If one recalls the expression by Hirst and Moore, <sup>26</sup> shear rate can be written in the following form

$$\dot{\gamma} = A \cdot k \cdot \Theta \cdot \exp\left(-\frac{E + v_p \cdot p}{k \cdot \Theta}\right) \cdot \sinh\left(\frac{v_\tau \cdot \tau}{k \cdot \Theta}\right) \quad (8)$$

where A is a constant, k the is Boltzmann constant  $(1.38 \cdot 10^{-23} \text{ J/K})$ , E is the thermal activation energy for flow and  $v_p$  and  $v_\tau$  are the activation volumes for pressure and shear, respectively, both of which are fluid-dependent. Rearranging terms

$$\dot{\gamma} = \frac{\frac{k \cdot \Theta}{\nu_{\tau}}}{\frac{1}{A \cdot \nu_{\tau}} \cdot \exp(\frac{E}{k \cdot \Theta}) \cdot \exp(\frac{\nu_{p} \cdot p}{k \cdot \Theta})} \cdot \sinh(\frac{\nu_{\tau} \cdot \tau}{k \cdot \Theta})$$
(9)

By direct comparison with the viscous term in equation (6)

$$\tau_E = \frac{k \cdot \Theta}{\nu_{\tau}} \tag{10}$$

$$\eta = \frac{1}{A \cdot \nu_{\tau}} \cdot \exp\left(\frac{E}{k \cdot \Theta}\right) \cdot \exp\left(\frac{\nu_{p} \cdot p}{k \cdot \Theta}\right) \tag{11}$$

The first exponential term in equation (11) is the Andrade equation representing the temperature dependence of viscosity and the second exponential term is the well-known Barus equation where the piezoviscosity coefficient is  $\alpha = v_p/(k \cdot \Theta)$ . If the latter is related to the reference stress in equation (10)

$$\tau_E = \frac{v_p/v_\tau}{\alpha} \tag{12}$$

Therefore, the reference stress is inversely proportional to the pressure–viscosity coefficient by some constant related to material properties which is consistent with Bair and Winer's results.<sup>24</sup> For instance, for LVI260 and 5P4E oils presented in Table 2 of this

reference,  $\Lambda$  takes the values 0.047 and 0.088, respectively, and according to equation (12),  $v_p/v_\tau=2\cdot\Lambda=0.094$  and 0.176 which is in perfect agreement with the values estimated by Hirst and Moore for the ratio of the activation volumes in several fluids.<sup>26</sup>

Finally assuming that the Couette flow dominates in the contact region, the strain rate in equation (7) is equal to the velocity gradient,  $\dot{\gamma} = V_s/(h_c \cdot \Phi_T)$ , where  $V_s$  is the sliding speed of the surfaces and  $h_c$  is the central film thickness between them corrected for thermal effects with factor  $\Phi_T$  (see equation (24)). If the relation  $\tau_E = 2 \cdot \Lambda/\alpha$  is assumed, fluid friction coefficient results in

$$\mu_f = \min \left[ \frac{2 \cdot \Lambda}{\alpha \cdot \bar{p}} \cdot \sinh^{-1} \left( \frac{\eta \cdot \alpha \cdot V_s}{2 \cdot \Lambda \cdot \Phi_T \cdot h_c} \right), \Lambda \right] \quad (13)$$

where viscosity,  $\eta$ , must be evaluated at the mean contact temperature and pressure while the local piezoviscosity coefficient,  $\alpha$ , is calculated at Hertz pressure according to Bair and Winer.<sup>24</sup>

# Load sharing function

In order to extend the range of applicability of the fluid friction coefficient model to the partial EHL regime (without increasing computational effort), a simple asperity load sharing model is suggested. Table 1 summarises some of the most relevant equations in gear literature. As it can be seen, several functions have been proposed with diverse definitions of the specific film thickness and different domains of application.

Doleschel<sup>7</sup> considered that asperity interactions begin at specific film thickness values below 2 and a second-order polynomial function was used to represent the amount of load carried by asperities. Diab et al.<sup>18</sup> computed the complementary error function to predict the load share parameter and assumed that partial EHL begins at  $\lambda \approx 3$ . In both cases, specific film thickness was computed using the ratio of the central film thickness (corrected to account for thermal effects) to the combined average or mean square roughness of the surfaces. However, in a recent work Matsumoto and Morikawa<sup>14</sup> considered that load share is governed by the ratio of the minimum film thickness to the sum of the maximum height of

Table 1. Asperity load sharing functions in gear literature.

|               |                                                  | -                                                                                  |                                                                |
|---------------|--------------------------------------------------|------------------------------------------------------------------------------------|----------------------------------------------------------------|
|               | Doleschel <sup>7</sup>                           | Diab <sup>18</sup>                                                                 | Matsumoto 14                                                   |
| ξ             | $\left(1-\frac{\lambda}{2}\right)^2$             | $\frac{1}{4} \cdot \left( I - erf \left( \frac{\lambda}{\sqrt{2}} \right) \right)$ | $\frac{1}{2} \cdot \log_{10} \left( \frac{1}{\lambda} \right)$ |
| λ             | $\frac{\phi_T \cdot h_c}{\frac{Ra_1 + Ra_2}{2}}$ | $\frac{\phi_{T}\cdot h_{c}}{\sqrt{Rq_1^2+Rq_2^2}}$                                 | $\frac{h_m}{Rz_1 + Rz_2}$                                      |
| $\mathcal{D}$ | [0, 2]                                           | [0, 3]                                                                             | [0.01, 1]                                                      |

asperities which implies that mixed lubrication regime begins long before expected. These load sharing functions have been tested indirectly in twin disc machines through the measurement of the mean friction coefficient but none of them has been compared directly to experimental asperity load sharing results, hence, as far as we know, the accuracy of each function still remains unknown.

Recently, Clarke et al.<sup>27</sup> have used the electrical contact resistance (ECR) technique in a twin disc machine to study the amount of load carried by asperities in rolling-sliding contacts with axially ground and lubricated surfaces subject to speed and contact pressure levels typical from gear applications. Their experimental results confirm that partial EHL begins at  $\lambda$  values below 2 with intermittent asperity interactions that increase steeply at values below 1. Such behaviour is only ensured by the complementary error function by Diab et al.<sup>18</sup> but the latter must be modified to cope with the experimentally measured limits  $\xi(\lambda \approx 2) = 0$  and  $\xi(\lambda = 0) = 1$  which results in

$$\xi = 1 - \operatorname{erf}(\lambda) \tag{14}$$

# Power loss prediction model

The friction coefficient model presented in the previous section is included in the power loss prediction flowchart shown in Figure 4. The model accounts for thermal effects and therefore accurately predicts friction when gears are subject to high speeds or torques that produce an increase in film temperatures due to high contact loads or sliding speeds.

The process starts with the discretisation of the computational domain and the contact analysis of the meshing gears (frequently known as LTCA). For a given gear pair, the contact path is separated into single and multiple teeth contact regions. The surface velocities,  $u_1$  and  $u_2$ , and its derivatives (sliding,  $V_s$ ) rolling,  $V_r$ , and entrainment speeds,  $V_e$ ) are determined for each point in the line of action and instantaneous load, W, is calculated following the load distribution model by Steward.<sup>28</sup> Then, maximum and mean contact pressure,  $p_H$  and  $\bar{p}$ , respectively, as well as the half contact width,  $b_H$ , can be calculated following Hertz theory; where local contact pressure increases due to changes in curvature in the vicinity of profile modifications or tooth tip are neglected. For other geometrical parameters or kinematic calculations, the reader is referred to classical gear literature.<sup>29</sup>

Once kinematic and load parameters are known, mean contact inlet temperature (at which film thickness must be evaluated) can be calculated following the procedure suggested by Olver<sup>30</sup> where surface temperature of pinion and gear teeth,  $\Theta_s$ , is calculated from equations (15) and (16). Skin temperature depends on the heat flux,  $\dot{q} = \mu \cdot \bar{p} \cdot V_s$ , the heat partitioning coefficient,  $\epsilon$ , and the steady-state and

![](_page_5_Figure_2.jpeg)

Figure 4. Flowchart for the power loss prediction methodology.

transient thermal resistances, M and B, respectively, which can be calculated either analytically or numerically. Finally,  $\Theta_b$ , is the reference or bulk temperature of pinion or gear above which the remaining temperatures are computed (e.g. inlet, flash or film temperatures). In the absence of experimental values, empirical equations from literature such as that from ISO/TR 15144-1<sup>31</sup> can be used to estimate both bulk temperatures and friction coefficients.

$$\Theta_{s1} = \epsilon \cdot \dot{q} \cdot M_1 + \Theta_{b1} \tag{15}$$

$$\Theta_{s2} = (1 - \epsilon) \cdot \dot{q} \cdot M_2 + \Theta_{b2} \tag{16}$$

$$\epsilon = \frac{1.06 \cdot B_2 + M_2}{1.06 \cdot (B_1 + B_2) + M_1 + M_2} \tag{17}$$

In the next step, surface temperatures are used to predict the mean contact inlet temperature,  $\bar{\Theta}_{in}$ , at which the film thickness must be evaluated. Mean film temperature,  $\bar{\Theta}_f$ , necessary to compute traction in a future step, is assumed to be equal to the inlet temperature as a first approximation.

$$\bar{\Theta}_{in} = \frac{u_1 \cdot \Theta_{s1} + u_2 \cdot \Theta_{s2}}{u_1 + u_2} \tag{18}$$

Considering that in the case of gears, the elastic deformation of the non-conformal contact region is significant relative to the film thickness and the contact pressure increases oil viscosity significantly, film thickness operates in the piezoviscous elastic regime. Therefore, an appropriate equation should be used to predict the latter as it directly affects friction coefficient through the fluid portion and the load sharing function

in equations (13) and (14), respectively. For this purpose, Hamrock and Dowson's approach<sup>32</sup> is used (see equation (19)) with  $k\rightarrow\infty$  in the case of line contacts.

$$\frac{h_c}{R_e} = 2.69 \cdot \frac{G^{0.53} \cdot U^{0.67}}{O^{0.067}} \cdot \left(1 - 0.61 \cdot e^{-0.73 \cdot k}\right) \tag{19}$$

$$G = \alpha^* \cdot E_r \tag{20}$$

$$U = \frac{\eta_0 \cdot Ve}{E_r \cdot R_e} \tag{21}$$

$$Q = \frac{W}{E_r \cdot R_e^2} \tag{22}$$

where, according to Hertz theory,  $E_r$  is the reduced modulus of elasticity and  $R_e$  is the effective radius of curvature in the entraining direction. Attention is to be paid to the value of  $\alpha^*$  which is the reciprocal asymptotic isoviscous pressure–viscosity coefficient (see equation (23)). This parameter is considered an effective value of  $\alpha$  over the pressure range encountered in the contact and it is proved to better characterise the oil film formation in EHL contacts rather than  $\alpha$ .<sup>33</sup>

$$\alpha^* = \left(\int_0^{pH} \frac{\eta_0}{\eta(p)} \mathrm{d}p\right)^{-1} \tag{23}$$

At this point, the film thickness reduction due to inlet shear heating effect,  $\Phi_T$ , is also determined (equation (24)) and finally specific film thickness is computed from equation (25).

$$\Phi_T = \frac{1 - 13.2 \cdot \frac{p_H}{E_r} \cdot L^{0.42}}{1 + 0.213 \cdot (1 + 2.23 \cdot SRR^{0.83}) \cdot L^{0.64}}$$
(24)

$$\lambda = \frac{\Phi_T \cdot h_c}{\sqrt{Rq_1^2 + Rq_2^2}} \tag{25}$$

where SRR is the slide-to-roll ratio and  $L = \eta_0 \cdot \beta \cdot V_e^2/k$  is the thermal loading parameter, with  $\beta$  and k the temperature-viscosity coefficient and fluid thermal conductivity, respectively.

Once the lubrication regime is known, friction coefficient is computed from equations (5), (13) and (14), where lubricant dynamic viscosity,  $\eta$ , is evaluated at the assumed mean film temperature,  $\bar{\Theta}_f$ , and mean contact pressure,  $\bar{p}$ , while the local pressure–viscosity coefficient,  $\alpha$ , required to compute the Eyring stress is calculated at Hertz pressure,  $p_H$ . Then, instantaneous traction coefficient,  $\mu$ , and sliding speed,  $V_s$ , are used to predict the average flash temperature rise,  $\Delta \bar{\Theta}_{Fl}$ , and internal heating of the oil film,  $\Delta \bar{\Theta}_v$ , which increases the mean film temperature and, therefore, affects friction coefficient.

$$\bar{\Theta}_f = \bar{\Theta}_{in} + \Delta \bar{\Theta}_{Fl} + \bar{\Delta\Theta}_{v} \tag{26}$$

$$\Delta\bar{\Theta}_{Fl} = \frac{1.06 \cdot \epsilon \cdot \dot{q}}{2 \cdot b_H \cdot b \cdot k_s} \cdot \left(\frac{\chi_1 \cdot b_H}{u_1}\right)^{1/2} \tag{27}$$

$$\bar{\Delta\Theta}_{v} = \frac{h_{c} \cdot \dot{q}}{16 \cdot b_{H} \cdot b \cdot k_{o}} \tag{28}$$

If the new film temperature is equal to the previous estimation, instantaneous mesh power loss  $P_{VZP,i} = W_i \cdot \mu_i \cdot V_{s,i}$  is calculated. If this condition is not fulfilled, a new film temperature is established and the procedure is repeated until the difference in estimated and calculated film temperatures is less than 1 °C. The procedure must be repeated for each point in the line of action until the full path of contact is covered. Finally, instantaneous power loss values are integrated over the path of contact to predict the mean power loss, which is used to update the initial estimations of bulk and skin temperatures.

$$\bar{P}_{VZP} = \frac{1}{p_{et}} \cdot \int_{A}^{E} P_{VZP}(x) \mathrm{d}x \tag{29}$$

# Validation of the friction coefficient model

The fluid friction coefficient model presented in equation (13) has been compared to the experimental results from Mann. Three different base oils ISO VG 100 were tested in a twin-disc machine: (i) a naphtenic mineral oil (N100), (ii) a paraffinic mineral oil (M100) and a polyalphaolefin (PAO100). The selected lubricants are known to have very different traction behaviour with the highest friction coefficients for the naphtenic base to the lowest for the polyalphaolefin. None of the selected lubricants included additives and the discs were smoothly polished up to  $R_a \approx 0.06 \, \mu m$  to avoid asperity interactions so that pure fluid traction could be tested. Table 2 outlines basic rheological parameters where the limiting-stress pressure coefficients have been obtained from experimental results in Höglund.

In order to model lubricant's dynamic viscosity as a function of temperature and pressure, the so-called "Modulus equation" is used (see equation (30)). The first exponential term describes the temperature dependence of viscosity following the Vogel, Tammann and Fulcher equation and the second exponential term is similar to that of the empirical equation by Paluch which captures the super-Arrhenius response at high pressures. Gold et al. Innearised the temperature dependence of parameters A and B and obtained an equation with seven unknown parameters:  $a_1$ ,  $a_2$ ,  $b_1$ ,  $b_2$ , c, d and d which can be obtained numerically from regression of experimental measurements at high pressures.

$$\eta(\Theta, p) = K \cdot \exp\left[\frac{C}{D + \Theta}\right] \cdot \exp\left[\frac{p}{A + B \cdot p}\right]$$
(30)

$$A(\Theta) = a_1 + a_2 \cdot \Theta \tag{31}$$

$$B(\Theta) = b_1 + b_2 \cdot \Theta \tag{32}$$

The viscosity-pressure-temperature (VPT) behaviour of the gear oils in Table 2 was fitted from Mann's high-pressure falling body viscosimeter measurements.<sup>34</sup> Table 3 summarises the regression coefficients along with the coefficient of determination  $(R^2)$ which shows general good agreement. Only in the case of the M100 oil, there is a slight discrepancy between the experimental measurements and the numerical predictions caused by the "faster-than-exponential" response of the fluid at pressures of 230 MPa, 350 MPa and 510 MPa at 25 °C, 40 °C and 60 °C, respectively (see Figure 5). If these outliers are ignored in the regression of the Modulus equation, the coefficient of determination increases up to 99.26%. Moreover, it is known that the inflection pressure increases with increasing temperature<sup>36</sup> and, in this case, it is not even observed at the highest temperature. Finally, considering the contact conditions in Mann's twin disc tests (described later in this paper), it is not probable that the pressure inflection is present in the analysed cases.

The main advantage of the modulus equation is that the pressure dependence of the viscosity-pressure coefficient,  $\alpha$ , is additionally taken into account. A simple analogy of equation (30) with the classical Barus equation yields

$$\alpha(\Theta, p) = [a_1 + a_2 \cdot \Theta + (b_1 + b_2 \cdot \Theta) \cdot p]^{-1}$$
 (33)

If the latter is used to predict the behaviour of the reference stress,  $\tau_E$ , it can be seen in Figure 6 that the numerical predictions are in good agreement with the experimentally measured values from Michaelis<sup>38</sup>

 Table 2. Selected lubricant properties from literature.

|        | $v_k _{40}$                            | ν <sub>k</sub>   <sub>100</sub>        | ρ  <sub>15.6</sub>                          | Λ                    |  |
|--------|----------------------------------------|----------------------------------------|---------------------------------------------|----------------------|--|
|        | $10^{-6} \left[ \frac{m^2}{s} \right]$ | $10^{-6} \left[ \frac{m^2}{s} \right]$ | $\left[\frac{\text{kg}}{\text{m}^3}\right]$ | 10 <sup>-2</sup> [-] |  |
| N100   | 97.9                                   | 8.61                                   | 900                                         | 5.3                  |  |
| M100   | 96                                     | 10.6                                   | 882                                         | 4.7                  |  |
| PAO100 | 94.1                                   | 14                                     | 840                                         | 3.5                  |  |

for the FVA3 oil (equivalent to M100 oil). These results were obtained by curve-fitting the classical Ree-Eyring non-Newtonian model to twin disc traction tests at 8 m/s rolling velocity, 10% slip, 1 GPa Hertz pressure and different oil jet temperatures.

Specific film thickness ratio has been included in Figure 6 in order to give insight into the lubrication regime in the contact. The Eyring stress must be regressed from measurements at full EHL regime; however, at  $110\,^{\circ}\mathrm{C}$   $\lambda=1.4$  which indicates that mixed EHL conditions prevail. Therefore, the regressed value of the Eyring shear stress is influenced by asperity interactions which explains the difference between the actual and the predicted value.

![](_page_7_Figure_12.jpeg)

Figure 5. VPT behaviour of M100 oil according to Table 3.

![](_page_7_Figure_14.jpeg)

**Figure 6.** Eyring stress as a function of temperature and pressure for M100 oil.

Table 3. Regression coefficients for the selected lubricants.

|        | a <sub>1</sub><br>10 <sup>7</sup> [Pa] | a <sub>2</sub><br>10 <sup>5</sup> [Pa/K] | b <sub>1</sub><br>10 <sup>-3</sup> [-] | $b_2$ $10^{-5}$ $\begin{bmatrix} \frac{1}{K} \end{bmatrix}$ |      | D<br>10 <sup>2</sup> [K] | <i>K</i><br>10 <sup>-6</sup> [Pa⋅s] | R <sup>2</sup><br>[%] |
|--------|----------------------------------------|------------------------------------------|----------------------------------------|-------------------------------------------------------------|------|--------------------------|-------------------------------------|-----------------------|
| N100   | -7.23                                  | 3.48                                     | 5.00                                   | -I.83                                                       | 0.65 | -2.06                    | 161.60                              | 99.67                 |
| M100   | -2.28                                  | 2.26                                     | -68.20                                 | 27.68                                                       | 0.89 | -1.82                    | 86.86                               | 88.73                 |
| PAO100 | -5.73                                  | 3.90                                     | -39.50                                 | 23.54                                                       | 1.74 | -1.16                    | 12.26                               | 99.59                 |

Mann<sup>34</sup> carried out several traction tests for different oils at varying rolling velocities from 2 m/s to 16 m/s and slip ratios in the range [0, 0.4] at constant Hertz pressure of 1 GPa and oil jet temperature of 60 C. Disc bulk temperatures were measured at different input speeds and slip ratios which allows predicting contact inlet temperatures accurately following the procedure described in Figure 4. Figure 7 shows the numerical predictions for the M100 mineral oil case. The model captures the typical behaviour of twin disc tests with a linear region showing Newtonian behaviour at the lowest slip ratios,

![](_page_8_Figure_2.jpeg)

Figure 7. Traction curves for M100 mineral oil.

followed by a non-linear region with a maximum and a thermal region at the highest slip ratios where the heat generated within the contact leads to a reduction of the friction coefficient.

As it can be concluded from Figure 7, the proposed model correctly captures the friction behaviour at the highest rolling speeds and slip ratios with errors below 10%. However, at low entrainment velocities there is a slight discrepancy caused by the parameter <sup>T</sup> overestimating the inlet shear heating effect which is also reflected in the original paper by Hili et al.<sup>39</sup>

Finally, Figure 8 shows the overall performance of the model for each lubricant type. The lowest friction coefficients are predicted for the polyalphaolefin oil and the highest values correspond to the naphtenic mineral oil which is in agreement with experimental evidence in scientific literature. The predicted values are within 5 - 10<sup>3</sup> error which is considerably lower than standard empirical models available in gear literature.<sup>5</sup>

# Case study: Spur gears

The numerical procedure described earlier is compared to the recent results by Ziegltrum et al.<sup>10</sup> who analysed the influence of the non-Newtonian rheological behaviour of different oil types in the friction coefficient and the power loss in gears using finite element-based TEHL simulations. Two comparisons

![](_page_8_Figure_9.jpeg)

Figure 8. General overview of the accuracy of the proposed model with respect to classical models in gear literature.

| /                      |                                                                                                                                                 |
|------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------|
| Symbol                 | FZG C <sub>mod</sub>                                                                                                                            |
| z <sub>1,2</sub> (-)   | 16, 24                                                                                                                                          |
| m (mm)                 | 4.5                                                                                                                                             |
| b (mm)                 | 14                                                                                                                                              |
| $\alpha_n$ (°)         | 20                                                                                                                                              |
| x <sub>1,2</sub> (–)   | 0.182, 0.172                                                                                                                                    |
| Ca <sub>1,2</sub> (μm) | 35                                                                                                                                              |
| H <sub>V L</sub> (–)   | 0.1680                                                                                                                                          |
| (-)                    | 16MnCr5                                                                                                                                         |
| $R_a$ ( $\mu$ m)       | 0.2                                                                                                                                             |
|                        | z <sub>1,2</sub> (-)<br>m (mm)<br>b (mm)<br>α <sub>n</sub> (°)<br>x <sub>1,2</sub> (-)<br>Ca <sub>1,2</sub> (μm)<br>H <sub>V L</sub> (-)<br>(-) |

Table 4. Spur gear geometry used in Ziegltrum et al. 10

are made, on the one hand the proposed analytical fluid friction coefficient model is compared to the numerical results, and on the other hand, the suggested power loss computation model and the general partial EHL friction coefficient model from equation (5) are compared to the experimental results.

A single gear set is considered (see Table 4) which is characterised by being a modified version of the standard FZG gear type C-PT with a tip relief of 35  $\mu$ m. The operating conditions comprise a constant torque of 183.4 Nm at load stage 7 (producing a maximum contact pressure of 1400 MPa near the lowest point of single tooth contact) and different pitch line velocities ranging from 0.5 to 20 m/s. The latter allows to vary the lubrication regime in the contact, from boundary lubrication at the lowest speed to almost full EHL in every point of the line of action at the highest pitch line velocity. These conditions will allow to test the validity of the proposed partial EHL model.

Three different lubricants are tested in Ziegltrum et al.<sup>10</sup>: (i) mineral oil (FVA3), (ii) polyalphaolefin and (iii) polyglycol; all of which are ISO VG 100 similarly to the lubricants tested in Table 2. Only the mineral oil is equal to the one used in the previous section and a new base oil (PG) is added to the previously tested. The limiting-stress pressure coefficient,  $\lambda$ , for the latter is set to 0.044 from Höglund.<sup>35</sup>

Finally, Ziegltrum et al. modelled the non-Newtonian behaviour of these lubricants following the Bair and Winer model (also neglecting the viscoelastic effects), where the limiting shear stress,  $\tau_L$ , was derived from twin disc machine measurement. The VPT behaviour of the lubricants followed the Roelands model, and therefore, the local piezoviscosity coefficient,  $\alpha(\Theta, p)$ , and the asymptotic isoviscous pressure-viscosity coefficient,  $\alpha^*$ , necessary to compute the Eyring shear stress and the film thickness, respectively, are calculated following van Leeuwen.<sup>33</sup>

$$\alpha(\Theta, p) = \left[\frac{\partial (\ln \eta(\Theta, p))}{\partial p}\right]_{\Theta} \tag{34}$$

![](_page_9_Figure_9.jpeg)

**Figure 9.** Predicted local friction coefficients against FEM results from literature. <sup>10</sup>

$$\alpha^*(\Theta) \approx \frac{\alpha(\Theta, p = 0)}{1 + ((1 - z)/(\alpha(\Theta, p = 0) \cdot p_R))}$$
(35)

where z and  $p_R$  are conventional Roelands parameters.

For further information on the numerical or experimental details, the reader is referred to the original article.

### Results

Figure 9 compares the analytically predicted local fluid friction coefficients with equation (13) against the numerically predicted values from the finite element-based TEHL simulations at 8.3 m/s pitch line velocity and bulk temperatures fixed at 90 °C. As it can be seen, predicted values are far from the numerical results except in the case of the mineral oil. Two reasons may explain such behaviour. First, the proposed model is based on the so-called Ree-Evring shear stress model while the simulated values have been obtained from the Bair and Winer limiting shear stress model. The behaviour of both non-Newtonian equations is similar up to the limiting shear stress value where the Ree-Eyring model departs from Bair and Winer's unless a limiting shear stress behaviour is included (as in the proposed model). None of the lubricant's has reached its  $\Lambda$  value and therefore the selected model does not explain such differences.

Second, the reference values,  $\tau_E$  and  $\tau_L$ , have been obtained with different methods. This is the most probable reason for the discrepancies found in Figure 9. The limiting shear stress model presented in Ziegltrum et al. 10 has been obtained empirically from twin disc measurements while the reference shear stress used in the current paper has been obtained analytically in Bair and Winer. 24 The latter departs from a the limiting stress pressure behaviour of the type  $\tau_L = \Lambda \cdot p$ , which only includes the

influence of pressure (at constant temperature), as verified experimentally in high pressure shear stress measurements, while Ziegltrum et al. also include the influence rolling velocity and slip ratio.

In order to give further insight into the proposed Eyring stress model, <sup>E</sup> ¼ 2 - =, the influence of the limiting stress pressure coefficient, , has been analysed in Figure 10. The latter has been varied 3 - 10<sup>3</sup> from its reference value, which is approximately the fluctuation of this coefficient at the range of operating temperatures in the contact. As it can be seen, the Eyring stress, and therefore the mean friction coefficient, is ruled primarily by the piezoviscosity coefficient, , as the influence of varying is negligible.

![](_page_10_Figure_3.jpeg)

Figure 10. Comparison of predicted mean coefficient of friction, numerical simulations and experimental results at 8.3 m/s and 183.4 Nm for Cmod gears.

In addition, Figure 10 has been completed with the experimental measurements from Hinterstoißer40 for the same gear set, operating conditions and oil types. The experimental results show that mean friction coefficient is higher in all cases, suggesting that mixed lubrication regime is prevailing along the line of action. This affirmation is supported by the fact that both the numerical and the analytical models predict film thicknesses just below 0.2 mm which is the value of the actual surface roughness in Hinterstoißer.40 Furthermore, in the case of the numerical simulations, there is a significant difference between the experimental measurements and the numerical predictions pointing out that either strong asperity interactions are occurring in the experiment or the simulated fluid friction coefficient is lower than expected.

Both the power loss prediction methodology and the partial EHL friction coefficient model are compared to the experimental results in Figure 11. In general terms, the load sharing factor proposed in equation (14) seems sufficiently accurate to predict the variation of the friction coefficient along the line of action with minimum computational effort. Power loss predictions show errors below 5% and the maximum error in the mean friction coefficient is 2.5% when the boundary friction coefficient is set to 0.065, which is the experimentally obtained upper limit of the friction coefficient at the lowest tangential speed (i.e. boundary lubrication along LOA). Finally, the shaded error-bar stresses the influence of varying 25% the solid friction coefficient, <sup>s</sup>. It is concluded that slight variations of the boundary friction coefficient result in significant deviations of the predicted power losses, especially at the highest tangential speeds where results are influenced by the increased sliding speeds.

![](_page_10_Figure_7.jpeg)

Figure 11. Comparison of predicted and measured mesh power losses and friction coefficients for Cmod gears at 183.4 Nm and variable tangential speeds. (a) Power losses (b) Friction coefficients.

### Discussion

The comparison of the analytical predictions and experimental results shows that accurate predictions of the friction coefficient and power losses in gears can be carried out for any lubricant from its VPT behaviour; at least for viscosity grades below 220, where the elastic behaviour of the lubricant can be neglected. However, even when only the viscous term is retained to compute traction, it is possible that at very high shear rates, shear thinning phenomena leads to lower film thickness than that predicted by classical Newtonian equations such as that from Hamrock and Dowson. This behaviour is found in mineral oilpolymer blends and some synthetic oils and therefore, in such situations, film thickness must be corrected again following Morales-Espejel and Wemekamp.<sup>41</sup> The subject has been extensively analysed by Bair and it is currently a subject of heated debate. 42-44

It is also found that such predictions depend on an accurate description of the local pressure—viscosity coefficient,  $\alpha$ , which can only be predicted from high pressure viscometer measurements. This type of rheometer is not commercially available and therefore few lubricants have been characterised by research centres up to the pressure levels encountered in EHL contacts. Therefore, only temperature-dependent piezoviscosity equations can be used, typically computed from the kinematic viscosity with expressions of the type  $\alpha = s \cdot v^t$  which are frequent in literature.<sup>37</sup> One could, as an approximation, use two-slope viscosity—pressure models such as that suggested by Xu et al.<sup>11</sup>

In addition, the type of equation used for the load sharing factor,  $\xi$ , is a simple relationship allowing the extension of the fluid friction coefficient model to the partial EHL regime without computational cost. Most of the models available in gear literature have no direct experimental support, as they have been tested indirectly in tribometers. However, a simple relation of this type allows a fast and sufficiently accurate prediction of the friction coefficient which, in any case, is better than the classical empirical equations found in gear literature. It would be interesting to validate such equations through direct measurements of the ratio of the real contact area to the apparent contact area,  $A_s/A_0$ , using electrical contact resistance technique.

Finally, it must be stressed that contrary to the generalised assumption that the value of the solid friction coefficient,  $\mu_s$ , does not affect friction coefficient, it does have a significant influence in the predicted values in partial EHL friction coefficient models based on the load sharing approach (see Figure 11). Recent experimental studies<sup>45,46</sup> support this affirmation where boundary friction coefficient is affected by the surface structure and running in. Other parameters such are lubricant additives or surface coatings are already known to significantly improve solid friction coefficient, but in the absence of such modifications a value of 0.07 is suggested for axially ground gears.

### **Conclusions**

A partial EHL friction coefficient model to predict power losses in cylindrical gears has been proposed in this paper. Two characteristics stand out in the proposed model.

On the one hand, the fluid friction coefficient is based on the Ree-Eyring non-Newtonian rheological model where the reference stress value of any lubricant is predicted from the accurate description of piezoviscosity coefficient,  $\alpha$  and base oil-dependent coefficients. The model also accounts for limiting shear stress behaviour and it has been generalised to predict friction coefficients with accuracy up to ISO viscosity grades of 220. The range of applicability of the friction coefficient model has been extended to cover partial EHL regime using Tallian's 13 load sharing approach, where the load sharing function,  $\xi$ , is described by the complementary error function. The influence of the boundary friction coefficient value has been discussed and a reference value of 0.07 has been proposed for ground gears.

On the other hand, an iterative thermal power loss prediction methodology has been described, which allows the prediction of the mean film temperature and contact inlet temperature necessary to compute traction and film thickness, respectively. The latter is computed from Hamrock and Dowson's equation<sup>32</sup> for the piezoviscous-elastic regime, where the reciprocal asymptotic isoviscous pressure–viscosity coefficient,  $\alpha^*$ , must be computed, again from the accurate description of the VPT behaviour of the lubricant.

Therefore, it has been shown that only high pressure viscosity behaviour of the lubricant is required to predict friction coefficient and power losses in gears with acceptable accuracy. Both models have been compared to experimental results from a twin-disc tribometer and an FZG test, respectively, with errors below 10% in both cases.

### **Declaration of Conflicting Interests**

The author(s) declared no potential conflicts of interest with respect to the research, authorship, and/or publication of this article.

### **Funding**

The author(s) received no financial support for the research, authorship, and/or publication of this article.

### **ORCID iD**

Aitor Arana http://orcid.org/0000-0001-6160-9977

### References

 Jurkschat T, Lohner T, Michaelis K, et al. Experimentelle Bestimmung des Reibungsverhaltens von Schrägverzahnungen mit Flankenmodifikationen. In: *Proceedings of the 57.GfT – Tribologie-Fachtagung*. Göttingen, Germany, September 2016.

2. Fernandes CM, Marques PM, Martins RC, et al. Gearbox power loss. Part II: Friction losses in gears. Tribol Int 2014; 88: 309–316.

- 3. Andersson M, Sosa M and Olofsson U. The effect of running-in on the efficiency of superfinished gears. Tribol Int 2016; 93: 71–77.
- 4. Ohlendorf H. Verlustleistung und Erwa¨rmung von Stirnra¨dern. PhD Thesis, Technische Universita¨t Mu¨nchen, 1958.
- 5. Wimmer AJ. Lastverluste von Stirnradverzahnungen Konstruktive Einflu¨sse, Wirkungsgradmaximierung, Tribologie. PhD Thesis, Technische Universita¨t Mu¨nchen, 2005.
- 6. Velex P and Ville F. An analytical approach to tooth friction losses in spur and helical gears-influence of profile modifications. J Mech Des 2009; 131: 101008– 1–101008–10.
- 7. Doleschel A. Wirkungsgradberechnung von Zahnradgetrieben in Abha¨ngigkeit vom Schmierstoff. PhD Thesis, Technische Universita¨t Mu¨nchen, 2003.
- 8. Xi Y, Bjo¨rling M, Shi Y, et al. Traction formula for rolling-sliding contacts in consideration of roughness under low slide to roll ratios. Tribol Int 2016; 104: 263–271.
- 9. Bobach L, Beilicke R, Bartel D, et al. Thermal elastohydrodynamic simulation of involute spur gears incorporating mixed friction. Tribol Int 2012; 48: 191–206.
- 10. Ziegltrum A, Lohner T and Stahl K. TEHL simulation on the influence of lubricants on load-dependent gear losses. Tribol Int 2016; 113: 252–261.
- 11. Xu H, Anderson N, Maddock D, et al. Prediction of mechanical efficiency of parallel-axis gear Pairs. J Mech Des 2007; 129: 58–68.
- 12. Li S and Kahraman A. A method to derive friction and rolling power loss formulae for mixed elastohydrodynamic lubrication. J Adv Mech Des Syst Manuf 2011; 5: 252–263.
- 13. Tallian T. The theory of partial elastohydrodynamic contacts. Wear 1972; 21: 49–101.
- 14. Matsumoto S and Morikawa K. The new estimation formula of coefficient of friction in rolling-sliding contact surface under mixed lubrication condition for the power loss reduction of power transmission gears. In: Proceedings of the international gear conference 2014. August 2014, Lyon, France: Woodhead Publishing, pp.1078–1088.
- 15. Robbe-Valloire F. Theoretical prediction and experimental results for mixed lubrication between parallel surfaces. In: Dowson D (ed.) Boundary and mixed lubrication – science and applications, vol. 40. Vienna, Austria: Elsevier Science, 2002, pp.129–137.
- 16. Faraon I and Schipper D. Stribeck curve for starved line contacts. J Tribol Transact ASME 2007; 129: 181–187.
- 17. Masjedi M and Khonsari M. Theoretical and experimental investigation of traction coefficient in line-contact EHL of rough surfaces. Tribol Int 2014; 70: 179–189.
- 18. Diab Y, Ville F and Velex P. Prediction of power losses due to tooth friction in gears. Tribol Transact 2006; 49: 260–270.
- 19. Johnson K and Tevaarwerk J. Shear behaviour of elastohydrodynamic oil films. Proc R Soc Lond Ser A (Math Phys Sci) 1977; 356: 215–236.

20. Sottomayor AG. Reologia de um lubrificante Na˜o-Newtoniano no interior de um contacto termoelastohidrodinaˆmico: Determinac¸ a˜o dos paraˆmetros reolo´gicos de um lubrificante. PhD Thesis, Faculdade de Engenharia da Universidade do Porto, 2002.

- 21. Bjo¨rling M, Habchi W, Bair S, et al. Towards the true prediction of EHL friction. Tribol Int 2013; 66: 19–26.
- 22. Sharif K, Morris S, Evans H, et al. Comparison of non-Newtonian EHL models in high sliding applications. In: Dalmaz G, Lubrecht A, Dowson D, et al. (eds) Tribology research: from model experiment to industrial problem, Tribology series, vol. 39. Lyon, France: Elsevier, 2001, pp.787–796.
- 23. Bercea M, Paleu V and Bercea I. Lubricant oils additivated with polymers in EHD contacts: Part 1. Rheological behaviour. Lubricat Sci 2004; 17: 1–24.
- 24. Bair S and Winer W. High shear stress rheology of liquid lubricants at pressures of 2 to 200 MPa. ASME J Tribol 1989; 112: 245–252.
- 25. Jacod B, Venner C and Lugt P. Extension of the friction mastercurve to limiting shear stress models. ASME J Tribol 2003; 125: 739–746.
- 26. Hirst W and Moore A. Elastohydrodynamic lubrication at high pressures. II. Non-Newtonian behaviour. Proc R Soc Lond Ser A (Math Phys Sci) 1979; 365: 537–565.
- 27. Clarke A, Weeks I, Evans H, et al. An investigation into mixed lubrication conditions using electrical contact resistance techniques. Tribol Int 2016; 93: 709–716.
- 28. Steward J. The compliance of solid, wide-faced spur gears. J Mech Des 1990; 112: 590–595.
- 29. Roth K. Zahnradtechnik. Band I: Stirnradverzahnungen-Geometrische Grundlagen. Berlin, Germany: Springer-Verlag, 1989.
- 30. Olver A. Testing transmission lubricants: the importance of thermal response. Proc IMechE, Part G: J Aerospace Engineering 1991; 205: 35–44.
- 31. ISO/TR 15144-1. Calculation of micropitting load capacity of cylindrical spur and helical gears – Part 1: Introduction and basic principles, 2nd Edition, International Organization for Standardization, September 2014.
- 32. Hamrock BJ and Dowson D. Isothermal elastohydrodynamic lubrication of point contacts: Part III – Fully flooded results. J Tribol 1977; 99: 264–275.
- 33. van Leeuwen H. The determination of the pressure–viscosity coefficient of a lubricant through an accurate film thickness formula and accurate film thickness measurements. Proc IMechE, Part J: J Engineering Tribology 2009; 223: 1143–1163.
- 34. Mann U. Schmierfilmbildung in Elastohydrodynamischen Kontakten. PhD Thesis, Technische Universita¨t Mu¨nchen, 1995.
- 35. Ho¨glund E. Influence of lubricant properties on elastohydrodynamic lubrication. Wear 1999; 232: 176–184.
- 36. Bair S. Choosing pressure-viscosity relations. High Temp High Pressure 2015; 44: 415–428.
- 37. Gold PW, Schmidt A, Dicke H, et al. Viscosity-pressure-temperature behaviour of mineral and synthetic oils. J Synthetic Lubricat 2001; 18: 51–79.
- 38. Michaelis K. Die Integraltemperatur zur Beurteilung der Freßtragfa¨higkeit von Stirnradgetrieben. PhD Thesis, Technische Universita¨t Mu¨nchen, 1987.

- 39. Hili J, Olver AV, Edwards S, et al. Experimental investigation of elastohydrodynamic (EHD) film thickness behavior at high speeds. Tribol Transact 2010; 53: 658–666.
- 40. Hinterstoißer M. Zur Optimierung des Wirkungsgrades von Stirnradgetrieben. PhD Thesis, Technische Universita¨t Mu¨nchen, 2014.
- 41. Morales-Espejel G and Wemekamp AW. Ertel-Grubin methods in elastohydrodynamic lubrication-a review. Proc IMechE, Part J: J Engineering Tribology 2008; 222: 15–34.
- 42. Spikes H and Jie Z. History, origins and prediction of elastohydrodynamic friction. Tribol Lett 2014; 56: 1–25.

- 43. Bair S, Vergne P, Kumar P, et al. Comment on ''history, origins and prediction of elastohydrodynamic friction''. Tribol Lett 2015; 58: 1–8.
- 44. Spikes H and Jie Z. Reply to the comment by Scott Bair et al. on ''History, origins and prediction of elastohydrodynamic friction'' by Spikes and Jie in Tribology Letters. Tribol Lett 2015; 58: 1–6.
- 45. Bobzin K, Brogelmann T, Stahl K, et al. Friction reduction of highly-loaded rolling-sliding contacts by surface modifications under elasto-hydrodynamic lubrication. Wear 2015; 328-329: 217–228.
- 46. Lohner T, Mayer J, Michaelis K, et al. On the runningin behavior of lubricated line contacts. Proc IMechE, Part J: J Engineering Tribology 2015; 231: 441–452.