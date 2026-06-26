# A New Dynamic Model for Cylindrical Roller Bearings with Flexible Rollers and Bearing Sliding Investigation

**Authors:** Ke Yan, Shuaijun Ma, Bin Fang, Fei Chen, Jun Hong, Pan Zhang  
**Affiliations:**  
- Key Laboratory of Education Ministry for Modern Design and Rotor-Bearing System, Xi'an Jiaotong University, Xi'an 710049, PR China  
- Department of Civil and Building Engineering, and Architecture, Polytechnic University of Marche, Ancona 60131, Italy  

**Published:** Mechanical Systems and Signal Processing 224 (2025) 112133  
**DOI:** https://doi.org/10.1016/j.ymssp.2024.112133

---

## Abstract

Cylindrical roller bearings are inevitably impacted by external moments or mounting error, which leads to uneven load distribution on the rollers and triggers deformation. Current methods are insufficient to simulate the deformation while ensuring solution accuracy. To address this, a new slicing approach is innovatively proposed in the paper, where springs are added between the neighboring slices to simulate the elastic deformation of the roller under external loads. Compared with the existing methods, the method presented in this paper has the best agreement with the finite element model. On this basis, a dynamic model for cylindrical roller bearing with roller deformation is further developed and verified experimentally. Finally, the sliding behavior inside the bearing under three typical conditions is investigated. A rich spectrum of frequencies emerges in the bearing contact load between the roller and raceway because of the roller deformation. These are all integer multiples of the roller passage frequency. An interesting phenomenon is observed that the sliding velocity is strongly influenced by the orbital speed of the roller compared to its rotational speed.

**Keywords:** Cylindrical roller bearings, Elastic deformation, Spectrum, Sliding velocity

---

## 1. Introduction

Cylindrical roller bearings are extensively used in aviation engines, new energy motors, gearboxes and other fields for their excellent load carrying and high speeds performance [1,2]. However, with the increasingly harsh conditions, they are exposed to more and more failures, such as excessive wear caused by severe sliding and bias loads [3,4]. With accurate numerical models, the bearing performance can be improved in the design phase to avoid such failures, thus reducing the development cycle and falling the cost of mechanical systems.

The cylindrical roller bearing model has been explored by many scholars for a long time. Harris [5] firstly presented the numerical model for cylindrical roller bearing, named static model, to investigate the effect of misalignment on its fatigue life. Liu [6] and Ramazan [7] introduced the driving speed to develop the quasi-static model. Harris [8] later used an empirical approach to consider the lubricant and further constructed a quasi-dynamic model to analyze the sliding inside the bearing. However, the nonlinear equations were used to obtain steady-state values, making it unsuitable for transient performance analysis [9].

Tu [10–12] introduced nonlinear differential equations to construct a dynamic model. Liu [13–16] studied the effects of surface roughness, raceway faults, etc. Liu [17–19] made improvements with consideration of cage flexibility and lubricant-thermal interaction. However, the motion of the rollers was restricted in the radial plane, meaning tilt and skew were ignored.

Cao [20,21] used the slicing method (Hertzian contact-based) to model complex roller motion. Han [22,23] integrated shear stress to obtain friction vectors. Deng [24–28] derived a five-parameter rheological model. Although all models accounted for complex roller motion using the slice method, interactions between slices were ignored. Yang [29] proposed an improved slicing method (influence coefficient-based) but assumed the roller axis is not deformed.

Several FEA methods have been proposed [32–34], but high computational demands and lack of lubrication consideration limit practical application. Combining analytical and numerical models can overcome these drawbacks [35–37].

Therefore, based on previous work [38,39], a slicing method of beam is innovatively proposed in this paper, where slice-slice interaction and deformation are all well addressed.

---

## 2. Discussion of Different Slicing Methods

### 2.1. Slicing Method Based on Beam

A new slicing method based on beam is presented, in which each slice has independent degrees of freedom of motion. The roller is first discretized into $n$ slices of same width. For any slice, a contact load from the raceway is applied in addition to the constraint load from neighboring slices. Springs and damping are added between adjacent slices to simulate roller deformation.

The displacement vector for the $i$-th slice:

$$X_i = \begin{bmatrix} x_i & y_i & z_i & \theta_{xi} & \theta_{yi} & \theta_{zi} \end{bmatrix}^T \tag{13}$$

According to Timoshenko beam theory, the constraint vector from the $(i-1)$-th slice on the $i$-th slice:

$$F_{i-1}^l = -\begin{bmatrix} K_{11} & 0 & 0 & 0 & 0 & 0 \\ 0 & K_{22} & 0 & 0 & 0 & K_{26} \\ 0 & 0 & K_{33} & 0 & K_{35} & 0 \\ 0 & 0 & 0 & K_{44} & 0 & 0 \\ 0 & 0 & K_{53} & 0 & K_{55} & 0 \\ 0 & K_{62} & 0 & 0 & 0 & K_{66} \end{bmatrix} \left( \begin{bmatrix} x_i - x_{i-1} \\ y_i - y_{i-1} \\ z_i - z_{i-1} \\ \theta_{xi} - \theta_{x,i-1} \\ \theta_{yi} - \theta_{y,i-1} \\ \theta_{zi} - \theta_{z,i-1} \end{bmatrix} - \begin{bmatrix} 2h \\ 0 \\ 0 \\ 0 \\ 0 \\ 0 \end{bmatrix} \right) \tag{1}$$

where $K$ is the deflection coefficient, a function of structural and material coefficients [38,39].

**Table 1: Deflection Coefficients of Timoshenko Beam**

| Name | Formula | Name | Formula |
|------|---------|------|---------|
| $K_{11}$ | $\dfrac{EA}{2h}$ | $K_{44}$ | $\dfrac{GI_{xx}}{2h}$ |
| $K_{22}$ | $\dfrac{12EI_{zz}}{(2h)^3(1+P_y)}$ | $K_{53}$ | $\dfrac{6EI_{yy}}{(2h)^2(1+P_z)}$ |
| $K_{26}$ | $\dfrac{-6EI_{zz}}{(2h)^3(1+P_y)}$ | $K_{55}$ | $\dfrac{(4+P_z)EI_{yy}}{2h(1+P_z)}$ |
| $K_{33}$ | $\dfrac{12EI_{yy}}{(2h)^3(1+P_z)}$ | $K_{62}$ | $\dfrac{-6EI_{zz}}{(2h)^3(1+P_y)}$ |
| $K_{35}$ | $\dfrac{6EI_{yy}}{(2h)^2(1+P_z)}$ | $K_{66}$ | $\dfrac{(4+P_y)EI_{zz}}{2h(1+P_y)}$ |

where $G$ is the shear modulus. $I_{xx}$ is the torsional constant, $I_{yy/zz}$ is the area moment of inertia:

$$\begin{cases} I_{xx} = 0.5\pi r^4 \\ I_{yy} = 0.25\pi r^4 \\ I_{zz} = 0.25\pi r^4 \end{cases} \tag{2}$$

$P_{y/z}$ is a coefficient for the shear deflection of Timoshenko beams:

$$\begin{cases} P_y = \dfrac{12EI_{yy}}{GA(2h)^2} \dfrac{I_z^2}{\displaystyle\int_A \left(\dfrac{\tau_z}{I_y}\right)^2 dA} \\[10pt] P_z = \dfrac{12EI_{zz}}{GA(2h)^2} \dfrac{I_y^2}{\displaystyle\int_A \left(\dfrac{\tau_y}{I_z}\right)^2 dA} \end{cases} \tag{3}$$

The normal load of the raceway on the slice:

$$q_i = K_{\text{Hertz}} \cdot \left\{\left(y_i - D_w/2 + C_s^i,\ 0\right)_{\max}\right\}^{10/9} \tag{4}$$

where $K_{\text{Hertz}}$ is the Hertzian contact deformation coefficient:

$$K_{\text{Hertz}} = \frac{\pi \cdot (L/n)^{8/9} E'}{3.891^{10/9}} \tag{3*}$$

where $E' = 1/[(1-\nu_1^2)/E_1 + (1-\nu_2^2)/E_2]$.

For logarithmic trimming:

$$C_s^i = A \cdot \ln\left[1 - (2x_i/L)^2\right]^{-1} \tag{5}$$

The equilibrium equation of the $i$-th slice:

$$F_{i-1}^l + F_{i+1}^r = \begin{bmatrix} 0 \\ q_i \\ 0 \\ 0 \\ 0 \\ 0 \end{bmatrix} + \begin{bmatrix} 0 \\ Q_i \\ 0 \\ 0 \\ 0 \\ 0 \end{bmatrix} \tag{6}$$

Superimposing all external loads:

$$Q_{\text{External}} = \sum_{i=1}^{n} Q_i \tag{7}$$

After solving by Newton-Raphson approach, the stress distribution using Hertzian line contact theory:

$$p_{ik} = p_{0i} \sqrt{1 - (z_k/a_i)^2} \tag{8}$$

where:

$$\begin{cases} p_{0i} = \sqrt{E' q_i / (\pi R)} \\ a_i = \sqrt{4Rq_i / (\pi E')} \end{cases} \tag{9}$$

### 2.2. Comparison of Different Slicing Methods

**Table 2: Parameters of the Roller**

| Name | Value | Name | Value |
|------|-------|------|-------|
| Roller diameter/mm | 15 | Trimming factor | 0.0156 |
| Roller length/mm | 17 | External load/N | 1000 |
| Trimming approach | logarithmic | Material | GCr15 |

The beam-based slicing method results are closest to those of the FE model. It has the highest solution accuracy because it considers coupling effects between slices and treats individual slices as independent rigid bodies. The Hertzian contact-based method gives the worst results because it ignores coupling.

### 2.3. Studies on Roller Contact Feature

**(a) Effect of structural parameters:** As the trimming factor increases, predicted maximum contact stress increases for all three methods. The beam-based method predicts the lowest maximum stress. The number of loaded slices remains constant for the beam-based method while it decreases for others.

**(b) Effect of operating conditions:** Both maximum contact stress and number of loaded slices grow with external load increase. The beam-based method shows slightly higher stress uniformity.

**(c) Effect of skew angle:** The skew angle has almost no effect on maximum contact stress. The beam-based method predicts a constant number of loaded slices due to accounting for minor roller deformations.

---

## 3. Dynamic Modeling of Cylindrical Roller Bearing

The coordinate frame system consists of seven frames: outer-fixed $(o\text{-}xyz)$, inner-fixed $(o_i\text{-}x_iy_iz_i)$, cage-fixed $(o_c\text{-}x_cy_cz_c)$, roller-fixed $(o_{bj}\text{-}x_{bj}y_{bj}z_{bj})$, pocket $(o_{cj}\text{-}x_{cj}y_{cj}z_{cj})$, slice $(o_{bjk}\text{-}x_{bjk}y_{bjk}z_{bjk})$, and slice azimuthal $(o_{ajk}\text{-}x_{ajk}y_{ajk}z_{ajk})$.

### 3.1. Force Vector Exerted on Roller

Position vector of the $k$-slice in the inertial frame:

$$q_{bjk} = \begin{bmatrix} r_{bjk} & \Theta_{bjk} \end{bmatrix}^T = \begin{bmatrix} x_{bjk} & y_{bjk} & z_{bjk} & \eta_{bjk} & \zeta_{bjk} & \lambda_{bjk} \end{bmatrix}^T \tag{10}$$

Displacement vector with respect to the inner ring:

$$r_{bjk}^i = [T_i](r_{bjk} - r_i) \tag{11}$$

Transformation matrix from inertial to inner-fixed frame:

$$[T_i] = \begin{bmatrix} \cos\zeta_i\cos\lambda_i & \cos\eta_i\sin\lambda_i + \sin\eta_i\sin\zeta_i\cos\lambda_i & \sin\eta_i\sin\lambda_i - \cos\eta_i\sin\zeta_i\cos\lambda_i \\ -\cos\zeta_i\sin\lambda_i & \cos\eta_i\cos\lambda_i - \sin\eta_i\sin\zeta_i\sin\lambda_i & \sin\eta_i\cos\lambda_i - \cos\eta_i\sin\zeta_i\sin\lambda_i \\ \sin\zeta_i & -\sin\eta_i\cos\zeta_i & \cos\eta_i\cos\zeta_i \end{bmatrix} \tag{12}$$

Deflection between the $k$-th slice and inner raceway:

$$\delta_{jk}^i = \sqrt{y_{bjk}^{i2} + z_{bjk}^{i2}} - (d_i - D_w)/2 + C_s^k \tag{13}$$

Normal load:

$$q_{jk}^i = K_{\text{Hertz}} \cdot \left\{\left(\delta_{jk}^i,\ 0\right)_{\max}\right\}^{10/9} \tag{14}$$

Friction force at the $k$-th slice/inner raceway interface (based on Wang's traction model [40]):

$$f_{jk}^i = \left[\left(A + B \cdot \left|\Delta v_{kj}^i\right| / u_{kj}^i\right) e^{C \cdot \left|\Delta v_{kj}^i\right| / u_{kj}^i} + D\right] \cdot q_{jk}^i \cdot \text{sign}(\Delta v_{kj}^i) \tag{15}$$

where $A$, $B$, $C$, $D$ are functions of lubrication, contact, and material parameters:

$$A = A_0 W^{A_1} |W_c/W - 1|^{} U^{A_2} T^{A_3} \tag{16}$$

$$B = B_0 W^{B_1} |W_c/W - 1|^{} U^{B_2} T^{B_3} \tag{17}$$

$$C = C_0 W^{C_1} |W_c/W - 1|^{} U^{C_2} T^{C_3} \tag{18}$$

$$D = D_0 W^{D_1} |W_c/W - 1|^{} U^{D_2} T^{D_3} \tag{19}$$

Relative sliding and rolling velocities:

$$\begin{cases} \Delta v_{kj}^i = v_k^i - v_{ki}^b \\ u_{kj}^i = (v_k^i + v_{ki}^b)/2 \end{cases} \tag{20}$$

Absolute velocity of inner ring at contact area:

$$v_k^i = w_x^i \cdot d_i/2 - (w_y^i \cdot \cos\varphi_{bjk}^i + w_z^i \cdot \sin\varphi_{bjk}^i) \cdot x_{bjk}^i \tag{21}$$

Absolute velocity of the $k$-th slice at contact area:

$$v_{ki}^b = w_m^b \cdot \sqrt{y_{bjk}^{i2} + z_{bjk}^{i2}} - w_{(x)bjk}^i D_w/2 \tag{22}$$

Churning resistance:

$$f_{jk}^l = \frac{\pi c_{sv} \rho_v (L/n \cdot D_w \cdot w_{(x)bjk})}{40} \tag{23}$$

Total force vector on the $k$-th slice in inertial frame:

$$F_{jk} = [T_i]^{-1}[T_{ajk}^i]^{-1} \cdot \begin{bmatrix} 0 \\ q_{jk}^i \\ f_{jk}^i \end{bmatrix} + [T_{ajk}]^{-1} \cdot \begin{bmatrix} 0 \\ -q_{jk}^o + f_{jk}^c \\ -f_{jk}^o - q_{jk}^c - f_{jk}^l \end{bmatrix} + [T_{bjk}]^{-1}\left([T_{bj(k-1)}^{bjk}]^{-1} F_{k-1}^l + F_{k+1}^r\right) \tag{24}$$

Total moment vector:

$$M_{jk} = \frac{D_w}{2} \cdot [T_{bjk}]^{-1} \left( [T_i]^{-1}[T_{ajk}^i]^{-1} \cdot \begin{bmatrix} -f_{jk}^i \\ 0 \\ 0 \end{bmatrix} + [T_{ajk}]^{-1} \begin{bmatrix} -f_{jk}^o - f_{jk}^c \\ 0 \\ 0 \end{bmatrix} \right) + [T_{bj(k-1)}^{bjk}]^{-1} M_{k-1}^l + M_{k+1}^r \tag{25}$$

Force and moment on inner raceway from slices:

$$F_i = \sum_{j=1}^{Z} \sum_{k=1}^{n} [T_i]^{-1}[T_{ajk}^i]^{-1} \cdot \begin{bmatrix} 0 & -q_{jk}^i & -f_{jk}^i \end{bmatrix}^T \tag{26}$$

$$M_i = \sum_{j=1}^{Z} \sum_{k=1}^{n} [T_{ajk}^i]^{-1} \cdot \begin{bmatrix} -d_i/2 \cdot f_{jk}^i \\ x_{bjk}^i \cdot f_{jk}^i \\ -x_{bjk}^i \cdot q_{jk}^i \end{bmatrix} \tag{27}$$

### 3.2. Force Vector Exerted on Cage

Deflection between the $k$-th slice and cage pocket:

$$\delta_{jk}^c = |z_{bjk}^c| - (D_p - D_w)/2 - C_s^k \tag{28}$$

Displacement vector in pocket-fixed frame:

$$r_{bjk}^c = [T_{cj}^c] \cdot [T_c] \cdot (r_{bjk} - r_c) - \begin{bmatrix} 0 & d_{cp}/2 & 0 \end{bmatrix}^T \tag{29}$$

Normal load and friction:

$$q_{jk}^c = K_{\text{Hertz}} \cdot \left\{\left(\delta_{jk}^c,\ 0\right)_{\max}\right\}^{10/9} \cdot \text{sign}(-z_{bjk}^c) \tag{30}$$

$$f_{jk}^c = \mu_{jk}^c \cdot q_{jk}^c \cdot \text{sign}(-w_x^b) \tag{31}$$

End slice collision with pocket side beam:

$$q_{ek}^c = K_{\text{Hertz}} \cdot \left\{\left(x_{bjk}^c - (L_c - L_p)/2,\ 0\right)_{\max}\right\}^{10/9} \cdot \text{sign}(x_{bjk}^c) \tag{32}$$

Fluid dynamic pressure from guiding ring:

$$F_{cg} = \frac{\eta_0 u_g L_c^3 \varepsilon_b^2}{0.25 C_g (1 - \varepsilon_b^2)^2} \tag{33}$$

$$f_{cg} = \frac{\pi \eta_0 u_g L_c^3 \varepsilon_b^2}{C_g (1 - \varepsilon_b^2)^{1.5}} \cdot \text{sign}(v_g) \tag{34}$$

Relative guiding clearance:

$$\varepsilon_c = \sqrt{y_c^2 + z_c^2} / C_g \tag{35}$$

Total cage force and moment:

$$F_c = [T_c]^{-1} \sum_{j=1}^{Z} \sum_{k=1}^{n} [T_{cj}^c]^{-1} \cdot \begin{bmatrix} q_{ek}^c \\ f_{jk}^c \\ q_{jk}^c \end{bmatrix} + \begin{bmatrix} 0 \\ -F_{cg} \\ f_{cg} \end{bmatrix} \tag{36}$$

$$M_c = \sum_{j=1}^{Z} \sum_{k=1}^{n} [T_{cj}^c]^{-1} \cdot \begin{bmatrix} d_{cp}/2 \cdot q_{jk}^c \\ x_{bjk}^c \cdot q_{jk}^c \\ -x_{bjk}^c \cdot f_{jk}^c + d_{cp}/2 \cdot q_{ek}^c \end{bmatrix} + \begin{bmatrix} -d_{c1}/2 \cdot f_{cg} \\ 0 \\ 0 \end{bmatrix} \tag{37}$$

### 3.3. Construction and Solution of Differential Equations

Newton-Euler equations of motion:

$$m \cdot \begin{bmatrix} \ddot{x} & \ddot{y} & \ddot{z} \end{bmatrix}^T = F_{\text{external}} \tag{38}$$

$$\begin{bmatrix} I_x \dot{w}_x - (I_y - I_z) w_y w_z \\ I_y \dot{w}_y - (I_z - I_x) w_z w_x \\ I_z \dot{w}_z - (I_x - I_y) w_x w_y \end{bmatrix} = M_{\text{external}} \tag{39}$$

The number of differential equations grows from $6 \cdot (Z + 1) + 3$ in previous models to $6 \cdot (n \cdot Z + 1) + 3$ in the current model. Equations are solved by the Newmark-$\beta$ algorithm.

### 3.4. Model Validation

**Table 3: Main Structural Parameters for NU215**

| Name | Value |
|------|-------|
| Diameter of inner raceway $d_i$ /mm | 88.5 |
| Diameter of outer raceway $d_o$ /mm | 118.57 |
| Bearing width $B$/mm | 25 |
| Roller diameter $D_w$/mm | 15 |
| Roller width $L$/mm | 17 |
| Number of roller $Z$ | 18 |
| Pocket diameter $D_p$/mm | 16.4 |
| Pocket Width $L_p$/mm | 17.3 |
| Cage width $L_c$/mm | 24.2 |

Driving speeds of 2400 r/min and 3600 r/min were applied. The deviation between model-predicted and experimentally measured cage speeds does not exceed 5%, effectively validating the model.

---

## 4. Investigation of Bearing Dynamic Behavior

### 4.1. Effect of Y-axis Load

With driving speed 5000 r/min, radial load increased from 500 N to 5000 N.

**(a) Contact feature:** With increasing radial load, the magnitude and frequency of change in contact load increases. The richness of frequency components decreases with increasing radial load, eventually dominated by first few orders. The frequencies are integer multiples of the roller passage frequency.

**(b) Sliding feature:** Sliding velocity up to 7 m/s at 1000 N radial load. Heavy radial load helps inhibit bearing sliding. The sliding velocity is strongly influenced by the orbital speed of the roller.

### 4.2. Effect of Y-axis Moment

Driving speed 5000 r/min, radial load 3000 N, y-axis moment from 500 to 5000 N·mm.

**(a) Contact feature:** Peak contact load decreases with increasing moment. The y-axis moment helps expand the contact area and reduce maximum contact stress. Maximum stress positions can increase from one to two, symmetrically distributed around roller center.

**(b) Sliding feature:** The y-axis moment significantly increases the difference in sliding velocity between loaded and unloaded regions.

### 4.3. Effect of Z-axis Moment

**(a) Contact feature:** Little effect on contact load in both time and frequency domains. The z-axis moment increases maximum contact stress and causes the position to move along the roller axis.

**(b) Sliding feature:** The z-axis moment has less effect on sliding velocity compared to y-axis moment.

### Summary Table (Table 4)

| Indicator | Y-axis Load | Y-axis Moment | Z-axis Moment |
|-----------|------------|---------------|---------------|
| Contact Load | Increasing | Decreasing | Almost unchanged |
| Stress | Increasing | Almost unchanged | Almost unchanged |
| Sliding velocity (Inner) | Decreasing | Almost unchanged | Almost unchanged |
| Sliding velocity (Outer) | Almost unchanged | Almost unchanged | Almost unchanged |
| Orbital velocity | Increasing | Almost unchanged | Almost unchanged |
| Rotational velocity | Increasing | Almost unchanged | Almost unchanged |

---

## 5. Conclusion

1. Due to the consideration of elastic deformation, the results predicted by the beam-based slicing method are closest to the FE model compared to other common methods.

2. A rich spectrum of frequencies emerges in the bearing performance index because of roller deformation. These frequencies are all determined by the roller passing frequency.

3. When sliding occurs inside the bearing, the sliding velocity is determined by the orbital speed of the roller, so a high orbital speed is required to minimize the effects of sliding.

4. The y-axis moment helps to widen the loaded zone, reduce maximum contact stresses, and extend bearing fatigue life. However, excessive moment can increase the loaded zone from one to two.

---

## Appendix A: Slicing Method of Hertzian Contact

The roller is divided into $n$ pieces along its axis. Normal load on the $l$-th slice:

$$q_l = K_{\text{Hertz}} \cdot \left\{\left(D_w/2 - \delta - C_s^l,\ 0\right)_{\max}\right\}^{10/9} \tag{A-1}$$

Logarithmic trimming:

$$C_s^l = A \cdot \ln\left[1 - (2x_l/L)^2\right]^{-1} \tag{A-2}$$

Hertzian contact deformation coefficient:

$$K_{\text{Hertz}} = \frac{\pi \cdot (L/n)^{8/9} E'}{3.891^{10/9}} \tag{A-3}$$

Equilibrium equation:

$$Q_{\text{External}} = \sum_{l=1}^{n} q_l = \sum_{l=1}^{n} K_{\text{Hertz}} \cdot \left(\max(D_w/2 - \delta - C_s^l,\ 0)\right)^{10/9} \tag{A-4}$$

Stress distribution:

$$p_k = p_0 \sqrt{1 - (z_k/a_l)^2} \tag{A-5}$$

$$\begin{cases} p_0 = \sqrt{E' q_l / (\pi R)} \\ a_l = \sqrt{4Rq_l / (\pi E')} \end{cases} \tag{A-6}$$

---

## Appendix B: Slicing Method of Influence Coefficient

Displacement at the $i$-th slice center from the $j$-th slice:

$$w_{ij} = p_{0j} D_{ij} / (\pi E') \tag{B-1}$$

Flexibility coefficient:

$$D_{ij} = \int_{-a_j}^{a_j} \int_{z_j - h}^{z_j + h} \frac{\sqrt{1 - (x'/a_j)^2}}{\sqrt{x'^2 + (z_i - z_j - z')^2}} \, dx' \, dz' \tag{B-2}$$

Contact width:

$$a_j = 2R \cdot p_{0j} / E' \tag{B-3}$$

For roller-raceway contact:

$$D_{ij} = \int_{-a_j}^{a_j} \sqrt{1 - (x'/a_j)^2} \ln\left(\frac{|z_i - z_j| + h + \sqrt{x'^2 + (z_i - z_j + h)^2}}{|z_i - z_j| - h + \sqrt{x'^2 + (z_i - z_j - h)^2}}\right) dx' \tag{B-4}$$

Equilibrium equations:

$$\pi h \sum_{j=1}^{n} a_j p_{0j} = Q_{\text{External}} \tag{B-5}$$

$$\frac{1}{\pi E'} \sum_{j=1}^{n} D_{ij} p_{0j} = \delta - C_s^j \tag{B-6}$$

Logarithmic trimming:

$$C_s^j = A \cdot \ln\left[1 - (2x_j/L)^2\right]^{-1} \tag{B-7}$$

---

## References

[1] H. Cao et al., "Mechanical model development of rolling bearing-rotor systems: A review," *Mech. Syst. Signal Pr.* 102 (2018) 37–58.  
[2] D. Ryan et al., "Prevention of Smearing Damage in Cylindrical Roller Bearings," *Tribol. T.* 56(5) (2013) 703–716.  
[3] D. Michael et al., "Slip Characteristics in Cylindrical Roller Bearings—Part III," *J. Tribol.* 145(2) (2022) 1–37.  
[4] C. Wei et al., "Analysis of enhanced heat transfer performance of the functional cage," *Int. J. Heat Mass Transf.* 219 (2024) 124860.  
[5] T. Harris, "The Effect of Misalignment on the Fatigue Life of Cylindrical Roller Bearings," *J. Lubr. Tech.* 91(2) (1969) 294–300.  
[6] J. Liu, "The Effect of Misalignment on the Life of High Speed Cylindrical Roller Bearings," *J. Lubr. Tech.* 93(1) (1971) 60–68.  
[7] B. Ramazan, S. Ahmet, "Fatigue life analysis of radial cylindrical roller bearings," *Mech. Based Des. Struc.* 51(12) (2023) 7030–7055.  
[8] T. Harris, "An Analytical Method to Predict Skidding in High Speed Roller Bearings," *Tribol. T.* 9(3) (1966) 229–241.  
[9] S. Ma et al., "Dimensional Discussion of Traction Force Vector," *J. Tribol.* 145(9) (2023) 091108.  
[10] W. Tu et al., "Dynamic Interactions Between the Rolling Element and the Cage," *J. Tribol.* 141(9) (2019) 091101.  
[11] W. Tu et al., "Investigation of the dynamic local skidding behavior," *P. I. Mech. Eng. K-J Mul.* 233(4) (2019) 899–909.  
[12] W. Tu et al., "A nonlinear dynamic vibration model," *Nonlinear Dyn.* 103 (2021) 2299–2313.  
[13–19] J. Liu, Y. Liu et al., Various publications on contact characteristics, vibration, and skidding dynamics.  
[20–21] L. Niu, H. Cao et al., Publications on vibration characteristics and dynamic modeling.  
[22–23] Q. Han et al., Publications on skidding behavior.  
[24–28] W. Zhang, S. Deng et al., Publications on roller convexity, cage dynamics, and wear.  
[29] H. Yang et al., "Analysis of the roller-race contact deformation," IEEE MEC (2011) 711–714.  
[30–31] S. Deng et al., J. Liu et al., Publications on friction torque and localized defect modeling.  
[32–34] S. Singh et al., D. He et al., FEM publications on defective bearings.  
[35–37] S. Singh et al., W. Li et al., A. Safian et al., Combined analytical-numerical approaches.  
[38] S. Ma et al., "A real-time coupling model of bearing-rotor system," *Int. J. Mech. Sci.* 245 (2023) 108098.  
[39] S. Ma et al., "New bearing model with flexible cage," *Mech. Syst. Signal Pr.* 208 (2024) 111045.  
[40] Y. Wang, "Study of Rheological Behavior of Aviation Lubricating Oil," PhD Thesis, Harbin Institute of Technology, 2006.  
[41] W. Zhang, "Study on the dynamics simulation," PhD Thesis, Northwestern Polytechnical University, 2017.
