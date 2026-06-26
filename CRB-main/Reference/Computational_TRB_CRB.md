# Computational Tapered and Cylinder Roller Bearings

**Author:** Hung Nguyen-Schäfer  
**Publisher:** Springer Nature Switzerland AG, 2019  
**ISBN:** 978-3-030-05444-1 (eBook)  
**DOI:** https://doi.org/10.1007/978-3-030-05444-1

---

## Preface

This monograph briefly deals with the computation of tapered and cylinder roller bearings using in automotive applications and other industries in which radial and axial loads and bending moments acting on the bearings are relatively large compared to ball bearings.

---

## Chapter 1: Tapered Roller Bearings

### 1.1 Components of Tapered Roller Bearings

Main components: cup (OR), cone (IR), tapered rolling elements, bearing cage, lubricant.

### 1.2 Geometry of Tapered Roller Bearings

Key parameters: bore diameter $d$, outside diameter $D$, width $T$, pitch diameter $D_{pw}$, RE length $L_{Re}$, mean RE diameter $D_m$.

Angles: $\alpha_i$ (half cone), $\alpha_o$ (half cup), $\alpha_m$ (half roller center line), $\alpha_{12}$ (half roller), $\alpha_f = \pi/2 - \alpha_m$ (kerb).

### 1.4 Computational Model of TRB

Total DOF: $(3Z + 3)$. IR DOF: $\delta_{r,IR}$, $\delta_{a,IR}$, $\theta_b$. Each RE DOF: $\delta_{yj}$, $\delta_{xj}$, $\psi_j$.

#### Elastic Deformations

**OR contact** (RE #$j$):

$$\delta_{ko}(j) = -(\delta_{yj}\cos\alpha_o + \delta_{xj}\sin\alpha_o) \tag{1.1}$$

**IR contact** (RE #$j$):

$$\delta_{ki}(j) = (\delta_{a,IR} + \delta_{xj})\sin\alpha_i + (\delta_{r,IR}\cos\varphi_j + \delta_{yj})\cos\alpha_i \tag{1.2}$$

**Kerb contact** (RE #$j$):

$$\delta_{fj}(j) = (\delta_{a,IR} + \delta_{xj})\cos\alpha_m^* - (\delta_{r,IR}\cos\varphi_j + \delta_{yj})\sin\alpha_m^* \tag{1.3}$$

where:

$$\alpha_m^*(j) = \alpha_m - \psi_j(j); \quad \alpha_m = \frac{1}{2}(\alpha_i + \alpha_o); \quad \varphi_j = \frac{2\pi j}{Z} \tag{1.4}$$

#### Normal Loads on Slices

$$Q_{ki}(k,j) = \frac{C_L}{n_S}\,\bar{\delta}_{ki}^{10/9}\,f_k(k); \quad Q_{ko}(k,j) = \frac{C_L}{n_S}\,\bar{\delta}_{ko}^{10/9}\,f_k(k) \tag{1.5}$$

where $C_L = 2^{10/9}\cdot C_L' = 7.765\times10^4\,L_{Re}^{8/9}$ (N/mm$^{10/9}$).

#### Reusner's Correction Factor

$$f_k(k) = 1 - \frac{10^{-2}}{\ln\!\left[1.985\cdot\left|\dfrac{k - n_{S,12}}{n_S - 1}\right|\right]}; \quad \forall\,k = 1,\ldots,n_S \tag{1.6}$$

where $n_{S,12} \equiv (n_S + 1)/2$.

#### Modified Deformations

$$\bar{\delta}_{ki}(k,j) = (\delta_{ki} - e_r/2) - 2P(x_k) + x_k\cos\varphi_j\tan\psi_j; \quad \bar{\delta}_{ki} = \max\!\big[0,\;\bar{\delta}_{ki}\big] \tag{1.7}$$

$$\bar{\delta}_{ko}(k,j) = (\delta_{ko} - e_r/2) - 2P(x_k) - x_k\cos\varphi_j\tan\psi_j; \quad \bar{\delta}_{ko} = \max\!\big[0,\;\bar{\delta}_{ko}\big]$$

#### Contour Profile (ISO/TS 16281 for TRB)

$$P(x_k) = -4.5\times10^{-4}\,D_m\ln\!\left[1 - \left(\frac{2x_k}{L_{Re}}\right)^{\!2}\right] \tag{1.8}$$

#### Equilibrium Equations

**Radial** (DOF $\delta_{r,IR}$):

$$F_r - \frac{C_L}{n_S}\sum_{j=1}^{Z}\sum_{k=1}^{n_S}\bar{\delta}_{ko}^{10/9}\,f_k(k)\cos\alpha_o\cos\varphi_j = 0 \tag{1.9}$$

**Axial** (DOF $\delta_{a,IR}$):

$$F_a - \frac{C_L}{n_S}\sum_{j=1}^{Z}\sum_{k=1}^{n_S}\bar{\delta}_{ko}^{10/9}\,f_k(k)\sin\alpha_o = 0 \tag{1.10}$$

**Bending** (DOF $\theta_b$):

$$M_b - \frac{C_L'}{n_S}\sum_{j=1}^{Z}\sum_{k=1}^{n_S}l_{kM}\,\bar{\delta}_{kM}^{10/9}\,f_k(k)\cos\varphi_j = 0 \tag{1.11}$$

where $C_L' = 3.5948\times10^4\,L_{Re}^{8/9}$ (one-side deformation).

**Modified bending deformation:**

$$\bar{\delta}_{kM}(k,j) = (\delta_{kM}\cos\varphi_j - e_r/2) - 2P(x_k) + x_k\cos\varphi_j\tan\psi_j \tag{1.12}$$
$$\bar{\delta}_{kM} = \max\!\big[0,\;\bar{\delta}_{kM}\big]$$

**Moment arm** $l_{kM}$:

- $M_b \geq 0$: $\;l_{kM} = +\!\left[\tfrac{1}{2}L_{Re} - x_k\right]\cos\alpha_{12} - \tfrac{1}{2}D_k\sin\alpha_{12}$
- $M_b < 0$: $\;l_{kM} = -\!\left[\tfrac{1}{2}L_{Re} + x_k\right]\cos\alpha_{12} - \tfrac{1}{2}D_k\sin\alpha_{12}$

**Slice coordinate and thickness:**

$$x_k(k) = (k - n_{S,12})\,\Delta x_k \tag{1.13}$$

$$\Delta x_k = \frac{L_{Re}}{n_S} \tag{1.14}$$

#### Total Normal Loads

$$Q_{ji}(j) = \frac{C_L}{n_S}\sum_{k=1}^{n_S}\bar{\delta}_{ki}^{10/9}\,f_k(k) \tag{1.15}$$

$$Q_{jo}(j) = \frac{C_L}{n_S}\sum_{k=1}^{n_S}\bar{\delta}_{ko}^{10/9}\,f_k(k) \tag{1.16}$$

#### Kerb Load

$$Q_{fj}(j) = C_L'\,\delta_{fj}^{10/9}(j); \quad \delta_{fj} = \max\!\big[0,\;\delta_{fj}\big] \tag{1.17}$$

#### Force Balance on RE #$j$

$$-Q_{ji}\cos\alpha_i + Q_{jo}\cos\alpha_o + Q_f\sin\alpha_m^* - F_c = 0 \tag{1.18}$$

$$-Q_{ji}\sin\alpha_i + Q_{jo}\sin\alpha_o - Q_f\cos\alpha_m^* = 0 \tag{1.19}$$

#### Moment Balance on RE #$j$

$$\left(-\!\sum_{k<n_{S,12}}\!l_{kL}Q_{ko} + \!\sum_{k\geq n_{S,12}}\!l_{kR}Q_{ko}\right)\cos\varphi_j + \left(\sum_{k<n_{S,12}}\!l_{kL}Q_{ki} - \!\sum_{k\geq n_{S,12}}\!l_{kR}Q_{ki}\right)\cos\varphi_j - F_cl_c\cos\varphi_j + Q_fh_{Q_f}\cos\varphi_j + M_{bj} = 0 \tag{1.20}$$

where:

$$l_{kL} = |x_k|\cos\alpha_{12} - \tfrac{D_k}{2}\sin\alpha_{12}; \quad l_{kR} = |x_k|\cos\alpha_{12} + \tfrac{D_k}{2}\sin\alpha_{12}$$

$$D_k(k) = D_m + 2x_k\tan\alpha_{12}; \quad h_{Q_f} = \frac{D_{M2} - d_1}{2\cos\alpha_m^*}; \quad D_{M2} = D_{pw} + L_{Re}\sin\alpha_m^*$$

$$M_{bj}(j) = \frac{C_L'}{n_S}\sum_{k=1}^{n_S}l_{kM}\,\bar{\delta}_{kM}^{10/9}\,f_k(k)\cos\varphi_j \tag{1.21}$$

### 1.5 Minimum Load and Preload

$$F_{r,\min} = 0.02\,C_r \tag{1.22}$$

$$F_{a,\min} = \frac{C_L}{n_S}\sum_{j=1}^{Z}\sum_{k=1}^{n_S}\bar{\delta}_{ko}^{10/9}\,f_k(k)\sin\alpha_o \tag{1.23}$$

$$\delta_r = \begin{cases}\delta_a\tan\alpha_o & \text{X arrangement}\\\delta_a\tan\alpha_i & \text{O arrangement}\end{cases} \tag{1.24}$$

### 1.6 Centrifugal Force

$$F_c = \frac{1}{2}m_{Re}D_{pw}\omega_R^2 = \frac{\pi}{6}\!\left(\frac{2\pi}{60}\right)^{\!2}\rho_{Re}L_{Re}(R^2+Rr+r^2)D_{pw}N_R^2 \tag{1.25}$$

$$m_{Re} = \rho_{Re}\frac{\pi L_{Re}}{3}(R^2+Rr+r^2) \approx 8.273\times10^{-6}L_{Re}(R^2+Rr+r^2) \tag{1.26}$$

$$F_c \approx 4.54\times10^{-11}L_{Re}(R^2+Rr+r^2)D_{pw}N_R^2 \tag{1.27}$$

### 1.7 Hertzian Pressures

$$b_{ki,ko}(k) = \sqrt{\frac{8\,Q_{ki,ko}(k)}{\pi E'\!\left(\dfrac{\Delta x_k}{\cos\alpha_{12}}\right)\!\sum\rho_{IR,OR}}} \tag{1.28}$$

$$E' = \frac{2}{\dfrac{1-\mu_1^2}{E_1} + \dfrac{1-\mu_2^2}{E_2}} \tag{1.29}$$

$$\sum\rho_{IR} = \frac{2}{D_k}\!\left(\frac{A_i}{A_i-1}\right); \quad \sum\rho_{OR} = \frac{2}{D_k}\!\left(\frac{A_o}{A_o+1}\right)$$

$$A_i = \frac{D_{pwk}}{D_k\cos\alpha_i};\; A_o = \frac{D_{pwk}}{D_k\cos\alpha_o};\; D_{pwk} = D_{pw}+2x_k\sin\alpha_m^*;\; \alpha_{12} = \tfrac{1}{2}(\alpha_o-\alpha_i)$$

$$p_{Hi,Ho,\max} = \frac{2\,Q_{ki,ko}(k)}{\pi\,b_{ki,ko}\!\left(\dfrac{\Delta x_k}{\cos\alpha_{12}}\right)} \tag{1.30}$$

### 1.8 Oil Film Thickness

$$H_{\min} = 1.714\cdot U^{*0.694}\cdot G^{*0.568}\cdot W^{*-0.128} \tag{1.31}$$

$$h_{\min} = \frac{1.806\cdot(\mu_0 U)^{0.694}\cdot\alpha_{EHL}^{0.568}\cdot R_f^{0.434}}{E'^{-0.002}\cdot(W/L_{Re})^{0.128}} \tag{1.32}$$

$$H_c = 2.922\cdot U^{*0.692}\cdot G^{*0.470}\cdot W^{*-0.166} \tag{1.33}$$

$$h_c = \frac{2.922\cdot(\mu_0 U)^{0.692}\cdot\alpha_{EHL}^{0.47}\cdot R_f^{0.474}}{E'^{0.056}\cdot(W/L_{Re})^{0.166}} \tag{1.34}$$

### 1.9 Bearing Friction

$$M_f = M_l + M_v + M_a \tag{1.35}$$

$$M_l = f_1 F_b D_{pw};\quad f_1 = 4\times10^{-4}\text{ to }5\times10^{-4} \tag{1.36}$$

$$F_{a,A} = F_{ax} + \frac{0.47\,F_{r,B}}{Y_B};\quad F_{a,B} = \frac{0.47\,F_{r,A}}{Y_A} - F_{ax} \tag{1.37}$$

$$M_v = \begin{cases}160\times10^{-7}f_oD_{pw}^3 & \nu N_R<2000\\10^{-7}f_o(\nu N)^{2/3}D_{pw}^3 & \nu N_R\geq2000\end{cases} \tag{1.38}$$

$$M_a = 0.06\,f_aF_aD_{pw} \tag{1.39}$$

$$P_f = (M_l+M_v+M_a)\cdot\frac{2\pi N_R}{60}\times10^{-3}$$

### 1.10 Lifetime

$$L_{hm} = \frac{1}{\sum_i\sum_j t_{ij}/L_{hm,ij}} \tag{1.40}$$

$$L_{hm,ij} = \frac{10^6}{60N_R}a_1a_{ISO,ij}\!\left(\frac{C_r}{P_{m,ij}}\right)^p;\quad p=10/3 \tag{1.41}$$

$$q_{kci} = Q_{ci}(1/n_S)^{7/9};\quad q_{kco} = Q_{co}(1/n_S)^{7/9} \tag{1.42}$$

$$L_{ir,j} = \sum_{k=1}^{n_S}\!\left(\frac{q_{kci}}{q_{kei}}\right)^{4.0};\quad L_{or,j} = \sum_{k=1}^{n_S}\!\left(\frac{q_{kco}}{q_{keo}}\right)^{4.5} \tag{1.43}$$

$$f_i(j,k) = \left(\frac{p_{Hi}}{271}\right)^{\!2}D_k(1-\gamma)\frac{L_{Re}}{n_S}\cdot\frac{1}{Q_{ki}} \tag{1.44}$$

$$q_{kei}(k) = \left(\frac{1}{Z}\sum_{j=1}^{Z}[f_i\cdot Q_{ki}]^{4.0}\right)^{1/4};\quad q_{keo}(k) = \left(\frac{1}{Z}\sum_{j=1}^{Z}[f_o\cdot Q_{ko}]^{4.5}\right)^{1/4.5} \tag{1.45}$$

$$L_r = (L_{ri}^{-\beta}+L_{ro}^{-\beta})^{-1/\beta};\quad \beta=9/8 \tag{1.46}$$

$$L_r = \left[\sum_{k=1}^{n_S}\left\{\left(\frac{q_{kci}}{q_{kei}}\right)^{-4.5}+\left(\frac{q_{kco}}{q_{keo}}\right)^{-5.0625}\right\}\right]^{-8/9} \tag{1.47}$$

$$L_{10,r}^* = a_{ISO}\cdot L_r \tag{1.48}$$

$$L_{h10}^* = \frac{10^6 L_{10,r}^*}{60\,N_R} \tag{1.49}$$

### 1.11 Bearing Stiffness

$$K_{b,r}\approx F_r/\delta_{r,IR} \tag{1.50}$$

$$K_{b,a}\approx F_a/\delta_{a,IR} \tag{1.51}$$

$$K_{bj}(j) = M'_{bj}(j)/(\psi_j\cos\varphi_j) \tag{1.52}$$

$$K_{b,M} = \sum_{j=1}^{Z}K_{bj}(j) \tag{1.54}$$

$$\theta_b = M_b/K_{b,M} \tag{1.55}$$

---

## Chapter 2: Cylinder Roller Bearings

Total DOF: $(2Z+2)$. IR DOF: $\delta_{r,IR}$, $\theta_b$. Each RE DOF: $\delta_{yj}$, $\psi_j$.

### 2.3 Computational Model

$$\delta_{ko}(j) = -\delta_{yj}(j) \tag{2.1}$$

$$\delta_{ki}(j) = \delta_{yj}(j) + \delta_{r,IR}\cos\varphi_j \tag{2.2}$$

$$Q_{fj}(j) = C_L'\,\delta_{fj}^{10/9}(j) \tag{2.3}$$

$$\delta_{fj}(j) = \max\!\left[0,\;\left(\frac{F_a}{C_L'}\cdot\eta_j\right)^{9/10}\right] \tag{2.6}$$

**Radial balance:**

$$F_r - \frac{C_L}{n_S}\sum_{j=1}^{Z}\sum_{k=1}^{n_S}\bar{\delta}_{ko}^{10/9}f_k\cos\varphi_j = 0 \tag{2.7}$$

**Bending moment:**

$$M_b - \frac{C_L'}{n_S}\sum_{j=1}^{Z}\sum_{k=1}^{n_S}l_{kM}\bar{\delta}_{kM}^{10/9}f_k\cos\varphi_j = 0 \tag{2.8a}$$

$$M_{bj}(j) = \frac{C_L'}{n_S}\sum_{k=1}^{n_S}l_{kM}\bar{\delta}_{kM}^{10/9}f_k\cos\varphi_j \tag{2.8b}$$

**Force balance on RE:**

$$Q_{jo}-Q_{ji}-F_c=0 \tag{2.9}$$

$$Q_{jo}(j) = \frac{C_L}{n_S}\sum_{k=1}^{n_S}\bar{\delta}_{ko}^{10/9}f_k \tag{2.10}$$

$$Q_{ji}(j) = \frac{C_L}{n_S}\sum_{k=1}^{n_S}\bar{\delta}_{ki}^{10/9}f_k \tag{2.11}$$

$$\frac{C_L}{n_S}\!\left(\sum_{k=1}^{n_S}\bar{\delta}_{ko}^{10/9}f_k - \sum_{k=1}^{n_S}\bar{\delta}_{ki}^{10/9}f_k\right) - F_c = 0;\quad\forall j \tag{2.12}$$

**Contour profile (CRB):**

$$P(x_k) = -3.5\times10^{-4}D_m\ln\!\left[1-\left(\frac{2x_k}{L_{Re}}\right)^2\right]$$

**Centrifugal force (steel CRB):**

$$F_c \approx 3.39\times10^{-11}D_m^2 L_{Re}D_{pw}N_R^2$$

**Moment balance:**

$$\left(-\!\sum_{k<n_{S,12}}\!l_{kL}Q_{ko}+\!\sum_{k\geq n_{S,12}}\!l_{kR}Q_{ko}\right)\cos\varphi_j + \left(\sum_{k<n_{S,12}}\!l_{kL}Q_{ki}-\!\sum_{k\geq n_{S,12}}\!l_{kR}Q_{ki}\right)\cos\varphi_j + Q_{fj}(h_1+h_2)\cos\varphi_j + M_{bj} = 0 \tag{2.13}$$

### 2.4 Hertzian Pressures

$$b_{ki,ko}(k) = \sqrt{\frac{8\,Q_{ki,ko}(k)}{\pi E'\Delta x_k\sum\rho_{IR,OR}}} \tag{2.14}$$

$$E' = \frac{2}{(1-\mu_1^2)/E_1+(1-\mu_2^2)/E_2} \tag{2.15}$$

$$p_{Hi,Ho,\max} = \frac{2Q_{ki,ko}(k)}{\pi b_{ki,ko}\Delta x_k} \tag{2.16}$$

### 2.5 Oil Film Thickness

$$H_{\min}=1.714\cdot U^{*0.694}\cdot G^{*0.568}\cdot W^{*-0.128} \tag{2.17}$$

$$h_{\min}=\frac{1.806(\mu_0U)^{0.694}\alpha_{EHL}^{0.568}R_f^{0.434}}{E'^{-0.002}(W/L_{Re})^{0.128}} \tag{2.18}$$

$$H_c=2.922\cdot U^{*0.692}\cdot G^{*0.470}\cdot W^{*-0.166} \tag{2.19}$$

$$h_c=\frac{2.922(\mu_0U)^{0.692}\alpha_{EHL}^{0.47}R_f^{0.474}}{E'^{0.056}(W/L_{Re})^{0.166}} \tag{2.20}$$

### 2.6 Bearing Friction

$$M_f=M_l+M_v+M_a \tag{2.21}$$

$$M_l=f_1P_mD_{pw};\;f_1=2.5\times10^{-4}\text{ to }3\times10^{-4} \tag{2.22}$$

$$M_v=\begin{cases}160\times10^{-7}f_oD_{pw}^3&\nu N_R<2000\\10^{-7}f_o(\nu N)^{2/3}D_{pw}^3&\nu N_R\geq2000\end{cases} \tag{2.23}$$

$$M_a=0.06f_aF_aD_{pw} \tag{2.24}$$

$$P_f=(M_l+M_v+M_a)\cdot\frac{2\pi N_R}{60}\times10^{-3} \tag{2.25}$$

### 2.7 Lifetime

$$L_{hm}=1/\sum_i\sum_j(t_{ij}/L_{hm,ij}) \tag{2.26}$$

$$L_{hm,ij}=\frac{10^6}{60N_R}a_1a_{ISO,ij}(C_r/P_{m,ij})^p \tag{2.27}$$

$$L_{ir,j}=\sum_k(q_{kci}/q_{kei})^{4.0};\;L_{or,j}=\sum_k(q_{kco}/q_{keo})^{4.5} \tag{2.29}$$

$$f_i(j,k)=(p_{Hi}/271)^2D_k(1-\gamma)(L_{Re}/n_S)/Q_{ki} \tag{2.30}$$

$$q_{kei}=\big(\tfrac{1}{Z}\sum_j[f_iQ_{ki}]^{4.0}\big)^{1/4};\;q_{keo}=\big(\tfrac{1}{Z}\sum_j[f_oQ_{ko}]^{4.5}\big)^{1/4.5} \tag{2.31}$$

$$L_r=\left[\sum_k\left\{(q_{kci}/q_{kei})^{-4.5}+(q_{kco}/q_{keo})^{-5.0625}\right\}\right]^{-8/9} \tag{2.33}$$

$$L_{10,r}^*=a_{ISO}\cdot L_r \tag{2.34}$$

$$L_{h10}^*=10^6L_{10,r}^*/(60N_R) \tag{2.35}$$

### 2.8 Bearing Stiffness

$$K_{b,r}\approx F_r/\delta_{r,IR} \tag{2.36}$$

$$K_{b,a}\approx F_a/\delta_{a,Z};\;\delta_{a,Z}=\delta_{fj}(Z) \tag{2.37}$$

$$K_{b,M}=\sum_{j=1}^Z M'_{bj}/(\psi_j\cos\varphi_j) \tag{2.40}$$

$$\theta_b=M_b/K_{b,M} \tag{2.41}$$

---

## References

1. Fritz, F.: Modellierung von Wälzlagern als generische Maschinenelemente. KIT Scientific Publishing (2011)
2. ISO/TS 16281:2008(E): Rolling bearings—modified reference rating life. ISO (2008)
3. Nguyen-Schäfer, H.: Computational Design of Rolling Bearings. Springer (2016)
4. Hamrock, B. et al.: Fundamentals of Fluid Film Lubrication, 2nd edn. Marcel Dekker (2004)
5. Nguyen-Schäfer, H.: Programs TRBOFT/CRBOFT. Internal MATLAB code (2018)
6. Harris, T.A., Kotzalas, M.N.: Essential Concepts of Bearing Technology, 5th edn. CRC (2006)
7. Harris, T.A., Kotzalas, M.N.: Advanced Concepts of Bearing Technology, 5th edn. CRC (2006)
8. Schaeffler: Wälzlagerpraxis, 4. Auflage (2015)
9. DIN/ISO 281: Wälzlager 1, 9. Auflage, Verlag Beuth (2012)
