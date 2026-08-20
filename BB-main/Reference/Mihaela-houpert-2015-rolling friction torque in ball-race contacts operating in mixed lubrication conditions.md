![](_page_0_Picture_1.jpeg)

*Article* 

# **Rolling Friction Torque in Ball-Race Contacts Operating in Mixed Lubrication Conditions**

**Mihaela Rodica D. Bălan 1 , Luc Houpert 2 , Ana Tufescu 1 and Dumitru N. Olaru 1,\*** 

- 1 Department of Mechanical Engineering, Mechatronics and Robotics, Technical University "Gheorghe Asachi" Iasi, Bld. D. Mangeron, 53, 700050 Iasi, Romania; E-Mails: ro\_balan@yahoo.com (M.R.D.B.); ana.tufescu@yahoo.com (A.T.)
- 2 TIMKEN Europe, 2 rue Timken, B.P. 60089, 68002 Colmar, France; E-Mail: houpert@timken.com
- **\*** Author to whom correspondence should be addressed; E-Mail: dolaru@mail.tuiasi.ro; Tel.: +4-0232-232337.

Academic Editors: Romeo P. Glovnea and Michel Fillon

*Received: 5 October 2014 / Accepted: 16 February 2015 / Published: 13 April 2015* 

**Abstract:** Based on a theoretical model and an experimental methodology for defining the friction torque for lubricated conditions in a modified thrust ball bearing having only three balls, the authors experimentally investigated the influence of the lubricant parameter Λ on friction torque for mixed IVR (isoviscous rigid) and EHL (elastohydrodynamic) lubrication conditions. The experiments were conducted using ball diameters of 3 mm, 3.97 mm and 6.35 mm loaded at 0.125 N, 0.400 N and 0.633 N. Two oils of viscosity 0.08 Pa·s and 0.05 Pa·s were used and rotational speed was varied in the range 60–210 rpm to obtain a lubricant parameter Λ varying between 0.3 and 3.2. The experiments confirmed that the measured friction torque can be explained using hydrodynamic rolling force relationships respecting the transition from an IVR to an EHL lubrication regime.

**Keywords:** rolling friction; lubricated contacts; thrust ball bearing; elastohydrodynamics

#### 1. Introduction

The rolling resistant moment of a ball on a raceway in a bearing is due to miscellaneous factors, including elastic hysteresis losses, curvature effects, pivoting effects, hydrodynamic lubricant resistance and roughness as described by Houpert in [1,2], but also form deviation effects. For low loaded ball-race contacts, Olaru *et al.* [3] demonstrated that the theoretical elastic hysteresis and curvature effect resistances do not exceed 12 percent of the experimentally rolling resistance moment.

The presence of lubricant in the rolling contact is responsible for a rolling moment increase due to the hydrodynamic rolling force FR calculated by integrating the Reynolds equation and accounting for the Poiseuille flow. Miscellaneous relationships for calculating FR have been suggested by Houpert [4] and, recently, by Biboulet and Houpert [5] who suggested relationships corresponding to fully flooded conditions, the meniscus distance  $x_e$  being located at -d/2 when conducting isoviscous rigid calculations or being a large multiple of b ( $x_e \approx -10 \cdot b$ , for example) when conducting elastohydrodynamic lubrication (EHL) calculations, where d is the ball diameter and b is the half contact width of the Hertzian contact.

The hydrodynamic rolling force relationships published by Houpert [4] correspond to moderately starved conditions; for example,  $x_e = -0.3 \cdot d/2$  in the IVR regime and  $x_e \approx -3 \cdot b$  in the EHL regime.

The relationships proposed by Biboulet and Houpert [5] are as follows.

In the IVR lubrication regime:

$$FR_{IVR} = 2.9766 \cdot E * \cdot R_x^2 \cdot k^{0.3316} \cdot W^{1/3} \cdot U^{2/3}$$
(1)

In the EHL lubrication regime:

$$FR_{EHL} = 7.5826 \cdot E * \cdot R_x^2 \cdot k^{0.4055} \cdot W^{1/3} \cdot U^{3/4}$$
 (2)

The transition from the IVR to the EHL lubrication regime is described by Biboulet and Houpert [5] using a single parameter *M*:

$$FR_{Trans} = \frac{FR_{IVR} - FR_{EHL}}{1 + \frac{M}{6.6}} + FR_{EHL}$$
(3)

The initial derivation of M was quite complex but has been simplified here by using some curve-fitted relationships described in [5]:

$$M = 0.5549 \cdot k^{-0.6029} \cdot W \cdot U^{-0.75} \tag{4}$$

Equation (3) can be expressed as a sum of two components  $FR_{IVR}$  and  $FR_{EHL}$  with different partitions from IVR and EHL conditions, respectively:

$$FR_{Trans} = [1/(1 + M/6.6)] \cdot FR_{IVR} + [(M/6.6)/(1 + M/6.6)] \cdot FR_{EHL}$$
(5)

where the IVR and EHL partitions are described by the factors [1/(1+M/6.6)] and [(M/6.6)/(1+M/6.6)], respectively.

The parameters included in Equations (1)–(4) can be defined as follows.

U is the dimensionless speed parameter:

$$U = \frac{\eta_0 \cdot v}{E^* \cdot R_x} \tag{6}$$

W is the dimensionless load parameter:

$$W = \frac{Q}{E^* \cdot R_x^2} \tag{7}$$

in which  $\eta_0$  is the oil dynamic viscosity in Pa·s at the operating temperature of the contact;  $v = (v_1 + v_2)/2$  is the average entrainment speed; m/s;  $E^*$  is the equivalent Young modulus of the two elements in contact (for steel ball-race contact,  $E^* \approx 2.3 \cdot 10^{11}$  Pa); and k is the radii ratio  $R_y/R_x$  with  $R_y$  defined as the equivalent radius of curvature in the y direction (perpendicular to the rolling direction) and  $R_x$  the equivalent radius in the rolling direction.

In order to demonstrate the influence of the lubricant on rolling friction in low load conditions, Bălan *et al.* [6] developed a comprehensive analytical model defining the total friction torque in a modified thrust ball bearing as a function of hysteresis losses, curvature effects, inertia effects, air churning effects on disc, and lubricant hydrodynamic rolling resistance described by Houpert [4] and Biboulet [5]. The experiments were conducted using three balls, 7.938 mm in diameter, in contact with two races having a race curvature radius of 4.16 mm. These experiments demonstrated that the hydrodynamic resistance is the main contributor to the final bearing torque in the tested operating conditions. Miscellaneous combinations of published (by Houpert and Biboulet) IVR and EHL hydrodynamic rolling forces have been tested. A very good correlation to the experimental results has been obtained when using Houpert's IVR and EHL forces [4] with Biboulet's transition proposal [5].

By reducing the ball diameter while keeping the same raceway and normal load as used in [6], Bălan *et al.* [7] observed that a good correlation to the experimental results is obtained when using Biboulet's IVR and EHL forces with Biboulet's transition proposal [5] as expected when increasing the oil meniscus distance.

Based on a theoretical model and an experimental methodology for defining the friction torque in a modified thrust ball bearing having only three balls and presented in [6], here the authors have experimentally investigated the influence of the hydrodynamic rolling force *FR* on the total friction torque at low normal loaded rolling contacts operating in mixed IVR and EHD lubrication conditions.

# 2. Theoretical Model for Friction Torque

Figure 1 shows the modified thrust ball bearing used by the authors to determine rolling friction torque. Three balls are mounted between the races of a thrust ball bearing at an equidistant angular position of 120 degrees. The lower race 1 is fastened to the rotating table, which can rotate at the angular speed  $\omega_1$ . A disc of known weight  $G_d$  is attached to the upper race 2, which is of weight  $G_r$ , in order to obtain an axial loading acting on the three balls. Because the balls are symmetrically located, each ball will take a load Q = G/3, where  $G = G_r + G_d$ . The spin-down method was used to determine the rolling friction torque between the balls and the two races. This method consists of imposing a constant angular speed on race 1 until race 2 and the attached upper disc reach a synchronous angular speed equal to that of race 1, as a result of the friction between the balls and the races.

Then, the rotational table and race 1 are suddenly stopped, while race 2 starts decelerating as a function of the friction torque of the six ball-race contacts.

The forces and moments acting on a ball during rolling contact between a ball and the two races are presented in Figure 2.

![](_page_3_Picture_3.jpeg)

Figure 1. Modified thrust ball bearing.

![](_page_3_Picture_5.jpeg)

Figure 2. Forces and moments acting on a ball as a result of rolling contact with two races.

 $FR_1$  and  $FR_2$  are the hydrodynamic braking rolling forces defined in miscellaneous lubrication regimes (IVR, EHL or transition between IVR and EHL) and are computed by using Equations (1)–(3).

 $FP_1$  and  $FP_2$  are the pressure forces due to the horizontal component of the lubricant pressure in the rolling direction. It can be reminded that FR is obtained by integrating  $dp/dx \cdot h/2$  while FP is obtained by integrating  $p \cdot dh/dx$ . Integrating the latter by parts, one obtains:

 $FP = \int p \cdot \frac{dh}{dx} = |p \cdot h|_{inlet\_meniscus}^{outlet\_meniscus} - \int \frac{dp}{dx} \cdot h = -2 \cdot FR$  because the pressure p is nil at the inlet and outlet meniscus.

So  $FP = 2 \cdot FR$  when pointing the FP arrow in the opposite direction relative to the FR arrow.

These two pressure forces do not contribute to the rolling moment around the ball center because they are applied at the center of the ball, but these two forces will be considered when using the force equilibrium in the *x* direction.

 $F_{ib}$  is the inertial force of the ball, acting at the ball center. If pure rolling between the balls and the two raceways is considered, this force is given by equation:

$$F_{ib} = -\frac{m_b \cdot r}{2} \cdot \frac{d\omega_2}{dt} \tag{8}$$

where  $m_b$  is the ball mass, r is the race radius (see Figure 1) and  $\omega_2$  is the angular speed of the upper race.

 $MER_1$  and  $MER_2$  are the rolling resistant moments due to elastic hysteresis losses in compression during the rolling process, operating in the Oy direction and can be determined by using [1]:

$$MER = 7.4810^{-7} \cdot \left(\frac{d}{2}\right)^{0.33} \cdot Q^{1.33} \cdot \left\{1 - 3.519 \cdot 10^{-3} \left(k - 1\right)^{0.8063}\right\}$$
 (9)

where d is ball diameter and Q is normal load.

Figure 2 also shows the ball pivoting friction moments  $MP_1$  and  $MP_2$  around the z-axis, normal to the center of the contact ellipse, defined in [1] for a large k ratio:

$$MP = \frac{3}{8} \cdot \mu_s \cdot Q \cdot a_c \tag{10}$$

where  $a_c$  is the major semi-axis of the contact ellipse and  $\mu_s$  is the average sliding friction coefficient. MP is, however, a function of k and can be approximated using:

$$MP \approx \left(\frac{3}{8} + \left[\frac{3.\pi}{16} - \frac{3}{8}\right]k^{-0.945 - 0.016 \cdot \ln(k)}\right) \cdot \mu_s \cdot Q \cdot a_c$$
 (11)

The factor 3/8 is valid for large k values, but is replaced by  $\frac{3.\pi}{16}$  when k is equal to 1.

The traction forces  $FS_1$  and  $FS_2$  are determined as a result of the equilibrium of the forces and moments acting on a ball. Details are presented in Appendix B.

Since both ball-upper race and ball-lower race contacts have identical geometry, speed and load conditions ( $MER_1 = MER_2$ ,  $MP_1 = MP_2$ ,  $FR_1 = FR_2$ ), the total tangential force developed between a ball and the upper race  $Ft_2$  obtained as a sum between forces  $FS_2$  and  $FR_2$  reads:

$$Ft2 = \frac{2 \cdot MER}{d} + 2 \cdot FR - \frac{Fib}{2} \tag{12}$$

For this modified thrust ball bearing with only three balls, the total friction torque acting on the upper race as a result of all three tangential forces and moments generated in the ball-race contacts can be written:

$$Tz = Tz_{MER} + Tz_{MP} + Tz_{FR} - Tz_I \tag{13}$$

The components of the rolling friction torque generated by hysteresis, pivoting effects, hydrodynamic effect and inertial effect are, respectively:

$$Tz_{MER} = 6 \cdot r \cdot MER/d$$
,  $Tz_{MP} = 3 \cdot MP$ ,  $Tz_{FR} = 6 \cdot r \cdot FR$ ,  $Tz_{I} = 3 \cdot r \cdot F_{ib}/2$  (14)

# **3. Experimental Procedure**

# *3.1. Experimental Equipment*

The experiments were carried out on tribometer CETR UMT 2 in the Tribology Laboratory of the Mechanical Engineering faculty of Iasi. Race 1 of the thrust ball bearing 51205 was fixed to the rotational table of the tribometer, so that it rotates with the table. On the raceway of lower race 1, the three balls were placed at equidistant angles (120 degrees). Upper race 2 was used to define the minimum axial load. To increase the normal load, two discs were attached to the upper race. No cage was used in the experiments. White marks were traced on race 1 and the upper race or on the disc in order to visualize the angular position for the two rotating elements. A video camera with 90 frames/second was mounted above the disc. The images captured by the camera were recorded on the computer in real time and subsequently processed with the program VIRTUAL DUB. A view of the testing equipment is presented in Figure 3.

![](_page_5_Picture_4.jpeg)

![](_page_5_Picture_5.jpeg)

![](_page_5_Picture_6.jpeg)

**Figure 3.** Experimental equipment: (**a**) main view of the tribometer; (**b**) the modified thrust ball bearing with attached disc mounted on the rotation table of the tribometer; (**c**) image of the rotating disc captured by camera.

The experiments were conducted using balls with diameters of 6.35 mm, 3.97 mm and 3 mm. The curvature radius of both raceways was 4.16 mm. In order to vary the normal load Q on the balls, two discs were attached to the upper race. By combining the upper race with the two discs, the following normal loads Q on every ball were obtained: 0.125 N (upper race only), 0.4 N and 0.633 N. The maximum contact Hertzian pressure between balls and raceways varied between 0.202 GPa and 0.76 GPa.

Two mineral oils with viscosities of 0.08 Pa·s and 0.05 Pa·s at the tested temperature (26–27 °C) were used for all the experiments. The oil quantity corresponds to two drops only to avoid drag losses.

The surface roughness parameter is  $Rq_r = 0.06 \mu m$  for the two rolling races and  $Rq_b = 0.03 \mu m$  for the balls. The rotational speed of the upper race varied between 60 and 210 rpm.

For every ball diameter and loaded disc, two series of experiments were conducted: in dry conditions and in lubricated conditions.

### 3.2. Experimental Methodology to Determine Friction Torque

The experimental methodology to determine the friction torque developed between the three balls and the upper race is based, both for dry and lubricated conditions, on the spin-down method.

## 3.2.1. Friction Torque in Dry Conditions

During the deceleration process of upper race 2 and the attached disc, the angular speed  $\omega_2$  decreases from an initial value  $\omega_{2,0}$  to zero during a time  $t_{\text{max}}$ . Using a dynamic balance of the moments acting on upper race 2 and the attached disc and disregarding the friction between disc and air, one obtains:

$$J \cdot \frac{d\omega_2}{dt} + Tz = 0 \tag{15}$$

where J is the moment of inertia of the system defined by upper race 2 and the attached disc, and Tz is the total friction torque generated by the three balls in contact with the upper race. For dry conditions, the total friction torque Tz includes only the following three components: friction torque generated by hysteresis ( $Tz_{MER}$ ), by pivoting effect ( $Tz_{MP}$ ) and by inertia ( $Tz_{I}$ ). As demonstrated in [6], the friction torque generated by inertia ( $Tz_{I}$ ) can be disregarded for rotational speeds between 60 and 210 rpm. In these circumstances, the total friction torque Tz is a sum of the two components  $Tz_{MER} + Tz_{MP}$ . In dry conditions, by imposing a constant value for the friction coefficient  $\mu_{S}$  it can be observed that both components  $Tz_{MER}$  and  $Tz_{MP}$  are not dependent on the rotational speed. Equation (14) can be easily integrated, resulting in the following relationships for variation of the angular speed  $\omega_{2}(t)$  and angular position  $\varphi_{2}(t)$ :

$$\omega_2(t) = \omega_{2,0} - \frac{T_z}{J} \cdot t \tag{16}$$

$$\varphi_2(t) = \omega_{2,0} \cdot t - \frac{T_z}{2J} \cdot t^2 \tag{17}$$

The time  $t_{\text{max}}$  and the total angular position  $\phi_{2,\text{max}}$  cumulated by the upper race and disc are defined experimentally. From Equation (17), by imposing the experimental values for  $t_{\text{max}}$  and  $\phi_{2,\text{max}}$  one can define the experimental values of Tz for every rotational speed in dry conditions:

$$T_Z = \frac{2 \cdot J \cdot (\omega_{2,0} \cdot t_{\text{max}} - \phi_{2,\text{max}})}{t_{\text{max}}^2}$$
(18)

## 3.2.2. Friction Torque in Lubricated Conditions

In lubricated conditions, Balan *et al.* [6] demonstrated that the friction torque generated by hysteresis ( $Tz_{MER}$ ) and inertia ( $Tz_I$ ) can be disregarded relative to the friction torque generated by hydrodynamic effect ( $Tz_{FR}$ ). It can also be demonstrated that the friction torque due to pivoting effect ( $Tz_{MP}$ ) is negligible in our tests. (See Appendix A.)

In these test conditions, the total friction torque Tz can then be expressed only as a function of hydrodynamic force FR:  $Tz = Tz_{FR} = 6 \cdot r \cdot FR$ . For a given geometry, lubricant and normal load, the hydrodynamic force FR is a function only of the rotational speed. The authors have suggested in [6] a nonlinear dependence between hydrodynamic force FR and angular speed  $\omega_2$ .

With these considerations, the differential Equation (15) becomes:

$$J.\frac{d\omega_2}{dt} = -K * \cdot \omega_2^n \tag{19}$$

where  $K^*$  is a constant parameter and the exponent is n < 1.

Equation (19) can be solved analytically to calculate the variation of the angular speed  $\omega_2(t)$  and angular position  $\varphi_2(t)$ :

$$\omega_2(t) = \left[ \omega_{2,0}^{1-n} - \frac{K^* \cdot (1-n)}{J} \cdot t \right]^{\frac{1}{1-n}}$$
(20)

$$\varphi_2(t) = \frac{J}{K^* \cdot (2-n)} \cdot \omega_{2,0}^{2-n} - \left[\omega_{2,0}^{1-n} - \frac{K^* \cdot (1-n)}{J} \cdot t\right]^{\frac{2-n}{1-n}}$$
(21)

The values for  $K^*$  and n were determined by solving the nonlinear Equations (20) and (21) with the following conditions obtained experimentally: (i) at the initial time t = 0,  $\omega_2(0) = \omega_{2,0}$ ; and (ii) at a time  $t_{\text{max}}$  defined experimentally as when the disc stops and the measured cumulative position angle  $\phi_{2,\text{max}}$  corresponds to the value given by the equation:  $\phi_2(t_{\text{max}}) = \phi_{2,\text{max}}$ .

For every experiment, both in dry and lubricated conditions, the values of  $t_{max}$  and  $\phi_{2,max}$  were obtained by analyzing the camera recorded results.

## 4. Experimental Results

Many experimental results, including the total friction torque Tz, the values for  $K^*$ , and the exponent n for lubricated conditions and lubrication parameter  $\Lambda$ , have been obtained following the methodologies presented previously for dry and lubricated conditions.

# *4.1. Total Friction Torque in Dry Conditions*

Figures 4–6 present the experimental total friction torque obtained in dry conditions for 6.35 mm, 3.97 mm and 3 mm diameter balls, respectively.

![](_page_8_Figure_3.jpeg)

**Figure 4.** Total friction torque in dry conditions for balls 6.35 mm in diameter.

![](_page_8_Figure_5.jpeg)

**Figure 5.** Total friction torque in dry conditions for balls 3.97 mm in diameter.

![](_page_9_Figure_1.jpeg)

**Figure 6.** Total friction torque in dry conditions for balls 3 mm in diameter.

The measured total friction torque shows no important variation between 60 rpm and 210 rpm, in line with the assumption of constant friction in dry conditions. Some increases of the total friction torque with rotational speed were observed in the 3 mm diameter balls.

# *4.2. Variation of the Lubricant Parameter Λ*

The lubrication parameter *Λ* is defined as a function of the minimum film thickness and roughness parameters *Rq* by using the equations presented in [6]. The values of the lubricant parameter *Λ* for all experiments are presented in Figures 7 and 8 for oil viscosity η0, equal to 0.05 Pa·s and 0.08 Pa·s, respectively.

![](_page_9_Figure_6.jpeg)

**Figure 7.** Lubricant parameter Λ calculated with η0 = 0.05 Pa·s.

![](_page_10_Figure_1.jpeg)

**Figure 8.** Lubricant parameter Λ calculated with η0 = 0.08 Pa·s.

In most experiments (except the experiments conducted with balls of 6.35 mm diameter, loaded at 0.125 N and operating at 210 rpm), the lubricant parameter *Λ* is smaller than 3—meaning that mixed lubrication conditions are simulated.

# *4.3. Total Friction Torque in Lubricated Conditions*

Figures 9 and 10 present the total friction torque experimentally determined in both dry and lubricated conditions for 6.35 mm and 3.97 mm diameter balls. Three normal loads were tested with the oil viscosity of 0.05 Pa·s.

The following remarks can be made:

- (i) By including a very small quantity of oil (two drops), the total friction torque increases substantially as result of the hydrodynamic force *FR*. It can be observed that total friction torque in dry conditions represents about 2% to 12% of the total friction torque measured in lubricated conditions.
- (ii) The total friction torque increases when the rotational speed is increased, and the exponent *n* used in Equations (19)–(21) has a value varying between 0.11 and 0.65 depending on the lubricant parameter *Λ*. Table 1 shows the values for *K\** and *n* corresponding to small values of the parameter *Λ* (*Λ* = 0.3 to 0.8) obtained with 3 mm diameter balls and loaded at *Q* = 0.633 N. It can be observed that the exponent *n* varies between 0.11 and 0.36. By increasing the lubricant parameter *Λ*, the value of the exponent *n* increases. Table 2 shows the values for *K\** and *n* corresponding to large values of the parameter *Λ* (*Λ* = 0.6 to 3.2) obtained with 6.35 mm diameter balls and loaded at *Q* = 0.125 N.

![](_page_11_Figure_1.jpeg)

**Figure 9.** Total friction torque in dry and lubricated conditions: *d* = 6.35 mm, η0 = 0.05 Pa·s.

![](_page_11_Figure_3.jpeg)

**Figure 10.** Total friction torque in dry and lubricated conditions: *d* = 3.97 mm, η0 = 0.05 Pa·s.

**Table 1.** Experimental values for *K\** and *n* with 3 mm diameter balls. *Q* = 0.633 N, *d* = 3 mm, η0 = 0.05 Pa·s.

| Initial Rotational Speed, rpm | *<br>2 ω n<br>Tz<br>K <br>exp                                |
|-------------------------------|----------------------------------------------------------------|
| 60                            | ω <br>5<br>0.116<br>Tz<br><br>4.001 10<br><br><br>exp<br>2 |
| 90                            | ω <br>5<br>0.277<br>Tz<br><br>3.287 10<br><br><br>exp<br>2 |
| 120                           | ω <br>5<br>0.318<br><br><br><br>Tz<br>2.864 10<br>exp<br>2 |
| 150                           | ω <br>5<br>0.366<br>Tz<br><br>2.796 10<br><br><br>exp<br>2 |
| 180                           | ω <br>5<br>0.344<br>Tz<br><br>3.21 10<br><br><br>exp<br>2  |
| 210                           | ω <br>5<br>0.339<br><br><br><br>Tz<br>3.213 10<br>exp<br>2 |
|                               |                                                                |

| Initial Rotational Speed, rpm | *<br>2 ω n<br>Tz<br>K <br>exp                                |
|-------------------------------|----------------------------------------------------------------|
| 60                            | ω <br>5<br>0.652<br>Tz<br><br>4.208 10<br><br><br>exp<br>2 |
| 90                            | ω <br>5<br>0.647<br>Tz<br><br>3.839 10<br><br><br>exp<br>2 |
| 120                           | ω <br>5<br>0.620<br><br><br><br>Tz<br>3.943 10<br>exp<br>2 |
| 150                           | ω <br>5<br>0.654<br>Tz<br><br>3.779 10<br><br><br>exp<br>2 |
| 180                           | ω <br>5<br>0.622<br><br><br><br>Tz<br>3.457 10<br>exp<br>2 |

210 5 0.649

exp 2 *Tz* 3.354 10 ω

**Table 2.** Experimental values for *K\** and *n* with 6.35 mm diameter balls. *Q* = 0.125 N, *d* = 6.35 mm, η0 = 0.05 Pa·s.

# *4.4. Validation of the Analytical Friction Torque in Lubricated Conditions*

The experimental values of the total friction torque were compared to the theoretical model developed in Section 2. The theoretical total friction torque *Tz* was calculated according to Equations (13) and (14) by considering only the hydrodynamic effect, hence disregarding the rolling friction torque generated by hysteresis, pivoting and inertial effect:

$$Tz = 6 \cdot r \cdot FR \tag{22}$$

The hydrodynamic rolling force *FR* is calculated using Equations (1)–(3). The calculated and experimental torques are compared in the next figures. The experimental final friction torque given in the following figures is defined as the difference between the total friction torque, defined in lubricated conditions, and the total friction torque defined in dry conditions.

Also, the EHL partition determined according to the relationship (*M* / 6.6)/(1 *M* / 6.6) is indicated in the figures. The following figures show the theoretical and experimental total friction torques *Tz* as well as the EHL partition expressed as a percent (on the second *y* axis).

Figures 11–14 show the variations of the total experimental and analytical friction torque *Tz* defined using 6.35 mm diameter balls and a normal load varying between 0.633 N and 0.125 N. For normal loads of 0.633 N and 0.400 N, the EHL partition exceeded 50% (using a viscosity of 0.08 Pa·s or 0.05 Pa·s) and the experimental results are in good agreement with the calculated ones using Equation (5), named in the figures as "Biboulet's transition equation". By decreasing the normal load to 0.125 N, IVR becomes the dominant partition (the EHL partition is between 40% and 20%) and the best theoretical/experimental match is obtained by using Equation (1), named in the figures as "Biboulet's IVR equation". (See Figure 14 showing both Biboulet's IVR and Biboulet's transition results.)

Figures 15–18 show the variations of the total experimental and analytical friction torque *Tz* defined using 3.97 mm diameter balls with a normal load varying between 0.633 N and 0.125 N. For normal loads of 0.633 N and 0.400 N and viscosities of 0.08 Pa·s and 0.05 Pa·s, the EHL partition exceeds 70% and the experimental results are close to those obtained using Biboulet's transition equation. By decreasing the normal load to 0.125 N, the EHL partition decreases and the IVR partition varies between 30% and 60% with a good theoretical correlation to the experimental results obtained when using Biboulet's IVR equation. (See Figure 18 showing both Biboulet's IVR and Biboulet's transition results.)

![](_page_13_Figure_1.jpeg)

**Figure 11.** Total friction torque for: *d* = 6.35 mm, *Q* = 0.633, η0 = 0.08 Pa·s.

![](_page_13_Figure_3.jpeg)

**Figure 12.** Total friction torque for: *d* = 6.35 mm, *Q* = 0.633 N, η0 = 0.05 Pa·s.

![](_page_13_Figure_5.jpeg)

**Figure 13.** Total friction torque for: *d* = 6.35 mm, *Q* = 0.400 N, η0 = 0.05 Pa·s.

![](_page_14_Figure_1.jpeg)

**Figure 14.** Total friction torque for: *d* = 6.35 mm, *Q* = 0.125 N, η0 = 0.05 Pa·s.

![](_page_14_Figure_3.jpeg)

**Figure 15.** Total friction torque for: *d* = 3.97 mm, *Q* = 0.633 N, η0 = 0.08 Pa·s.

![](_page_14_Figure_5.jpeg)

**Figure 16.** Total friction torque for: *d* = 3.97 mm, *Q* = 0.633 N, η0 = 0.05 Pa·s.

![](_page_15_Figure_1.jpeg)

**Figure 17.** Total friction torque for: *d* = 3.97 mm, *Q* = 0.400 N, η0 = 0.08 Pa·s.

![](_page_15_Figure_3.jpeg)

**Figure 18.** Total friction torque for: *d* = 3.97 mm, *Q* = 0.125 N, η0 = 0.05 Pa·s.

Figures 19–22 present the variations of the total experimental and analytical friction torque *Tz* defined using 3 mm diameter balls and a normal load varying between 0.633 N and 0.125 N. For normal loads of 0.633 N and 0.4 N (viscosities of 0.08 Pa·s and 0.05 Pa·s), the EHL partition exceeds 70%–80%, and the experimental results are in good agreement with the ones obtained using Biboulet's transition equation. By decreasing the normal load to 0.125 N, the EHL partition decreases and the IVR partition varies from 20% to 40%. The experimental value for *Tz* then varies between the theoretical ones calculated using Biboulet's transition equation and Biboulet's IVR equation (see Figure 22).

![](_page_16_Figure_1.jpeg)

**Figure 19.** Total friction torque for: *d* = 3 mm, *Q* = 0.633 N, η0 = 0.08 Pa·s.

![](_page_16_Figure_3.jpeg)

**Figure 20.** Total friction torque for: *d* = 3 mm, *Q* = 0.633 N, η0 = 0.05 Pa·s.

![](_page_16_Figure_5.jpeg)

**Figure 21.** Total friction torque for: *d* = 3 mm, *Q* = 0.400 N, η0 = 0.08 Pa·s.

![](_page_17_Figure_1.jpeg)

**Figure 22.** Total friction torque for: *d* = 3 mm, *Q* = 0.125 N, η0 = 0.05 Pa·s.

# *4.5. Comments*

The experimental investigations presented in [6] were achieved by using balls 7.938 mm in diameter and similar raceways with a curvature radius of 4.16 mm corresponding to a conformity (*Rc/d*) of 0.524 or *k* ratio of 21.78. By using a very small quantity of oil (two or three drops), the inlet meniscus of the oil from the ball-race contact was reduced and the experimental total friction torque was in good agreement with Houpert's transition equation calculated in moderately starved conditions: *x* 0.3 (*d* / 2) *<sup>e</sup>* in the IVR regime and *xe* 3 in the EHL regime. *b*

In the present study, by reducing the ball diameter to 6.35 mm, 3.97 mm or 3 mm and by using the same upper and lower raceway radii, important increases in the free spaces between the balls and raceways can be obtained. In these circumstances, when using the same oil quantity, the inlet meniscus of the oil from the ball-race contacts increases and the Biboulet's transition equation for *FR* (calculated using fully flooded conditions *xe* 10 in EHL, for example, and *b xe* = −*d/*2 in IVR) appears more appropriate for obtaining a good match between the experimental and theoretical total friction torque *Tz*, especially when the EHL lubrication conditions are dominant (*i.e.*, when the EHL partition exceeds 50%).

For a very low normal load (*Q* = 0.125 N), IVR is the dominant regime for all ball diameters. In this case, Biboulet's IVR equation is recommended for matching the experimental total friction torque *Tz*.

# **5. Conclusions**

By using a theoretical model and an experimental methodology for defining the friction torque in a modified thrust ball bearing having only three balls, the authors investigated the total friction torque in dry and mixed EHL and IVR lubrication conditions. Small balls with 6.35 mm, 3.97 mm and 3 mm diameters were used, loaded with normal loads of 0.633 N, 0.400 N and 0.125 N. Two lubricants of viscosity 0.08 Pa·s and 0.05 Pa·s were used in the experiments. The rotational speed varied between 60 rpm and 210 rpm to obtain a lubrication parameter *Λ* varying between 0.3 and 3.2, thereby simulating mixed lubrication conditions.

To demonstrate the effect of the hydrodynamic rolling force *FR* on the total friction torque, the authors compared the experimental results obtained in lubricated conditions and dry conditions. For dry conditions, the authors developed a simplified methodology to evaluate the total friction torque.

It was shown that the total friction torque measured in mixed lubricated conditions exceeds the total dry friction torque by more than an order of magnitude.

The experimental total friction torque was compared to the theoretical torque calculated using Biboulet's transition equations, and a good correlation was obtained when the dominant lubrication regime was EHL.

When the dominant regime was IVR, the experimental total friction torque was in good correlation with Biboulet's IVR equation.

# **Author Contributions**

All the authors contributed to the theoretical model elaboration and interpretation of the experimental results. The experiments were carried out by the authors from Iasi.

# **Appendix A**

When simulating lubricated conditions, the integration of the differential Equation (15) is conducted by considering only the effect of the hydrodynamic force *FR*; all the other effects being disregarded because they are very small, as shown next.

Figures A1 and A2 present the components of the total friction torque *Tz* for balls of 3 mm and 6.35 mm, respectively, operating at a normal load *Q* = 0.633 N.

![](_page_18_Figure_10.jpeg)

**Figure A1.** The friction torque components for: *d* = 3 mm, *Q* = 0.633 N, η0 = 0.05 Pa·s.

![](_page_19_Figure_1.jpeg)

Figure A2. The friction torque components for: d = 6.35 mm, Q = 0.633 N,  $\eta_0 = 0.05$  Pa·s.

The hydrodynamic component  $Tz_{FR}$  was determined according to the methodology presented in Paragraph 3.2.2.

The inertial component  $Tz_I$  was calculated according to the relationship  $Tz_I = 3 \cdot r \cdot F_{ib}/2$ , where the inertial force  $F_{ib}$  was calculated using Equation (8). The angular deceleration  $\frac{d\omega_2}{dt}$  was determined from Equation (20) using the maximum deceleration value (at the start of the deceleration process of the upper disc). Values between 0.5 and 3.3 rad/s<sup>2</sup> have been obtained for all the experiments.

The friction torque generated by pivoting motion was determined using  $Tz_{MP} = 3 \cdot MP$ , where Equation (11) was used for the pivoting moment. For defining the friction coefficient  $\mu_s$  in mixed lubrication conditions, we used the following Equation (1):

$$\mu_S = \mu_{EHL} \cdot \left[ 1 - \frac{Q_a}{Q} \right] + \mu_a \cdot \frac{Q_a}{Q} \tag{A1}$$

where  $\mu_{EHL}$  is the friction coefficient due to the shearing of the lubricant film thickness,  $\mu_a$  is the friction coefficient due to the asperities contacts,  $Q_a$  is the load carried by the asperities and Q is the total normal load.

The ratio  $Q_a/Q$  is determined as a function of the lubrication parameter  $\Lambda$  [6]:

$$\frac{Q_a}{Q} = 1 - \frac{1.21 \cdot \Lambda^{0.64}}{1 + 0.37 \cdot \Lambda^{1.26}} \tag{A2}$$

If the rolling contacts operated in full film conditions,  $\mu_S = \mu_{EHL}$  was fixed to a value of 0.03 while  $\mu_a$  was fixed to 0.11 [6].

The friction torque generated by hysteresis effect  $Tz_{MER}$  is calculated by the relationship  $Tz_{MER} = 6 \cdot r \cdot MER / d$  where the rolling resistance moment MER is given by Equation (9) and is not dependent on rotational speed.

For the 3 mm balls, the sum of the torques generated by hysteresis ( $Tz_{MER}$ ), pivoting motion ( $Tz_{MP}$ ) and inertia ( $Tz_{I}$ ) is about 5.6% to 2.6% of the hydrodynamic friction torque ( $Tz_{FR}$ ) when the rotation speed varies between 60 and 210 rpm.

For the 6.35 mm balls, the sum of the torques generated by hysteresis ( $Tz_{MER}$ ), pivoting motion ( $Tz_{MP}$ ) and inertia ( $Tz_{I}$ ) is about 2.6% to 0.4% of the hydrodynamic friction torque ( $Tz_{FR}$ ) when the rotation speed varies between 60 and 210 rpm.

These are the maximum values because when the normal loads are 0.400 N and 0.125 N, hysteresis and pivoting torques decrease.

As a consequence, the three components of the total friction torque Tz ( $Tz_{MER}$ ,  $Tz_{MP}$  and  $Tz_{I}$ ) have been disregarded in our analysis.

### Appendix B

 $FS_1$  and  $FS_2$  are the traction forces (driving the balls against all the previously described braking forces and moments) developed at the ball-race contacts.

Using the ball force and the moment equilibrium, the following analytical relationships have been derived in [1,6] for calculating the traction forces  $FS_1$  and  $FS_2$ :

$$FS_1 = \frac{MER_1 + MER_2}{d} + \frac{Fib}{2} + FR_2$$
 (B1)

$$FS2 = \frac{MER_1 + MER_2}{d} - \frac{Fib}{2} + FR_1$$
 (B2)

At the contact between the ball and the upper race 2, the hydrodynamic forces  $FR_2$  are applied on both surfaces (ball and race 2) in the same direction while the traction forces  $FS_2$  applied on race 2 and ball are pointing in the opposite direction (in virtue of the action reaction principle), see [6]. As a consequence, a final tangential force  $Ft_2$  (used for calculating the final torque) applied on race 2 can be calculated by summing the two components  $FS_2$  and  $FR_2$ :

$$Ft_2 = \frac{MER_1 + MER_2}{d} + FR1 + FR2 - \frac{Fib}{2}$$
 (B3)

For the modified thrust ball bearing having only three balls, the total friction torque acting on the upper race as a result of the all tangential forces and moments generated in the ball-race contacts is finally given by using 3 times  $Ft_2$ .

#### **Conflicts of Interest**

The authors declare no conflict of interest.

#### References

- 1. Houpert, L. Numerical and Analytical Calculations in Ball Bearings. In Proceeding of 8th European Space Mechanism and Tribology Symp., Toulouse, France, 29 September–1 October 1999.
- 2. Houpert, L. Ball Bearing and Tapered Roller Bearing Torque: Analytical, Numerical and Experimental Results. *STLE Tribol. Trans.* **2002**, *45*, 345–353.

3. Olaru, D.; Stamate, C.; Dumitrascu, A. Prisacaru Gh. New micro tribometer for rolling friction. *Wear* **2011**, *271*, 842–852.

- 4. Houpert, L. Piezoviscous-rigid rolling and sliding traction forces. Application: the rolling element-cage pocket contact. Proceedings of the 1986 ASLE/ASME Conference, Pittsburgh. *ASME J. Tribol.* **1987**, *109*, 363–371.
- 5. Biboulet, N.; Houpert, L. Hydrodynamic force and moment in pure rolling lubricated contacts: Part 2, Point contacts. *Proc. Inst. Mech. Eng. Part J J. Eng. Tribol.* **2010**, *224*, 777–787.
- 6. Bălan, M.R.; Stamate, V.C.; Houpert, L.; Olaru, D. The influence of the lubricant viscosity on the rolling friction torque. *Tribol. Int.* **2014**, *72*, 1–12.
- 7. Bălan, M.R.; Stamate, V.C.; Houpert, L.; Tufescu, A.; Olaru, D.N. Influence of the Geometry on the Rolling Friction Torque in Lubricated Ball-Race Contacts. *Appl. Mech. Mater. Adv. Concepts Mech. Eng. I* **2014**, *658*, 271–276.
- © 2015 by the authors; licensee MDPI, Basel, Switzerland. This article is an open access article distributed under the terms and conditions of the Creative Commons Attribution license (http://creativecommons.org/licenses/by/4.0/).