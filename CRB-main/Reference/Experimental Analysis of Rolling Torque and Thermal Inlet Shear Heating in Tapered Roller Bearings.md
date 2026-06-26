![](_page_0_Picture_0.jpeg)

![](_page_0_Picture_1.jpeg)

*Article*

# **Experimental Analysis of Rolling Torque and Thermal Inlet Shear Heating in Tapered Roller Bearings**

**Manjunath Manjunath 1,\* [,](https://orcid.org/0000-0003-2990-3062) Dieter Fauconnier 1,2 [,](https://orcid.org/0000-0002-0257-4687) Wouter Ost 1,[2](https://orcid.org/0000-0002-9931-0513) and Patrick De Baets 1,2,3**

- <sup>1</sup> Soete Laboratory, Department of Electromechanical, Systems & Metal Engineering, Faculty of Engineering and Architecture, Ghent University, Technologiepark 903, 9052 Ghent, Belgium; dieter.fauconnier@ugent.be (D.F.); wouter.ost@ugent.be (W.O.); patrick.debaets@ugent.be (P.D.B.)
- <sup>2</sup> Flanders Make @ UGent-Core Lab MIRO, 9000 Ghent, Belgium
- <sup>3</sup> Royal Institute of Technology KTH, Systems and Component Design, School of Electrical Engineering and Computer Science, Lindstedtsvägen 3, 100 44 Stockholm, Sweden
- **\*** Correspondence: manjunath.manjunath@ugent.be

**Abstract:** The investigation in this article focuses on the rolling resistance torque and thermal inlet shear factor in tapered roller bearings (TRBs) through systematic experiments using a modular test setup. TRBs typically operate under Elastohydrodynamic Lubrication (EHL) conditions. At sufficiently high speeds, the majority of rolling friction is due to a significant shift of the pressure centre in the EHL contact. While at lower speeds, sliding friction in the roller-rib contact becomes dominant, which operates under mixed lubrication conditions. Limited literature exists on the impact of inlet shear heating on effective lubricant temperature (*Tin*\_*<sup>c</sup>* ) and rolling friction in TRBs. To fill this gap, experimental measurements of the total frictional torque under axial loading at different speeds and oil temperatures are performed. With existing models for different friction contributions described in the literature, the rolling resistance due to EHL has been determined for various operating conditions. The effects of dimension-less speed (*U*), material (*G*), and load (*W*) parameters have also been investigated. Under fully flooded conditions, it was observed that the influence of material (*G*) and load (*W*) parameters on rolling friction is minor, while the impact of velocity (*U*) is significant. In the context of rolling resistance, the heating due to shear of the lubricant in the inlet zone plays a significant role. For higher rotational velocities, the estimated rotational torque reduction resulting from inlet shear heating was found to be approximately 6–8%.

**Keywords:** tapered roller bearings; rolling friction; thermal inlet shear factor

![](_page_0_Picture_11.jpeg)

**Citation:** Manjunath, M.; Fauconnier, D.; Ost, W.; De Baets, P. Experimental Analysis of Rolling Torque and Thermal Inlet Shear Heating in Tapered Roller Bearings. *Machines* **2023**, *11*, 801. [https://doi.org/](https://doi.org/10.3390/machines11080801) [10.3390/machines11080801](https://doi.org/10.3390/machines11080801)

Academic Editors: Mahdi Mohammadpour and Ehsan Fatourehchi

Received: 10 July 2023 Revised: 28 July 2023 Accepted: 31 July 2023 Published: 3 August 2023

![](_page_0_Picture_15.jpeg)

**Copyright:** © 2023 by the authors. Licensee MDPI, Basel, Switzerland. This article is an open access article distributed under the terms and conditions of the Creative Commons Attribution (CC BY) license [\(https://](https://creativecommons.org/licenses/by/4.0/) [creativecommons.org/licenses/by/](https://creativecommons.org/licenses/by/4.0/) 4.0/).

# **1. Introduction**

Modern heavy-duty drivetrains and industrial machinery benefit from high-power density. Often, tapered roller bearings (TRBs) are preferred for their high load capacity and rigidity. However, TRBs experience higher frictional losses due to additional contact between conical rollers and the raceway rib, reducing efficiency and causing increased power losses at higher speeds and loads in mixed lubrication conditions. Adequate lubrication ensures energy-efficient and durable TRB operation under varying velocities, loads, and temperatures.

Ideally, bearings operate in an Elasto-Hydrodynamic Lubrication (EHL) regime [\[1–](#page-20-0)[3\]](#page-20-1), with contact surfaces fully separated by a thin layer (about 50 nm to 1 µm thick) of pressurized lubricant film, enabling efficient load transmission. Under mixed lubrication conditions, e.g., lower speed-to-load ratio or lower lubricant viscosity, direct metallic asperity contact occurs, leading to higher friction and higher risk of damage and wear [\[4–](#page-20-2)[11\]](#page-21-0).

The friction generated in TRBs is composed into load-dependent and load-independent contributions (Figure [1a](#page-1-0)). The load-dependent frictional torque includes (a) The bearing torque due to tractive rolling of the rollers on the raceways *Mrr*; and (b) The sliding torque due to the contact between the rib and the roller faces *Msl*\_*rib*. *Mrr* originates from elastic *Machines* **2023**, *11*, 801 2 of 22

<span id="page-1-0"></span>hysteresis losses in the loading and unloading of the solid material and the viscous rolling resistance due to the lubricant film viscous dissipation [\[12](#page-21-1)[–14\]](#page-21-2). The latter is typically much larger than the former for oil lubrication [\[12\]](#page-21-1). On the other hand, *Msl*\_*rib* consists of both viscous and Coulombic contributions. The load-independent contributions are governed by (c) The cage friction *Mcage*; and (d) The viscous drag losses *Mdrag*. At intermediate speeds, the load-independent contributions are relatively small [\[12\]](#page-21-1) (Figures 11b and 13b). Hence, the load-dependent raceways and rib are essential for the analysis.

![](_page_1_Figure_2.jpeg)

**Figure 1.** (**a**)TRB torque composition; (**b**) Thermal inlet shear region.

Ideally, EHL prevails in the roller-raceway contacts. The rolling resistance is due to viscous shear stresses in the deforming contact geometry and inlet region. For very high entrainment velocities and, particularly, high slide-to-roll ratios, the oil close to the contact area is repelled and produces a recirculating flow. This flow is accompanied by viscous shear heating leading to the so-called thermal inlet shear effect (Figure [1b](#page-1-0)), which raises the oil temperature (*Tin*\_*<sup>c</sup>* ) at the entry of the contact area and, hence, reduces the oil viscosity, the film thickness, and, finally, the *Mrr* [\[15\]](#page-21-3). Therefore, a thorough understanding of the thermal inlet shear effect on *Mrr* is needed for different operating conditions of TRBs.

### *1.1. State of the Art*

In the past 50 years, many authors analytically derived and experimentally validated equations for predicting load-dependent frictional loss in TRBs [\[16](#page-21-4)[,17\]](#page-21-5). Very few authors derived the viscous rolling resistance in roller-raceway contact of TRBs using the dimensionless velocity (*U*), load (*W*), and material (*G*) parameters (Equation (1)). They are presented in Table [1.](#page-3-0)

$$U = \frac{\eta_o \mu_r}{E' R_e} ; W = \frac{w}{l E' R_e} ; G = \alpha E'$$
 (1)

In 1987, Aihara [\[12\]](#page-21-1) derived an analytical formula for the load-dependent frictional torque in TRBs operating under pure axial external loads. The formula accounts for rolling resistance at the raceway and friction at the rib-roller contact, with modifications based on the work of Murch and Wilson [\[18\]](#page-21-6) (see Appendix [A,](#page-19-0) Table [A1\)](#page-19-1) and a wide range (*L* = 0 to 5000; Equation (2)) of the non-dimensional thermal loading parameter [\[19,](#page-21-7)[20\]](#page-21-8). This parameter characterizes heat generation within the lubricating film and provides valuable insights into the lubricant's thermal behavior and its impact on EHL contact performance and reliability. The formula for rib sliding friction was derived based on Patir and Cheng [\[21\]](#page-21-9) (see Appendix [A,](#page-19-0) Table [A1\)](#page-19-1) by focusing on the asperity load, oil film– surface roughness ratio, and a partial EHL model. Experimental validation under various conditions (see Table [1\)](#page-3-0) showed that the rolling resistance in TRBs had minimal dependence on the axial load, but the specific sources of traction forces and moments contributing to the TRB's resistance torque remain unclear. In 1991, Zhou and Hoepprich [\[13\]](#page-21-10) developed

*Machines* **2023**, *11*, 801 3 of 22

an analytical torque model for TRBs, using a single contact load distribution. Their model accurately predicted torque components by balancing forces and moments, shedding light on the sources of torque and heat generation. They obtained the rolling resistance equation through numerical solutions for lubricant film thickness and pressure distribution in an isothermal EHL line contact, considering the Reynolds equation and viscosity–pressure relationship. Through power fitting, they determined functions for rolling resistance forces during pure rolling. They also derived an equation for rib sliding friction by numerically solving the lubricant film thickness equation for an elliptical contact, considering rib roller surface roughness. Bair and Winer's [\[22\]](#page-21-11) (see Appendix [A,](#page-19-0) Table [A1\)](#page-19-1) rheological model for EHL lubricant shearing was also employed. Experiments were conducted using different oils and load conditions to measure the torque of the raceway and rib separately (see Table [1\)](#page-3-0). The study highlighted the importance of the rib-roller end contact torque for low Lambda (1 to 2; film thickness-to-surface roughness ratio) values and the significant impact of thermal effects in predicting torque for high-speed bearing applications. The experimental and calculated results aligned well when incorporating the thermal reduction factor. This research is noteworthy for considering thermal effects and surface roughness in rib-roller torque calculations.

$$L = \frac{\eta_o \beta_o \mu_r^2}{k} \tag{2}$$

Between 1998 and 2001, H. Matsuyama [\[14](#page-21-2)[,23](#page-21-12)[,24\]](#page-21-13) proposed a torque model for TRBs by numerically solving the Reynolds equation in conjunction with Roelands' equation (see Appendix [A,](#page-19-0) Table [A1\)](#page-19-1) and the Dowson–Higginson equation. This model enables the calculation of viscous rolling resistance in TRBs. A simplified formula for viscous rolling resistance under fully flooded conditions was obtained by conducting full numerical EHL calculations. The equation for the rib sliding friction was determined by measuring the moment (mr) at the rib contact. A comparison between the measured torque (see Table [1\)](#page-3-0) and a simplified formula derived from theoretical analysis revealed differences in the exponents of certain variables. Adjustments were made to these exponents to align them with the theoretical values. In 2002, Houpert [\[25\]](#page-21-14) formulated analytical equations for forces and moments based on the TRB geometry. The rolling friction model proposed by Zhou and Hoeprich [\[13\]](#page-21-10) was utilized, and an equation for the rib normal force (*Fr*) was derived based on the rib geometry and Hertzian contact width. The friction coefficient of TRBs, which is dependent on the dimensionless film thickness parameter *λ*, was calculated. They employed experimental data (see Table [1\)](#page-3-0) from Mircea Gradu (2000) [\[26\]](#page-21-15), which exhibited a good agreement between measured and calculated torque. From 2003–2004, SKF bearing catalogue [\[15\]](#page-21-3) presented a computational mode incorporating starvation and inlet shear heating effects for calculating rolling frictional moments. The model also considers the sliding friction coefficient for full-film, mixed lubrication conditions, which is applicable to oil and grease-lubricated bearings. This frictional torque (*Mt*) calculation method is accounting for rolling friction (a) and sliding friction (b), seal friction (c), drag (d), and seal loss based on the experimental results.

In 2018, Schwarz et al. [\[27\]](#page-21-16) experimentally studied TRB's frictional torque and temperature, focusing on finding a minimal oil flow rate to achieve non-starved lubrication. Results showed that a small amount of oil could be sufficient for TRBs to operate safely and stably. The direction of oil supply was observed to have a significant impact on friction, with oil supply opposite to the pumping direction reducing frictional losses by minimizing hydraulic losses. To understand the TRB rib-roller contact, Wirsching et al. [\[17\]](#page-21-5) (2021) studied its tribological behaviour as a function of different macro-geometries using machine-learning techniques. The geometric parameters describing the roller end-face and rib geometry were sampled by a statistical design of experiments. Their study demonstrated a trade-off between high load-carrying capacity and low friction losses exists. TRBs operating at low axial loads and/or sufficiently high velocities combine a moderate risk of wear and a minimised energy loss. Liu et al. (2022) [\[28\]](#page-21-17) studied how roller geometric homogeneity affects TRB friction torque. They developed a mathematical model considering

Machines **2023**, 11, 801 4 of 22

dimensional and shape deviations among rollers, simulating machining-induced geometric variations using Gaussian-distributed diameter deviation. Results showed that overall friction torque on raceways was minimally affected by roller homogeneity but significantly impacted sliding friction between individual rollers and inner raceway. Diameter deviation was crucial in determining maximum sliding friction, especially under light loads. Liu et al. (2023) [29] developed a thermal EHL (TEHL) model using a shear-thinning fluid (Carreau model) to analyze TRB geometries. They compared two rib geometries: tapered and spherical, for their coefficient-of-friction (COF) variations. The TEHL contact's film thickness and temperature increased with the inner raceway's velocity. The tapered rib had a nearly constant difference between minimum and central film thickness, while the spherical rib exhibited lower COF and minimum film thickness due to its higher film temperature.

<span id="page-3-0"></span>**Table 1.** TRBs frictional losses composition models.

| Author                                 | Equations                                                                                                                                                        | Applicability Range                                                                                                                                                                                                                             |  |
|----------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--|
| Aihara, 1987 [12]                      | $M_{i,o} = \left(\frac{1.76e2}{1+0.29L^{0.78}}\right) \frac{1}{\alpha} (GU)^{0.658} W^{0.31} R_e^2 I$                                                            | The equations were experimentally validated under conditions: Axial load ∈ [0.45 to 1.2 GPa] Rolling speed ∈ [100 to 3000 rpm]                                                                                                                  |  |
|                                        | $M_{sl\_rib} = e\mu_{rib}F_a exp^{-1.8\lambda_r^{1.2}}$                                                                                                          | Oil type ∈ [Gear oil (80 W)] Oil temperature ∈ [50 to 80 °C] Lubricated condition ∈ [Fully flooded]                                                                                                                                             |  |
| Zhou and Hoepprich,<br>1991 [13]       | $M_{i,o} = \varphi_{ish}\varphi_{bl}58.4\frac{R_e^2}{\alpha}(GU)^{0.648}W^{0.246}l$                                                                              | The equations were experimentally validated under conditions:  Axial load $\in$ [0.85 to 1.47 GPa]  Rolling speed $\in$ [100 to 8000 rpm]                                                                                                       |  |
|                                        | $M_{sl\_rib} = \frac{\varnothing_{c,S_o}}{\sigma_o} F_r exp^{B\lambda_r} + \varphi_{T,rib} \iint \tau(x,y) dxdy$                                                 | Oil type $\in$ [SAE20, Vactra oil]<br>Oil temperature $\in$ [50 $^{\circ}$ C]<br>Lubricated condition $\in$ [Fully flooded]                                                                                                                     |  |
| H. Matsuyama<br>(1998–2001) [14,23,24] | $M_{i/o} = \varphi_{ish} 14.2E' l R_e^2 U^{0.75} G^{-0.04} W^{0.08}$                                                                                             | The equations were experimentally validated under conditions:  - Axial load ∈ [0.3 to 1.3 GPa]  Rolling speed ∈ [100 to 1500 rpm]  Oil type ∈ [Paraffin-based, traction oil]  Oil temperature ∈ [26 °C]  Lubricated condition ∈ [Fully flooded] |  |
|                                        | $M_{sl\_rib} = rac{eR_om_r}{D_a(R_i + rac{1}{2}sinlpha_i - ecos	heta)} \ M_{sl\_rib} = e\mu_{rib}F_acos\gamma \ \mu_{rib} = c_1exp(-c_2\lambda_r^{c_3} + c_4)$ |                                                                                                                                                                                                                                                 |  |
| Houpert 2002 [25]                      | $M_{i/o} = 0.04E'lR_e^2 U^{0.44} W^{0.37}$                                                                                                                       | The equations were experimentally validated under conditions:  Axial load $\in$ [3500 N]  Radial load $\in$ [4250 N]  Rolling speed $\in$ [100 to 4500 rpm]                                                                                     |  |
|                                        | $M_{sl\_rib} = \mu_{rib}F_re\sqrt{1 + 0.18\left(rac{b}{e} ight)^2}$                                                                                             | Oil type ∈ [ATF oil] Oil temperature ∈ [50 °C] Lubricated condition ∈ [Fully flooded]                                                                                                                                                           |  |
| SKF 2003–2004 [15]                     | $M_{i/o} = \varphi_{ish}\varphi_{rs}G_{rr}(\eta_o n)^{0.6}$                                                                                                      | The equations were validated for all types of roller<br>bearings and are applicable to both grease- and<br>oil-lubricated bearings under constant loads in<br>magnitude and direction.                                                          |  |
|                                        | $M_{sl\_rib} = G_{sl}[\varphi_{bl}\mu_{bl} + (1 - \varphi_{bl})\mu_{EHL}]$                                                                                       |                                                                                                                                                                                                                                                 |  |

Where i, o, and f correspond to inner raceways, outer raceways, and flange contacts.

### 1.2. Goal of the Paper

In the literature discussed above, the analyses of rolling and sliding frictional torques were focused towards the dimensionless load (W) and speed (U) parameters used in EHL, along with the effects of the geometrical influence of the rib-roller contact. However, the effect of inlet shear heating on the effective lubricant temperature  $(T_{in\_c})$ . Hence, the rolling friction in TRBs and the corresponding torque are rarely reported in the literature. In this work, experiments have been conducted to determine the load-dependent frictional losses

*Machines* **2023**, *11*, 801 5 of 22

in an axially loaded TRB as a function of the external axial load, speed, and supply oil temperature under fully flooded lubrication conditions. The literature models presented in Table [1](#page-3-0) are compared to the experimental results and used to unravel the different friction contributions. Furthermore, the influence of shear heating at the inlet of the EHL contacts on the rolling friction, as well as on the bearing outer race temperature, are investigated at different operating conditions. Additionally, systematic experiments are conducted to find the optimal oil flow rate for the oil circulation lubrication system.

## **2. Materials and Methods**

# *2.1. Experimental Setup*

The experimental setup used in this work is a vertical-shaft roller-bearing tribometer (RBT) capable of accommodating bearings with inner diameters of 35–60 mm. It comprises six sub-systems: the drive unit, support-bearing unit, test-bearing unit, hydraulic loading unit, oil circulation unit, and control unit. The hydraulic cylinder has a load capacity of 45 kN (Table [2\)](#page-4-0).

<span id="page-4-0"></span>**Table 2.** Roller-bearing test rig and its operating boundaries (RBT).

| Bearing                                                   | Roller Bearing                                  |
|-----------------------------------------------------------|-------------------------------------------------|
| Axial Load<br>Oil flow to test bearing<br>Oil temperature | 2.5–45 kN<br>0.07 to 3 lpm<br>30 ◦C to 80<br>◦C |

Figure [2](#page-5-0) shows a cross-sectional view of the RBT. The main shaft (1) is supported by a cylindrical roller bearing (2) and two angular contact ball bearings (3) and is driven by an AC motor. The testing unit comprises a test-bearing adapter (4) and a test-bearing housing (5). A hydraulic cylinder is used to apply the axial load on the test bearing. Firstly, the complete test bearing unit is raised and aligned with the rotating shaft using a cylinder piston stroke. Once assembled, the desired test load is applied to the test bearings. The test bearing is assembled inside the test cup with the assistance of a bearing holder. At the bottom, a thrust ball bearing supports the test-bearing cup.

The frictional torque is measured using a lever arm (6) attached to the bearing housing. A load cell is mounted on the arm between the test-bearing housing and the thrust-bearing housing to measure the tangential frictional force while excluding the shaft support-bearing influence. A 300 N or 500 N load cell is used depending on the expected friction force and measurement accuracy [\[30\]](#page-21-19). The friction measurements of the test bearing are not significantly affected by the bottom support thrust bearing under standing-still conditions.

The RBT allows for the use of forced circulation (Figure [3a](#page-5-1)), as well as oil bath lubrication (Figure [3b](#page-5-1)). The lubricant flow rate and temperature are properly monitored and controlled. By controlling the oil flow rate into the bearing cup, as well as controlling the oil outlet from the bearing cup through a scavenging system, the oil level inside the test bearing cup can be adjusted (Figure [3b](#page-5-1)).

In this work, the bearing setup is employed to investigate the heavily loaded EHL line contacts (2–3 GPa) of TRBs under a purely axial load and well-controlled fully flooded lubricant conditions (Figure [3a](#page-5-1)). The pure axial loading of the TRB ensures equal contact conditions for all rollers in the bearing. Thermocouples have been installed in the supply oil inlet (SOI) and supply oil outlet (SOO), as well as in the TRB outrace temperature (BT). All measuring signals are registered continuously using digital data acquisition.

*Machines* **2023**, *11*, 801 6 of 22

<span id="page-5-0"></span>![](_page_5_Picture_1.jpeg)

**Figure 2.** Cross-sectional view of the RBT. (1) Main shaft; (2) Cylindrical roller-support bearing; (3) Angular contact ball-support bearing; (4) Test-bearing adapter; (5) Test-bearing housing; (6) Measuring lever arm.

<span id="page-5-1"></span>![](_page_5_Picture_3.jpeg)

**Figure 3.** Test-bearing lubrication modes and thermocouple locations. (**a**) Forced oil circulation method; (**b**) oil bath lubrication method.

### <span id="page-5-2"></span>*2.2. Test-Bearing Geometry and Forces*

In this research, a single-row TRB with an inner diameter of 40 mm, outer diameter of 68 mm, and width of 19 mm is used (Figure [4\)](#page-6-0). When a pure axial load (*Fa*) is applied to a TRB, three distinct loads act on each roller, as shown in Figure [4.](#page-6-0) These three loads are the roller end and rib force (*Fr*), and the outer raceway load (*Fo*) that is almost the same as the inner raceway load (*F<sup>i</sup>* ). Each load may be expressed as follows, with *Z* the number of rollers [\[12\]](#page-21-1):

$$F_i = \frac{F_a}{Z sin \alpha_i}$$
;  $F_o = \frac{F_a}{Z sin \alpha_o}$  ;  $F_r = \frac{F_a sin 2 \gamma}{Z sin \alpha_o}$  (3)

Figure [5a](#page-6-1),b provide an overview of all sliding friction forces and moments that occur in TRB contacts. However, it is important to note that the analysis does not consider the spin moment in the rib, cage-to-cage friction, and churning resistances.

*Machines* **2023**, *11*, 801 7 of 22

<span id="page-6-0"></span>![](_page_6_Picture_1.jpeg)

**Figure 4.** Forces acting on TRB [\[12–](#page-21-1)[14,](#page-21-2)[23–](#page-21-12)[25](#page-21-14)[,31](#page-21-20)[,32\]](#page-21-21).

<span id="page-6-1"></span>![](_page_6_Picture_3.jpeg)

**Figure 5.** Simplified model for analysis [\[12–](#page-21-1)[14,](#page-21-2)[23](#page-21-12)[–25](#page-21-14)[,31\]](#page-21-20) (**a**) Raceway contact; (**b**) Rib contact.

From the balance of forces and moments on one single roller, the following equations [\[12–](#page-21-1)[14,](#page-21-2)[23](#page-21-12)[–25,](#page-21-14)[31\]](#page-21-20) are obtained,

$$F_{so} - F_{si} = F_{sf} \tag{4}$$

$$M_i + M_o = \frac{D_a}{2} F_{so} + \frac{D_a}{2} F_{si} + \left(\frac{D_a}{2} - e\right) F_{sf}$$
 (5)

By combing Equations (4) and (5), *Fso*, *Fsi* can be eliminated, leaving *M<sup>i</sup> Mo*, and *Fs f* (detailed derivation in Appendix [B\)](#page-19-2). The torque of TRB is defined as the moment acting on the outer ring (*M*).

$$M = Z(R_o F_{so} - M_o) \tag{6}$$

$$M = Z \frac{1}{D_a} (R_o M_i + R_i M_o) + Z \frac{R_o}{D_a} e F_{sf} = M_{rr} + M_{sl\_rib}$$
 (7)

Equation (6) encompasses the rolling resistance torque (*Mrr*) and sliding rib torque (*Msl*\_*rib*). *M<sup>i</sup>* and *M<sup>o</sup>* in Equation (7) consist of the EHL viscous rolling resistance and elastic hysteresis. However, with oil lubrication, the effect of elastic hysteresis is much smaller than that of viscous resistance [\[12](#page-21-1)[–14\]](#page-21-2). Before delving further, it is essential to discuss the rolling resistance (*Mi*/*<sup>o</sup>* ). The rollers are supposed to "roll" along the raceways. Still, some slippage exists in the roller-raceway contact areas, which causes sliding friction or traction, Machines 2023, 11, 801 8 of 22

as shown in Figure 5. This sliding friction is determined by the moments  $(M_{i/o})$  and the sliding friction between the roller ends and rib  $(F_{sf})$  [12].

# 2.2.1. Theoretical Analysis of Viscous Rolling Resistance ( $M_i$ , $M_o$ )

Ideally, the contacts between the rollers and the raceways operate in EHL, and the rolling friction is governed by viscous shearing at the inlet and in the pressurized zone. As shown in Figure 6, the asymmetry of the pressure distribution in the EHL contacts induces a moment on the roller that counteracts the rolling motion, as the center of pressure is displaced from the center of Hertzian contacts [14]. The coordinate  $x_{cp}$  of the center of pressure is provided by:

 $x_{cp} = -\frac{1}{w} \int_{x_{11}}^{x_{12}} p(x) x dx \tag{8}$ 

<span id="page-7-0"></span>![](_page_7_Figure_5.jpeg)

**Figure 6.** Pressure distribution in EHL contact [14,23].

The moment due to w acting on the coordinate  $x_{cp}$  is equivalent to the moment due to the EHL pressure distribution. The viscous rolling resistance  $(M_{i/o})$  is provided as follows [14,23,24]:

$$M_{i/o} = -\int_{0}^{1} x_{cp} w dy = -x_{cp} w l \tag{9}$$

where l is the effective length of the roller.

The dimensionless coordinate of the center of pressure,  $X_{cp}$  is expressed from Equation (9) as follows:

$$X_{cp} = \frac{x_{cp}}{b} = -\frac{2}{\pi} \int_{X_{L1}}^{X_{L2}} PX dX$$
 (10)

Thus, the dimensionless viscous rolling resistance  $(m_{i/o})$  is provided as the following equation:

$$m_{i/o} = \frac{M_{i/o}}{hE'IR} = -X_{cp}W \tag{11}$$

When the pressure distribution is obtained from EHL analysis, the dimensionless viscous rolling resistance ( $m_{i/o}$ ) is estimated using Equations (10) and (11). The numerical EHL analysis of Matsuyama [14,23] is used to obtain  $m_{i/o}$  and demonstrate the influence of dimensionless parameters U, G, and W on  $m_{i/o}$  under fully flooded conditions.

$$m_{i/o} = \left[ 8.89 U_{i/o}^{0.75} G_{i/o}^{-0.04} W_{i/o}^{0.42} \right]$$
 (12)

From Equations (11) and (12), a simplified formula for viscous rolling resistance ( $M_{i/o}$ ) under fully flooded conditions is provided by the following [14,23]:

*Machines* **2023**, *11*, 801 9 of 22

$$m_{i/o} = \frac{M_{i/o}}{bE'lR} = \left[8.89U_{i/o}^{0.75}G_{i/o}^{-0.04}W_{i/o}^{0.42}\right]$$
With
$$b = R\sqrt{\frac{8W}{\pi}}$$

$$M_{i/o} = \left[14.2E'lR_e^2U_{i/o}^{0.75}G_{i/o}^{-0.04}W_{i/o}^{0.08}\right]$$
(13)

Rolling resistance torque (*Mrr*) can be predicted by the following Equation (14), which is obtained by substituting (*Mi*/*<sup>o</sup>* ) in Equation (6) [\[14](#page-21-2)[,23\]](#page-21-12).

$$M_{rr} = \frac{Z}{D_a} \left( R_o \cdot 14 \cdot 2E' l R_e^2 U_i^{0.75} G_i^{-0.04} W_i^{0.08} + R_i \cdot 14 \cdot 2E' l R_e^2 U_o^{0.75} G_o^{-0.04} W_o^{0.08} \right)$$
(14)

*Mrr* is estimated using dimensionless speed (*U*), load (*W*), and material (*G*) parameters with the help of the available EHL rolling frictional model (Table [1\)](#page-3-0).

# 2.2.2. Sliding Friction in Roller End and Rib Contacts (*Msl*\_*rib*)

The second term in Equation (7) represents the force resulting from the sliding between the roller end and the rib. By introducing a coefficient of friction *µrib*, *Fs f* can be expressed as *µF<sup>r</sup>* . Here, *F<sup>r</sup>* is the force against the rib, as shown in Figure [4.](#page-6-0) For a purely axial load (*Fa*), *F<sup>r</sup>* is given by Equation (3) and *Msl*\_*rib* can be rewritten as follows (detailed derivation in Appendix [C\)](#page-20-3):

$$M_{sl\_rib} = e\mu_{rib}cos\gamma F_a$$
; Usually,  $\gamma$  very small so  $cos\gamma = 1$  (15) 
$$M_{sl\_rib} = e\mu_{rib}F_a$$

The frictional torque contribution of the rib *Mrib* was experimentally measured using a special loose rib [\[13](#page-21-10)[,14\]](#page-21-2). The ribs were deliberately separated from the inner ring in order to isolate and measure the frictional torque, specifically generated at the rib. The authors reported that the sliding friction on the rib is at its maximum value when at a starting rotation from a standstill and decreases rapidly with the rotational speed due to the formation of a thin lubricant film (Figure [7a](#page-8-0)). SKF has simplified the sliding friction coefficient for both full-film and mixed lubrication conditions (Figure [7b](#page-8-0)). The calculated sliding friction based on the SKF model exhibits a similar trend to the experimentally measured values by Zhou [\[13\]](#page-21-10).

$$\mu_{rib} = \varphi_{bl}\mu_{bl} + (1 - \varphi_{bl})\mu_{EHL} \tag{16}$$

$$\varphi_{bl} = \frac{1}{ex^{2.6 \cdot 10^{-8} (n\eta_o)^{1.4} d_m}} \tag{17}$$

<span id="page-8-0"></span>![](_page_8_Figure_11.jpeg)

**Figure 7.** Coefficient of friction of rib-roller end contacts. (**a**) Experimentally measured by Zhou [\[13\]](#page-21-10); (**b**) Calculated with SKF Model.

*Machines* **2023**, *11*, 801 10 of 22

## *2.3. Frictional Measurements Using RBT Setup*

The configuration for measuring the global frictional force is depicted in Figure [8](#page-9-0) where *rarm* represents the horizontal distance between the points of action of *Fapplied* and *Ff* . *Rmean* denotes the mean radius of the bearing. During the tests, the hydraulic actuator applies a normal constant axial load *Faxial* (Figure [2\)](#page-5-0). The TRB torque can be measured by the reaction force of the floating bearing housing, where the thrust-bearing friction has relatively minimal friction. The deviation of the piston load from its vertical position is assumed to be very small. Internally in the test-bearing chamber, a reaction force per roller occurs that can be decomposed in a normal component *FN*, and a tangential component *F<sup>t</sup>* . The tangential component results from viscous friction between inner and outer raceway contacts, including side rib torque. As both *F<sup>t</sup>* and *F<sup>N</sup>* cannot be measured directly, they have to be derived from the applied *Fapplied* and frictional forces *F<sup>f</sup>* from the vertical and horizontal force balance equation.

![](_page_9_Picture_3.jpeg)

<span id="page-9-0"></span>![](_page_9_Picture_4.jpeg)

**Figure 8.** Configuration to measure the global frictional force.

Rearranging Equation (18) for tangential force provides

$$F_t = \frac{F_f \cdot r_{arm}}{r_{mean}} \tag{19}$$

In this manner, the global torque (*Mt*) of TRBs is measured for different operating conditions. To minimize the drag loss in the global measurements, systematic experiments were conducted to find the optimum oil-flow rate for the oil-circulation lubrication method. The corresponding drag losses are verified to be minimal using models from the literature [\[33](#page-21-22)[,34\]](#page-21-23) (detailed in Section [4.1\)](#page-11-0). Since the global friction (*Mt*) of test TRB is being measured, it primarily consists of the significant contributions from rolling-resistance torque (*Mrr*) and rib torque (*Msl*\_*rib*) while minimizing drag losses.

$$M_t \approx M_{rr} + M_{sl\_rib} \tag{20}$$

Using the SKF model (Equations (15)–(17)), the frictional torque of the rib-roller contact (*Msl*\_*rib*) is computed at the same experimental operating conditions and subtracted from the measured total frictional torque *M<sup>t</sup>* (Equation (20)).

$$M_{rr} \approx M_t - M_{sl_{rib}} \tag{21}$$

The value of *Mrr*, reconstructed from the measurements using Equation (21) is compared to the models of Aihara, Zhou, and Matsuyama in Section [4.3](#page-13-0) for a wide range of dimensionless load and speed parameters.

Machines 2023, 11, 801 11 of 22

### 2.4. Importance of Thermal Reduction Factor on Raceway Friction Prediction

At very high entrainment speeds, especially for non-zero values of the SRR, the lubricant in the contact inlet becomes heated  $(T_{in\_c})$ , leading to a reduction of its viscosity and film thickness and, consequently, the flow rate of the entrained lubricant. In TRB with low lubricant viscosity and surface velocity, the thermal loading parameter L is sufficiently small (L=0.1 in Equation (2)) for thermal effects to be negligible in inlet heating. In this case, conventional isothermal theory can be used. However, with high-speed and/or high-viscosity lubricants (L greater than 0.1), inlet heating can substantially reduce the film thickness and rolling friction [35]. Hence, the film thickness, as well as the rolling torque, depend significantly on the properties of the lubricant in the inlet region ( $T_{in\_c}$ ). Matsuyama [14] incorporated the shear heating correction  $\varphi_T \in [0, 1]$  into the calculation of the rolling torque  $M_{rr}$  as

$$M_{rr} = \varphi_T \cdot \left\{ \left[ 14 \cdot 2E' l R_e^2 U_i^{0.75} G_i^{-0.04} W_i^{0.08} \right] R_o + \left[ 14 \cdot 2E' l R_e^2 U_o^{0.75} G_o^{-0.04} W_o^{0.08} \right] R_i \right\}$$
(22)

For  $\varphi_T = 1$ , isothermal conditions prevail, whereas for  $\varphi_T < 1$ , thermal effects become increasingly more important. In this work, the shear heating correction  $\varphi_T$  is determined experimentally from the measured rolling torque  $M_{rr}$  using Equation (22).

# 3. Design of Experiments and Methodology

The following Algorithm 1, provides an overview of the experimental workflow. Prior to the experiments, a continuous run-in procedure had been performed [36], running the TRB at 10 different speeds increasing from 220 rpm to 2200 rpm, each for 1 h to get smooth (polish) raceways. During the run-in, a constant oil flow rate of 0.5 lpm was supplied to the TRB at a constant temperature of 30 °C. All experimental test series are conducted after run-in period.

### Algorithm 1. Procedure for thermal inlet study.

- ho Experiments to determine the optimal flow rate to minimize drag, i.e.,  $M_{drag} \ll M_t$  for  $W \approx 0$
- $\triangleright$  Experiments for all U, G & W:
  - $\triangleright$  Measure the total torque  $M_t$
  - $\triangleright$  Calculate  $M_{Sl_{rih}}$  using (Equations (15)–(17))
  - $hipsip Calculate <math>M_{rr} pprox M_t M_{sl_{rib}}$  (Equation (21))
  - $\triangleright$  Calculate thermal inlet shear heating factor  $\varphi_T = \varphi_T(M_{rr}, U, W, G)$  (Equation (22))

Result: Thermal inlet shear heating,  $\varphi_T$ , effect on raceway torque

Experiments were conducted to determine the optimal flow rate for minimizing drag in TRBs. The measurements involved varying flow rates (0.2 lpm to 0.5 lpm) and speeds (500 rpm to 2000 rpm) while keeping the load constant and controlling the oil supply temperature at 30  $^{\circ}$ C. Each test had a duration of 1 h, allowing for thermal equilibrium and steady-state frictional torques to be achieved. Experiments were carried out to investigate the effect of dimensionless velocity (U) and material (G) parameters on rolling re-resisting torque and thermal inlet shear effect of TRBs under different temperatures and speeds.

The specifications of the test TRB are explained in Section 2.2. Experiments are conducted to measure frictional torque and temperature within the supply oil temperature range from 35 °C to 65 °C (due to the limited value of  $\alpha$ ), speed from 200 rpm to 2200 rpm, and for two contact loads (9.6 and 12.85 kN). These experimental parameters were expressed in terms of dimensionless parameters, which differ for the inner and outer raceway connections. Hence U, G, and W provide the following representation (Equations (23)–(25)). Equation (24) is a dimensionless material parameter (G).  $\alpha$  is the pressure–viscosity coefficient (Barus coefficient) in the EHL regime and depends on the pressure and viscosity (oil temperature 35 to 65 °C) of the lubricating fluid. These conditions are equivalent to  $U = 2 \cdot 10^{-11}$  to  $1 \cdot 10^{-10}$ ,  $W = 1 \cdot 10^{-5}$ , and G = 3950 to 5000.

Machines 2023, 11, 801 12 of 22

$$U = \frac{\pi d_m n \eta_o}{60 D_a E'} \tag{23}$$

$$G = \alpha E' \tag{24}$$

$$W = \frac{2F_a}{D_a E' lzsin\alpha_o} \tag{25}$$

During these experiments and calculations, the experimentally characterized FVA3A oil properties are used (Table 3). The methodology for studying the dimensionless speed (U) and material parameters (G) in relation to rolling friction and calculating the thermal inlet shear heating effect in TRB raceway contacts is explained in the above Algorithm 1.

<span id="page-11-1"></span>**Table 3.** Oil FVA 3A properties.

| FVA 3A                   |                                                                       | Units             |
|--------------------------|-----------------------------------------------------------------------|-------------------|
| Oil type                 | Paraffin-based solvent raffinate                                      |                   |
| Density                  | 884.1                                                                 | kg/m <sup>3</sup> |
| Viscosity at 40 °C       | 90.02                                                                 | $mm^2/s$          |
| Viscosity at 100 °C      | 10.41                                                                 | $mm^2/s$          |
| Viscosity index          | 97                                                                    |                   |
| Viscosity-pressure       | $2.16 \times 10^{3}  \mathrm{bar^{-1}} \ @ \ 25  ^{\circ} \mathrm{C}$ |                   |
| Coefficient (at 200 MPa) | $1.58 \times 10^3  \mathrm{bar^{-1}} @  80  ^{\circ}\mathrm{C}$       |                   |

#### 4. Experimental Results

# <span id="page-11-0"></span>4.1. Determination of Optimum Oil-Flow Rate

<span id="page-11-2"></span>The effect of lubrication on TRB power consumption is influenced by viscosity, quantity, and mode of supply, affecting both power consumption [37] and TRB temperature. The first test series aims to determine the optimal oil-flow rate for circulation lubrication. Figure 9 shows steady-state average data of global frictional torque and temperature.

![](_page_11_Figure_10.jpeg)

![](_page_11_Figure_11.jpeg)

**Figure 9.** (a) Global frictional torque for different flow rates and TRB speed; (b) TRB outer race and oil out temperature for different flow rates and TRB speed.

Global frictional torque and power loss increase with the lubricant supply rate to the bearing (Figure 9a). However, TRB temperature (Figure 9b) shows a characteristic curve with a flow rate (1000 to 2500 rpm), where it decreases significantly at the beginning and then slightly as flow rate increases. The slight decrease is due to the effect of the supply oil outlet temperature (Figure 9b; SOO), where at very high flow rates, the temperature difference with the TRB temperature becomes minimal. Reducing the oil-flow rate from 0.5 to 0.2 lpm provides less benefit for 500 rpm compared to other speeds. At 500 rpm, the TRB's temperature has reached a steady state (37–40  $^{\circ}$ C) due to the balance between generated friction power and cooling power (Figure 10a).

*Machines* **2023**, *11*, 801 13 of 22

<span id="page-12-0"></span>![](_page_12_Figure_1.jpeg)

![](_page_12_Figure_2.jpeg)

**Figure 10.** (**a**) Power ratio for friction and cooling power; (**b**) Frictional torque compositions.

Increasing the oil-flow rate from 0.2 lpm to 0.3 lpm increases frictional torque by 9.15% and extracts 25.29% of heat from TRB (Figure [9a](#page-11-2),b). Further increasing to 0.3 lpm leads to a 17.63% increase in frictional torque, with 14.18% heat removal (Figure [9a](#page-11-2),b). Beyond 0.3 lpm, the heat-removal rate does not improve significantly, but the friction torque increases. For oil-flow rates above 0.5 lpm can lead to insufficient pressurization and increased shear heating, causing an increase in friction. The optimum-flow rate for reliable TRB operation is 0.3 lpm, which is chosen for the experiments mentioned in Sections [4.2,](#page-12-1) [4.3](#page-13-0) and [4.5.](#page-15-0)

At 0.3 lpm oil-flow rate, the oil level in the bearing cup is 25.50 mm (1.35 × bearing width). The SKF model was used to quantify drag friction contribution in the measured global friction under 0.3 lpm. Figure [10b](#page-12-0) shows computed friction compositions under fully flooded conditions and frictional torque at the raceway contacts dominant, compared to rib and drag friction. Thus, when experimentally analysing the EHL viscous rolling resistance at the roller-raceway contact of TRBs, the drag contributions are small enough and, therefore, not considered.

# <span id="page-12-1"></span>*4.2. Measurements of the Total Torque M<sup>t</sup>*

First, the so-called starting and running torques are measured and analyzed, and the calculated global friction is compared to that predicted by the SKF model. Under slow-speed or starved lubrication, rib-roller contacts have higher frictional forces due to inadequate lubricant film at lower speeds, making them dominant over roller-raceway contacts. The starting torque (Figure [11a](#page-13-1),b: Zone 1) of the TRBs is primarily influenced by the sliding and spinning motion of the roller ends at the rib.

However, the running torque (Figure [11a](#page-13-1),b: Zone 2) is mainly due to hysteresis and viscous-rolling resistance at the roller-raceway contacts, combined with slide-and-spinrelated resistance at the roller-rib contacts. At higher speeds, the slide-and-spin-related resistance decreases as a thicker hydrodynamic lubricant film forms [\[38\]](#page-21-27), substantially reducing friction loss at the roller-end and rib contact.

Figure [11](#page-13-1) shows the measured frictional torque, as well as the calculated torque using the SKF model. The viscosity is calculated at the outer-ring temperature. The trends are similar, but in Zone 1 (starting torque), the predicted value is 15% lower than the experimental value, possibly due to lubrication's influence on sliding friction constants utilized in the boundary lubrication calculations. In Zone 2 (running torque), the predicted values are within 5% to 8% of the experimental readings, with the sliding-friction coefficient assumed to be 0.002. The product exponent of rotational speed and operating viscosity is lower than the models in Table [1.](#page-3-0) However, the literature [\[7](#page-20-4)[,9](#page-21-28)[,16\]](#page-21-4) shows it depends significantly on the used lubricant and its additives. Considering 0.002 in the calculation may lead to a deviation in the order of about 5%.

Machines 2023, 11, 801 14 of 22

<span id="page-13-1"></span>![](_page_13_Figure_1.jpeg)

**Figure 11.** Starting and running torque of TRB and comparison with SKF model ((**a**,**b**) = supply oil temperature 55 and 65 degrees under 12.85 kN axial load).

# <span id="page-13-0"></span>4.3. Roller-Rib Sliding Torque $M_{sl_{rib}}$

Using Equations (15)–(17), the rib torque is calculated for an axial load of 12.85 kN under two different oil-supply temperatures, namely 55 °C and 65 °C. Figure 12a presents the estimated results, with the operating viscosity of the oil at 55 °C and 65 °C being 70.11 and 56.90 mm<sup>2</sup>/s, respectively. At an oil-supply temperature of 55 °C, the weighting factor of the sliding friction coefficient ( $\varphi_{bl}$ ; Equation (17)) is small, while the viscosity ratio (actual operating viscosity/rated viscosity) is large. As a result, the sliding rib friction is reduced compared to 65 °C. The calculated rib torque decreases as the rotational speed increases and remains constant after 1400 rpm due to the  $\varphi_{bl}$  value approaching zero. The calculated sliding friction in Figure 12a follows the same trend as experimentally measured by Matsuyama [14] and Zhou [13].

<span id="page-13-2"></span>![](_page_13_Figure_5.jpeg)

**Figure 12.** (a) Rib torque from Matsuyama model for an axial bearing load of 12.85 kN; (b) Computed drag loss.

#### 4.4. Rolling Resistance Torque M<sub>rr</sub>

The raceway's rolling-resistance torque ( $M_{rr}$ ) is obtained by subtracting the calculated roller-rib sliding torque ( $M_{sl_{rib}}$ ) from the total torque ( $M_t$ ), while neglecting the drag losses that were verified to be small (Figure 12b). The contribution of rib torque to raceway

*Machines* **2023**, *11*, 801 15 of 22

torque remains almost constant, and the raceway torque is due to a moment that acts in the direction to prevent the roller from rolling.

<span id="page-14-0"></span>Figure [13a](#page-14-0),b illustrate *Mrr* obtained from the experiments. Moreover, the torques predicted by the models of Aihara [\[12\]](#page-21-1), Zhou [\[13\]](#page-21-10), and Matsuyama [\[14\]](#page-21-2) are compared to the results. It is observed that the Aihara model corresponds with the measured values the least. In contrast, the Matsuyama model, which has been well-validated in a wide range of *U, G,* and *W,* agrees with our experimental results (Figure [13a](#page-14-0),b). The measured raceway torque remained linear at high *U,* but the models exhibit non-monotonically behaviour because the models mainly focus on the effect of load (*W*) on rolling torque and less on speed (*U*) parameters.

![](_page_14_Figure_3.jpeg)

![](_page_14_Figure_4.jpeg)

**Figure 13.** TRB rolling-resistance torque models and measured results (**a**,**b**) = supply oil temperature 55 and 65 degrees under 12.85 kN axial load [\[12](#page-21-1)[–14\]](#page-21-2).

Figure [14](#page-15-1) shows the relationship between raceway frictional torque and dimensionless speed parameters (*U*) under different *G* and *W* parameters. Figure [14a](#page-15-1)–d compares calculated (Table [1\)](#page-3-0) with experimental raceway torques.

Under fully flooded conditions, the relationship between the rolling-resistance torque (*Mrr*) and the dimensionless EHD parameters are shown in Figure [14a](#page-15-1)–d. *Mrr* exhibits an increase with the dimensionless speed parameter *U* (Figure [14a](#page-15-1)–d). As *U* increases, the center of EHL pressure distribution (Figure [7\)](#page-8-0) shifts significantly towards the inlet region, resulting in an increase of the rolling torque *Mrr*.

The effect of *G* on *Mrr* is limited. Indeed, *G* has only a minor influence on the offset of the pressure center in EHL. For small values of *W* in Figure [14a](#page-15-1),b), *Mrr* increases monotonically with *U*. For larger values of *W* (Figure [14c](#page-15-1),d)), *Mrr* first decreases with *U* after which it increases. This behavior is attributed to the thinner oil-film thickness resulting from the formation of mixed lubrication.

Due to the fixed-bearing geometry, the number of rollers and test rig loading constraint, the experimental analysis of *W* could not cover an extensive range. However, the results in previous studies (Table [1\)](#page-3-0) clearly indicate that the raceway torque indeed depends on the load. Still, this influence is much smaller compared to the influence of the velocity *U*.

Therefore, the dependence of the applied axial load on the EHL rolling friction is less significant, which is strongly agreed by all the models. Hence, under fully flooded conditions, the effects of *G* and *W* are small in the rolling friction, but the effect of *U* is large. *Machines* **2023**, *11*, 801 16 of 22

<span id="page-15-1"></span>![](_page_15_Figure_1.jpeg)

**Figure 14.** TRB rolling-resistance torque with dimensionless speed parameter (**a**–**d**) = Rollingresistance torque under 35, 45, 55 and 65 degrees [\[12–](#page-21-1)[14\]](#page-21-2).

# <span id="page-15-0"></span>*4.5. Thermal Inlet Shear Factor*

The thermal EHL factor plays an important role in predicting the raceway torque. Additionally, the film thickness depends on the properties of the lubricant in the inlet region, so this factor is essential for the calculation of the inlet temperature. This effect is a function of the bearing speed and operating viscosity of the oil. Equation (22) is used to calculate the thermal inlet shear factor from the measured rolling-resistance torque for all operating conditions (Figure [15\)](#page-16-0). The thermal shear factor (*ϕT*) is close to 1 at low speeds, meaning little shear heating. It decreases to 94% or 96% for higher speeds. This is mainly because SRR = 0. Sliding friction often exceeds rolling friction for low-viscosity lubricants, but the inverse is true for very high-viscosity lubricants [\[39\]](#page-21-29).

To assess the influence of the thermal inlet factor on the rolling-resistance torque, the measured value of the rolling torque *Mrr* is compared to the isothermal rolling torque value *Mrr*,*iso* = *Mrr*/*ϕ<sup>T</sup>* at 9.6 kN and a supply oil temperature of 55 ◦C (Figure [16\)](#page-16-1). Without the thermal inlet shear factor correction, the rolling-resistance torque would be overestimated by 6 to 8% at speeds above 1400 rpm (Figure [16\)](#page-16-1).

During TRB operation, the rolling-resistance torque ( *Mrr*) generates frictional forces and dissipates energy, leading to the production of heat. This frictional heat can have various consequences. Firstly, it raises the operating temperature of the TRB, which can cause thermal stress and potentially result in detrimental effects such as reduced lubricant performance and the occurrences of mixed lubrication. Excessive heat can also affect the overall efficiency and performance of the TRB system. To manage the frictional heat in bearings, it is crucial to optimize the design, lubrication, and operating conditions.

*Machines* **2023**, *11*, 801 17 of 22

<span id="page-16-0"></span>![](_page_16_Figure_1.jpeg)

**Figure 15.** Thermal inlet shear factor for different oil supply temperatures.

<span id="page-16-1"></span>![](_page_16_Figure_3.jpeg)

**Figure 16.** Influence of the thermal inlet factor on the rolling-resistance torque.

To determine the contributions of *Mrr* and *Mslrib* components to TRB frictional heat, the experiment is conducted under 12.85 kN and supply oil temperature of 55 ◦C to measure the global frictional torque. *Mrr*, reconstructed from the measurements using Equation (21) and computed *Mslrib* (Equations (15)–(17)) are the leading cause of TRBs to heat up. Ideally, rolling-element bearings operate under pure rolling with slide-to-roll ratios of less than 0.1% [\[40\]](#page-21-30). Sliding friction originates from these small amounts of micro-slip in the finitesized EHL contact and, thus, represents EHL friction. For lower speeds, the main source of heating is due to viscous and compressive heating in the core of the EHL contact. However, at higher speeds and fully flooded conditions, the recirculating lubricant flow in front of the EHL contact induces substantial shear heating in the inlet zone (Figure [17\)](#page-16-2).

<span id="page-16-2"></span>![](_page_16_Figure_6.jpeg)

**Figure 17.** Frictional power of rolling-resistance torque and sliding-rib torque.

Machines 2023, 11, 801 18 of 22

#### 5. Conclusions

In this article, experiments were conducted to determine the global frictional torque in a particular Tapered Roller Bearing (TRB). The frictional torque was thoroughly analysed by separating it into rolling and sliding friction contributions using analytical EHL rolling-friction models. The experimental measurements were found to align with the predictions of those theoretical rolling-friction models. Additionally, we investigated the influence of dimensionless parameters U, G, and W on rolling friction and examined the effect of inlet shear heating.

- In the first part of the study, experiments were performed to determine the optimal oilflow rate that minimizes drag-loss contributions in the global frictional torque while ensuring adequate lubrication and thermal equilibrium. Following that, a comparison was made between the global friction results and the global SKF friction model.
- The predicted global frictional loss by the SKF model for velocities below 400 rpm (referred to as the starting zone) were found to be 15% lower than the experimental values. However, for rotational velocities above 400 rpm (referred to as the running zone), the predicted values fell within a range of 5% to 8% of the experimental values. In this study, rolling friction and sliding-rib friction were identified as the primary contributors to the frictional torque of TRBs.
- The rolling-resistance torque of the TRB was measured for different operating conditions and compared to the theoretical EHL rolling-torque models of Table 1. The model of Matsuyama exhibited a strong predictive capability and demonstrated good agreement with the experimental results.
- Under fully flooded conditions, the EHL rolling torque  $M_{rr}$  exhibits a significant increase with increasing dimensionless speed parameter U. This is due to a significant shift of the pressure centre of the hydrodynamic pressure distribution towards the inlet, resulting in an increase in  $M_{rr}$ .
- The effect of the dimensionless material parameter G on the rolling torque is relatively small. As G increases, the rolling torque decreases for oil temperatures below 45 °C in this work. However, for oil temperatures above 45 °C, G slightly increases the raceway torque at lower G. The effects of G and G0 on G1 are minor, whereas the effect of G1 is significant.
- The thermal inlet shear factor plays a crucial role in rolling friction. For higher rotational velocities, the decrease of rotational torque due to shear heating was estimated to be in the order 6–8%.
- Analysis of frictional power reveals that the TRB experiences heating at low speeds is
  primarily due to mixed lubrication friction between the roller and rib contact. At higher
  velocities, ELH friction becomes dominant and rises quickly with velocity, whereas
  the rib-raceway friction decreases as it shifts from mixed to full film lubrication.

### 6. Patents

The proposed test setup is under IP protection and covered in European patent EP22170243.4.

**Author Contributions:** Test rig conceptualization, M.M. and W.O. Methodology, software and validation, M.M. Resources, W.O. Writing—original draft preparation, M.M., W.O., D.F. and P.D.B. Supervision, D.F. and P.D.B. Project administration, D.F. Funding acquisition, P.D.B. and D.F. All authors have read and agreed to the published version of the manuscript.

**Funding:** This research was funded by Fonds Wetenschappelijk Onderzoek (FWO), SBO project CONTACTLUB ('S006519N').

Data Availability Statement: Not applicable.

**Conflicts of Interest:** The authors declare no conflict of interest.

Machines **2023**, 11, 801 19 of 22

#### Nomenclature

ATF Automatic Transmission Fluid b Hertzian half width contact (m)

 $c_1, c_2, c_3$  and  $c_4$  Constants determined from the experiments.

 $d_w$  Pitch circle diameter (m)

ERib contact height and roller end (m)ExBase of natural logarithm = 2.718 $h_c$ Central film thickness of the oil (m)KThermal conductivity of the oil (W/m/°C)

L Effective roller length (m)

 $m_{i/o}$  Dimensionless viscous-rolling resistance

 $m_r$  Moment at the rib contact N Rotational speed (rpm)

P Pressure (Pa)

 $p_h$  Maximum contact pressure (Pa)  $r_{arm}$  Length of measuring lever arm (m)

 $r_{mean}$  Mean radius of TRB (m)  $u_r$  Mean surface velocity (m/s) W Load-per-unit length (Nm) X Coordinate of rolling direction  $x_{cp}$  X coordinate of center of pressure, m

B Exponent of asperity load BT Bearing temperature

*C<sub>i</sub>* Weighting factor used to integrate P

COF Coefficient of friction

D Dynamic load rating  $(9\pi^3/2)^{1/2}$ GUW<sup>-3/2</sup>

*D<sub>a</sub>* Mean roller diameter (m)

E' Equivalent young's modulus (Pa)EHL Elasto–Hydrodynamic Lubrication

 $F_a$  Axial load (N)

 $F_c$  Mean asperity contacts  $F_f$  Frictional force (N) Normal load (N)

 $F_r$  Load on the roller-end rib (N)

 $F_{si}$  Sliding friction in the raceway roller contact area (N)  $F_{so}$  Sliding friction in the raceway roller contact area (N)  $F_{sf}$  Sliding friction in the roller end-rib contact area (N)

 $F_t$  Tangential force (N)

G Dimensionless material parameter  $G_{\infty}$  Limiting elastic shear modulus  $(N/m^2)$  L Thermal-loading factor  $(L = \eta_0 \beta_0 u_r^2/k)$   $M_L$  Load-dependent frictional loss (Nm)  $M_O$  Load-independent frictional loss (Nm)

 $M_{i/o}$  Viscous-rolling resistance (Nm) of inner/outer raceway (Nm)

 $M_t$  Global frictional torque (Nm)  $M_{rr}$  Rolling-resistance torque (Nm)

 $M_{sl\_rib}$  Sliding friction in the roller end and rib contacts (Nm)

P Dimensionless pressure RBT Roller-bearing tribometer  $R_i$  Mean inner-raceway radius (m)  $R_o$  Mean outer-raceway radius (m)

 $\begin{array}{ll} \textit{R}_{1,2} & \text{Equivalent radius of roller-raceway (inner, outer) contacts (m)} \\ \textit{Re} & \text{Equivalent radius in the rolling direction (m)} \ \frac{1}{R_{e}} = \frac{1}{R_{1}} + \frac{1}{R_{2}} = \frac{R_{1} + R_{2}}{R_{2}R_{1}} \end{array}$ 

SRR Slide-to-roll ratio SOI Supply-oil inlet SOO Supply-oil outlet

TEHL Thermo-Elasto-Hydrodynamic Lubrication

*Machines* **2023**, 11, 801 20 of 22

| $T_{in\_c}$        | Temperature of oil at the entry of the Hertzian contact (°C).       |
|--------------------|---------------------------------------------------------------------|
| $T_r$              | Rolling tangential force (N)                                        |
| TRB                | Tapered roller bearing                                              |
| Н                  | Dimensionless oil-film thickness (m)                                |
| U                  | Dimensionless speed parameter                                       |
| W                  | Dimensionless load parameter                                        |
| X                  | Dimensionless coordinate                                            |
| $X_{cp}$           | Dimensionless x coordinate of center of pressure                    |
| Z                  | Number of rollers                                                   |
| U                  | Dimensionless speed parameter                                       |
| $\eta_o$           | Operating viscosity of the oil at atmospheric pressure (Pas)        |
| α                  | Pressure viscosity coefficient of lubricant (Pa <sup>-1</sup> )     |
| $eta_o$            | Temperature-viscosity coefficient of the lubricant (°C).            |
| $\lambda_r$        | Fluid film factor for rib contacts                                  |
| $\varnothing_c$    | Surface film constant                                               |
| $S_o$              | Critical shear stress of the material $(N/m^2)$                     |
| $\sigma_o$         | Yield stress of the material $(N/m^2)$                              |
| $\omega_r$         | Angular velocity of roller (rad/s)                                  |
| $\omega_i$         | Angular velocity of the inner ring (rad/s)                          |
| $arphi_T$ , rib    | Thermal reduction factor of rib contact                             |
| $arphi_{\_T}$      | Thermal reduction factor of raceway                                 |
| $arphi_{rs}$       | Kinematic replenishment/starvation reduction factor                 |
| $\mu_{rib}$        | Frictional coefficient of the rib                                   |
| $\mu_{EHL}$        | Sliding friction coefficient in full film                           |
| au                 | Shear stress $(N/m^2)$                                              |
| $	au_L$            | Limiting shear stress $(N/m^2)$                                     |
| $\gamma$ .         | Shear rate                                                          |
| $\mu_{bl}$         | Constant depending on speed; 0.12 for $n \neq 0$ ; 0.15 for $n = 0$ |
| $\varphi_{bl}$     | Weight factor for the sliding friction coefficient                  |
| $G_{rr} \& G_{sl}$ | Bearing type and geometry                                           |
| $\alpha_i$         | Inner raceway angle (rad)                                           |
| $\alpha_0$         | Outer raceway angle (rad)                                           |
| $\rho_m H_m$       | Dimensionless constant                                              |

# <span id="page-19-0"></span>Appendix A

<span id="page-19-1"></span>**Table A1.** Test-Bearing Geometry and Forces.

| Murch Wilson                     | $T_r = \left(\frac{1}{1 + 0.29L^{0.78}}\right) \frac{R_e W}{\alpha} exp\left(\left(\frac{52}{79} + \frac{1}{94}L^{\frac{20}{63}}\right) lnD - \left(\frac{114}{701} + ln\left(1 + \frac{86}{209}L^{\frac{70}{99}}\right)\right)$ |
|----------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Thermal inlet shear factor       | $\varphi_T = \frac{exp(1.06 \cdot 10^{-2} L^{0.317} lnD)}{1 + 0.411 L^{0.707}}$                                                                                                                                                  |
| Patir and Cheng                  | $\frac{F_c}{F_c} = exp(-1.8\lambda^{1.2})$                                                                                                                                                                                       |
| Hamrock Dowson's                 | $h_{mim} = 3.63U^{0.68}G^{0.49}W^{-0.073}(1 - e^{-0.68k})R_x$                                                                                                                                                                    |
| Bair and Winder's                | $\gamma^{\cdot} = \left(\frac{1}{G_{\infty}}\right) \tau^{\cdot} - \frac{\tau_L}{\eta} ln(1 - \frac{\tau}{\tau_L})$                                                                                                              |
| Reynolds                         | $H_i^3 \left( \frac{dp}{dx} \right) = \frac{3\pi^2 U\eta}{4W^2} [H_i - \frac{(\rho_m H_m)}{\rho}]$                                                                                                                               |
| Dimensionless oil-film thickness | $H_i = H_O + X_i^2 - \frac{1}{2\pi} \sum_{i=1}^n D_{ij} P_j$                                                                                                                                                                     |
| Force equilibrium                | $\frac{\pi}{2} = \sum_{i=1}^{n} C_i P_i$                                                                                                                                                                                         |

# <span id="page-19-2"></span>Appendix B. Test-Bearing Geometry and Forces

From Figures 4 and 5, the balance of forces and moments on a roller, the following equations are obtained,

$$D_a F_{si} + F_{sf} (D_a + lsin\gamma - e) M_i - M_o = 0$$
(A1)

*Machines* **2023**, *11*, 801 21 of 22

where *Fo, F<sup>i</sup>* , and *F<sup>r</sup>* act on the outer, inner and rib roller, respectively. *M* and *M<sup>o</sup>* EHL viscous-rolling resistance cause the frictional moment (viscous-rolling resistance). Frictional force *M*<sup>0</sup> , which acts on the inner ring of a roller, is provided as follows

$$M' = R_i F_{si} + F_{sf} \left( R_i + \frac{1}{2} sin\alpha_i + e cos\theta \right) + M_i = 0$$
 (A2)

Thus, the sum of torque acting on the inner ring from the rollers, that is, the frictional torque *M* acting on the bearing can be obtained by the following equation where *Z* is the number of rollers.

$$M = \sum M' = M'Z \tag{A3}$$

From Equations (A1)–(A3),

$$M = Z \frac{1}{D_a} [(R_i D_a) M_i + R_i M_o)] + Z \frac{1}{D_a} F_{sf} e(R_i + D_a cos\theta) + Z \frac{1}{D_a} F_{sf} l\left(\frac{D_a}{2} sin\alpha_i - R_i sin\gamma\right)$$
(A4)

Here,

$$\frac{D_a}{2}sin\alpha_i - R_i sin\gamma = \frac{D_a}{2} \cdot \frac{R_i}{OP} - R_i \frac{D_a}{2.OP} = 0$$

Hence, Equation (6) becomes,

$$M = M_i + M_o = Z \frac{1}{D_a} (R_o M_i + R_i M_o) + Z \frac{R_o}{D_a} e.F_{sf}$$
 (A5)

<span id="page-20-3"></span>**Appendix C. Sliding Rib Forces and Moment**

$$M_{sl\_rib} = Z \frac{R_o}{D_a} e F_{sf} \tag{A6}$$

$$M_{sl\_rib} = Z \frac{R_o}{D_a} e \mu F_r \tag{A7}$$

$$M_{sl\_rib} = Z rac{r_o}{D_w} e \mu rac{F_a sin 2 \gamma}{Z sin lpha}$$

$$M_{sl\_rib} = \frac{2R_o sin\gamma}{D_a sin\alpha} e\mu cos\gamma F_a \tag{A8}$$

$$OP = \frac{D_a}{2} \frac{1}{\sin \gamma} = 1$$

*Msl*\_*rib* = *e*µ*cosγFa*; Usually, *γ* very small so *cosγ* = 1

$$M_{sl\_rib} = e\mu F_a$$

# **References**

- <span id="page-20-0"></span>1. Dowson, D.; Higginson, G.R. Reflections on Early Studies of Elasto-Hydrodynamic Lubrication. *Solid Mech. Its Appl.* **2006**, *134*, 3–21. [\[CrossRef\]](https://doi.org/10.1007/1-4020-4533-6_1)
- 2. Dowson, D. Elastohydrodynamic and Micro-Elastohydrodynamic Lubrication. *Wear* **1995**, *190*, 125–138. [\[CrossRef\]](https://doi.org/10.1016/0043-1648(95)06660-8)
- <span id="page-20-1"></span>3. Gohar, R.; Cameron, A. The Mapping of Elastohydrodynamic Contacts. *Wear* **1968**, *11*, 387. [\[CrossRef\]](https://doi.org/10.1080/05698196708972181)
- <span id="page-20-2"></span>4. Neurouth, A.; Changenet, C.; Ville, F.; Arnaudon, A. Thermal Modeling of a Grease Lubricated Thrust Ball Bearing. *Proc. Inst. Mech. Eng. Part J J. Eng. Tribol.* **2014**, *228*, 1266–1275. [\[CrossRef\]](https://doi.org/10.1177/1350650114526387)
- 5. Niel, D.; Changenet, C.; Ville, F.; Octrue, M. A New Test Rig to Study Rolling Element Bearing Thermomechanical Behavior. In Proceedings of the International Gear Conference, Villeurbanne, France, 27–29 August 2018; pp. 121–133. [\[CrossRef\]](https://doi.org/10.13140/RG.2.2.34403.20000)
- 6. Pinel, S.I.; Signer, H.R.; Zaretsky, E.V. Design and Operating Characteristics of High-Speed, Small-Bore Ball Bearings. *Taylor Fr. Online* **2008**, *41*, 423–434. [\[CrossRef\]](https://doi.org/10.1080/10402009808983767)
- <span id="page-20-4"></span>7. Doll, G.L. Causes and Effects of Bearing Damage. In *Rolling Bearing Tribology*; Elsevier: Amsterdam, The Netherlands, 2023; pp. 205–231. [\[CrossRef\]](https://doi.org/10.1016/B978-0-12-822141-9.00011-X)
- 8. Doll, G.L. Mitigation of Rolling Bearing Damage Modes. In *Rolling Bearing Tribology*; Elsevier: Amsterdam, The Netherlands, 2023; pp. 233–252. [\[CrossRef\]](https://doi.org/10.1016/B978-0-12-822141-9.00006-6)

*Machines* **2023**, *11*, 801 22 of 22

<span id="page-21-28"></span>9. Dhanola, A.; Garg, H.C. Tribological Challenges and Advancements in Wind Turbine Bearings: A Review. *Eng. Fail. Anal.* **2020**, *118*, 104885. [\[CrossRef\]](https://doi.org/10.1016/j.engfailanal.2020.104885)

- 10. Hammami, M.; Martins, R.; Fernandes, C.; Seabra, J.; Abbes, M.S.; Haddar, M. Friction Torque in Rolling Bearings Lubricated with Axle Gear Oils. *Tribol. Int.* **2018**, *119*, 419–435. [\[CrossRef\]](https://doi.org/10.1016/j.triboint.2017.11.018)
- <span id="page-21-0"></span>11. Graf, S.; Werner, M.; Koch, O.; Götz, S.; Sauer, B. Breakdown Voltages in Thrust Bearings: Behavior and Measurement. *Tribol. Trans.* **2023**, *66*, 488–496. [\[CrossRef\]](https://doi.org/10.1080/10402004.2023.2185560)
- <span id="page-21-1"></span>12. Aihara, S. A New Running Torque Formula for Tapered Roller Bearings Under Axial Load. *J. Tribol.* **1987**, *109*, 471–477. [\[CrossRef\]](https://doi.org/10.1115/1.3261475)
- <span id="page-21-10"></span>13. Zhou, R.S.; Hoeprich, M.R. Torque of Tapered Roller Bearings. *J. Tribol.* **1991**, *113*, 590–597. [\[CrossRef\]](https://doi.org/10.1115/1.2920664)
- <span id="page-21-2"></span>14. Matsuyama, H.; Kamamoto, S. Analysis of Frictional Torque in Raceway Contacts of Tapered Roller Bearings. *KOYO Eng. J. Engl. Ed.* **2001**, *159*, 53–60.
- <span id="page-21-3"></span>15. SKF Group. *Rolling Element Bearings Catalogue*; SKF: Göteborg, Sweden, 2018; pp. 1–1722.
- <span id="page-21-4"></span>16. Harris, T.A.; Kotzalas, M.N. *Rolling Bearing Analysis Essential Concepts of Bearing Technology*; CRC Press: Boca Raton, FL, USA, 2006.
- <span id="page-21-5"></span>17. Palmgren, A. *Ball and Roller Bearing Engineering*; SKF Industries Inc.: Philadelphia, PA, USA, 1959.
- <span id="page-21-6"></span>18. Murch, L.E.; Wilson, W.R.D. A Thermal Elastohydrodynamic Inlet Zone Analysis. *J. Lubr. Technol.* **1975**, *97*, 212–216. [\[CrossRef\]](https://doi.org/10.1115/1.3452559)
- <span id="page-21-7"></span>19. Goksem, P.G.; Hargreaves, R.A. The Effect of Viscous Shear Heating on Both Film Thickness and Rolling Traction in an EHL Line Contact—Part I: Fully Flooded Conditions. *J. Lubr. Technol.* **1978**, *100*, 346–352. [\[CrossRef\]](https://doi.org/10.1115/1.3453183)
- <span id="page-21-8"></span>20. Goksem, P.G.; Hargreaves, R.A. The Effect of Viscous Shear Heating on Both Film Thickness and Rolling Traction in an EHL Line Contact—Part II: Starved Conditions. *J. Lubr. Technol.* **1978**, *100*, 353–358. [\[CrossRef\]](https://doi.org/10.1115/1.3453184)
- <span id="page-21-9"></span>21. Patir, N.; Cheng, H.S. An Average Flow Model for Determining Effects of Three-Dimensional Roughness on Partial Hydrodynamic Lubrication. *J. Lubr. Technol.* **1978**, *100*, 12–17. [\[CrossRef\]](https://doi.org/10.1115/1.3453103)
- <span id="page-21-11"></span>22. Bair, S.; Jacobson, B. Chapter 5 Rheological Models for Non-Newtonian Fluids. In Rheology and Elastohydrodynamic Lubrication; Tribology Series; Elsevier: Amsterdam, The Netherlands, 1991; Volume 19, pp. 53–68.
- <span id="page-21-12"></span>23. Matsuyama, H.; Kamamoto, S.; Asano, K. The Analysis of Frictional Torque for Tapered Roller Bearings Using EHD Theory. *SAE Trans.* **1998**, *107*, 320–329.
- <span id="page-21-13"></span>24. Matsuyama, H. High Efficiency and Tribology in Rolling Bearings. *JTEKT Eng. J.* **2012**, *1009*, 108–113.
- <span id="page-21-14"></span>25. Houpert, L. Ball Bearing and Tapered Roller Bearing Torque: Analytical, Numerical and Experimental Results. *Tribol. Trans.* **2002**, *45*, 345–353. [\[CrossRef\]](https://doi.org/10.1080/10402000208982559)
- <span id="page-21-15"></span>26. Gradu, M. Tapered Roller Bearings with Improved Efficiency and High Power Density for Automotive Transmissions. *SAE Tech. Pap.* **2000**, *109*, 1696–1705. [\[CrossRef\]](https://doi.org/10.4271/2000-01-1154)
- <span id="page-21-16"></span>27. Schwarz, M.; Liebrecht, J.; Gonda, A.; Sauer, B. A Study on the Frictional Torque and Temperature Behavior in Tapered Roller Bearings. *Forschungsvereinigung Antriebstechnik E.V.* **2018**, *3*, 31–39.
- <span id="page-21-17"></span>28. Liu, Y.; Fan, X.; Wang, J.; Liu, X. An Investigation for the Friction Torque of a Tapered Roller Bearing Considering the Geometric Homogeneity of Rollers. *Lubricants* **2022**, *10*, 154. [\[CrossRef\]](https://doi.org/10.3390/lubricants10070154)
- <span id="page-21-18"></span>29. Liu, X.; Long, T.; Li, X.; Guo, F. Thermal EHL Analysis of the Inner Ring Rib and Roller End in Tapered Roller Bearings with the Carreau Model. *Front. Manuf. Technol.* **2023**, *2*, 29. [\[CrossRef\]](https://doi.org/10.3389/fmtec.2022.1029860)
- <span id="page-21-19"></span>30. Van Wittenberghe, J.; Ost, W.; Rezaei, A.; De Baets, P.; Zsidai, L.; Kalácska, G. Test Setup for Friction Force Measurements of Large-Scale Composite Bearings. *Exp. Tech.* **2009**, *33*, 45–50. [\[CrossRef\]](https://doi.org/10.1111/j.1747-1567.2008.00371.x)
- <span id="page-21-20"></span>31. Tong, V.-C.; Hong, S.-W. The Effect of Angular Misalignment on the Running Torques of Tapered Roller Bearings. *Tribol. Int.* **2016**, *95*, 76–85. [\[CrossRef\]](https://doi.org/10.1016/j.triboint.2015.11.005)
- <span id="page-21-21"></span>32. Jiang, Z.; Huang, X.; Zhu, H.; Jiang, R.; Du, S. A New Method for Contact Characteristic Analysis of the Tapered Roller Bearing in Wind Turbine Main Shaft. *Eng. Fail. Anal.* **2022**, *141*, 106729. [\[CrossRef\]](https://doi.org/10.1016/j.engfailanal.2022.106729)
- <span id="page-21-22"></span>33. Liebrecht, J.; Si, X.; Sauer, B.; Schwarze, H. Investigation of Drag and Churning Losses on Tapered Roller Bearings. *Stroj. Vestn. J. Mech. Eng.* **2015**, *61*, 399–408. [\[CrossRef\]](https://doi.org/10.5545/sv-jme.2015.2490)
- <span id="page-21-23"></span>34. Marchesse, Y.; Changenet, C.; Ville, F. Drag Power Loss Investigation in Cylindrical Roller Bearings Using CFD Approach. *Tribol. Trans.* **2019**, *62*, 403–411. [\[CrossRef\]](https://doi.org/10.1080/10402004.2018.1565009)
- <span id="page-21-24"></span>35. Wilson, W.R.D.; Sheu, S. Effect of Inlet Shear Heating Due to Sliding on Elastohydrodynamic Film Thickness. *J. Lubr. Tech.* **1983**, *105*, 187–188. [\[CrossRef\]](https://doi.org/10.1115/1.3254563)
- <span id="page-21-25"></span>36. *NSK Rolling Bearings*; General Lubrication Engineering Practice; Springer: Berlin/Heidelberg, Germany, 2015.
- <span id="page-21-26"></span>37. Dindar, A.; Hong, I.; Garg, A.; Kahraman, A. A Methodology to Measure Power Losses of Rolling Element Bearings under Combined Radial and Axial Loading Conditions. *Tribol. Trans.* **2022**, *65*, 137–152. [\[CrossRef\]](https://doi.org/10.1080/10402004.2021.1937760)
- <span id="page-21-27"></span>38. Bhushan, B.; Ko, P.L. *Introduction to Tribology*; John Wiley & Sons: Hoboken, NJ, USA, 2013.
- <span id="page-21-29"></span>39. Vengudusamy, B.; Enekes, C.; Spallek, R. On the Film Forming and Friction Behaviour of Greases in Rolling/Sliding Contacts. *Tribol. Int.* **2019**, *129*, 323–337. [\[CrossRef\]](https://doi.org/10.1016/j.triboint.2018.08.026)
- <span id="page-21-30"></span>40. Spikes, H. Basics of EHL for Practical Application. *Lubr. Sci.* **2015**, *27*, 45–67. [\[CrossRef\]](https://doi.org/10.1002/ls.1271)

**Disclaimer/Publisher's Note:** The statements, opinions and data contained in all publications are solely those of the individual author(s) and contributor(s) and not of MDPI and/or the editor(s). MDPI and/or the editor(s) disclaim responsibility for any injury to people or property resulting from any ideas, methods, instructions or products referred to in the content.