![](_page_0_Picture_2.jpeg)

![](_page_0_Picture_3.jpeg)

### OPEN ACCESS

EDITED BY Yebing Tian, Shandong University of Technology, China

REVIEWED BY

Aqib Mashood Khan, Nanjing University of Aeronautics and Astronautics, China Lixiao Wu, Lanzhou University of Technology, China Van-Canh Tong, Samsung Display Vietnam, Vietnam Bing Liu, Shandong University of Technology, China

\*CORRESPONDENCE Xiaoling Liu, [liu\\_xiaoling06@126.com](mailto:liu_xiaoling06@126.com)

SPECIALTY SECTION

This article was submitted to Material Forming and Removal, a section of the journal Frontiers in Manufacturing Technology

RECEIVED 28 August 2022 ACCEPTED 16 December 2022 PUBLISHED 10 January 2023

Liu X, Long T, Li X and Guo F (2023), Thermal EHL analysis of the inner ring rib and roller end in tapered roller bearings with the Carreau model. Front. Manuf. Technol. 2:1029860. doi: [10.3389/fmtec.2022.1029860](https://doi.org/10.3389/fmtec.2022.1029860)

© 2023 Liu, Long, Li and Guo. This is an open-access article distributed under the terms of the [Creative Commons](https://creativecommons.org/licenses/by/4.0/) [Attribution License \(CC BY\)](https://creativecommons.org/licenses/by/4.0/). The use, distribution or reproduction in other forums is permitted, provided the original author(s) and the copyright owner(s) are credited and that the original publication in this journal is cited, in accordance with accepted academic practice. No use, distribution or reproduction is permitted which does not comply with these terms.

# [Thermal EHL analysis of the inner](https://www.frontiersin.org/articles/10.3389/fmtec.2022.1029860/full) [ring rib and roller end in tapered](https://www.frontiersin.org/articles/10.3389/fmtec.2022.1029860/full) [roller bearings with the Carreau](https://www.frontiersin.org/articles/10.3389/fmtec.2022.1029860/full) [model](https://www.frontiersin.org/articles/10.3389/fmtec.2022.1029860/full)

Xiaoling Liu\*, Tao Long, Xinming Li and Feng Guo

School of Mechanical and Automotive Engineering, Qingdao University of Technology, Qingdao, China

The roller end/rib contact of tapered roller bearings significantly affects lubricating condition and power loss. To improve the lubrication performance of the inner ring rib and the large end of the roller in tapered roller bearings used in railway coaches, based on the structural analysis of the inner rib and the large end of the roller and considering spin–slide effects between the rib and the large end of the roller, a thermal elastohydrodynamic lubrication model with a Carreau rheological model was established in a tapered roller bearing. Two kinds of rib structures were provided: the tapered rib and spherical rib. Under different conditions, variations in the friction coefficient versus the ratio of curvature radius of the large end of the roller to that of the rib were compared, and the film thickness and film temperature varied with the rotational speed and the effect of load was compared between the two rib structures. Results showed that spinning motion has little effect on the lubrication at the contact point between the inner ring rib and the large end of the tapered roller. There exists an optimal ratio of the curvature radius between the large end of the roller and the spherical or tapered rib; moreover, the friction coefficient corresponding to this optimal ratio value is the smallest. With the increase in the inner ring speed, both film thickness and temperature increase for the two rib structures. Different from the spherical rib, the difference between the minimum and the central film thickness is almost unchangeable, and the tapered rib shows a slight temperature rise. As the load increases, the difference between the minimum and the central film thickness becomes larger, and the temperature in the contact zone gradually increases for the two ribs. Different from the tapered rib, the lower frictional coefficient and lower minimum film thickness are generated for the spherical rib because of higher film temperature.

KEYWORDS

tapered roller bearing, Carreau model, inner rib structures, roller end, thermal EHL

# 1 Introduction

A tapered roller bearing has the ability to carry combinations of large radial and thrust load or to carry only a thrust load. Moreover, due to large stiffness and convenient installation, it has wide applications in automobile, mining, metallurgy, and mechanical industries.

Because the contact angle is different between the inner and outer raceway, a contact force exists between the large end of the roller and the inner ring rib, and relatively large sliding friction will be generated at the contact zone [\(Jamison et al., 1976](#page-9-0); [Majdoub et al., 2020](#page-9-1)) in tapered roller bearings. Therefore, wear and sliding friction happen easily at the rib, and the load-carrying capacity and service life will be affected. It is indicated that the main failure mode

![](_page_1_Picture_1.jpeg)

<span id="page-1-0"></span>of the inner rib is wear ([Gadallah and Dalmaz, 1984\)](#page-9-2), and the tapered roller bearing is not suitable for high-speed operation without paying special attention to cooling and lubrication [\(Harris and Kotzalas,](#page-9-3) [2009\)](#page-9-3).

To improve the lubrication between the rib of the inner race and the large end of the roller, it is necessary to analyze the structure and the lubrication between the rib and the roller end. [Jamison et al. \(1976\)](#page-9-0) found that there are good lubricating effects between the rib of the inner race and the large end of the roller, when the nominal contact lies in the middle part of the inner race rib. [Zhang et al. \(1988](#page-9-4)) investigated the lubrication between the rib of the inner race and the large end of the roller and obtained the numerical solution corresponding to different structures of the rib in tapered roller bearings. [Jiang et al. \(1994](#page-9-5)), based on the method proposed by [Zhang et al. \(1988\)](#page-9-4), considered the thermal and non-Newtonian effects and showed that the thermal and non-Newtonian effects are negligible under moderate- and lowspeed operation and load. Taking into account the complex geometry pairings and kinematics, the tribological behavior of the roller end/face rib contact of tapered roller bearings was predicted by EHL contact simulations, and it is mainly determined by the basic geometric pairing and the radii ([Wirsching et al., 2021](#page-9-6)). Through finite element analysis, the temperature distribution of railway double-row tapered roller bearings under test conditions was investigated ([Gao et al.,](#page-9-7) [2022](#page-9-7)). A computational fluid dynamics (CFD) model was developed to consider effects of aeration on the lubricant behavior in tapered roller bearings [\(Maccioni et al., 2022](#page-9-8)). Under angular misalignment, two types of logarithmic crowning models for the roller profile were theoretically designed [\(Wu et al., 2020\)](#page-9-9). To disclose the mechanism of defect effects, contact states of roller–raceways and roller–rib were analyzed ([Liu et al., 2020](#page-9-10)).

A rheological model based on power function was put forward by [Carreau \(1972\).](#page-9-11) The Carreau–Yasuda model was developed by [Yasuda \(1979\)](#page-9-12) and [Yasuda et al. \(1981\)](#page-9-13), and the shear stress of its constructive equation can be expressed by generalized Newtonian viscosity and strain rate ([Bair, 2004](#page-9-14); [Bair](#page-9-15) [and Khonsari, 2006](#page-9-15); [Bair, 2009;](#page-9-16) [Kumar and Khonsari, 2009;](#page-9-17) [Katyal and Kumar, 2012](#page-9-18); [Kumar and Kumar, 2014\)](#page-9-19). [Liu et al.](#page-9-20) [\(2007\)](#page-9-20) analyzed the point contact EHL and found that numerical

![](_page_1_Picture_5.jpeg)

<span id="page-1-1"></span>results for polyalphaolefin (PAO) obtained with the Carreau–Yasuda rheological model are coherent with the experimental ones. Therefore, the Carreau rheological model is well suited to the majority of synthetic lubricating oil widely used at present.

Above all, the optimum design of the rib and the large end of the roller in tapered roller bearings based on EHL can improve the lubricating performance of the contact and prolong the life of the bearing. To simulate the engineering practice, the non-Newtonian effects, the thermal effects, and the spinning motion should be combined. Therefore, with the aim of designing and manufacturing the rib surface of the inner race in tapered roller bearings ([Tian](#page-9-21) [et al., 2021a](#page-9-21); [Tian et al., 2021b\)](#page-9-22), a thermal EHL model for the rib–roller end contact is built with the Carreau rheological model, which is different from the previous model in the literature, and thermal EHL analysis for the contact between the inner ring rib and the roller end in tapered roller bearings used in railway coaches is carried out.

# 2 Velocity analysis and rib structures

### 2.1 Velocity analysis

The geometry structure is important for the lubricating status in bearings. As shown in [Figure 1](#page-1-0), the schematic of a tapered roller bearing is given. The rib is called Solid 1, and the roller is called Solid 2.

<span id="page-2-4"></span>TABLE 1 Parameters of the lubricant.

| Parameter                                      | Squalane oil |
|------------------------------------------------|--------------|
| Thermal conductivities, $k$ , W/(m·K)          | 0.14         |
| Environmental density, ρ, kg/m³                | 864          |
| Specific heat, c, J/(kg·K)                     | 2000         |
| Environmental viscosity, η <sub>0</sub> , Pa·s | 0.0276       |
| Environmental temperature, $t_0$ , K           | 298          |
| В                                              | 5.258        |
| $V_{ m occ}/V_0$                               | 0.6633       |
| K' <sub>0</sub>                                | 11.74        |
| K <sub>0</sub> , GPa                           | 1.312        |

The coordinate system is built at the contact point B. O is the center of the pitch circle of the bearing, and  $O_1$  is the center of the spherical surface of the roller's large end.  $O_2$  is the conical point of the raceway, and  $O_3$  is the center of the spherical surface of the rib. A is the vertex of the rib cone.  $R_1$  is the curvature radius of the rib, and  $R_2$  is the curvature radius of the roller end.  $R_i$  is the radius at the point of tangency for the large end of the tapered roller and the rib of the inner race, and R is the cone distance of the roller.  $\alpha_i$  is the angle between the inner race and the axis of the bearing, and  $\alpha_0$  is the angle between the outer race and the axis of the bearing.  $\psi$  is the cone angle of the rib.

In this analysis, parameters of a double-row tapered roller bearing used in railway coaches (Hu et al., 2013) are as follows:  $\alpha_{\rm i}=11.11^{\circ}$ ,  $\alpha_{\rm o}=15.11^{\circ}$ , the diameter for the large end of the roller  $D_{\rm max}=17.56$  mm, the diameter for the small end of the roller  $D_{\rm min}=16.15$  mm, the length of the roller l=18.97 mm, and the pitch diameter l=155 mm.

When the tapered roller bearing is running, pure roll exists between the roller and the race. The outer race is stationary, and the inner race rotates with angular speed  $\omega_i$ , assuming that the direction of the angular speed is positive. According to the kinetic principle and the structure analysis of tapered roller bearings, the rotational speed  $\omega_b$  and revolution speed  $\omega_c$  (Jiang et al., 1994) are given, respectively, as follows:

<span id="page-2-0"></span>
$$\omega_{\rm b} = -\frac{2R_{\rm i}\omega_{\rm i}(D_{\rm max}\cos(0.5(\alpha_{\rm i} + \alpha_{\rm o})) + R_{\rm i})}{2R_{\rm i}D_{\rm max} + D_{\rm max}^2\cos(0.5(\alpha_{\rm i} + \alpha_{\rm o}))}.$$
 (1)

Note that the minus sign in Eq. 1 denotes the opposite direction of  $\omega_b$  to that of the angular speed of the inner race  $\omega_b$ 

$$\omega_{c} = \frac{R_{i}\omega_{i}}{2R_{i} + D_{\max}\cos(0.5(\alpha_{i} + \alpha_{o}))}.$$
 (2)

The spinning speed of the roller relative to the inner race is

$$\omega_{\rm s} = -(\omega_{\rm i} - \omega_{\rm c})\cos(\alpha_{\rm i} + \alpha) + \omega_{\rm b}(\beta - \alpha), \tag{3}$$

where  $\alpha$  is the angle between the direction of the spinning speed and the center axis of the roller, and  $\beta$  is the half conical angle, i.e.,  $\beta = 0.5$  ( $\alpha_{\rm o} - \alpha_{\rm i}$ ).

Defining  $h_{\rm c}$  as the contact height from the nominal contact point B to the inner raceway, the velocity of the rib of the inner race in the contact region is written as

$$u_1 = (R_i + h_c)\omega_i$$

The velocity of the large end of the tapered roller in the contact area is

$$u_2 = (0.5D_{\text{max}} - h_c)\omega_b + (R_i + h_c)\omega_c.$$

Velocity analysis shown in Figure 2 defines that

$$u_0 = (u_1 + u_2)/2,$$
 (4)

$$v_0 = (v_1 + v_2)/2, \tag{5}$$

where  $u_1$  and  $v_1$  are velocities of the rib in the direction of x and y, respectively;  $u_2$  and  $v_2$  are velocities of the roller in the direction of x and y, respectively.

The entrainment velocities  $u_R$  and  $v_R$  in the direction of x and y are

$$u_{\rm R} = u_0 + y\omega_{\rm s}/2,\tag{6}$$

$$v_{\rm R} = v_0 - x\omega_{\rm s}/2. \tag{7}$$

### 2.2 Structures of the rib

### 2.2.1 Spherical rib

To guarantee the wedge gap and surface tangent between the rib face and the roller end, the relationship of  $R_1 > R > R_2$  should be satisfied in tapered roller bearings.

Provided that the curvature radii of the rib on vertical and axial planes are  $R_{x1}$  and  $R_{y1}$  and curvature radii of the roller end on vertical and axial planes are  $R_{x2}$  and  $R_{y2}$ , respectively, then  $R_{x1} = R_{y1}$  and  $R_{x2} = R_{y2}$  are suitable for the spherical rib and spherical roller end.

<span id="page-2-1"></span>The contact height  $h_c$  (Zhang et al., 1988) between the nominal contact point B and the inner raceway can be written as follows:

$$h_{c} = R_{2} \left( \sin(\psi - \alpha_{i}) - \frac{1}{\cos(\psi - \alpha_{i}) \cot \beta + \sin \beta} \right) + \frac{R}{\cot \beta + \tan(\psi - \alpha_{i})}.$$
 (8)

<span id="page-2-2"></span>If  $\beta$  is small, then

$$h_c = R \sin \beta - R_2 \left( \sin \beta - \sin \left( \psi - \alpha_i \right) \right). \tag{9}$$

<span id="page-2-3"></span>From Figure 1, the following relationship can be obtained:

$$R_1 = R_2 + \left[ R \cos \beta + \left( R_2^2 - R^2 \sin^2 \beta \right)^{1/2} \right] \frac{\sin \left( \alpha_i + \beta \right)}{\sin w}. \tag{10}$$

### 2.2.2 Tapered rib

Different from the spherical rib, the generatrix of the tapered rib is a straight line on the axial plane; therefore, the curvature radius on the axial plane is  $R_{y1} = \infty$ .  $R_{x2} = R_{y2}$  and  $R_{x1} > R_{x2}$  are still satisfied. Eqs. 8, 9, and 10 are also suitable for the tapered rib and spherical roller end.

### 3 Mathematical model

### 3.1 Rheological model

The Carreau rheological model is used, and its constitutive equation (Yasuda et al., 1981; Maccioni et al., 2022) is as follows:

$$\eta^* = \eta \left[ 1 + (\eta \dot{\gamma}/G)^2 \right]^{(n-1)/2},$$
 (11)

where  $\eta^*$  is the effective viscosity of the non-Newtonian fluid. G is the shear modulus, and it is the demarcation point between the Newtonian fluid and Carreau–Yasuda fluid. n is the power exponent, and it can be obtained by the experiment or the non-equilibrium molecular dynamics simulation.

### 3.2 Reynolds equation

<span id="page-3-0"></span>The Reynolds equation is simplified from the generalized Reynolds equation proposed by Yang and Wen (1990).

$$\frac{\partial}{\partial x} \left[ (\rho/\eta)_{c} h^{3} \frac{\partial p}{\partial x} \right] + \frac{\partial}{\partial y} \left[ (\rho/\eta)_{c} h^{3} \frac{\partial p}{\partial y} \right]$$

$$= 12 \frac{\partial}{\partial x} (\rho_{x}^{*} u_{R} h) + 12 \frac{\partial}{\partial y} (\rho_{y}^{*} u_{R} h), \tag{12}$$

where p is the film pressure and h is the film thickness.

The boundary conditions of Eq. 12 are

$$\begin{cases}
 p(x_{\text{in}}, y) = p(x_{\text{out}}, y) = p(x, y_{\text{in}}) = p(x, y_{\text{out}}) = 0, \\
 p(x, y) \ge 0 \quad (x_{\text{in}} < x < x_{\text{out}}, \quad y_{\text{in}} < y < y_{\text{out}}).
\end{cases}$$
(13)

### 3.3 Temperature control equations

Thermal convection can be neglected as the entraining speed is not very high in spinning EHL. The temperature equation for the oil film is simplified as

$$k\frac{\partial^2 t}{\partial z^2} - \frac{t}{\rho} \frac{\partial \rho}{\partial t} \left( u \frac{\partial p}{\partial x} + v \frac{\partial p}{\partial y} \right) + \eta^* \left[ \left( \frac{\partial u}{\partial z} \right)^2 + \left( \frac{\partial v}{\partial z} \right)^2 \right] = 0, \quad (14)$$

where  $\rho$  is the density of the lubricant, k is the thermal conductivity of the lubricant, t is the temperature, and u, v are the flow velocities in the direction of x and y, respectively.

The equation of thermal conduction for moving Solid 1 is

$$c_1 \rho_1 u_1 \frac{\partial t}{\partial x} = k_1 \frac{\partial^2 t}{\partial z_1^2},\tag{15}$$

where  $c_1$ ,  $\rho_1$ ,  $k_1$ , and  $z_1$  are the specific heat, the density, the thermal conductivity, and the coordinate of Solid 1, respectively.

<span id="page-3-1"></span>The equation of thermal conduction for stationary Solid 2 can be written using the Laplace equation as follows:

$$\frac{\partial^2 t}{\partial x^2} + \frac{\partial^2 t}{\partial y^2} + \frac{\partial^2 t}{\partial z_2^2} = 0,$$
 (16)

where  $z_2$  is the coordinate for Solid 2.

Different from the previous model, Solid 2 is assumed as a half-infinitely large solid, and the surface temperature can be derived from Eq. 16, i.e.,

$$t_{2}(x,y) = t_{0} - \frac{k}{2\pi k_{2}} \iint \frac{\partial t}{\partial z} \Big|_{z=h} \frac{dx'dy'}{\sqrt{(x-x')^{2} + (y-y')^{2}}},$$
 (17)

where  $t_0$  is the environmental temperature and  $k_2$  is the thermal conductivity of Solid 2.

![](_page_3_Figure_22.jpeg)

# <span id="page-3-2"></span>3.4 Film thickness equation

The film thickness equation between the roller and the rib can be expressed as

![](_page_4_Figure_1.jpeg)

<span id="page-4-1"></span> $h(x,y) = h_{00} + \frac{x^2}{2R_x} + \frac{y^2}{2R_y} + \frac{2}{\pi E'} \iint_{\Omega} \frac{p(x',y')}{\sqrt{(x-x')^2 + (y-y')^2}} dx' dy',$  (18)

![](_page_4_Figure_3.jpeg)

<span id="page-4-2"></span>where  $\Omega$  is the computing domain and  $R_x$  and  $R_y$  are equivalent curvature radii along the x and y direction, respectively.

# 3.5 Density-pressure-temperature relationship

<span id="page-4-0"></span>If the Tait state equation (Liu et al., 2006) is amended by the thermally modified function, then the relationship of density-pressure-temperature is described as

![](_page_5_Figure_1.jpeg)

<span id="page-5-0"></span>Effect of rotating speeds on film thickness and temperature of two rib structures for  $R_{x2}/R_{x1} = 0.7$  and w = 320 N. (A) Spherical rib/spherical roller end; (B) tapered rib/spherical roller end.

$$\frac{\rho}{\rho_0} = \frac{V_0}{V} = \left\{ 1 - \frac{1}{1 + K_0'} \ln \left[ 1 + \frac{p}{K_0} \left( 1 + K_0' \right) \right] \right\}^{-1} [1 - C(t - t_0)], \quad (19)$$

where  $\rho_0$  and  $V_0$  are the density and volume of the lubricant at the atmospheric pressure. V is the volume of the lubricant at pressure p.  $K_0$  and  $K'_0$  are constants, which are determined by the experiment. C is the coefficient of thermal expansion, and  $C = 0.0008 \; \mathrm{K}^{-1}$  is used in the analysis.

### 3.6 Viscosity equation

The viscosity of the lubricant is calculated by the Doolittle equation. Because the Doolittle equation is solved with the density equation, if the density equation under thermal conditions is substituted into the viscosity equation, the viscosity-temperature relationship of the lubricant can be expressed as (Liu et al., 2006)

$$\eta = \eta_0 \exp \left\{ B \frac{V_{\text{occ}}}{V_0} \left[ \frac{1}{\frac{V}{V_0} - \frac{V_{\text{occ}}}{V_0}} - \frac{1}{1 - \frac{V_{\text{occ}}}{V_0}} \right] \right\}, \tag{20}$$

where  $V_{\text{occ}}/V_0$  and B are unknown constants, and  $V/V_0$  is determined using Eq. 19.

### 4 Numerical method

In tapered roller bearings, the position of the nominal contact point or the curvature radius of the rib and the roller large end can be determined by the optimal design. Based on  $\alpha_i$ ,  $\alpha_o$ ,  $\beta$ , and  $R_i$  of the bearing, if  $h_c/R$  and  $R_2/R_1$  are given,  $R_1$ ,  $R_2$ , and  $\psi$  will be obtained by solving Eqs. 8, 9, and Eq. 10. The numerical methods such as the secant method (Zhang et al., 1988) are used to solve the problem in this article. The details are as follows.

Define the residual error a as follows:

$$a = \left(R_2/R\right)_1 - \left(R_2/R\right)_{11},\tag{21}$$

where

$$(R_2/R)_1 = \left(\frac{h_c}{R} - \frac{1}{\cot \beta + \tan \varphi}\right) / \left(\sin \varphi - \frac{1}{\cos \varphi \cot \beta + \sin \beta}\right), (22)$$

$$(R_2/R)_{II} = \left(\frac{R_2}{R_1} \frac{\sin(\alpha_i + \beta)}{\sin(\alpha_i + \varphi)}\right) / \left\{1 - \frac{R_2}{R_1} \left[1 - \frac{\sin(\alpha_i + \beta)}{\sin(\alpha_i + \varphi)}\right]\right\}, \quad (23)$$

where  $\varphi = \psi - \alpha_i$ 

The secant method is employed, and the range of  $\varphi$  is defined as

$$\varphi \in \left\{ \frac{e}{R} - \left( \frac{1}{R_2/R_1} - 1 \right) \tan \beta, \frac{e}{R} \right\}.$$

![](_page_6_Figure_1.jpeg)

<span id="page-6-0"></span>The iteration will stop after |a| < ε, and ε is set at 0.0001 to ensure accuracy. φ and R2/R are output at last. R<sup>1</sup> and R<sup>2</sup> can be obtained by calculating R1/R = (R2/R)/(R2/R1).

After R1, R2, and ψ are computed, the equivalent curvature radii Ry and Rx can be obtained for the two kinds of rib structures. The numerical calculation for TEHL is carried out based on the dimensionless form. Different from the previous model, the film pressure is obtained with the multi-grid method [\(Venner et al., 1990\)](#page-9-26), the elastic deformation is computed with the multi-level multi-integration method [\(Brandt and](#page-9-27) [Lubrecht, 1990\)](#page-9-27), and the temperature field is calculated with the columnby-column technique ([Qu et al., 2000](#page-9-28)). Five levels are used, and the node number is 257 and 257 along the X- and Y-direction, respectively, at the finest level. A total of 16 layers are arranged in the Z-direction, of which 10 layers are arranged across the film and six layers are arranged for moving Solid 1. The temperature of stationary Solid 2 is calculated with the multi-level multi-integration method. The convergence criterion is that the relative error is less than 1 × 10–<sup>4</sup> for the pressure and temperature, and the relative error is less than 1 × 10–<sup>3</sup> for the load.

# 5 Results and discussion

Squalane oil with a low pour point is selected as the lubricant, and its parameters are shown in [Table 1](#page-2-4). The shear modulus G = 6 MPa, and the power exponent n = 0.42 for the Carreau model. The nominal contact height h<sup>c</sup> = 4 mm.

# 5.1 Typical solutions

Surface plots of the film thickness, the pressure, and the mid-layer film temperature for the spherical rib are shown in [Figure 3](#page-3-2) for Rx2/ Rx<sup>1</sup> = 0.3, w = 320 N and n<sup>i</sup> = 1,000 r/min. From [Figures 3A, B](#page-3-2), it can be found that typical characteristics of EHL are exhibited by the spherical rib, i.e., the exit constriction of the film thickness and the second spikes for the pressure distributions. Moreover, the surface plots of the film temperature in [Figure 3C](#page-3-2) are similar to those of the pressure.

With the same input parameters as in [Figure 3,](#page-3-2) the surface plots of the film thickness, the pressure, and the mid-layer film temperature are shown in [Figure 4](#page-4-1) for the tapered rib. It can be seen that the characteristics of EHL for the tapered rib are weaker than those for the spherical rib. This is because the contact region is so large that the maximum pressure is small with the same load, and it contributes less to the elastic deformation. Compared to the spherical rib, the film temperature is lower than that for the tapered rib. Hence, the film thickness for the tapered rib is larger. It can be observed that the spinning motion has little influence on the film thickness, the pressure, and the temperature for the two rib structures in the present analysis. The reason is that the maximum film temperature occurs near the center of the contact region, which is similar to that of the maximum pressure, and illustrates the primary effect of pressure on the temperature. Therefore, the contribution of the spinning velocity is small near the contact center, and the effects of spinning on the maximum temperature are minimal.

## 5.2 Effects of rotational speed

[Figure 5](#page-4-2) shows variations in the frictional coefficient versus the ratio of the curvature radius of the roller large end to that of the rib with various rotational speeds for the two rib structures for w = 560 N. It can be found that the variation tendency of the frictional coefficient is similar for the two types of ribs, i.e., decreasing at first and then increasing with the ratio of the curvature radius of the roller large end to that of the rib. Therefore, there exists an optimal ratio of the curvature radius. For the three rotational speeds, the optimal ratio of the curvature radius is 0.75 for the spherical rib. However, the optimal ratio with three speeds is 0.55, 0.5, and 0.45, respectively, for the

![](_page_7_Figure_1.jpeg)

<span id="page-7-0"></span>tapered rib. Hence, the ratio for the tapered rib/spherical roller Rx2/Rx<sup>1</sup> ranges from 0.45 to 0.55 to obtain the least frictional coefficient. When the ratio approaches unity for the tapered rib, the frictional coefficient increases remarkably, and the reason is that the film breakdown occurs due to the lack of wedge effect. In addition, effects of Rx2/Rx<sup>1</sup> on the frictional coefficient for the spherical rib are less sensitive than those for the tapered rib. Different from the tapered rib, the frictional coefficient for the spherical rib is always low with the increase in Rx2/Rx1. This is because the Hertzian contact region is circular for the spherical rib/spherical roller large end, whereas it is elliptical for the tapered rib/spherical roller large end which produces lower maximum pressure and film temperature.

It can be concluded that the optimal ratio of the curvature radius is determined by the rotational speed for the tapered rib, so it can be adjusted according to the bearing speed when the tapered rib is designed. Additionally, it can be observed that when the speed is low, the difference in the frictional coefficient is small corresponding to the optimal ratio of the curvature radii for the two ribs, whereas when the speed is high, the frictional coefficient for the spherical rib is less than that for the tapered rib corresponding to the optimal ratio of the curvature radii. This can evidently be attributed to the reduction of viscosity caused by thermal effects under higher speed conditions. Additionally, the maximum film temperature for the spherical rib is higher than that for the tapered rib.

Effects of the rotational speed of the inner race on the film thickness and temperature for the two ribs are shown in [Figure 6](#page-5-0) for Rx2/Rx<sup>1</sup> = 0.7 and w = 320 N; [Figure 6A](#page-5-0) represents the spherical rib, and [Figure 6B](#page-5-0) represents the tapered rib. As the rotational speed increases, both the minimum film thickness and central film thickness increase for the two ribs. This clearly demonstrates the dominant effect of speed on the film thickness in EHL. In addition, for the spherical rib, the difference between the central and minimum film thickness increases with the increasing speed. However, the difference is almost unchangeable for the tapered rib. The reason is that, compared to the tapered rib, the temperature of the film and the roller large end rises obviously for the spherical rib, and thermal effects cause an obvious reduction in film thickness. Moreover, due to the sliding effect, the highest temperature of the film and the roller large end increases with the increasing rotational speed, but the rib temperature changes negligibly. This is because the rib is running more quickly than the roller, and the heat transfer from the film to the rib is difficult. Therefore, when the ratio of the curvature radius and load are fixed, the minimum film thickness is larger for the tapered rib. The optimal lubricating performance is determined by the dealing of the frictional coefficient and the film thickness.

## 5.3 Effects of the load

When n<sup>i</sup> = 1,000 r/min, variations in the frictional coefficient versus the ratio of curvature radius of the roller large end to that of the rib with various loads for the two ribs are seen, as shown in [Figure 7](#page-6-0). It

can be observed that the frictional coefficient decreases first and then increases with the ratio of curvature radius when the load is different, and the difference between the two types of rib becomes large when the load increases. This is because the position of the nominal contact point changes with the ratio of curvature radius for the tapered rib/ spherical roller. Moreover, large load will lead to high temperature and thus lower viscosity of the lubricant and low frictional coefficient. With the increase in the load, the optimal ratio of the curvature radius is always 0.75 for the spherical rib, while the optimal ratio for the tapered rib is 0.35, 0.45, and 0.5, respectively. Therefore, the optimal ratio of the curvature radius is determined by both the load and the velocity for the tapered rib. However, the optimal ratio is hardly influenced by the velocity and the load for the spherical rib. This can be attributed to the characteristics of the spherical rib/spherical roller end, and thermal effects on the frictional coefficient are more significant than those for the tapered rib/spherical roller end.

[Figure 8](#page-7-0) shows effects of the load on the film thickness and temperature for the two rib structures for Rx2/Rx<sup>1</sup> = 0.7 and n<sup>i</sup> = 1,000 r/min; [Figure 8A](#page-7-0) represents the spherical rib, and [Figure 8B](#page-7-0) represents the tapered rib. It can be found that the spherical rib is much more sensitive to the load, and the film thickness varies more widely than that for the tapered rib. In addition, the minimum film thickness decreases remarkably, and the central film thickness increases first and then decreases slightly with the increase in the load. This is because the central film thickness is the function of the load. The central film thickness decreases slightly as the load surpasses a value. This is coherent with the film thickness formula proposed by Hamrock and Downson. Moreover, the minimum film thickness decreases because of the high pressure and temperature in the contact region with large load. However, with the increase in the load, the central film thickness increases for the tapered rib. There may be two reasons: one is that the thermal expansion caused by the film temperature rises in the contact region; the other is that the special contact between the rib and the roller end leads to the low pressure and small elastic deformation in the contact region, resulting in weak thermal effects. As the load increases, the central elastic deformation and film thickness increase to balance the uneven distribution of the pressure. It can be concluded that variations in the central film thickness versus the load are abnormal for the tapered rib/spherical roller end [\(Jiang et al., 1994\)](#page-9-5).

It can be seen from the temperature profiles that, as the load increases, for the spherical rib, higher temperature for the film and the roller will be generated because the contact between the rib and the roller end prevents the heat loss. However, the rib temperature increases insignificantly due to high rotational speed and slow heat transfer. Different from the spherical rib, there is a little change in temperature for the film, the rib, and the roller for the tapered rib–roller contact. Therefore, to improve the lubricating performance, the decisive effects on the minimum film thickness are the rib structures, followed by the speed and the load.

# Conclusion

Based on the structure analysis for the rib of the inner race and the large end of the roller in tapered roller bearings, a thermal EHL model considering spin–slide effects is established with Carreau fluid. The main conclusions are as follows:

- 1. Effects of spinning motion on the lubricating performance are minimal between the rib of the inner race and the large end of the tapered roller.
- 2. The optimal ratio of the curvature radius of the roller large end to that of the rib exists both for the tapered rib and the spherical rib, and the frictional coefficient corresponds the least to the optimal ratio. Moreover, the optimal ratio changes with the rotational speed and the load for the tapered rib. Different from the tapered rib, the frictional coefficient is low for the spherical rib with the increase in Rx2/Rx1.
- 3. With the increase in the rotational speed of the inner race, the film thickness and the temperature increase for the two rib structures. Different from the spherical rib, the difference between the minimum and the central film thickness is nearly unchangeable, and the minimum film thickness is larger due to low film temperature rise for the tapered rib.
- 4. With the increase in the load, the difference between the minimum and central film thickness becomes large for the two rib structures, and the film temperature rises. Compared to the tapered rib, variation in the temperature is significant, and the minimum film thickness is lower for the spherical rib.
- 5. Further study on this problem is needed for practical engineering.

# Data availability statement

The original contributions presented in the study are included in the article/Supplementary Material;, further inquiries can be directed to the corresponding author.

# Author contributions

XLiu and TL carried out the theoretical analysis. XLiu wrote the manuscript with support from XLi. FG and XLiu conceived the original idea and supervised the project.

# Funding

This work was supported financially by the National Natural Science Foundation of People's Republic of China (grant no. 52275196 and 51475250) and the "Taishan Scholar" Talents Project from Shandong Province (No. TS20190943).

# Conflict of interest

The authors declare that the research was conducted in the absence of any commercial or financial relationships that could be construed as a potential conflict of interest.

# Publisher's note

All claims expressed in this article are solely those of the authors and do not necessarily represent those of their affiliated organizations, or those of the publisher, the editors, and the reviewers. Any product that may be evaluated in this article, or claim that may be made by its manufacturer, is not guaranteed or endorsed by the publisher.

# References

<span id="page-9-14"></span>Bair, S. (2004). A rough shear thinning correction for EHL film thickness. STLE Tribol. Trans. 47 (3), 361–365.

<span id="page-9-15"></span>Bair, S., and Khonsari, M. M. (2006). Reynolds equations for common generalized Newtonian models and an approximate Reynolds-Carreau equation. Proc. Instn. Mech. Engrs, Part J, J. Eng. Tribol. 220, 365–374. doi[:10.1243/13506501jet79](https://doi.org/10.1243/13506501jet79)

<span id="page-9-16"></span>Bair, S. (2009). Rheology and high-pressure models for quantitative elastohydrodynamics. Proc. Instn. Mech. Engrs, Part J, J. Eng. Tribol. 223, 617–628. doi:[10.1243/13506501jet506](https://doi.org/10.1243/13506501jet506)

<span id="page-9-27"></span>Brandt, A., and Lubrecht, A. A. (1990). Multilevel matrix multiplication and fast solution of integral equations. J. Comput. Phys. 90, 494. doi:[10.1016/0021-9991\(90\)90264-2](https://doi.org/10.1016/0021-9991(90)90264-2)

<span id="page-9-11"></span>Carreau, P. J. (1972). Rheological equations from molecular network theories. Trans. Soc. Rheology 16 (1), 99–127. doi[:10.1122/1.549276](https://doi.org/10.1122/1.549276)

<span id="page-9-2"></span>Gadallah, N., and Dalmaz, G. (1984). Hydrodynamic lubrication of the rib-roller end contact of a tapered roller bearing. ASME J. Tribol. 106, 265–272. doi[:10.1115/1.3260898](https://doi.org/10.1115/1.3260898)

<span id="page-9-7"></span>Gao, P., Tang, W., Cui, Y., Wang, Y., Mo, G., and Yin, J. (2022). Theoretical and experimental investigation on thermal characteristics of railway double-row tapered roller bearing. Energies 15, 4217. doi:[10.3390/en15124217](https://doi.org/10.3390/en15124217)

<span id="page-9-3"></span>Harris, T. A., and Kotzalas, M. N. (2009). Rolling bearing analysis FIFTH EDITION: Essential concepts of bearing Technology. Beijing: Mechanical Industry Press.

<span id="page-9-23"></span>Hu, L., Wang, W. Z., Zhao, Z. Q., and Kong, L. J. (2013). Lubricated contact analysis of roller large end flange in double row tapered roller bearing. Tribology 33 (1), 22~28. (in Chinese).

<span id="page-9-0"></span>Jamison, W. E., Kauzlarich, J. J., and Mochel, E. V. (1976). Geometric effect on the ribroller contact in tapered roller bearing. ASLE Trans. 20, 79–88.

<span id="page-9-5"></span>Jiang, X. F., Wong, P. L., and Hong, Y. (1994). Thermal non-Newtonian EHL analysis of rib-roller end contact in tapered roller bearing. Hawaii, USA October: ASME/STLE annual conference.

<span id="page-9-18"></span>Katyal, P., and Kumar, P. (2012). Central film thickness formula for shear thinning lubricants in EHL point contacts under pure rolling. Tribol. Int. 48, 113–121. doi[:10.1016/j.](https://doi.org/10.1016/j.triboint.2011.11.007) [triboint.2011.11.007](https://doi.org/10.1016/j.triboint.2011.11.007)

<span id="page-9-17"></span>Kumar, P., and Khonsari, M. M. (2009). On the role of lubricant rheology and piezoviscous properties in line and point contact EHL. Tribol. Int. 42, 1522–1530. doi[:10.1016/j.](https://doi.org/10.1016/j.triboint.2008.11.006) [triboint.2008.11.006](https://doi.org/10.1016/j.triboint.2008.11.006)

<span id="page-9-19"></span>Kumar, P., and Kumar, N. (2014). Surface roughness effects in pure sliding EHL line contacts with carreau-type shear-thinning lubricants. Int. J. Mech. Aerosp. Industrial Mechatronics Eng. 8, 1104–1109.

<span id="page-9-20"></span>Liu, Y. C., Wang, J., Bair, S., and Vergne, P. (2007). A quantitative solution for the full shear-thinning EHL point contact problem including traction. Tribol. Lett. 28 (2), 171–181. doi:[10.1007/s11249-007-9262-5](https://doi.org/10.1007/s11249-007-9262-5)

<span id="page-9-10"></span>Liu, Y., Kang, W., Zhu, Y., Yan, K., and Hong, J. (2020). Effects of defect on rollerraceway contact state and friction torque of tapered roller bearings. ASME J. Tribol. 142, 111501. doi[:10.1115/1.4047194](https://doi.org/10.1115/1.4047194)

<span id="page-9-25"></span>Liu, Y., Wang, Q. J., Wang, W., Hu, Y., Zhu, D., Krupka, I., et al. (2006). EHL simulation using the free-volume viscosity model. Tribol. Lett. 23 (1), 27–37. doi:[10.1007/s11249-006-](https://doi.org/10.1007/s11249-006-9101-0) [9101-0](https://doi.org/10.1007/s11249-006-9101-0)

<span id="page-9-8"></span>Maccioni, L., Chernoray, V. G., Mastrone, M. N., Bohnert, C., and Concli, F. (2022). Study of the impact of aeration on the lubricant behavior in a tapered roller bearing: Innovative numerical modelling and validation via particle image velocimetry. Tribol. Int. 165, 107301. doi:[10.1016/j.triboint.2021.107301](https://doi.org/10.1016/j.triboint.2021.107301)

<span id="page-9-1"></span>Majdoub, F., Saunier, L., Sidoroff-Coicaud, C., and Mevel, B. (2020). Experimental and numerical roller skew in tapered roller bearings. Tribol. Int. 145, 106142.

<span id="page-9-28"></span>Qu, S., Yang, P., and Guo, F. (2000). Theoretical investigation on the dimple occurrence in the thermal EHL of simple sliding steel-glass circular contacts. Tribol. Int. 33 (1), 59–65. doi[:10.1016/s0301-679x\(00\)00031-1](https://doi.org/10.1016/s0301-679x(00)00031-1)

<span id="page-9-21"></span>Tian, Y. B., Li, L. G., Han, J. G., Fan, Z. H., and Liu, K. (2021). Development of novel high-shear and low-pressure grinding tool with flexible composite. Mater Manuf. Process 36 (4), 479–487. doi:[10.1080/10426914.2020.1843673](https://doi.org/10.1080/10426914.2020.1843673)

<span id="page-9-22"></span>Tian, Y. B., Li, L. G., Fan, S., Guo, Q. J., and Cheng, X. (2021). A novel high-shear and low-pressure grinding method using specially developed abrasive tools. P I Mech. Eng. B-J Eng. 235 (1-2), 166–172. doi[:10.1177/0954405420949106](https://doi.org/10.1177/0954405420949106)

<span id="page-9-26"></span>Venner, C. H., ten Napel, W. E., and Bosma, R. (1990). Advanced multilevel solution of the EHL line contact problem. ASME J. Tribol. 112, 426–431. doi:[10.1115/1.2920277](https://doi.org/10.1115/1.2920277)

<span id="page-9-6"></span>Wirsching, S., Marian, M., Bartz, M., Stahl, T., and Wartzack, S. (2021). Geometrical optimization of the EHL roller face/rib contact for energy efficiency in tapered roller bearings. Lubricants 9, 67. doi:[10.3390/lubricants9070067](https://doi.org/10.3390/lubricants9070067)

<span id="page-9-9"></span>Wu, Z., Xu, Y., and Liu, K. (2020). Study on logarithmic crowning of tapered roller profile considering angular misalignment. ASME J. Tribol. 142, 111201. doi:[10.1115/1.](https://doi.org/10.1115/1.4047106) [4047106](https://doi.org/10.1115/1.4047106)

<span id="page-9-24"></span>Yang, P., and Wen, S. (1990). A generalized Reynolds equation for non-Newtonian thermal elastohydrodynamic lubrication. ASME J. Tribol. 112, 631–636. doi:[10.1115/1.](https://doi.org/10.1115/1.2920308) [2920308](https://doi.org/10.1115/1.2920308)

<span id="page-9-13"></span>Yasuda, K. Y., Armstrong, R. C., and Cohen, R. E. (1981). Shear flow properties of concentrated solutions of linear and star branched polystyrenes. Rheol. Acta 20 (2), 163–178. doi:[10.1007/bf01513059](https://doi.org/10.1007/bf01513059)

<span id="page-9-12"></span>Yasuda, K. Y. (1979). Investigation of the analogies between viscometric and linear viscoelastic properties of polysyrene fluids. Cambiridge, MA: Ph.D thesis, MIT.

<span id="page-9-4"></span>Zhang, Z., Qiu, X., and Hong, Y. (1988). EHL analysis of rib-roller end contact in tapered roller bearings. ASLE Trans. 31, 461–467. doi:[10.1080/10402008808981849](https://doi.org/10.1080/10402008808981849)