![](_page_0_Picture_0.jpeg)

![](_page_0_Picture_1.jpeg)

*Article*

# **An Investigation for the Friction Torque of a Tapered Roller Bearing Considering the Geometric Homogeneity of Rollers**

**Yuwei Liu \* [,](https://orcid.org/0000-0002-6157-9971) Xingyu Fan, Jia Wang and Xiayi Liu**

School of Mechanical and Precision Instrument Engineering, Xi'an University of Technology, Xi'an 710049, China; 105266@xaut.edu.cn (X.F.); wangxiaojia@xaut.edu.cn (J.W.); liuxyee@xaut.edu.cn (X.L.)

**\*** Correspondence: liuyw\_mg@xaut.edu.cn

**Abstract:** The geometric homogeneity of rollers, namely the dimension and shape deviations among rollers in a roller bearing, is one of the most important manufacturing errors. However, to the best of the authors' knowledge, no specified investigation has been carried out on the effects of the geometric homogeneity of rollers on the friction torque of tapered roller bearings (TRBs). By introducing the diameter deviation of rollers and the distribution form of rollers with a diameter deviation, this study presents a mathematic model to reveal the effects of the geometric homogeneity of rollers on the friction torque of TRBs. The geometric homogeneity of the rollers, although having only a minimal influence on the overall friction torque acting on rings, can lead to a significant increase in the slide friction force between the individual rollers and the inner raceway. By comparing the distribution form of rollers with a diameter deviation, the diameter deviation value of the roller shows a significant influence on the maximum sliding friction between the roller and the inner raceway. The impact of the geometric homogeneity of rollers on the sliding friction between the roller and the inner raceway is more pronounced under light load conditions. The above-mentioned comparisons and conclusions can be used in formulating machining error criteria for TRB rollers.

**Keywords:** geometric homogeneity of rollers; tapered roller bearings; friction torque; contact force distribution

![](_page_0_Picture_9.jpeg)

**Citation:** Liu, Y.; Fan, X.; Wang, J.; Liu, X. An Investigation for the Friction Torque of a Tapered Roller Bearing Considering the Geometric Homogeneity of Rollers. *Lubricants* **2022**, *10*, 154. [https://doi.org/](https://doi.org/10.3390/lubricants10070154) [10.3390/lubricants10070154](https://doi.org/10.3390/lubricants10070154)

Received: 10 May 2022 Accepted: 9 July 2022 Published: 12 July 2022

**Publisher's Note:** MDPI stays neutral with regard to jurisdictional claims in published maps and institutional affiliations.

![](_page_0_Picture_13.jpeg)

**Copyright:** © 2022 by the authors. Licensee MDPI, Basel, Switzerland. This article is an open access article distributed under the terms and conditions of the Creative Commons Attribution (CC BY) license [\(https://](https://creativecommons.org/licenses/by/4.0/) [creativecommons.org/licenses/by/](https://creativecommons.org/licenses/by/4.0/) 4.0/).

# **1. Introduction**

Tapered roller bearings (TRBs) are generally used in heavy equipment such as wind power generators and high-speed trains. The operating temperature and limited rotating speed of the machinery are determined by the friction torques of their interior TRBs [\[1\]](#page-10-0). Thus, the behavior of the friction torque on TRBs of high-speed industrial machinery has continually attracted the attention of researchers and field engineers in the last few decades.

Throughout the years of development and investigation, efforts have been made in theoretical and experimental analyses to calculate TRBs' friction torque. Witte [\[2\]](#page-10-1) analyzed the friction between each roller and raceway and presented an analytical model to calculate the friction torque of TRBs. However, the model did not consider the mixed lubrication state between bearing components. Consequently, the simulated results of the starting torque of TRBs showed a non-negligible error. With the development of elastohydrodynamic lubrication (EHL) theory, Karna [\[3\]](#page-10-2) and Aihara [\[4\]](#page-10-3) further modified and perfected the friction torque model of TRBs to meet engineering accuracy requirements. Zhou [\[5\]](#page-10-4) explored a novel friction torque model to analyze the influences of a starved condition and a contaminant on the friction torques of TRBs. The Svenska Kullager Fabriken (SKF) Group [\[6\]](#page-10-5) proposed a concise empirical formulation for predicting the friction torques of the TRBs based on the experimental results.

Despite numerous previous studies focusing on analyzing the friction torque of TRBs with the designed geometric dimensions, in practical engineering applications, manufacturing errors are unavoidable, which can affect the contact state between mating bearing

*Lubricants* **2022**, *10*, 154 2 of 12

components [\[7–](#page-10-6)[10\]](#page-10-7) and thus the friction torque of TRBs. Aschenbrenner [\[11\]](#page-10-8) presented a variational simulation framework for the analysis of the load distribution of cylindrical roller bearings with component geometric deviations, providing a basis for investigating the frictional torque of tapered roller bearings with component geometric deviations. Deng [\[12\]](#page-10-9) developed a theoretical method to investigate the effects of the surface waviness of bearing components on the friction torque of ball bearings. Heras [\[13\]](#page-10-10) proposed a finite element model to calculate the friction torque of four contact point slewing bearings and analyzed the effects of raceway geometric errors on bearing friction torque. Halminen [\[14\]](#page-10-11) and Xu [\[15\]](#page-10-12) established a multibody model of ball bearings involving the surface waviness of the components. In their study, the impact of surface waviness on the dynamic performance of ball bearings was explored. Liu [\[16\]](#page-10-13) utilized a time-varying calculation method to estimate the friction moments of angular contact ball bearings and analyzed the effects of the waviness amplitude and order on bearing friction torque under different operating conditions. Liu [\[17\]](#page-10-14) analyzed the effects of roundness errors on the friction torque of bearings and found that both the magnitude and order of the roundness error have a significant impact on the bearing friction torque.

In addition to raceway machining errors, the geometric homogeneity of rollers, namely the dimension deviations between each roller, also plays a significant role in the evaluation of manufacturing errors in roller bearings [\[18](#page-10-15)[,19\]](#page-10-16). Similar to a localized defect, the geometric homogeneity of rollers first affects the contact state between the roller and the raceway [\[20,](#page-10-17)[21\]](#page-10-18), thereby changing the overall bearing performance. However, the effects of the geometric homogeneity of rollers on the friction torque of TRBs are rarely reported in the literature. It is therefore timely to present this research to fill the knowledge gap.

In this paper, a quasi-statics model and a friction torque model for TRBs concerning the geometric homogeneity of rollers have been proposed, in which the geometric homogeneity of rollers was represented by the diameter deviation value of rollers and the distribution form of rollers with a diameter deviation. Based on the proposed models, the effects of the geometric homogeneity of rollers on the contact force distribution and friction characteristics of TRBs have been analyzed under different axial external loads, rotating speeds, and cage slip rates.

## **2. Materials and Methods**

## *2.1. Quasi-Statics Analysis of TRB Considering the Geometric Homogeneity of Rollers*

In order to estimate TRB friction torque accurately, the contact forces between each roller and raceway should be obtained in advance. As shown in Figure [1a](#page-2-0), when the inner ring is constrained by displacement, an external load {*F*} *<sup>T</sup>* = {*Fx*, *Fy*, *Fz*} acting on the outer ring causes the corresponding displacement of the outer ring {*d*} *<sup>T</sup>* = {*dx*, *dy*, *dz*}. As presented in Figure [1b](#page-2-0), for the cross-section at the *j*th roller with the location angle *ϕ<sup>j</sup>* , the displacement of the outer ring along the *r<sup>j</sup>* -axis and *z*-axis can be represented as:

$$\begin{cases}
d_{rj} = d_x \cos \varphi_j + d_y \sin \varphi_j \\
d_{zj} = d_z
\end{cases}$$
(1)

The displacement of the outer ring at the *j*th roller {*d<sup>j</sup>* } *<sup>T</sup>* = {*drj*, *dzj*} determines the contact deformations of the *j*th roller-raceways and *j*th roller-flange {*δ<sup>j</sup>* } *<sup>T</sup>* = {*δij*, *δej*, *δfj*}, as well as the contact forces acting on the *j*th roller {*Q<sup>j</sup>* } *<sup>T</sup>* = {*Qej*, *Qij*, *Qfj*}. Here, the subscripts *i*, *e*, and *f* denote the inner raceway, outer raceway, and flange, respectively. The above contact deformation process and contact forces can be represented as [\[22\]](#page-11-0):

$$\begin{cases} \delta_{ij} = X_{rj} \cos \alpha_i + X_{zj} \sin \alpha_i \\ \delta_{ej} = (d_{rj} - X_{rj}) \cos \alpha_e + (d_{zj} - X_{zj}) \sin \alpha_e \\ \delta_{fj} = (d_{rj} - X_{rj}) \cos \alpha_f + (d_{zj} - X_{zj}) \sin \alpha_f \end{cases}$$
(2)

Lubricants 2022, 10, 154 3 of 12

$$\begin{cases}
Q_a = K_a \delta_a^{10/9}, \ a = i, e \\
Q_f = K_f \delta_f^{3/2}
\end{cases}$$
(3)

where  $X_{rj}$ ,  $X_{zj}$  indicate the displacement of the jth roller along the  $r_j$ -axis and z-axis, respectively;  $\alpha_i$ ,  $\alpha_e$ ,  $\alpha_f$  indicate the contact angles of the roller-inner raceway, roller-outer raceway, and roller-flange, respectively;  $\delta_a$ ,  $\delta_f$  indicate the contact deformations of the roller-raceways and roller-flange, respectively;  $K_a$ ,  $K_f$  indicate the load-deformation factors of the roller-raceways and roller-flange, respectively, which depend on the material and geometry at the contact.

<span id="page-2-0"></span>![](_page_2_Picture_3.jpeg)

**Figure 1.** The displacement of the outer ring under the external load: (**a**) a global view of TRBs; (**b**) a cross-section at the *j*th roller.

Since the surface topography deviation of rollers is ignored in this study, the geometric homogeneity of rollers can be simplified as the diameter dimension deviation of rollers. When the diameter dimension deviation occurs on rollers, the contact force between the roller and its mating components will be changed. As a result, Equation (3) should be modified based on Taylor's series as

$$\begin{cases} Q_a = K_a \delta_a^{10/9} + \frac{10K_a}{9} \delta_a^{1/9} \frac{\Delta D \cos \varepsilon}{2} \ a = i, e \\ Q_f = K_f \delta_f^{3/2} \end{cases}$$
(4)

where  $\Delta D$  is the diameter deviation value of the roller;  $\varepsilon$  is the half roller angle.

As shown in Figure 2, from the force equilibrium with respect to the  $\xi$  and  $\zeta$  axes, the roller equilibrium equations are established considering the centrifugal force as Equation (4). Equations (2), (4) and (5) describe a local equilibrium system that can be solved by numerical methods to obtain the contact forces acting on the jth roller  $\{Q_j\}^T = \{Q_{ej}, Q_{ij}, Q_{fj}\}$  under a certain displacement of the outer ring at the jth roller  $\{d_j\}^T = \{d_{rj}, d_{zj}\}$ .

$$\begin{cases} (Q_i - Q_e)\cos\varepsilon + Q_f\sin\mu + F_c\cos\kappa = 0\\ (Q_i + Q_e)\sin\varepsilon + Q_f\cos\mu - F_c\sin\kappa = 0 \end{cases}$$
 (5)

where  $F_c$  is the roller centrifugal force;  $\kappa$  is the angle between the roller center line and TRB center line; and  $\beta$  is the angle between the roller center line and the roller-flange contact line.

Lubricants 2022, 10, 154 4 of 12

<span id="page-3-0"></span>![](_page_3_Picture_1.jpeg)

Figure 2. Contact forces acting on the roller.

The global equilibrium system of external loads acting on the outer ring and the contact forces of the roller-outer raceway can be represented as

$$\begin{cases} \sum_{j=1}^{Z} Q_{ej} \cos \alpha_e \cos \varphi_j + F_x = 0\\ \sum_{j=1}^{Z} Q_{ej} \cos \alpha_e \sin \varphi_j + F_y = 0\\ \sum_{j=1}^{Z} Q_{ej} \sin \alpha_e + F_z = 0 \end{cases}$$

$$(6)$$

By giving a certain external load,  $\{F\}^T = \{F_x, F_y, F_z\}$ , the unknown contact forces of each roller can be obtained from the coupled solution of the global equilibrium system and the local equilibrium system, which is usually called the quasi-statics analysis of TRB. The detailed calculation procedure for the above coupled solution is shown in Figure 3. Since the global equilibrium equations and local equilibrium equations are nonlinear, the iterative Newton–Raphson method is adopted in this study.

<span id="page-3-1"></span>![](_page_3_Figure_6.jpeg)

**Figure 3.** The coupled solution procedure of the global equilibrium system and the local equilibrium system.

*Lubricants* **2022**, *10*, 154 5 of 12

### *2.2. Friction Torque Analysis of TRBs Considering the Geometric Homogeneity of Rollers*

The friction in TRBs is mainly composed of the rolling friction between the roller and raceways *Fro*, the sliding friction between the roller and raceways *Frs* and the sliding friction between the roller and flange *Ffs*. According to theoretical analyses and experimental studies, the above components of friction in TRBs can be represented as in [\[23\]](#page-11-1):

$$F_{ro} = \frac{0.88 \times 10^2}{\alpha_0} (GU)^{0.658} W^{0.31} R l_w \tag{7}$$

$$F_{rs} = 0.168v_s \eta_0 l_w I U^{-0.74} G^{-0.4} W^{0.2} R^{-1}$$
(8)

$$F_{fs} = Q_f \mu_0 e^{-1.8\Lambda_r^{1.2}} \tag{9}$$

where *α*<sup>0</sup> is the viscosity-pressure coefficient; *G*, *U,* and *W* are the dimensionless material parameter, the dimensionless velocity parameter, and the dimensionless load parameter, respectively [\[24\]](#page-11-2); *R* is the equivalent radius between the roller and raceway; *l<sup>w</sup>* is the effective contact length between the roller and raceway; *v<sup>s</sup>* is the slide speed between the roller and raceway; *I* is the integrals used to describe the tractive effect; *η*<sup>0</sup> is the viscosity at atmospheric pressure; *µ*<sup>0</sup> is the Coulomb friction coefficient; and Λ*<sup>r</sup>* is the oil film parameter.

According to an ideal Hertz line contact pressure distribution, the integrals of tractive effect *I* can be approximated as in [\[5\]](#page-10-4):

$$I = \int_{0}^{b} \exp\left\{ (\ln \eta_0 + 9.67) [(1 + 5.1 \times 10^{-9} p) \cdot 0.601 - 1] \right\} dx$$
 (10)

where *b* is the semi-width of the Hertz line contact; and *p* is the contact pressure.

Based on the above analyses, the total friction torque acting on the outer ring *Mer* and that acting on the inner ring *Mir* can be represented as

$$M_{er} = \sum_{j=1}^{z} (F_{ro,ej} - F_{rs,ej}) R_e$$
 (11)

$$M_{ir} = \sum_{j=1}^{z} \left[ (F_{ro,ij} + F_{rs,ij}) R_{i} + F_{fs,fj} (R_{i} + e_{r}) \right]$$
 (12)

where *R<sup>e</sup>* , *R<sup>i</sup>* are the radius of the roller-outer raceway contact point and the radius of the roller-inner raceway contact point on the roller mean diameter, respectively; and *e<sup>r</sup>* is the height of the roller end and flange contact. Here the subscripts *ej*, *ij*, and *fj* denote the roller-outer raceway contact, roller-inner raceway contact, and roller-flange contact, respectively.

As shown in Equation (8), in addition to the contact force, the sliding friction between the roller and raceway *Frs* is also affected by the sliding speed between the roller and raceway *vs* . The kinematic relationships of the bearing components are assumed to follow the outer raceway control hypothesis [\[25](#page-11-3)[,26\]](#page-11-4) in this article; therefore, the sliding speed between the roller and raceways can be represented as:

$$\begin{cases}
v_{se} = 0 \\
v_{si} = \omega_i R_i S_c
\end{cases}$$
(13)

where *S<sup>c</sup>* is the cage slip rate; and *ω<sup>i</sup>* is the angular velocity of the inner ring.

## *2.3. Geometrical and Material Parameters*

Based on the theoretical analyses described above, the friction torque of TRBs considering the geometric homogeneity of rollers is analyzed and discussed in this section. Take

Lubricants 2022, 10, 154 6 of 12

TRB 30228J as an example to construct the corresponding analysis model, and its geometric characteristics are given in Table 1.

<span id="page-5-0"></span>

| <b>Table 1.</b> The geometric and material properties of TRB 3200 | Table 1. The | geometric and | material pro | operties of | TRB 32008 |
|-------------------------------------------------------------------|--------------|---------------|--------------|-------------|-----------|
|-------------------------------------------------------------------|--------------|---------------|--------------|-------------|-----------|

| Geometrical Characteristics             | Value               |
|-----------------------------------------|---------------------|
| Small diameter of taper roller (mm)     | 6.131               |
| Large diameter of taper roller (mm)     | 6.846               |
| Length of roller (mm)                   | 13.66               |
| Number of rollers                       | 23                  |
| Outer raceway angle (rad)               | 0.2473              |
| Inner raceway angle (rad)               | 0.1949              |
| Roller angle (rad)                      | 0.0262              |
| Flange angle (rad)                      | 1.5621              |
| Elasticity modulus (N/mm <sup>2</sup> ) | $2.1 \times 10^{5}$ |
| Poisson's ratio                         | 0.278               |

To simulate the geometric homogeneity of rollers caused by machining, the diameter deviation value of roller  $\Delta D$  is assumed to be a standard Gaussian distribution and can be represented as

$$f(\frac{\Delta D}{S_d}) = \frac{1}{\sqrt{2\pi}} \exp\left(-\left(\frac{\Delta D}{S_d}\right)^2/2\right) \tag{14}$$

where  $S_d$  is the deviation magnitude determined by the roller diameter and the machining methods.

Multiple series of  $\Delta D$  were generated according to the above distribution characteristics, which consider different deviation magnitudes and distribution forms of the rollers with diameter deviations (random distribution and distribution in descending order of deviation value). As shown in Table 2, Series 1 corresponds to an  $S_d$  value of 0.5  $\mu$ m and random distribution, Series 2 corresponds to an  $S_d$  value of 0.5  $\mu$ m and distribution in descending order of deviation value, Series 3 corresponds to an  $S_d$  value of 1  $\mu$ m and random distribution, Series 4 corresponds to an  $S_d$  value of 1  $\mu$ m and distribution in descending order of deviation value, Series 5 corresponds to an  $S_d$  value of 1.5  $\mu$ m and random distribution, and Series 6 corresponds to an  $S_d$  value of 1.5  $\mu$ m and distribution in descending order of deviation value.

<span id="page-5-1"></span>**Table 2.** The generating data of diameter deviation value  $\Delta D$ .

| Roller     | Number   | 1     | 2     | 3     | 4     | 5     | 6     | 7     | 8     |
|------------|----------|-------|-------|-------|-------|-------|-------|-------|-------|
|            | Series 1 | 0.28  | 0.52  | -0.56 | 0.63  | 0.33  | -0.03 | -0.10 | -0.11 |
|            | Series 2 | 0.76  | 0.63  | 0.52  | 0.47  | 0.41  | 0.33  | 0.31  | 0.28  |
| $\Delta D$ | Series 3 | 0.55  | 1.04  | -1.12 | 1.26  | 0.66  | -0.07 | -0.20 | -0.22 |
| (µm)       | Series 4 | 1.53  | 1.26  | 1.04  | 0.95  | 0.83  | 0.66  | 0.63  | 0.55  |
|            | Series 5 | 0.83  | 1.56  | -1.68 | 1.89  | 0.99  | -0.10 | -0.29 | -0.33 |
|            | Series 6 | 2.29  | 1.89  | 1.56  | 1.42  | 1.24  | 0.99  | 0.94  | 0.83  |
| Roller     | Number   | 9     | 10    | 11    | 12    | 13    | 14    | 15    | 16    |
|            | Series 1 | -0.15 | 0.01  | 0.03  | 0.41  | 0.76  | 0.23  | -0.10 | 0.31  |
|            | Series 2 | 0.26  | 0.23  | 0.15  | 0.13  | 0.09  | 0.07  | 0.03  | 0.01  |
| $\Delta D$ | Series 3 | -0.30 | 0.02  | 0.05  | 0.83  | 1.53  | 0.47  | -0.21 | 0.63  |
| (µm)       | Series 4 | 0.52  | 0.47  | 0.31  | 0.26  | 0.18  | 0.14  | 0.05  | 0.02  |
|            | Series 5 | -0.45 | 0.03  | 0.08  | 1.24  | 2.29  | 0.70  | -0.31 | 0.94  |
|            | Series 6 | 0.77  | 0.70  | 0.46  | 0.39  | 0.27  | 0.20  | 0.08  | 0.03  |
| Roller     | Number   | 17    | 18    | 19    | 20    | 21    | 22    | 23    |       |
|            | Series 1 | 0.09  | -0.51 | 0.47  | 0.15  | 0.07  | 0.26  | 0.13  |       |
|            | Series 2 | -0.03 | -0.10 | -0.10 | -0.11 | -0.15 | -0.51 | -0.56 |       |
| $\Delta D$ | Series 3 | 0.18  | -1.03 | 0.95  | 0.31  | 0.14  | 0.52  | 0.26  |       |
| (µm)       | Series 4 | -0.07 | -0.20 | -0.21 | -0.22 | -0.30 | -1.03 | -1.12 |       |
| ,          | Series 5 | 0.27  | -1.54 | 1.42  | 0.46  | 0.20  | 0.77  | 0.39  |       |
|            | Series 6 | -0.10 | -0.29 | -0.31 | -0.33 | -0.45 | -1.54 | -1.68 |       |

Lubricants 2022, 10, 154 7 of 12

#### 3. Results and Discussions

3.1. Effects of the Geometric Homogeneity of Rollers on Contact Force Distribution

According to the diameter deviation values of the rollers shown in Table 2, the internal contact force distribution of TRBs considering the geometric homogeneity of rollers is analyzed.

The contact force between each roller and the outer raceway is shown in Figure 4, in which the axial external load is 10 kN and the bearing rotating speed is 1000 rpm. In this figure, the contact force distribution of a TRB with an ideal roller diameter was used as a benchmark to reflect the effects of the geometric homogeneity of rollers. Changes in the diameter deviations of the rollers, whether positive or negative, will cause corresponding changes in the contact force between the roller and the outer raceway. Comparing Figure 4a,b, it can be seen that the distribution form of rollers with a diameter deviation also affects the internal contact force distribution of the TRB. It should be noted that the centrifugal force of rollers is much smaller than the roller-raceway contact force in the limited speed range of the TRBs; therefore, the influence of the bearings' rotating speeds on the contact force distribution is not further analyzed [20].

<span id="page-6-0"></span>![](_page_6_Figure_5.jpeg)

**Figure 4.** Effects of the geometric homogeneity of rollers on contact force distribution of TRBs: (a) Rollers with a diameter deviation are randomly distributed; (b) Rollers with a diameter deviation are distributed by deviation size; (c) Schematic diagram of the position angle of the *j*th roller.

According to the analysis results shown in Figure 4, the maximum value and the variance of contact forces are extracted and shown in Table 3. Comparing the maximum value and the variance of contact force corresponding to different series, it can be found that the maximum contact force is mainly determined by the diameter deviation value of the roller. However, both the diameter deviation value of the roller and the distribution form of rollers with a diameter deviation will have a significant effect on the uniformity of contact force distribution (reflected by variance values).

<span id="page-6-1"></span>**Table 3.** Maximum contact force and variance of contact force.

|                    | Series 1 | Series 2 | Series 3 | Series 4 | Series 5 | Series 6 |
|--------------------|----------|----------|----------|----------|----------|----------|
| Maximum value (kN) | 2.026    | 2.013    | 2.280    | 2.253    | 2.539    | 2.495    |
| Variance (kN²)     | 0.0162   | 0.0096   | 0.0645   | 0.0383   | 0.1441   | 0.0856   |

Lubricants 2022, 10, 154 8 of 12

#### 3.2. Effects of the Geometric Homogeneity of Rollers on Friction Force and Torque

According to the contact force distribution and Equations (11) and (12), we can obtain the friction torque acting on the outer ring and inner ring. The friction torque acting on the inner = and outer rings is shown in Figure 5, in which the axial external load is 10 kN and the rollers have an ideal diameter.

<span id="page-7-0"></span>![](_page_7_Figure_3.jpeg)

Figure 5. Friction torque acting on rings as a function of cage slip rate.

Based on the outer raceway control hypothesis, the friction torque acting on the outer ring only involves the rolling friction between the rollers and the raceway; however, the friction torque acting on the inner ring involves the rolling friction and sliding friction between the rollers and the raceway as well as the slide friction between the rollers and the flange. Therefore, the cage slip rate, which determines the slide speed between the rollers and the raceway, only has a significant effect on the friction torque acting on the inner ring as shown in Figure 5. In addition, as the TRBs' rotating speed increases, the lubrication state between the rollers and the flange changes from mixed lubrication to full-film lubrication, which leads to an initial decrease in the friction torque acting on the inner ring. The variation trends in friction torque obtained from the proposed model are consistent with those in the literature [4,5], thus validating the rationality of the proposed model.

The effects of the geometric homogeneity of rollers on friction torque acting on the rings are shown in Figure 6, in which the axial external load is 10 kN and the cage slip rate is 0.01%. As shown in Figure 6, the geometric homogeneity of rollers causes a decrease in the friction torque acting on the rings. Compared to the distribution form of the rollers with a diameter deviation, the diameter deviation value of the roller has a greater influence on the friction torque acting on the rings. When the TRBs' rotating speed is low, the roller-flange is in a mixed lubrication state, which causes the percentage change in the friction torque acting on the inner ring to be affected by the TRBs' rotating speed, as shown in Figure 6a. After the roller-flange lubrication enters the full oil film lubrication state, the percentage change in the friction torque tends to be constant. Since the friction torque acting on the outer ring only involves the rolling friction between the rollers and the raceway, the percentage change in the friction torque acting on the outer ring is not affected by the changes in the lubrication state between the roller and the flange. It should be noted that the above friction torque c reduction caused by the geometric homogeneity of rollers is very small at only a 10<sup>-2</sup>% order of magnitude, which means

Lubricants 2022, 10, 154 9 of 12

<span id="page-8-0"></span>that the geometric homogeneity of rollers will not have a significant influence on the overall energy loss or heat generation of TRBs.

![](_page_8_Figure_2.jpeg)

**Figure 6.** Effects of the geometric homogeneity of rollers on friction torque acting on rings: (a) Friction torque acting on inner ring; (b) Friction torque acting on outer ring.

Excessive sliding friction between individual rollers and raceways may induce excessive localized high temperatures inside the TRBs. The maximum increment in the slide friction between the roller and inner raceway caused by the geometric homogeneity of rollers (occurring at the roller with the greatest contact force) is shown in Figure 7. As shown in Figure 7a, the diameter deviation value of the roller results in a significant increase in the sliding friction between the roller and the inner raceway, whereas the distribution form of the rollers with a diameter deviation has little influence on the slide friction between the roller and the inner raceway.

![](_page_8_Figure_5.jpeg)

Figure 7. Cont.

Lubricants 2022, 10, 154 10 of 12

<span id="page-9-0"></span>![](_page_9_Figure_1.jpeg)

**Figure 7.** Effects of geometric homogeneity of rollers on roller-inner raceway maximum sliding friction force: (a) Axial external load is 10 kN and cage slip rate is 0.01%; (b) Axial external load is 10 kN and cage slip rate is 0.05%; (c) Axial external load is 2 kN and cage slip rate is 0.05%.

Comparing Figure 7a,b, it can be seen that the increments in the sliding friction caused by the geometric homogeneity of rollers increase as the cage slip rate increases, but the corresponding increase in the ratio remains constant. In addition, as shown in Figure 7b,c, the increments in the sliding friction caused by the geometric homogeneity of rollers and the corresponding increase in the ratio increase when the axial external load increases from 2 kN to 10 kN.

#### 4. Conclusions

In this paper, a quasi-statics model and a friction torque model for TRBs considering the geometric homogeneity of rollers were proposed, and the effects of the geometric homogeneity of rollers on the contact force distribution and friction torque of TRBs are analyzed thoroughly. In the proposed model, the diameter deviation value of rollers and the distribution form of rollers with a diameter deviation were fully considered to improve the accuracy of the model. From the results, the following conclusions have been obtained.

Both the diameter deviation value of the roller and the distribution form of rollers with a diameter deviation have a significant effect on the uniformity of contact force distribution and the maximum contact force. The geometric homogeneity of the rollers, although having only a minimal influence on the overall friction torque acting on the rings, can lead to a significant increase in the slide friction force between the individual rollers and the inner raceway. Compared to the distribution form of rollers with a diameter deviation, the diameter deviation value of the roller has a greater influence on the maximum sliding friction between the roller and the inner raceway. As the cage slip rate increases, the slide friction force increment caused by the geometric homogeneity of the rollers will obviously increase. The effects of the geometric homogeneity of rollers on the sliding friction between the roller and the inner raceway are more pronounced under light axial external load conditions.

This paper provides a mathematical model for the friction characteristic analysis for TRBs considering the geometric homogeneity of rollers as well as a theoretical basis for formulating machining error criteria for TRB rollers. It should be noted that although this paper only analyzes the case of pure axial loading, the proposed model is also applicable to the case of axial and radial combined loading. Under axial and radial combined loading, there are load-bearing and non-load-bearing areas inside the TRBs. Therefore, the contact load of the individual roller is significantly affected by its position angle at some point in the

*Lubricants* **2022**, *10*, 154 11 of 12

TRBs' operation. To make the analysis more reasonable, for TRBs under radial loading and combined loading, the friction torque analysis should consider the time-varying angular position of rollers.

**Author Contributions:** Conceptualization, Y.L. and X.F.; methodology, Y.L.; software, Y.L. and X.F.; validation, X.F. and J.W.; investigation, Y.L.; writing—original draft preparation, Y.L. and X.L.; writing—review and editing, X.F. and J.W.; visualization, Y.L. and J.W.; supervision, X.F.; project administration, X.F. and J.W.; funding acquisition, X.L. All authors have read and agreed to the published version of the manuscript.

**Funding:** This research was funded by the Province Key Research and Development Program of Shaanxi (2022KW-18).

**Institutional Review Board Statement:** Not applicable.

**Informed Consent Statement:** Not applicable.

**Data Availability Statement:** Detailed data are contained within the article.

**Conflicts of Interest:** The authors declare no conflict of interest.

### **References**

<span id="page-10-0"></span>1. Harris, T.A.; Kotzalas, M.N. Chapter 7: Rolling bearing temperatures. In *Advanced Concepts of Bearing Technology: Rolling Bearing Analysis*, 5th ed.; Taylor & Francis Group: Boca Raton, FL, USA, 2006.

- <span id="page-10-1"></span>2. Witte, D.C. Operating torque of tapered roller bearings. *ASLE Trans.* **1973**, *16*, 61–67. [\[CrossRef\]](http://doi.org/10.1080/05698197308982705)
- <span id="page-10-2"></span>3. Karna, C.L. Performance characteristics at the rib roller end contact in tapered roller bearings. *ASLE Trans.* **1974**, *17*, 14–21. [\[CrossRef\]](http://doi.org/10.1080/05698197408981434)
- <span id="page-10-3"></span>4. Aihara, S. A new running torque formula for tapered roller bearings under axial load. *J. Tribol.* **1987**, *109*, 471–478. [\[CrossRef\]](http://doi.org/10.1115/1.3261475)
- <span id="page-10-4"></span>5. Zhou, R.S.; Hoeprich, M.R. Torque of Tapered Roller Bearings. *J. Tribol.* **1991**, *113*, 590–597. [\[CrossRef\]](http://doi.org/10.1115/1.2920664)
- <span id="page-10-5"></span>6. SKF. Friction. In *General Catalogue 4000*; SKF: Gothenburg, Sweden, 2003; Available online: [https://imparayaycia.com/SKF%20](https://imparayaycia.com/SKF%20CATALOGO%20GENERAL.pdf) [CATALOGO%20GENERAL.pdf](https://imparayaycia.com/SKF%20CATALOGO%20GENERAL.pdf) (accessed on 11 July 2022).
- <span id="page-10-6"></span>7. Ma, S.; Zhang, X.; Yan, K.; Zhu, Y.; Hong, J. A Study on Bearing Dynamic Features under the Condition of Multiball–Cage Collision. *Lubricants* **2022**, *10*, 9. [\[CrossRef\]](http://doi.org/10.3390/lubricants10010009)
- 8. Wu, H.; Liu, J.; Shao, Y. Vibration characteristics of a roller bearing with the waviness error. In Proceedings of the 2019 58th Annual Conference of the Society of Instrument and Control Engineers of Japan, Higashi-Hiroshima, Japan, 10–13 September 2019. [\[CrossRef\]](http://doi.org/10.23919/SICE.2019.8859782)
- 9. Savalia, R.; Ghosh, M.K.; Pandey, R.K. Vibration Analysis of Lubricated Angular Contact Ball Bearing of Rigid Rotor Considering Waviness of Ball and Races. *Tribol. Online* **2008**, *3*, 322–327. [\[CrossRef\]](http://doi.org/10.2474/trol.3.322)
- <span id="page-10-7"></span>10. Nan, G.; Zhang, Y.; Zhu, Y.; Guo, W. Nonlinear dynamics of rotor system supported by bearing with waviness. *Sci. Prog.* **2020**, *103*, 003685042094409. [\[CrossRef\]](http://doi.org/10.1177/0036850420944092)
- <span id="page-10-8"></span>11. Aschenbrenner, A.; Schleich, B.; Tremmel, S.; Sandro, W. A variational simulation framework for the analysis of load distribution and radial displacement of cylindrical roller bearings. *Mech. Mach. Theory* **2020**, *147*, 103769. [\[CrossRef\]](http://doi.org/10.1016/j.mechmachtheory.2019.103769)
- <span id="page-10-9"></span>12. Deng, S. Analysis on the Friction Torque Fluctuation of Angular Contact Ball Bearings. *J. Mech. Eng.* **2011**, *47*, 104. [\[CrossRef\]](http://doi.org/10.3901/JME.2011.23.104)
- <span id="page-10-10"></span>13. Heras, I.; Aguirrebeitia, J.; Abasolo, M. Friction torque in four contact point slewing bearings: Effect of manufacturing errors and ring stiffness. *Mech. Mach. Theory* **2017**, *112*, 145–154. [\[CrossRef\]](http://doi.org/10.1016/j.mechmachtheory.2017.02.009)
- <span id="page-10-11"></span>14. Halminen, O.; Aceituno, J.F.; Escalona, J.L.; Sopanen, J.; Mikkola, A. A touchdown bearing with surface waviness: Friction loss analysis. *Mech. Mach. Theory* **2017**, *110*, 73–84. [\[CrossRef\]](http://doi.org/10.1016/j.mechmachtheory.2017.01.002)
- <span id="page-10-12"></span>15. Xu, L.; Li, Y. Modeling of a deep-groove ball bearing with waviness defects in planar multibody system. *Multibody Syst. Dyn.* **2014**, *33*, 229–258. [\[CrossRef\]](http://doi.org/10.1007/s11044-014-9413-z)
- <span id="page-10-13"></span>16. Liu, J.; Li, X.; Ding, S.; Pang, R. A time-varying friction moment calculation method of an angular contact ball bearing with the waviness error. *Mech. Mach. Theory* **2020**, *148*, 103799. [\[CrossRef\]](http://doi.org/10.1016/j.mechmachtheory.2020.103799)
- <span id="page-10-14"></span>17. Liu, J.; Yan, Z.; Shao, Y. An investigation for the friction torque of a needle roller bearing with the roundness error. *Mech. Mach. Theory* **2018**, *121*, 259–272. [\[CrossRef\]](http://doi.org/10.1016/j.mechmachtheory.2017.10.028)
- <span id="page-10-15"></span>18. Yao, W.; Yuan, J.; Zhou, F.; Chen, Z.; Zhao, T.; Zhong, M. Trajectory analysis and experiments of both-sides cylindrical lapping in eccentric rotation. *Int. J. Adv. Manuf. Technol.* **2017**, *88*, 2849–2859. [\[CrossRef\]](http://doi.org/10.1007/s00170-016-8980-y)
- <span id="page-10-16"></span>19. Yuan, J.; Yao, W.; Zhao, P.; Lyu, B.; Chen, Z.; Zhong, M. Kinematics and trajectory of both-sides cylindrical lapping process in planetary motion type. *Int. J. Mach. Tools Manuf.* **2015**, *92*, 60–71. [\[CrossRef\]](http://doi.org/10.1016/j.ijmachtools.2015.02.004)
- <span id="page-10-17"></span>20. Liu, Y.; Zhu, Y.; Yan, K.; Wang, F.; Hong, J. A novel method to model effects of natural defect on roller bearing. *Tribol. Int.* **2018**, *122*, 169–178. [\[CrossRef\]](http://doi.org/10.1016/j.triboint.2018.02.028)
- <span id="page-10-18"></span>21. Kang, W.; Zhu, Y.; Yan, K.; Ren, Z.; Gao, D.; Hong, J. Research on extracting weak repetitive transients of fault rolling element bearing. *ISA Trans.* **2021**, *123*, 381–397. [\[CrossRef\]](http://doi.org/10.1016/j.isatra.2021.05.016) [\[PubMed\]](http://www.ncbi.nlm.nih.gov/pubmed/34024624)

*Lubricants* **2022**, *10*, 154 12 of 12

<span id="page-11-0"></span>22. Tong, V.; Hong, S. The effect of angular misalignment on the running torques of tapered roller bearings. *Tribol. Int.* **2016**, *95*, 76–85. [\[CrossRef\]](http://doi.org/10.1016/j.triboint.2015.11.005)

- <span id="page-11-1"></span>23. Harris, T.A. An analytical method to predict skidding in high speed roller bearings. *ASLE Trans.* **1966**, *9*, 229–241. [\[CrossRef\]](http://doi.org/10.1080/05698196608972139)
- <span id="page-11-2"></span>24. Marian, M.; Bartz, M.; Wartzack, S.; Andreas, R. Non-dimensional groups, film thickness equations and correction factors for elastohydrodynamic lubrication: A review. *Lubricants* **2020**, *8*, 95. [\[CrossRef\]](http://doi.org/10.3390/lubricants8100095)
- <span id="page-11-3"></span>25. Harris, T.A.; Kotzalas, M.N. Chapter 7: Distributions of internal loading in statically loaded bearings. In *Essential Concepts of Bearing Technology: Rolling Bearing Analysis*, 5th ed.; Taylor & Francis Group: Boca Raton, FL, USA, 2006.
- <span id="page-11-4"></span>26. Zhang, Y.; Fang, B.; Kong, L.; Li, Y. Effect of the ring misalignment on the service characteristics of ball bearing and rotor system. *Mech. Mach. Theory* **2020**, *151*, 103889. [\[CrossRef\]](http://doi.org/10.1016/j.mechmachtheory.2020.103889)