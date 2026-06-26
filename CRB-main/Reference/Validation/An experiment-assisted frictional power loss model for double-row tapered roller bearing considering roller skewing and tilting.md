#### **TECHNICAL PAPER**

![](_page_0_Picture_2.jpeg)

# **An experiment-assisted frictional power loss model for double-row tapered roller bearing considering roller skewing and tilting**

**Zhixiang Zhao<sup>1</sup> · Yi Wu<sup>1</sup> · Pengpai Zhang<sup>1</sup> · Guanzhen Zhang<sup>1</sup> · Yide Feng<sup>1</sup> · Xiang Li<sup>1</sup> · Yue Zhao<sup>2</sup>**

Received: 15 April 2025 / Accepted: 17 November 2025 / Published online: 7 March 2026 © The Author(s), under exclusive licence to The Brazilian Society of Mechanical Sciences and Engineering 2026

#### **Abstract**

Accurate prediction of frictional power loss in bearings is essential for optimizing anti-friction designs and ensuring operational reliability. However, bearing frictional mechanisms under abnormal conditions remain insufficiently explored due to the neglect of irregular roller postures such as skewing and tilting. This study develops a comprehensive power loss model of double-row tapered roller bearings (DTRBs) that explicitly quantifies the effects of roller postures on total and local power loss. The roller motions, which serve as critical inputs of the proposed model, were measured by applying eccentric loads to the DTRB. Systematic testing evaluated the effects of eccentric displacement, radial load, axial load, and rotational speed on power dissipation. Furthermore, the effects of roller postures on power dissipation were decoupled from the total amount, and a sensitivity analysis was conducted. Results indicate that these postures contribute approximately 11% to 21% of total power loss as eccentric displacement ranges from 0 to 40 mm, with tilting generating almost 2.5 times greater heat than skewing. The increasing sensitivity further confirms that the two roller postures play an indispensable role in total frictional power loss.

**Keywords** Double-row tapered roller bearing · Frictional power loss · Roller skewing · Roller tilting · Experimentassisted model

| List of symbols |                                    |
|-----------------|------------------------------------|
| DTRB            | Double-row tapered roller bearing  |
| TRB             | Tapered roller bearing             |
| EHL             | ElastoHydrodynamic lubrication     |
| FEM             | Finite element model               |
| DOF             | Degree of freedom                  |
| CPU             | Central processing unit            |
| RAM             | Random access memory               |
| P               | Uppercase letters<br>Power loss, W |

| Technical Editor: Marcelo A. Savi. |
|------------------------------------|
| Yi Wu<br>wuyi930@163.com           |

<sup>1</sup> Metals and Chemistry Research Institute, China Academy of Railway Sciences Corporation Limited, Beijing 100081, China

| T            | Frictional torque, N m                            |
|--------------|---------------------------------------------------|
| Troller-race | Frictional torque at roller-raceway contacts, N   |
|              | m                                                 |
| Tend-rib     | Frictional torque at roller end-rib contacts, N m |
| Z            | Number of rollers in each row                     |
| Dm           | Roller mean diameter, mm                          |
| Ra           | Radius of the raceway contact points on the       |
|              | roller mean diameter (a=i, o), mm                 |
| L            | Thermal loading parameter                         |
| G            | Dimensionless material                            |
| U            | Dimensionless speed                               |
| E′           | Equivalent Young's modulus, Pa                    |
| Rea          | Equivalent radius at the roller-raceway contact   |
|              | (a=i, o), mm                                      |
| Dpw          | Pitch diameter of the bearing, mm                 |
| Qo           | Contact load at roller-outer ring, N              |
| Qf           | Contact load at roller end-rib, N                 |
|              |                                                   |

#### **Lowercase letters**

ni Rotational speed of bearing, r/min ω<sup>i</sup> Angular velocity of bearing, rad/s

ua Entrainment speed at the roller-race contact (a=i, o),

![](_page_0_Picture_17.jpeg)

State Key Laboratory of Mechanical Behavior and System Safety of Traffic Engineering Structures, Shijiazhuang Tiedao University, Shijiazhuang 050043, China

mm/s

 $\beta_0$  Temperature coefficient of viscosity,  $K^{-1}$ 

 $\eta_0$  Dynamic viscosity, m<sup>2</sup>/s

p<sub>0</sub> Pressure coefficient of viscosity, Pa<sup>-1</sup>

k' Coefficient of thermal conductivity, W (m K<sup>-1</sup>)

i Roller index number

k Slice index number

w Load parameter of slice k

v<sub>A</sub>, v<sub>B</sub> Poisson's ratio

α Nominal contact angle of bearing, °

 $\alpha_0$  Contact angle of roller-outer ring, °

 $\alpha_i$  Contact angle of roller-inner ring,  $\circ$ 

 $\alpha_f$  Contact angle of roller end-rib, °

 $\Delta l$  Width of each slice of roller, mm

h Contact height of roller and rib, mm

 $v_{m}$  Rolling speed of cage, mm/s

v<sub>i</sub> Inner raceway speed, mm/s

 $\mu_{sl}$  Sliding friction coefficient

μ<sub>a</sub> Coulomb friction coefficient

μ<sub>EHL</sub> EHL friction coefficient

 $\sigma_c$  Surface roughness

h<sub>o</sub> Central oil film thickness, mm

h<sub>min</sub> Minimum oil film thickness, mm

#### Subscripts

A Roller
B Outer ring and inner ring
i Inner ring
o Outer ring

f Flange or rib x X-axis direction y Y-axis direction

z Z-axis direction

#### 1 Introduction

Energy consumption in drivetrains of high-speed trains has drawn significant attention from both science and industry [1]. Rolling bearings are the major contributors to power loss and heat generation within these drivetrains [2]. Double-row tapered roller bearings (DTRBs) in high-speed locomotives emerge as particularly vulnerable parts due to their critical role in power transmission. Friction torque and induced power loss will directly affect bearing temperature rise and further reduce the service reliability of the bearings [3]. Thus, accurate prediction of the frictional torque is crucial in guiding bearing iterative design and improving operating performance. In addition, current fault diagnosis methods based on vibration, noise and temperature signals [4–6] can't provide timely feedback on the bearing faulty development in early stages due to the inherent delay of

these signals. In contrast, the friction torque as mechanical signal demonstrates greater sensitivity to minor faults, enabling effective capture of the early bearing failure. Therefore, accurate prediction of the friction torque can also contribute to improving fault diagnosis capabilities.

Theoretical research on bearing frictional torque mainly employs two fundamental methods: global approach and local approach. For studies with the global method, [7] focused only on coulomb friction at sliding contacts. [8] first introduced the effect of lubricant viscosity into an empirical formula of the overall frictional torque. However, the torque formula lacks applicability under conditions of high axial load and high speed. [9] subsequently proposed a semi-empirical formula to predict the overall friction torque of tapered roller bearings (TRBs) under combined radial and axial load. However, as the bearing design evolves, the exponents of load, velocity, and viscosity in the torque formula derived by fitting curves are no longer applicable. Several bearing manufacturers such as NSK LTD [10]. and Timken Company [11] improved the applicability of Palmgren's empirical equations by providing detailed coefficients for different typed rolling bearings. However, the formula is not applicable to thrust ball bearings and some TRBs under light-load and high-speed conditions. Later, the improved Palmgren's empirical equation was adopted by the ISO standard to calculate frictional moments [12]. Overall, the global method can only achieve the calculation of total frictional torque within different types of bearings, which is insufficient for predicting the power loss at various contact areas in bearings.

To solve this issue, the local method has been implemented. [13] proposed a dimensionless torque formula for TRBs based on experimental measurements. It defines that the total frictional torque is composed of ElastoHydrodynamic Lubrication (EHL) rolling frictional torque at rollerraceway contacts, sliding frictional torque at roller end-rib. and internal friction of lubrication films. To reduce the calculation effort of Aihara's model, [14] optimized this model by correlating the load, material, and speed parameters with rolling torque for isothermal EHL line contacts by data curvefitting. Then, [15] further simplified the formula of rolling resistance at roller-raceway contacts of TRBs. Afterwards, [16] from Timken company refined the components of the total friction torque. The TRB running torque was assumed to consist of hydrodynamic rolling forces and elastic rolling moments at roller-raceway contacts and friction moments at roller end-rib regions. SKF Group [17] and Schaeffler Group [18] have also developed their own running torque models of rolling bearings including rolling resistance, sliding friction, churning moment, and seal force. Moreover, the Schaeffler model considers the load distribution inside the bearing, while the SKF model neglects it.

![](_page_1_Picture_34.jpeg)

As for the experimental measurement of friction torque, [[15](#page-18-13)] designed a specialized experimental device capable of independently quantifying the overall friction torque and the torque at the roller end-rib contacts of TRBs with a separate inner ring flange. Complementing this, [[19](#page-18-17)] from Timken evaluated the whole friction torque of TRBs under a combination of radial and axial loads to study the fuel economy of the bearing. [[20,](#page-18-18) [21](#page-18-19)] developed an experimental device to test the friction torque of journal bearings based on equilibrium torque method. [\[22](#page-18-20)] improved the four-ball machine that can achieve the measurement of rolling bearing torque under purely axial loads. Based on this device, [[23](#page-18-21)] measured the total friction torque of cylindrical thrust bearings and TRBs.

In general, whether employing theoretical calculations or experimental tests, existing studies can only determine the overall frictional power loss of the entire bearing or different contact regions. A critical challenge persists in quantifying the localized heat generation rate, especially along roller length. This limitation may induce errors into calculated power loss, which will be transferred into simulated temperature fields by combining with some typical heat transfer and diffusion models [[24](#page-18-22)–[29](#page-18-23)]. Moreover, most works on frictional torque were developed under normal conditions. Accurate prediction remains particularly challenging under abnormal conditions such as polygonal wheels, uneven tracks, or eccentric loading which can induce roller skewing, tilting and uneven contact load distribution. While [\[30](#page-18-24)] and [[31](#page-18-25)] investigated the influence of misalignment on the friction torque of rolling bearings, their analyses were solely limited to load redistribution effects but omitted the significant impacts of roller skewing and tilting, which is insufficient for comprehensively understanding the bearing characteristics. Additionally, the boundary conditions of existing friction torque models relied exclusively on theoretical calculations. Discrepancies between these theoretical values and actual operating parameters may lead to systematic errors. Finally, most research focused on singlerow TRBs, while DTRBs have received less attention due to their complex geometric structure and large load-bearing capacity.

To address these gaps, this paper presented an experiment-assisted frictional power loss model of DTRBs, which not only innovatively integrated roller skewing and tilting effects, but also achieved the determination of localized frictional power loss along roller length. Specialized experiments were employed to provide precise boundary conditions including contact load distribution, roller skew angles, and tilt angles for the frictional torque framework. Furthermore, the power loss and its influence factors such as eccentric displacement, radial load, axial load, and rotating speed were analyzed via the comprehensive model. It facilitates a

quantitative analysis of separate contributions of mechanical parameters to heat generation, thereby offering critical insights into the frictional mechanisms and failure causes of thermally distressed bearings in practical applications.

## **2 Analysis procedure**

In this study, analysis process consists of two parts including experimental measurement and numerical simulation as shown in Fig. [1](#page-3-0). The first step is to determine testing conditions such as eccentric displacement, radial load, axial load, and shaft speed. Then, the testing bearing was instrumented by arranging strain gauges on the outer surface of outer ring. Afterwards, tests were conducted on a comprehensive test rig to investigate the contact load distribution, roller skew angles, and roller tilt angles. The measuring method of contact load has been elaborated in Hou's work [[32](#page-18-26)]. The measurement principle of the roller skew angle and tilt angle are illustrated in previous work [[33\]](#page-18-27) and [[34](#page-18-28)] in detail and briefly introduced in Appendix 1 and Appendix 2 respectively.

Then, the experimental contact mechanical parameters were used to simulate contact pressure distribution at rollerraceway contacts with specialized finite element models (FEMs). Contact forces at roller end-rib can be calculated with the experimental contact load at roller-outer ring based on Harris's model [\[35](#page-18-29)]. Subsequently, the torque and power loss at roller-raceway and roller end-rib interfaces were deduced with the model of power loss, which has been introduced in the next section. Then, the total power loss was compared with that of previous models. Finally, separate contributions of various contact mechanical parameters on power loss were decoupled from the total amounts and sensitivity analysis was conducted.

## **3 Model of power loss**

Power loss in bearings is closely related to frictional torque and rotational speed. The formula can be described as follows:

$$P = T \cdot \omega_i = T \cdot \frac{2\pi n_i}{60} \tag{1}$$

The frictional torque models at different contact areas are presented below.

![](_page_2_Picture_13.jpeg)

<span id="page-3-0"></span>**Fig. 1** Flowchart of the analysis

![](_page_3_Figure_3.jpeg)

<span id="page-3-1"></span>**Fig. 2** Schematic diagram of **a** line contact at roller-races, and **b** point contact at roller end-rib

![](_page_3_Picture_5.jpeg)

## **3.1 Frictional torque at roller-raceway contacts**

Several torque models [[8,](#page-18-6) [9](#page-18-7), [13,](#page-18-11) [14](#page-18-12), [16](#page-18-14)] proposed previously are applicable only for the axially loaded TRBs, in which the load distribution across all rollers and along the roller-raceway contact length is uniform. However, under combined conditions of radial and axial loading or even abnormal conditions, the number of loaded roller and the roller motion postures will change, leading to non-uniform load distribution among rollers and along the roller-raceway contact length. Moreover, the actual roller modified profile also can make an uneven load distribution along their lengths. Therefore, to accurately estimate the frictional torque *Troller−race* at roller-raceway contacts, the existing model was improved by introducing the slicing technique in which the actual roller length was divided into a finite number of laminas *ns*, as shown in Fig. [2a](#page-3-1). The total torque is calculated by separate estimation of individual roller and that of each contact slice. Specific formula is as follows [[13](#page-18-11)]:

![](_page_3_Picture_9.jpeg)

$$T_{roller-race} = \sum_{row=1}^{2} \sum_{j=1}^{Z} \left[ \frac{1}{D_m} \left( R_o T_i + R_i T_o \right) \right]$$
 (2)

$$T_a = \left(\sum_{k=1}^{n_s} T_k\right) \tag{3}$$

$$T_k = \left(\frac{1.76 \times 10^2}{1 + 0.29L^{0.78}}\right) \frac{1}{p_0} (GU)^{0.658} w^{0.31} R_{ea}^2 \Delta l \tag{4}$$

where thermal loading parameter L, dimensionless material parameter G, and speed parameter U serve as the physical quantities for characterizing the EHL mechanisms. In addition, w represents the dimensionless load parameter of each slice. Their formulas are written as follows:

<span id="page-4-0"></span>
$$L = \frac{\eta_0 \beta u_a^2}{k'} \tag{5}$$

$$G = p_0 E' \tag{6}$$

<span id="page-4-1"></span>
$$U = \frac{\eta_0 u_a}{E' R_{aa}} \tag{7}$$

<span id="page-4-3"></span>
$$w = \frac{q_k}{R_{eq}\Delta lE'} \tag{8}$$

where  $p_0$  and  $\beta$  are the pressure and temperature coefficient of viscosity respectively. k' and  $\eta_0$  are the coefficient of thermal conductivity and dynamic viscosity respectively. These parameters are significantly affected by temperature fluctuations. However, this study focused on isothermal conditions. According to the operational data in actual service, the temperature of lubrication grease in DTRB can reach an average of  $100^{\circ}$ C under thermal equilibrium state. Therefore, the parameters of grease FAG L055 at  $100^{\circ}$ C [36–39] were selected to compute steady-state frictional torque, as shown in Table 1. E' represents the equivalent Young's modulus.  $R_{ea}$  (a = i, o) is the equivalent radius at the roller-raceway contact. Their formulas are written as follows.

$$E' = \frac{2}{\left(\frac{1 - v_A^2}{E_A} + \frac{1 - v_B^2}{E_B}\right)} \tag{9}$$

<span id="page-4-4"></span>**Table 1** Parameters of lubricating grease of FAG L055 (100°C)

| Performance parameters               | Units              | Values                 |
|--------------------------------------|--------------------|------------------------|
| Density                              | Kg m <sup>-3</sup> | 870                    |
| Pressure coefficient of viscosity    | $Pa^{-1}$          | $1.136 \times 10^{-8}$ |
| Temperature coefficient of viscosity | $K^{-1}$           | 0.04666                |
| Coefficient of thermal conductivity  | $W (m K^{-1})$     | 0.147                  |
| Dynamic viscosity                    | m <sup>2</sup> /s  | $12.5 \times 10^{-6}$  |
|                                      |                    |                        |

(2) 
$$\frac{1}{R_{ea}} = \frac{1}{R_m} + \frac{1}{R_a}$$

where  $E_{A, B}$  = elastic modulus of the contact components A and B;  $v_{A, B}$  = poisson's ratio of A and B;  $R_m$  = roller mean radius;  $R_n$  (a = i, o) = the radius of raceways.

The entrainment speed  $u_a(a=i, o)$  at the roller-race contacts in Eqs. (5) and (7) can be calculated as below.

<span id="page-4-2"></span>
$$u_0 = \frac{1}{2}v_m \tag{11}$$

$$u_i = \frac{1}{2} \left( v_m + v_i \right) \tag{12}$$

$$v_m = \frac{1}{2}\omega_m D_{pw} \tag{13}$$

$$v_i = \frac{1}{2}\omega_i \left(D_{pw} - D_m \cos \alpha\right) \tag{14}$$

where  $D_{pw}$  = bearing pitch diameter;  $D_m$  = mean diameter of roller;  $\alpha$  = nominal contact angle;  $v_m$  and  $\omega_m$  = rolling speed and angular velocity of cage;  $v_i$  and  $\omega_i$  = rolling speed and angular velocity of inner ring.

To accurately calculate the friction torque at each slice of roller and raceways in Eq. (4), the dimensionless load parameter w should be determined in advance. The parameter can be derived by simulating contact pressure  $q_k$  in Eq. (8) through FEMs, which consist of an instrumented housing integrated with a single roller and either the outer or inner ring as illustrated in Fig. 3.

The FEMs were meshed using hexahedral elements (C3D8R type) and featured 11,432,080 elements and 12,148,453 nodes. The outer surface of the housing was fixed along XYZ-axis, and the outer surface of the outer ring was tied to the inner surface of the housing. The roller was only constrained in the T-axis degree of freedom (DOF) through three rows of the nodes in the center of the roller in the cylindrical coordinate system. The inner surface of the inner ring is fully constrained in the roller-inner ring model. As shown in Fig. 3a, b, the specialized configuration enables experimentally parameters including skew, tilt angles and contact loads to be directly applied on the roller through reference points  $RP_1$  and  $RP_2$ , which were established by coupling half of rolling surface of the roller.

Hard contact was used for normal contact interaction at roller-raceway contacts, while an isotropic Coulomb friction formulation was adopted for tangential contact interaction. The friction coefficient was chosen to be 0.01 for both contact interfaces. Considering separation and rotation might occur, the finite sliding formulation was used for these two contact pairs. The governing equations were

![](_page_4_Picture_25.jpeg)

<span id="page-5-0"></span>**Fig. 3** FEMs of **a** roller and outer ring, and **b** roller and inner ring

![](_page_5_Picture_3.jpeg)

constructed based on the principles of continuous static equilibrium. To ensure mesh convergence and improve calculation accuracy, the grids at contact regions were refined to 0.5 mm in axial direction and 0.2 mm in circumferential direction. Moreover, a smooth step curve was employed to gradually increase the applied loads towards FEMs. The simulation utilizes an adaptive time step with an initial time increment of 0.01s and maximum loading time of 0.1s. The initial stiffness scale factor for contact constraints was set to 2.0 to mitigate potential convergence issues stemming from insufficient contact stiffness.

## **3.2 Frictional torque at roller end-rib interfaces**

There is sliding friction at the roller end-rib contact interface as shown in Fig. [2](#page-3-1)b. The torque can be deduced as follows.

$$T_{end-rib} = \sum_{row=1}^{2} \sum_{j=1}^{Z} \mu_{sl} Q_f h \sqrt{1 + 0.18 \left(\frac{a_e}{h}\right)^2} \frac{R_o}{D_m}$$
 (15)

where *h*=height of roller end and rib contact. *αe*=semiwidth of the roller end-flange contact. The contact load *Qf* at roller end-rib can be calculated from the experiment normal contact load *Qo* between the roller and the outer ring.

$$Q_f = Q_0 \frac{\sin \alpha_0 - \tan \alpha_i \cos \alpha_f}{\sin \alpha_f + \tan \alpha_i \cos \alpha_f}$$
 (16)

Parameters *αo*, *αi* , and *α<sup>f</sup>* represent the contact angle of outer ring, inner ring and flange respectively. The sliding friction coefficient at roller end-rib *µsl* is composed of Coulomb friction and EHL rolling friction.

$$\mu_{sl} = \phi_c \mu_c + (1 - \phi_c) \,\mu_{EHL} \tag{17}$$

where EHL friction coefficient *µEHL* **=** 0.002. The weight factor *Φc* can refer to SKF catalogue [[17\]](#page-18-15). The coulomb friction coefficient *µc* is related to the roughness *σc* of the contact areas and central film thickness *ho*.

$$\mu_c = \mu_0 \times \exp\left(-1.8 \left(\frac{h_o}{\sigma_c}\right)^{1.2}\right) \tag{18}$$

where *µ0* **=** 0.2. The surface roughness *σc* is set to 0.418 μm [\[40](#page-19-1)].

The central film thickness *ho* can be calculated by the minimum film thickness *hmin* [[13\]](#page-18-11).

$$h_0 = 1.25 \times h_{\min} \tag{19}$$

$$h_{\min} = 3.63U^{0.68}G^{0.49}W^{-0.073}\left(1 - e^{-0.68k_{end-rib}}\right)R_y \qquad (20)$$

where *kend−rib*=1.03(*Rz*/*Ry*) 0.64; *Ry*=equivalent radius in the rolling direction; *Rz*=equivalent radius perpendicular to rolling direction.

## **4 Experiment**

## **4.1 Instrumented test bearing and axle box**

The experiment utilized a DTRB of type FAG807811.09 as the test bearing, with detailed specifications provided in Table [2.](#page-6-0) As shown in Fig. [4](#page-6-1)a, the outer surface of the outer ring was evenly partitioned into seven regions corresponding to seven adjacent rollers indexed from 3 L to 3R, defining as the potential loaded zones. Strain gauges were circumferentially glued at the center of each loaded region along the roller length. Among them, strain gauges 1 and 4 correspond to the roller large end and small end, while strain gauges 2 and 3 serve as the tilt-insensitive load

![](_page_5_Picture_22.jpeg)

<span id="page-6-0"></span>**Table 2** Specification of the DTRB used in the experiment

| Main parameters                     | Units | Values |
|-------------------------------------|-------|--------|
| Contact angle of inner race αi      | °     | 9      |
| Contact angle of outer race αo      | °     | 12     |
| Contact angle of flange αf          | °     | 79.5   |
| Nominal contact angle α             | °     | 10.5   |
| Number of rollers per row Z         |       | 17     |
| Number of rows k                    |       | 2      |
| Roller effective contact length Lwe | mm    | 50     |
| Bearing pitch diameter Dpw          | mm    | 185    |
| Mean diameter of the roller Dm      | mm    | 26.7   |

measuring points for clockwise and counterclockwise tilting determined in calibration, corresponding to measuring points 3 and 5 respectively in [[34\]](#page-18-28). The specifications of the strain gauge are listed in Table [3.](#page-6-2) Full-bridge connections of strain gauges were used to eliminate the effects of temperature variations. In addition, the strain gauges were sealed with a 704 silicone rubber to mitigate minor deformations and rapid temperature fluctuations induced by airflow. Two temperature sensors were pasted at the most loaded position of the two rows of the outer ring to monitor the bearing temperature.

To accommodate the strain gauge arrays, the axle box was modified by machining a series of axial notches with a depth of 4 mm and a central angle of 6° at the inner surface, as presented in Fig. [4b](#page-6-1). During assembly, each axial notch corresponded to a loaded roller position, and the center lines of strain gauges were aligned with those of the notches. The instrumented axle box consists of two parts. The top part contains 7 notches corresponding to seven possible loaded positions, while the bottom part has 3 notches spaced 60°

<span id="page-6-1"></span>**Fig. 4** Photo of **a** the location of strain gauge arrays at different loaded positions and temperature sensors at the most loaded position, and **b** the instrumented

axle box

![](_page_6_Picture_8.jpeg)

![](_page_6_Picture_9.jpeg)

<span id="page-6-2"></span>**Table 3** Specifications of the strain gauge

| Sensors      | Main parameters                 | Values                |
|--------------|---------------------------------|-----------------------|
| Strain gauge | Type                            | KFG-1-120-C1-1 1L3M2R |
|              | Gauge factor(24 ℃, 50%RH)       | 2.13±1.0%             |
|              | Gauge length                    | 1 mm                  |
|              | Gauge resistance (24 °C, 50%RH) | 120.4±0.4 Ω           |
|              | Adoptable thermal expansion     | 11.7 PPM/°C           |

apart from adjacent notches. These two parts were secured together with bolts to form a complete device.

To create the roller skewing and tilting motions, eccentric loading condition was applied to the test bench. A deep groove with a depth of 12 mm was machined into the upper surface of the top axle box and several crescent-shaped positioning keys were fabricated. By positioning different keys at designated locations within the groove as presented in Fig. [5](#page-7-0), the loading unit indicated by the red circle can be accurately positioned, thereby enabling concentric loading and eccentric loading with distinct eccentric displacements of 10 mm, 20 mm, 30 mm, and 40 mm.

## **4.2 Experimental system and conditions**

The experiment was performed on a dedicated test rig, whose general structure is illustrated in Fig. [6.](#page-7-1) Figure [7](#page-8-0) shows the photographic view of the experimental system. The instrumented bearing and axle box were installed on one side of the shaft, while an identical DTRB without sensors was mounted on the opposite side. Two support bearings were situated in the middle region of the shaft, which was driven by a motor through a belt pulley.

During operation, radial actuators were used to apply radial load along the Z-direction to the fixed outer rings of DTRBs. Moreover, Axial loading needs to simulate the curve conditions in actual service, where the outboard row of the bearing sustains most of the axial load. Since the axial loads can only be imposed on the outer rings of DTRBs on the test rig, axial tensile loads were adopted with axial actuators for the testing. In addition, surface burning

<span id="page-7-0"></span>**Fig. 5** Isometric view of instrumented axle box and positioning scheme of eccentric radial

![](_page_7_Figure_3.jpeg)

<span id="page-7-1"></span>**Fig. 6** The general structure of the test rig

![](_page_7_Picture_5.jpeg)

![](_page_8_Picture_2.jpeg)

<span id="page-8-0"></span>![](_page_8_Picture_3.jpeg)

**Fig. 7** The test rig of DTRB

<span id="page-8-2"></span>**Table 4** Experimental conditions of the dynamic testing for strain distribution

| Eccentric displacement | Radial load (kN) | Axial load (kN) | Shaft |
|------------------------|------------------|-----------------|-------|
| (mm)                   |                  |                 | speed |
|                        |                  |                 | (rpm) |
| 10                     | 70               | 12              | 500   |
| 20                     | 80               | 15              | 1000  |
| 30                     | 90               | 18              | 1500  |
| 40                     |                  |                 | 2000  |

and raceway spalling were observed more frequently on the outboard row of the bearings than on the inboard row in real service indicating that the outside row is more susceptible to eccentric loading. Hence, this study specifically investigated outward-directed eccentric displacements along the axle centerline.

The experimental conditions are detailed in Table [4](#page-8-2). Initially, eccentric displacements were applied on the test bearing increasing from 10 mm to 40 mm with an increment of 10 mm by adjusting the axial position of the loading frame. According to the actual loading state of the test bearing served in CRH380B high-speed train, three radial loads of 70 kN, 80 kN and 90 kN along with three tensile axial loads of 12 kN, 15 kN, and 18 kN were selected and sequentially applied to the bearing. For each combination of eccentric radial load and axial load, the rotational speeds progressively varied from 500 rpm to 2000 rpm (corresponding to the train speed of 320 km/h) in 500 rpm interval to cover different speed levels. A relative wind speed of 8 m/s-10 m/s provided by fans sustain cools the axle box until reaching thermal equilibrium [[41\]](#page-19-2). A DH5922D data acquisition equipment from DONGHUA Company in China was adopted to collect strain and temperature signals. The acquisition equipment was grounded to eliminate interference from power-frequency signals. The sampling frequency was set to 50 kHz to ensure accurate acquisition of peak and frequency information of the strain signals. The subsequent

<span id="page-8-1"></span>**Table 5** The ratio of standard deviation to mean value

| Parameter                 | 2 L   | 1 L   | 0     | 1R    | 2R    |
|---------------------------|-------|-------|-------|-------|-------|
| Roller skew angle         | 1.89% | 1.65% | 1.64% | 1.83% | 1.88% |
| Roller tilt angle         | 1.72% | 1.81% | 1.90% | 1.83% | 1.78% |
| Contact load distribution | 1.30% | 1.45% | 1.62% | 1.59% | 1.44% |

experiments at different eccentric displacements, radial and axial loads as well as shaft speeds were conducted with the same procedure.

# **5 Results and discussion**

## **5.1 Experimental contact mechanical parameters**

After obtaining the original strain signals, signal processing methods such as filtering, smoothing, and zero-drift correction were adopted to preprocess these signals and address some anomalous data. Subsequently, the processed strain signal can be converted into the required contact mechanical parameters. To evaluate measurement repeatability quantitatively, five independent tests were repeatedly conducted under typical conditions of eccentric displacement 40 mm, radial load 80 kN, axial load 15 kN and shaft speed 2000 rpm. Table [5](#page-8-1) summarizes the ratio of the standard deviation to the total mean value for each mechanical parameter at mainly loaded positions of the loading row in five repeated tests. It was found that the percentages of various parameters at these key positions were within 2%, which further indicated the high measurement consistency and repeatability with the experimental setup and measurement methods in this study.

Under combined radial and axial loads, contact loads at roller-outer ring interfaces of two rows exhibited distinct patterns with increasing eccentric displacements, as shown in Fig. [8](#page-9-0)a. For the loading row (Row 1), the contact loads at various loaded positions keep increasing, while the unloading row (Row 2) shows a reduced trend. Even under concentric loading conditions, row 1 still carries slightly higher loads than row 2 due to the axial load. Figure [8](#page-9-0)b demonstrates the induced contact loads at roller end-rib calculated with the roller-outer ring contact loads. Furthermore, both rows engage five loaded rollers under the concentric condition. As the eccentric displacement grows, rollers at 3 L and 3R positions of row 1 will participate in loading, leading the loading range expands to seven rollers. In contrast, row 2 maintains five engaged rollers but experiences a gradual load reduction at 2 L and 2R positions. However, further displacement escalation may potentially reduce the loaded rollers to three. This load redistribution mechanism stems from eccentricity-induced moment variations that amplify roller skewing and tilting effects.

![](_page_8_Picture_16.jpeg)

<span id="page-9-0"></span>![](_page_9_Figure_2.jpeg)

**Fig. 8** The experimental load distribution at **a** roller-outer race interface and **b** roller end-rib contact under different eccentric displacements (test condition: eccentric radial load 80 kN, axial load 15 kN and shaft speed 2000 rpm)

<span id="page-9-1"></span>![](_page_9_Figure_4.jpeg)

**Fig. 9** Roller skew angles among rollers of **a** loading row (row 1) and **b** unloading row (row 2) under different eccentric displacements (test condition: eccentric radial load 80 kN, axial load 15 kN and shaft speed 2000 rpm)

Figure [9](#page-9-1) illustrates the experimental roller skew angles and fluctuation ranges at mainly loaded positions under various eccentric displacements. The angles at specific edgeloading positions (3 L, 3R, 4 L, 4R) were excluded from the analysis due to their excessive vibrational noise and negligible contribution to total frictional heat generation. The mean values of each position were derived from averaging the angles of 5 cage rotations. As the eccentric displacement increases, the skewing moments acting on the rollers in both rows intensify, leading to a progressive rise in roller skew angles. Because the increased contact loads can press rollers more tightly as approaching position No.0, the mean skew angle reduces with the fluctuation range narrowing at fixed

displacement conditions. Additionally, the skew angles of the loading row are slightly smaller than those at the same positions of the unloading row, which also attributes to the discrepancy in restricting skew effects from different contact forces of the two rows.

As depicted in Fig. [10](#page-10-0), the roller tilt angle becomes more pronounced near the maximum loaded position No.0 with reduced fluctuations of the tilt angles. Higher contact loads generate stronger tilting moments, resulting in more severe tilting motions. As the eccentric displacement increases, the tilting moments acting on the rollers intensify, leading to a progressive tilt angle growth across all loaded rollers in both rows. Moreover, the tilt angles of the two rows exhibit

![](_page_9_Picture_9.jpeg)

<span id="page-10-0"></span>![](_page_10_Figure_2.jpeg)

**Fig. 10** Roller tilt angle among rollers of **a** loading row (row 1) and **b** unloading row (row 2) under different eccentric displacements (test condition: eccentric radial load 80 kN, axial load 15 kN and shaft speed 2000 rpm)

<span id="page-10-1"></span>![](_page_10_Figure_4.jpeg)

**Fig. 11** Contact pressure *Po* at roller-outer ring interface at position No.0 under different eccentric displacements: **a** loading row (row 1) and **b** unloading row (row 2) (test condition: eccentric radial load 80 kN, axial load 15 kN and shaft speed 2000 rpm)

opposite signs, indicating that the rollers in each row tilt in opposite directions. This phenomenon highlights the impact of asymmetric load distribution along roller length on roller behaviors under eccentric loading conditions.

## **5.2 Total power loss under various conditions**

To avoid severe edge loading, the roller profile was modified using a fully logarithmic curve to ensure a smooth contact pressure distribution across the roller-raceway interfaces [[42](#page-19-3)]. Under normal operating conditions with 0 mm eccentric displacement, the Hertzian contact pressure distribution along roller length is relatively symmetric as depicted in

Fig. [11.](#page-10-1) As the eccentric displacements increase, roller tilt angles of both rows increase together with opposite tilting directions, as mentioned above. The clockwise roller tilting in the loading row intensifies the Hertzian contact pressure near the roller large end-outer ring contact region, with the pressure peak gradually shifting toward the roller large end in Fig. [11](#page-10-1)a. Meanwhile, the unloading row exhibits an opposite trend. The reduced Hertzian pressure magnitude gradually migrates toward the roller small end with counterclockwise roller tilting, as shown in Fig. [11](#page-10-1)b. However, if the displacement continues to increase until exceeding a critical threshold, a sharp pressure drop will occur at the roller ends due to stress concentration. Furthermore, the pressure

![](_page_10_Picture_10.jpeg)

increment in the loading row significantly outweighs the pressure reduction in the unloading row as shown in Fig. [11,](#page-10-1) which was primarily caused by the contact load redistribution. The cumulative effect of growing contact loads and clockwise roller tilting in the loading row imposes substantially greater impacts than the counteractive effect between decreasing contact loads and counterclockwise tilting of unloading row. As shown in Fig. [12](#page-11-0), the power loss distribution along the roller length essentially follows an identical pattern as the contact pressure distribution under various eccentric displacements due to the positive relationship as depicted in the formulas (1) and (4).

Figure [13](#page-12-0) illustrates the impact of eccentric displacement, radial load, axial load, and rotating speed on total power loss across different contact areas in the DTRB. As depicted in Fig. [13](#page-12-0)a, the two rows of rollers exhibit different power loss characteristics with the eccentric displacement increase from 10 mm to 40 mm. The roller-raceway contacts of the loading row demonstrate marked power loss escalation accompanied by slight power increase at the roller-rib contacts due to different contact characteristics. In contrast, the unloading row exhibits an inverse power dissipation behavior within the eccentric displacement range. Figure [13b](#page-12-0) demonstrates that the increasing eccentric radial load ranging from 70 kN to 90 kN elevates power dissipation across all contact regions in DTRB, with roller-inner ring contacts exhibiting greater frictional power loss compared to the roller-outer ring interfaces, while the roller-rib contacts experience much lower power loss levels than the other two regions. In addition, the power loss characteristics of the two rows also reveal a distinct pattern with increasing axial force from 12 kN to 18 kN, as shown in Fig. [13](#page-12-0)c, where compressed loads on row 1 induce the power loss accumulation while row 2 experiences decreased power dissipation. Furthermore, the total power loss at various contacts shows a positive relationship with rotating speed ranging from 500 rpm to 2000 rpm in Fig. [13](#page-12-0)d, which is identical to the effect of eccentric radial load. This parametric analysis confirms that the interfacial energy dissipation mechanisms in DTRBs are predominantly governed by the interactions between mechanical loading parameters and kinematic conditions.

For the computationalspeed of the comprehensive model, five cases were established to compare central processing unit (CPU) time, as summarized in Table [6.](#page-12-1) The numerical simulation consists of two main components: FEM simulation for contact pressure distribution in ABAQUS software and theoretical calculation for frictional power loss on MAT-LAB R2021a. A desktop computer consisting of an Intel Core i7-7700 3.60 GHz CPU and a 32 GB random access memory (RAM) was used in this work. Analysis reveals that the CPU time correlates strongly with the number of operating conditions, whereas shows minimal dependence on the specific scale of each individual condition. Significantly, even for the complex load cases indexed 5 in Table [6](#page-12-1), the CPU time remains below two minutes, which represents the high computational efficiency of the comprehensive model.

To validate the calculation accuracy of the experimentalassisted friction power loss model, comparative analysis were performed using current model alongside the torque formulas developed by Palmgren [[8](#page-18-6)], Houpert [\[16](#page-18-14)], and Aihara [\[13](#page-18-11)], which only consider the influence of contact load on frictional torque. As illustrated in Fig. [14](#page-13-0), it is evident that Palmgren's and Houpert's formulas are likely to underestimate the DTRB power loss, whereas Aihara's model yields results that are closest to those of the current

<span id="page-11-0"></span>![](_page_11_Figure_7.jpeg)

**Fig. 12** Frictional power loss at roller-outer ring interface at position No.0 under different eccentric displacements: **a** loading row (row 1) and **b** unloading row (row 2) (test condition: eccentric radial load 80 kN, axial load 15 kN and shaft speed 2000 rpm)

![](_page_11_Picture_9.jpeg)

<span id="page-12-0"></span>![](_page_12_Figure_2.jpeg)

**Fig. 13** The influence of **a** eccentric displacement, **b** eccentric radial load, **c** axial load, and **d** shaft speed on the overall friction power loss at various contacts

<span id="page-12-1"></span>**Table 6** CPU time for numerical simulation

| Case | Eccentric dis  | Radial    | Axial | Shaft | CPU     |
|------|----------------|-----------|-------|-------|---------|
|      | placement (mm) | load (kN) | load  | speed | time(s) |
|      |                |           | (kN)  | (rpm) |         |
| 1    | 0              | 10        | 0     | 500   | 72.36   |
| 2    | 0              | 80        | 0     | 500   | 74.58   |
| 3    | 0              | 80        | 0     | 2000  | 76.96   |
| 4    | 0              | 80        | 15    | 2000  | 86.94   |
| 5    | 40             | 80        | 15    | 2000  | 100.86  |

study due to the shared conceptual approach. Nevertheless, a divergence persists between Aihara's outcomes and the results of present study. The discrepancy is mainly caused by incorporating the combined effects of roller skewing, tilting as well as the contact load distribution in current study. In addition, the input mechanical parameters of this study were derived from experimental measurements, as illustrated in

the flowchart in Fig. [1](#page-3-0), thereby providing values closer to actual operating conditions. In contrast, the contact load distribution used for calculating frictional torques of previous models was typically derived based on Quasi-static model. The differences between experimental and simulated input values may also contribute to the gaps.

# **5.3 Separate power loss of roller skewing and tilting**

As noted above, the total power loss is deduced from the superposition effect of multiple mechanical parameters including roller skew angles, roller tilt angles, and contact load distribution. Roller skewing and tilting are coupling motions accompanied by uneven contact load distribution. To evaluate separate contributions, the effects of different mechanics parameters on the frictional power loss were

![](_page_12_Picture_10.jpeg)

<span id="page-13-0"></span>![](_page_13_Figure_2.jpeg)

**Fig. 14** Comparison of total power loss using the current method and three other formulas (test condition: eccentric displacement 40 mm, radial load 80 kN, axial load 15 kN)

decoupled from the total amount with the comprehensive model. The independent contribution ratios were analyzed by gradually superposing the mechanical parameters and comparing the differences in increments. Four sequentially stacked cases were organized for single contact load distribution, a combination of contact load and roller skew, a combination of contact load and roller tilt, and the total effect of the three parameters.

Figure [15](#page-13-1) illustrates the contact pressure distribution at the roller-outer ring interfaces at position No.0 under the four sequentially stacked cases. For the case of single contact loads, the contact pressure exhibits symmetric distribution, which is identical to the pattern under concentric condition. The introduction of roller skew based on the contact load distribution causes almost even pressure amplification at the contact interfaces. Moreover, incorporating roller tilt with contact load distribution results in a greater but uneven increment in the contact pressure across the two rows compared to that from roller skewing. The axial position of pressure amplitude in the loading row shifts toward the roller large end, while that in the unloading row moves toward the roller small end. The comparison confirms that roller tilt exerts greater influence on the contact pressure than the skewing effect. The total case combining load, skew, and tilt effects replicates the contact pressure profile observed under the load-tilt combined condition, primarily differing in magnitude. As presented in Fig. [16](#page-14-0), the distribution pattern of the friction power loss under the four sequentially stacked cases is equally identical to the contact pressure in Fig. [15](#page-13-1).

Figure [17](#page-14-1)a presents the total power loss at roller-raceway contacts under four sequentially stacked cases at the eccentric displacement of 40 mm. Comparison reveals that the loading row exhibits greater power loss than the unloading row at roller-raceway contacts attributing to the larger loading state. Furthermore, roller-inner ring interactions generate higher power dissipation than the roller-outer ring contacts within the same column. Additionally, it can be concluded from Fig. [17](#page-14-1)a that the contact loads serve as the primary contributor to frictional power loss across interfaces, though the abnormal roller postures play an indispensable role in power loss. Notably, roller tilting demonstrates greater influence on power loss generation than roller skewing. Moreover, the total power loss of the entire DTRB under the four sequentially stacked cases and four eccentric displacements was illustrated in Fig. [17b](#page-14-1), with the data listed in Table [7](#page-14-2). QQuantitative evaluation indicates that the contributions of roller motion postures on total power loss

<span id="page-13-1"></span>![](_page_13_Figure_8.jpeg)

**Fig. 15** Contact pressure *Po* at roller-outer raceway interface at position No.0 under four sequentially stacked cases: **a** loading row (row 1) and **b** unloading row (row 2) (test condition: eccentric displacement 40 mm, eccentric radial load 80 kN, axial load 15 kN and shaft speed 2000 rpm)

![](_page_13_Picture_10.jpeg)

<span id="page-14-0"></span>![](_page_14_Figure_2.jpeg)

**Fig. 16** Frictional power loss at roller-outer raceway interface at position No.0 under four sequentially stacked cases: **a** loading row (row 1) and **b** unloading row (row 2) (test condition: eccentric displacement 40 mm, eccentric radial load 80 kN, axial load 15 kN and shaft speed 2000 rpm)

<span id="page-14-1"></span>![](_page_14_Figure_4.jpeg)

**Fig. 17 a** Frictional power loss at the roller-outer race and roller-inner race contacts, and **b** total power loss under four sequentially stacked cases (test condition: eccentric displacement 40 mm, eccentric radial load 80 kN, axial load 15 kN and shaft speed 2000 rpm)

<span id="page-14-2"></span>**Table 7** Power loss of DTRB under four sequentially stacked cases in various eccentric displacements

| Eccentric displacement | Stacked case |           |           |        |                                                  |                                               |  |  |
|------------------------|--------------|-----------|-----------|--------|--------------------------------------------------|-----------------------------------------------|--|--|
|                        | Load         | Load+Skew | Load+Tilt | Total  | Contribution of roller<br>motions on total power | Ratio of<br>tilt to skew<br>on total<br>power |  |  |
| 0 mm                   | 1641.7       | 1700.5    | 1797.9    | 1880.5 | 11.4%                                            | 2.6                                           |  |  |
| 10 mm                  | 1660.9       | 1740.5    | 1861.2    | 1935.5 | 14.5%                                            | 2.5                                           |  |  |
| 20 mm                  | 1682.1       | 1773.1    | 1920.8    | 2000.4 | 16.5%                                            | 2.6                                           |  |  |
| 30 mm                  | 1703.9       | 1814.2    | 1977.6    | 2070.5 | 18.5%                                            | 2.6                                           |  |  |
| 40 mm                  | 1721.8       | 1851.8    | 2042.3    | 2150.8 | 20.9%                                            | 2.5                                           |  |  |

![](_page_14_Picture_8.jpeg)

escalate from 11% to 21% approximately with the increased eccentric displacements varying from 0 to 40 mm. Specifically, roller tilting accounts for about 2.5 times more power loss than roller skewing throughout the displacement range. These findings underscore the necessity of incorporating roller motion postures effects in bearing frictional power loss model. The enhanced predictive capability of the current model highlights its significance for precision engineering applications requiring accurate power loss estimation in complex loading scenarios.

To evaluate the sensitivity of total power loss to the changes of roller skew and tilt angles, functional relationships were established between these roller motions and their induced power loss. Within the eccentric displacement ranging from 0 to 40 mm, the roller angles at the most loaded position in the loading row were selected as independent variables, while their respective induced power loss were taken as dependent variables for sensitivity analysis. As depicted in Fig. [18](#page-15-0), the frictional power loss under different roller skew and tilt angles all exhibits an almost exponential growth pattern. Moreover, the growth rates of the power loss attributed to both skew and tilt progressively accelerate with increasing roller angles, reaching the peak at angles corresponding to the maximum eccentric displacement of 40 mm within the testing conditions. This phenomenon originates from exacerbated transverse sliding friction at roller-raceway interfaces and intensified tangential sliding friction at roller end-flange contacts under increasing skew angles, thereby elevating thermal generation rates. Concurrently, greater tilt angles induce unilateral stress concentration at roller-raceway interfaces, triggering rapid power loss escalation. Consequently, the increasing growth rate indicates that greater skew angles and tilt angles produce higher sensitivity on the power loss. This evidence further substantiates that skew and tilt effects play a critical and non-negligible role in the analysis of total frictional power loss in roller bearings.

## **6 Conclusions and future works**

This study developed an experiment-assisted frictional power loss model of DTRB incorporating the effect of roller skewing and tilting. Specialized experiment was implemented to measure the roller skew angles, roller tilt angles, and contact load distribution, which furtherserve asthe input of numerical models for frictional power loss. The impact of eccentric displacement, radial load, axial load, and shaft speed on the total power loss was examined. Moreover, the separate effects of three contact mechanical parameters on power loss were decoupled from the total values with the comprehensive model to explore their respective effects on the frictional power loss. Finally, sensitivity analysis of roller postures on total power loss was conducted. The associated conclusions are as follows:

- 1. Roller skew angles and tilt angles increase with the eccentric displacements. Mean skew angles decline and tilt angles grow with both fluctuation ranges narrowing as approaching the most loaded position. Moreover, skewing motions of both rows share common orientations, while the tilting directions are totally opposite.
- 2. Radial load and shaft speed universally elevate power loss at both roller-raceway and roller-rib interfaces of both rows, whereas eccentric displacement and axial force induce opposite effects of the two rows, with the

<span id="page-15-0"></span>![](_page_15_Figure_9.jpeg)

**Fig. 18** Frictional power loss caused by roller skewing (**a**) and tilting (**b**) (test condition: eccentric radial load 80 kN, axial load 15 kN and shaft speed 2000 rpm)

![](_page_15_Picture_11.jpeg)

increased power loss of the loading row and deducted power loss of unloading row.

- 3. The calculated power loss by the proposed model is much larger than those from the existing models primarily due to the incorporation of the combined effects of roller skewing, tilting as well as the contact load distribution.
- 4. Though contact loads serve as the primary contributor to the total frictional power loss, the separate contribution ratio of abnormal roller motion postures on the total power loss rises from 11% to 21% approximately as the eccentric displacement increases from 0 to 40 mm. Specifically, roller tilting effects on the power dissipation dominate skew contributions by about 2.5 times. The separate power loss induced by skewing and tilting presents a higher sensitivity at larger roller skew angles and tilt angles.

In the future, the current model will be upgraded to transient frictional power loss model by incorporating thermal effects and lubrication behavior under variable temperature. Then, the enhanced model can be applied to calculate the heat generation rate at different operating stages and improve model accuracy. Moreover, it should be developing experimental ability to measure the friction torque especially for the combined eccentric radial and axial loading conditions or collecting the operational and maintenance data in real-world or industrial scale to complete the model verification from both experimental and on-site perspectives.

## **Appendix**

# **Appendix 1: measurement method for roller skew angle**

The method is based on strain detection and has been verified to be with good accuracy [[33](#page-18-27)]. The measurement

<span id="page-16-0"></span>**Fig. 19** Schematic diagram of **a** the relative position of strain gauges and skewed roller and **b** the strain gauges showing temporally separated response for a skewed roller

method is illustrated in Ref [[33](#page-18-27)]. in detail and is briefly introduced here.

Test bearing and its axle box were instrumented as shown in Fig. [4.](#page-6-1) The specific pasting scheme of strain gauges has been detailly illustrated in Sect. 4.1. To achieve nondestructive measurement of roller skew angles in real-world applications, a series of axial notches are introduced into the inner surface of the axle box as shown in Fig. [4](#page-6-1)b, which provides enough space to the strain gauges and makes it possible to detect the strain signal of the bearing outer ring. With the existence of the notches, the corresponding part of the outer ring can be considered as a beam structure. According to beam theory, whenever there is a roller rolling over the notch position, several strain pulses of the strain gauge array can be produced at the outer surface due to slight deformation of the outer ring. Besides, it has been validated by finite element analysis and experiment that the introduction of the notches does not affect the roller-raceway load within the bearing [[43](#page-19-4), [44](#page-19-5)].

The contact line between skewed roller and raceway causes temporally separate strain responses as shown in Fig. [19.](#page-16-0) The skew angle *θskew* can be determined with the peak time difference ∆*t*.

$$\theta_{skew} = \arcsin\left[\frac{\omega_c \Delta t}{2l} \left(R_1 + R_2\right)\right]$$
 (21)

where *ωc* is cage angular speed, *R1* and *R2* are the raceway radii of the outer ring at the two strain gauges, and *l* is the distance of the two strain gauges. The cage angular speed can be calculated with the roller passing frequency *fr* .

$$\omega_c = \frac{2\pi f_r}{Z} \tag{22}$$

![](_page_16_Picture_17.jpeg)

## **Appendix 2: measurement method for roller Tilt angle**

For the measurement of roller tilt angles in actual applications, only a brief description is provided here. The detailed testing method and calibration results are introduced in [[34](#page-18-28)].

For a normal roller, there exists a strain amplitude difference *εL-εS* between two strain gauges at roller ends due to the tapered shape of outer ring as shown in Fig. [20](#page-17-0). When the roller is tilted, the strain amplitude difference changes to *εL−θtilt-εS−θtilt*. The variation of the strain amplitude difference between the two strain gauges is not only related to the roller tilt angle but also to the contact load *Fβ*. To calculate the roller tilt angle distribution, the contact load *Fβ* needs to be determined in advance. The measured strain signals *εα* and the contact load *Fβ* can be related with a compliance matrix *kα−β*:

<span id="page-17-0"></span>**Fig. 20** Schematic diagram of **a** the detail of roller tilting and **b** strain response of strain gauge 1 and 2 of the tilted and normal roller

Based on the calculated contact load distribution *Fβ*, the strain signals at both ends of roller *β* under non-tilted condition can be deduced as

$$\varepsilon_{\beta-L-0} = k_L \cdot F_{\beta} \tag{25}$$

$$\varepsilon_{\beta-S-0} = k_S \cdot F_{\beta} \tag{26}$$

Afterwards, the slope *kβ−Fβ* of the fitting line at roller position *β* can be calculated using the calculated load *Fβ* and fitting polynomial. Once the slope is obtained, the roller tilt angle *θtilt* at roller position *β* can be finally determined with the variation of strain amplitude difference at both roller ends.

$$\theta_{tilt} = k_{\beta - F_{\beta}}^{-1} \left[ (\varepsilon_{\beta - L - \theta_{tilt}} - \varepsilon_{\beta - L - 0}) - (\varepsilon_{\beta - S - \theta_{tilt}} - \varepsilon_{\beta - S - 0}) \right]$$
 (27)

![](_page_17_Picture_11.jpeg)

$$\varepsilon_{\alpha} = k_{\alpha - \beta} \cdot F_{\beta} \tag{23}$$

where the subscripts *α* and *β* represent the sensing location number (1, 2, …, *Z*). *Z* is equal to the number of loaded rollers. *kα−β* represents the strain at the sensing location *α* due to a unit force applied at sensing location *β* when all other loads are zero. Once a compliance matrix *kα−β* is obtained and the strain distribution *εα* is measured, the contact load distribution *Fβ* within the bearing can be determined by

$$F_{\beta} = k_{\alpha - \beta}^{-1} \cdot \varepsilon_{\alpha} \tag{24}$$

The calibration method and compliance matrix *kα−β* are described in detail in [[32,](#page-18-26) [34](#page-18-28)]. The strain-load transfer coefficients of roller large end *kL* and roller small end *kS* can also be determined with the similar method. Afterwards, the linear fitting relationship between the variation of the strain amplitude difference and the roller tilt angle *θtilt* can be established under different calibration loads. Then, the relationship between the slopes *kP* of the above fitting lines and calibration loads can be fitted using quadratic polynomial.

**Acknowledgements** The author appreciates the financial support from the National Science and Technology Major Project (Grant No. 2025ZD0610801), the Project of Scientific Research and Development of China Railway Corporation (Grant No.J2024J004), and the Foundation of China Academy of Railway Sciences (Grant No. 2025YJ006).

**Funding** This article is funded by National Key Research and Development Program Project (2025ZD0610801), Project of Scientific Research and Development of China Railway Corporation (J2024J004), Foundation of China Academy of Railway Sciences (2025YJ006).

**Data availability** The datasets generated and supporting the findings of this article are obtainable from the corresponding author upon reasonable request.

## **Declarations**

**Conflict of interest** The author declares that there are no known competing financial interests or personal relationships that could have appeared to influence the work reported in this paper.

![](_page_17_Picture_21.jpeg)

# **References**

- <span id="page-18-0"></span>1. Ta W, Qiu S, Wang Y, Yuan J, Gao Y, Zhou Y (2021) Volumetric contact theory to electrical contact between random rough surfaces. Tribol Int 160:107007. [https://doi.org/10.1016/j.triboint.20](https://doi.org/10.1016/j.triboint.2021.107007) [21.107007](https://doi.org/10.1016/j.triboint.2021.107007)
- <span id="page-18-1"></span>2. Matsuyama H, Dodoro H, Ogino K, Ohshima H, Toda K (2004) Development of Super-Low friction torque tapered roller bearing for improved fuel Efficiency, SAE technical paper series no. 2004-01-2674
- <span id="page-18-2"></span>3. Biboulet N, Houpert L (2010) Hydrodynamic force and moment in pure rolling lubricated contacts. Part 1: line contacts. Proc Institution Mech Eng Part J: J Eng Tribology 224:765–775. [https](https://doi.org/10.1243/13506501jet790) [://doi.org/10.1243/13506501jet790](https://doi.org/10.1243/13506501jet790)
- <span id="page-18-3"></span>4. Rohani Bastami A, Aasi A, Arghand HA (2018) Estimation of remaining useful life of rolling element bearings using wavelet packet decomposition and artificial neural network. Iran J Sci Technol Trans Electr Eng 43:233–245. [https://doi.org/10.1007/s](https://doi.org/10.1007/s40998-018-0108-y) [40998-018-0108-y](https://doi.org/10.1007/s40998-018-0108-y)
- 5. Aasi A, Tabatabaei R, Aasi E, Jafari SM (2021) Experimental investigation on time-domain features in the diagnosis of rolling element bearings by acoustic emission. J Vib Control 28:2585– 2595. <https://doi.org/10.1177/10775463211016130>
- <span id="page-18-4"></span>6. Li J, Chen W, Xue J, Han K, Wang Q (2019) Effect of multiple factors on identification and diagnosis of skidding damage in rolling bearings under Time-Varying slip conditions. Appl Sci [https://](https://doi.org/10.3390/app9153033) [doi.org/10.3390/app9153033](https://doi.org/10.3390/app9153033)
- <span id="page-18-5"></span>7. Jones AB (1959) Ball motion and sliding friction in ball bearings. J Basic Eng 81:1–12. <https://doi.org/10.1115/1.4008346>
- <span id="page-18-6"></span>8. Palmgren A (1959) Ball and roller bearing engineering, 3rd ed., S. H. Burbank, Philadelphia, PA.
- <span id="page-18-7"></span>9. Witte DC (1973) Operating torque of tapered roller bearings. ASLE Trans 16:61–67. [https://doi.org/10.1080/05698197308982](https://doi.org/10.1080/05698197308982705) [705](https://doi.org/10.1080/05698197308982705)
- <span id="page-18-8"></span>10. NSK Ltd. (2016) Rolling bearings catalog: rolling bearings for industrial machinery, NSK Ltd.
- <span id="page-18-9"></span>11. The Timken Company (2017) Timken engineering manual, The Timken Company
- <span id="page-18-10"></span>12. ISO standard (2018) Rolling bearings—Thermal speed rating— Calculation, ISO 15312-2018
- <span id="page-18-11"></span>13. Aihara S (1987) A new running torque formula for tapered roller bearings under axial load. ASME J Tribology 109:471–477. [https](https://doi.org/10.1115/1.3261475) [://doi.org/10.1115/1.3261475](https://doi.org/10.1115/1.3261475)
- <span id="page-18-12"></span>14. Zhou RS, Hoeprich MR (1991) Torque of tapered roller bearings. ASME J Tribology 113:590–597. [https://doi.org/10.1115/1.2920](https://doi.org/10.1115/1.2920664) [664](https://doi.org/10.1115/1.2920664)
- <span id="page-18-13"></span>15. Matsuyama H, Kamamoto S, Asano K (1998) The analysis of frictional torque for tapered roller bearings using EHD theory, SAE international Off-Highway and powerplant Congress and exposition. Milwaukee, Wisconsin. [https://doi.org/10.4271/9820](https://doi.org/10.4271/982029) [29](https://doi.org/10.4271/982029)
- <span id="page-18-14"></span>16. Houpert L (2002) Ball bearing and tapered roller bearing torque: analytical, numerical and experimental results. Tribol Trans 45:345–353. <https://doi.org/10.1080/10402000208982559>
- <span id="page-18-15"></span>17. SKF (2018) Rolling bearings catalogue 17000 EN. SKF Group, Sweden
- <span id="page-18-16"></span>18. Schaeffler, BEARINX—Online Easy Friction (2011) Detailed Friction Calculations for Rolling Bearings
- <span id="page-18-17"></span>19. Gradu M (2000) Tapered roller bearings with improved efficiency and high power density for automotive transmissions. SAE Int 1:1696–1705. <https://doi.org/10.4271/2000-01-1154>
- <span id="page-18-18"></span>20. Biyiklioglu A, Cuvalci H, Adatepe H, Bas H, Duman MS (2005) A new test apparatus and method for friction force measurement in journal bearings under dynamic loading: part I, experimental

- techniques. 29:22–24. [https://doi.org/10.1111/j.1747-1567.2005.t](https://doi.org/10.1111/j.1747-1567.2005.tb00244.x) [b00244.x](https://doi.org/10.1111/j.1747-1567.2005.tb00244.x)
- <span id="page-18-19"></span>21. Biyiklioglu A, Cuvalci H, Adatepe H, Bas H, Duman MS (2005) A new test apparatus and method for friction force measurement in journal bearings under dynamic loading: part II, experimental techniques. 29:33–36. [https://doi.org/10.1111/j.1747-1567.2005.t](https://doi.org/10.1111/j.1747-1567.2005.tb00244.x) [b00244.x](https://doi.org/10.1111/j.1747-1567.2005.tb00244.x)
- <span id="page-18-20"></span>22. Cousseau T, Graça B, Campos A, Seabra J (2010) Experimental measuring procedure for the friction torque in rolling bearings. Lubr Sci 22:133–147. <https://doi.org/10.1002/ls.115>
- <span id="page-18-21"></span>23. Hammami M, Martins R, Fernandes C, Seabra J, Abbes MS, Haddar M (2018) Friction torque in rolling bearings lubricated with axle gear oils. Tribol Int 119:419–435. [https://doi.org/10.1016/j.t](https://doi.org/10.1016/j.triboint.2017.11.018) [riboint.2017.11.018](https://doi.org/10.1016/j.triboint.2017.11.018)
- <span id="page-18-22"></span>24. Marin M, Abbas I, Kumar R (2014) Relaxed Saint-Venant principle for thermoelastic micropolar diffusion. Struct Eng Mech 51:651–662. <https://doi.org/10.12989/sem.2014.51.4.651>
- 25. Marin M, Agarwal RP, Abbas I (2014) Effect of intrinsic rotations, microstructural expansion and contractions in initial boundary value problem of thermoelastic bodies. Bound Value Probl 129:1–16. <https://doi.org/10.1186/1687-2770-2014-129>
- 26. Saeed T, Abbas I (2020) Finite element analyses of nonlinear DPL bioheat model in spherical tissues using experimental data. Mech Based Des Struct Mach 50:1287–1297. [https://doi.org/10.1](https://doi.org/10.1080/15397734.2020.1749068) [080/15397734.2020.1749068](https://doi.org/10.1080/15397734.2020.1749068)
- 27. Hobiny A, Abbas I (2020) Nonlinear analysis of dual-phase lag bio-heat model in living tissues induced by laser irradiation. J Therm Stresses 43:503–511. [https://doi.org/10.1080/01495739.2](https://doi.org/10.1080/01495739.2020.1722050) [020.1722050](https://doi.org/10.1080/01495739.2020.1722050)
- 28. Marin M, Hobiny A, Abbas I (2021) Finite element analysis of nonlinear bioheat model in skin tissue due to external thermal sources. Mathematics 9:1–9. [https://doi.org/10.3390/math91314](https://doi.org/10.3390/math9131459) [59](https://doi.org/10.3390/math9131459)
- <span id="page-18-23"></span>29. Hobiny A, Abbas I, Marin M (2022) The influences of the hyperbolic Two-Temperatures theory on waves propagation in a semiconductor material containing spherical cavity. Mathematics 10:1–12.<https://doi.org/10.3390/math10010121>
- <span id="page-18-24"></span>30. Tong V-C, Hong S-W (2016) The effect of angular misalignment on the running torques of tapered roller bearings. Tribol Int 95:76–85. <https://doi.org/10.1016/j.triboint.2015.11.005>
- <span id="page-18-25"></span>31. Balyakin VB, Zhilnikov EP, Kosenok BB, Lavrin AV (2017) Study of the influence of ring misalignment in rolling bearings on frictional torque and the fatigue life of supports. J Frict Wear 38:7–12.<https://doi.org/10.3103/s1068366616060027>
- <span id="page-18-26"></span>32. Hou Y, Wang X (2021) Measurement of load distribution in a cylindrical roller bearing with an instrumented housing: finite element validation and experimental study. Tribol Int 155:106785. [ht](https://doi.org/10.1016/j.triboint.2020.106785) [tps://doi.org/10.1016/j.triboint.2020.106785](https://doi.org/10.1016/j.triboint.2020.106785)
- <span id="page-18-27"></span>33. Zhao ZX, Wang X, Hou Y (2024) Measuring the roller skew angle in the loading zone of a cylindrical roller bearing with strain gaugesfor long-term monitoring.ASME J Tribology 146:024301. <https://doi.org/10.1115/1.4063211>
- <span id="page-18-28"></span>34. Zhao ZX, Wang X, Hou Y (2024) Measurement of the roller Tilt angle in a double-row tapered roller bearing with strain gauges. Measurement 226:114106. [https://doi.org/10.1016/j.measuremen](https://doi.org/10.1016/j.measurement.2023.114106) [t.2023.114106](https://doi.org/10.1016/j.measurement.2023.114106)
- <span id="page-18-29"></span>35. Harris TA, Kotzalas MN (2009) Rolling bearing analysis: essential concepts of bearing technology. [https://doi.org/10.1201/b157](https://doi.org/10.1201/b15723-9) [23-9](https://doi.org/10.1201/b15723-9)
- <span id="page-18-30"></span>36. Schaeffler Group (2010) Arcanol Bearing Lubricant Technical Manual, Schaeffler Group
- 37. Han QL, Jiang JL, Liu C, Liu C (2019) Study on influence of grease on temperature of axle box bearings for high speed EMU. Bearing 8:42–45. [https://doi.org/10.19533/j.issn1000-3762.2019.](https://doi.org/10.19533/j.issn1000-3762.2019.08.011) [08.011](https://doi.org/10.19533/j.issn1000-3762.2019.08.011)

![](_page_18_Picture_41.jpeg)

- 38. Han QL, Jiang JL, Liu C, Liu C (2021) Simulation study on effect of lubricating grease type on temperature field of axle box bearing for high speed train. Rolling Stock 59:35–39. [https://doi.org/10.3](https://doi.org/10.3969/j.issn.1002-7602.2021.03.009) [969/j.issn.1002-7602.2021.03.009](https://doi.org/10.3969/j.issn.1002-7602.2021.03.009)
- <span id="page-19-0"></span>39. Ai SY, Wang WZ, Wang YL, Zhao ZQ (2015) Temperature rise of double-row tapered roller bearings analyzed with the thermal network method. Tribol Int 87:11–22. [https://doi.org/10.1016/j.tr](https://doi.org/10.1016/j.triboint.2015.02.011) [iboint.2015.02.011](https://doi.org/10.1016/j.triboint.2015.02.011)
- <span id="page-19-1"></span>40. Wang W, Wong PL, Zhang Z (1996) Partial EHL analysis of ribroller end contact in tapered roller bearings. Tribol Int 29:313– 321. [https://doi.org/10.1016/0301-679X\(95\)00059-D](https://doi.org/10.1016/0301-679X(95)00059-D)
- <span id="page-19-2"></span>41. Test methods on test machine for rolling Bearing of locomotive and rolling stock—Part 1: axleboxes rolling bearings. TB/T 3017.1–2016
- <span id="page-19-3"></span>42. Rolling (2008) bearings — Methods for calculating the modified reference rating life for universally loaded bearings, ISO/TS 16281
- <span id="page-19-4"></span>43. Zhou G, Li G, Chen P, Hou Y, Wang X (2022) Research on the load distribution measurement of high-speed train axlebox bearing. Part I: simulation analysis of the Notching method of bearing

- housing. J Experimental Mech 37:18–26. [https://doi.org/10.7520](https://doi.org/10.7520/1001-4888-21-214) [/1001-4888-21-214](https://doi.org/10.7520/1001-4888-21-214)
- <span id="page-19-5"></span>44. Zhou G, Li G, Chen P, Hou Y, Wang X (2022) Research on the load distribution measurement of high-speed train axlebox bearing. Part II: experimental set-up and experimental study. J Experimental Mech 37:27–32. [https://doi.org/10.7520/1001-4888-21-2](https://doi.org/10.7520/1001-4888-21-215) [15](https://doi.org/10.7520/1001-4888-21-215)

**Publisher's Note** Springer Nature remains neutral with regard to jurisdictional claims in published maps and institutional affiliations.

Springer Nature or its licensor (e.g. a society or other partner) holds exclusive rights to this article under a publishing agreement with the author(s) or other rightsholder(s); author self-archiving of the accepted manuscript version of this article is solely governed by the terms of such publishing agreement and applicable law.

![](_page_19_Picture_12.jpeg)