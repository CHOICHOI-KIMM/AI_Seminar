## 제3장 부하 동력손실 해석 솔버 개발

## 3.1 개요

본 장에서는 부하 동력손실을 예측하기 위한 해석 기법 개발과 검증과정을 다룬다. 본 장의 구성은 다음과 같다. 먼저 3.2절에서 탄성유체윤활 해석을 통한 기어 쌍의 부하 동력손실 해석 방법론에 대해 기술하였고, 3.3절에서 기존의 시험 결과와 윤활막 두께 분포 및 압력 분포 비교를 통한 HMEHL(homogenized mixed elasto-hydrodynamic lubrication) 해석 모델의 검증 과정을 다루었다. 본 연구에서는 해석 솔버(solver)의 가속화를위해 머신러닝을 적용하였으며 3.4절에서 이를 다루었다. 3.5절에서는 본연구에서 제안하는 머신러닝 기반 해석 기법을 검증하기 위해 실제 기어쌍의 부하 동력손실 시험 결과와 해석 결과를 비교하였으며 그 과정을기술하였다.

기어 쌍의 부하 동력손실은 1.3.2항에 기술된 것과 같이 경험식 기반의 마찰계수와 손실계수를 이용하는 해석적 방법, 탄성유체윤활 해석을 통한 방법, 그리고 탄성유체윤활 다중 선형 회귀 방법으로 구분할 수 있다. 그중 탄성유체윤활 기반의 부하 동력손실 해석은 물리 지식 기반의해석으로 높은 정확성을 가지며, 해석 조건의 제한이 없다는 장점이 있다. 그러나 윤활막 내 비선형성이 강하여 수렴을 위해 많은 반복 계산이필요하며, 이에 따라 해석 시간이 많이 소요된다. 따라서 이를 최적설계에 직접 적용하는 것은 바람직하지 않으며 해석 솔버(solver)의 가속화가필요하다.

본 연구에서는 머신러닝을 적용하여 이를 가속화하고자 했으며, 회 귀 모델에 널리 사용되는 다층 퍼셉트론(multi-layer perceptron)뿐만 아니 라 도메인 지식을 활용할 수 있는 멀티 모달 딥러닝(multimodal deep learning)을 적용하고자 하였다. 멀티 모달 딥러닝(multimodal deep learning)을 적용하면 효율적인 아키텍처를 구성하여 과적합을 예방하고 회귀 성능을 향상시킬 수 있다.

또한, 유사하게 머신러닝을 탄성유체윤활 해석에 적용한 선행 연구 [101, 102]와 달리 HMEHL 모델을 통해 접촉하는 두 물체의 표면 조도를 고려하여 해석 모델의 정확성을 높였다. 기어 쌍의 표면 조도는 경계 윤활 영역에서 부하 동력손실에 큰 영향을 미치며 Table 1.1에 나타낸 것과 같이 낮은 등급의 기어를 사용하는 농업 기계 분야에서는 이를 반드시고려해 주어야 한다.

마지막으로 HMEHL 해석 모델과 머신러닝 기반 기어 쌍 부하 동력 손실 해석 기법을 검증하기 위해 선행 연구의 시험 결과와 비교를 수행 하였다. 먼저 HMEHL 해석 솔버(solver)를 검증하기 위해 Kaneta 등[165] 의 시험 결과와 윤활막 두께를, Coulon 등[166]의 시험 결과와 윤활막 내 압력 구배를 비교하였다. 이후, 부하 동력손실 해석 기법을 검증하기 위 해 Petry-Johnson 등[167]의 시험 결과와 비교하였다.

# 3.2 탄성유체윤활 해석을 통한 부하 동력손실 해석 3.2.1 계산 과정

본 연구에서는 롤 각도에 따라 변화하는 기어 접촉 파라미터를 고려하여 준 정적 조건에서 기어 쌍의 부하 동력손실을 해석하였다. 또한, 기어 쌍의 접촉을 선 접촉으로 가정하였다. Figure 3.1은 기어 쌍의 부하 동력손실을 해석하는 과정을 나타낸 것이다. 먼저 기어 쌍의 부하 접촉 해석[40]을 통해 각 물림 위치에 인가되는 하중을 하중분할비로 도출하고,

기어 물림 이론을 통해 접촉 기하 정보와 운동학적 정보를 계산한다. 그후, 도출된 접촉 파라미터를 이용하여 HMEHL 해석을 통해 기어 물림 위치에 따른 압력, 윤활막 두께, 윤활유의 점도를 도출한다. 마지막으로, 윤활유의 두께 방향으로 전단응력을 적분하여 각 물림 위치에서 발생하는 부하 동력손실을 해석한다.

![](_page_3_Picture_0.jpeg)

Figure 3.1 Analysis process of HMEHL-based load dependent power loss in gear pair.

## 3.2.2 접촉 파라미터

HMEHL 시뮬레이션을 위해서는 기어에 인가되는 하중, 접촉하는 두 표면의 속도, 두 표면의 기하학적 형상과 같은 접촉 파라미터 정보가 필

요하다. 이를 위해, 접촉하는 기어 쌍의 물림 위치에 따른 하중을 도출해야 한다. 본 연구에서는 하중분할비를 해석하기 위해 제2장에서 다룬 부하 접촉 해석 모델을 이용하였으며, 기어 기하 정보와 운동학적 정보는 Li 등[168]과 동일하게 기어의 물림 이론을 이용하였다.

## 3.2.3 HMEHL 해석 솔버[169]

본 연구에서는 표면 조도와 캐비테이션 현상을 고려하여 기어 쌍의부하 동력손실을 해석하기 위해 MATLAB 기반의 HMEHL-FBNS 솔버 (solver)를 적용하였다[169-171].

#### 3.2.3.1 하중 및 마찰력

접촉하는 두 물체에 작용하는 하중 $(F_{N,imp})$ 은 윤활막에 전달되며, 이는 접촉 영역 내 상대 압력 $(p_{tot}-p_{amb})$ 의 적분 값과 동일하다. Eq(3.1)은 이를 나타낸다.

$$F_{N,imp} = F_N(p_{tot}) = \int_0^{L_{x1}} \int_0^{L_{x2}} p_{tot} - p_{amb} \, dx_2 dx_1 \tag{3.1}$$

where,  $F_{N,imp}$  = Imposed normal force, N

 $F_N$  = Normal force transmitted to tribological contact area, N

 $p_{tot}$  = Total pressure, MPa

 $p_{amb}$  = Ambient pressure, MPa

 $L_{x1}$  = Domain length in  $x_1$  direction, mm

 $L_{x2}$  = Domain length in  $x_2$  direction, mm

또한, 그때 작용하는 마찰력의 크기는 전단응력의 적분을 통해 Eq(3.2) 와 같이 계산할 수 있다.

$$F_T(\tau_{tot}) = \int_0^{L_{x1}} \int_0^{L_{x2}} \tau_{tot} \, dx_2 dx_1 \tag{3.2}$$

where,  $F_T$  = Friction force, N

#### 3.2.3.2 윤활막 두께 해석

Figure 3.2는 접촉하는 두 물체의 윤활막 두께를 나타낸다. 위쪽 표면 과 아래쪽 표면은  $x_1$  방향으로 각자의 속도로 운동하며, 두 표면 사이에는 윤활유에 의한 압력 $(p_{fl})$ 이 작용한다. 이때 각 위치에 대한 윤활막의두께는 두 표면의  $x_3$  방향 위치 차이로 계산되며  $h(x_1,x_2,t)=z_{up}(x_1,x_2,t)-z_{low}(x_1,x_2,t)$ 로 나타낼 수 있다.

![](_page_5_Picture_3.jpeg)

Figure 3.2 Schematic sketch of a generalized lubrication gap[170].

혼합 윤활 영역에서 접촉하는 두 물체의 돌기는 모양에 따라 윤활막의 압력 구배에 큰 영향을 미치며, 윤활막이 얇은 경우 돌기간 직접 접촉이 발생하게 된다. 따라서 윤활막 내 발생하는 압력 구배는 윤활막에 의한 압력과 돌기 접촉에 의한 압력으로 구분할 수 있으며, 기어 표면 조도와 윤활막 두께에 따라 돌기 접촉의 발생 여부가 결정된다. 윤활막두께 또한 매크로 윤활 두께와 돌기에 의한 마이크로 윤활 두께로 구분되며, Eq(3.3)과 같이 표현할 수 있다. 여기서,  $h_0$ 는 매크로 윤활 두께를,  $h_1$ 은 마이크로 윤활 두께를 나타낸다[169]. 또한, X와  $\xi$ 는 각각 매크로

도메인과 마이크로 도메인의 위치를 나타낸다.

$$h = h_0(X_1, X_2) + h_1(\xi_1, \xi_2) \tag{3.3}$$

where, h = Lubrication gap, mm

 $h_0$  = Macroscopic lubrication gap, mm

 $h_1$  = Microscopic lubrication gap, mm

마이크로 윤활 두께는 Eq( 3.4 )와 같이 돌기의 형상에 따라 결정되는 강체 마이크로 두께 $(h_{1,ri})$ 와 탄성 변형에 의한 탄성 마이크로 두께 $(h_{1,el})$ 로 구분할 수 있으며,  $h_{1,el}$ 은 Eq( 3.5 )와 같이 계산할 수 있다. 여기서,  $p_{asp}$ 는 돌기 접촉에 의해 발생하는 압력을 의미하며 Akchurin 등[172]이 제안한 방법을 이용하여 돌기의 탄성 변형을 고려한  $p_{asp}$ 를 도출하였다[171]. 탄성 변형 해석을 위한 드라이 접촉 솔버(solver)는 Polonsky와 Keer[173], Sainsot과 Lubrecht[174]가 제안한 켤레 기울기 고속 푸리에 변환 알고리 즘(conjugate gradient-fast Fourier transform algorithm)을 적용하였다. 여기서, K는 커널 함수로 Eq( 3.6 )과 같이 나타낼 수 있다. 축소 영률 E'는 E' =  $2[(1-v_1^2)/E_1+(1-v_2^2)/E_2]^{-1}$ 과 같이 구할 수 있다. 여기서,  $v_1$ ,  $v_2$ 는 접촉하는 물체의 포아송 비를,  $E_1$ ,  $E_2$ 는 두 물체의 영률을 나타낸다[175].

$$h_1(\xi_1, \xi_2) = h_{1,ri} + h_{1,el} \tag{3.4}$$

$$h_{1,el}(\xi_1, \xi_2) = \sum_{0}^{N_{\xi_{1,R}}} \sum_{0}^{N_{\xi_{1,R}}} \frac{2}{\pi E'} K(\xi_1 - \xi_{1,R}, \xi_2 - \xi_{2,R}) p_{asp}(\xi_{1,R}, \xi_{2,R})$$
(3.5)

$$K(\tilde{x}_{1}, \tilde{x}_{2}) = \left( (\tilde{x}_{1} + a) \ln \left( \frac{(\tilde{x}_{2} + b) + \sqrt{(\tilde{x}_{2} + b)^{2} + (\tilde{x}_{1} + a)^{2}}}{(\tilde{x}_{2} - b) + \sqrt{(\tilde{x}_{2} - b)^{2} + (\tilde{x}_{1} + a)^{2}}} \right) + (\tilde{x}_{2} + b) \ln \left( \frac{(\tilde{x}_{1} + a) + \sqrt{(\tilde{x}_{2} + b)^{2} + (\tilde{x}_{1} + a)^{2}}}{(\tilde{x}_{1} - a) + \sqrt{(\tilde{x}_{2} + b)^{2} + (\tilde{x}_{1} - a)^{2}}} \right) + (\tilde{x}_{1} - a) \ln \left( \frac{(\tilde{x}_{2} - b) + \sqrt{(\tilde{x}_{2} - b)^{2} + (\tilde{x}_{1} - a)^{2}}}{(\tilde{x}_{2} + b) + \sqrt{(\tilde{x}_{2} + b)^{2} + (\tilde{x}_{1} - a)^{2}}} \right) + (\tilde{x}_{1} - b) \ln \left( \frac{(\tilde{x}_{1} - a) + \sqrt{(\tilde{x}_{2} - b)^{2} + (\tilde{x}_{1} - a)^{2}}}{(\tilde{x}_{1} + a) + \sqrt{(\tilde{x}_{2} - b)^{2} + (\tilde{x}_{1} + a)^{2}}} \right) \right)$$

where,  $h_{1,ri}$  = Microscopic rigid lubrication gap, mm

 $h_{1,el}$  = Microscopic elastic lubrication gap, mm

 $p_{asp}$  = Asperity contact pressure, MPa

E' = Reduced Young's modulus, MPa

K = Kernel function

 $N_{\xi_{\Pi,R}}$  = Amount of the rectangular discretization cells in space on the microscale

 $\tilde{\mathbf{x}}_{\mathbf{n}}$  = Distances to the center of the rectangular discretization cell on the surface of an elastic half space ( $\tilde{\mathbf{x}}_{\mathbf{n}} = x_n - x_{n,R}$ )

매크로 윤활 두께 $(h_0)$ 가 결정됨에 따라 표면 조도에 의해 정해지는 강체 마이크로 두께 $(h_{1,ri})$ 를 알고 있을 때, 탄성 마이크로 두께 $(h_{1,el})$ 와 돌기 압력 $(p_{asp})$ 의 평형은 Eq(3.5)를 통해 계산할 수 있다[169]. h>0일 때는  $p_{asp}=0$ 를, h=0일 때는  $p_{asp}\geq0$ 를 만족하며 h<0 (negative gap height) 일 때  $p_{asp}$ 가 상한인 H (hardness)에 도달하게 된다. 이때 h는 0이 되고  $p_{asp}=H$ 가 된다[169,172]. 매크로 윤활 두께는 Eq(3.7)과 같이 표현할수 있다. 먼저  $h_{0,d}$ 는 강체 변위로 힘 평형 방정식 Eq(3.1)을 만족하도록 PID 루프를 통해 결정한다. 자세한 방법은 참고문헌을 통해 확인할 수 있다[169,171].  $h_{0,ri}$ 는 강체 매크로 두께로, 기하학적 형상에 따라 결정되

며 본 연구에서는 기어 쌍의 유효반경을 통해 도출할 수 있다.  $h_{0,el}$ 는 매크로 탄성 두께를 나타내며  $\mathrm{Eq}(3.8)$ 과 같이 계산할 수 있다[169].

$$h_0 = h_{0,d} + h_{0,ri} + h_{0,el} (3.7)$$

where,  $h_{0,d}$  = Macroscopic rigid body displacement, mm

 $h_{0,ri}$  = Macroscopic rigid lubrication gap, mm

 $h_{0,el}$  = Macroscopic elastic lubrication gap, mm

 $p_{tot}$ 은 윤활막 내 압력 $(p_{0,fl})$ 과 돌기 접촉에 의한 압력 $(p_{con})$ 의 합으로, Eq(3.9)와 같이 나타낼 수 있다[169].

$$h_{0,el}(X_1, X_2) = \sum_{0}^{N_{X_{1,R}}} \sum_{0}^{N_{X_{1,R}}} \frac{2}{\pi E'} K(X_1 - X_{1,R}, X_2 - X_{2,R}) p_{tot}(X_{1,R}, X_{2,R})$$
(3.8)

$$p_{tot} = p_{0,fl} + p_{con} (3.9)$$

where,  $p_{tot}$  = Total pressure, MPa

 $p_{0,fl}$  = Hydraulic pressure, MPa

 $p_{con}$  = Dry contact pressure, MPa

여기서  $p_{con}$ 는 이미 알고 있는  $h_{1,ri}$ 와 주어진  $h_0$ 에 대해 마이크로 도메인에서 발생하는 평균 돌기 접촉 압력으로 Eq( 3.10 )과 같이 계산할 수 있으며 하나의 매크로 도메인 셀 내에서 동일하다[169].  $p_{0,fi}$ 는 유한체적법을 이용하여 다음 항에 기술된 Reynolds 방정식을 통해 기어의 물림위치에 따라 계산할 수 있다.

$$p_{con}(h_0) = \frac{1}{\lambda_{\xi_1} \lambda_{\xi_2}} \int_0^{\lambda_{\xi_1}} \int_0^{\lambda_{\xi_2}} p_{asp} d\xi_2 d\xi_1$$
 (3.10)

where,  $\lambda_{\xi_n}$  = Periodic length or width on the microscale, mm

#### 3.2.3.3 HMEHL 지배 방정식

본 연구에서는 HMEHL 모델을 활용하여 기어 쌍의 부하 동력손실을 예측하였다. 캐비테이션 현상은 Jakobsson-Floberg-Olsson 모델을[176, 177], 표면 조도는 homogenized Reynolds 방정식을[178, 179] 적용하여 고려하였다. 또한 윤활유의 비뉴턴 유동을 고려하기 위해 Ree-Eyring 모델을 이용하였다[168]. 이를 반영한 homogenized Reynold 방정식은 Eq(3.11)과 같이 나타낼 수 있다[176, 177]. 여기서  $p_0 = p_{0,fl} - p_{cav}$ 는 축소 압력을 나타내며,  $p_{0,fl}$ 와  $p_{cav}$ 는 각각 유체 압력과 캐비테이션 압력을 의미한다. 캐비테이션 fraction  $\theta$ 는  $\theta = 1 - \frac{\rho}{\rho_l}$ 을 의미한다. 여기서  $\rho$ 는 혼합 밀도를,  $\rho_l$ 은 액체 상태의 밀도를 나타낸다. 캐비테이션 fraction과 압력은 Eq(3.12)를 만족한다.  $u_m$ 은 평균 표면 속도,  $h_m$ 은 평균 윤활 두께를 각각 나타내며, Eq(3.13)과 같이 구할 수 있다. A와  $\vec{b}$ 는 homogenization factor로 Hansen 등[171]의 방법을 이용하여 계산하였다.

윤활막 내 온도 변화의 경우, 기어 치 표면의 벌크 온도는 공간과 시간에 따라 변화함에 따라 열 탄성유체윤활 모델은 정확한 해를 찾지 못하고 불필요한 계산 비용을 초래할 수 있다. 이에 따라 온도는 등온 조건으로 가정하였다[180].

$$0 = \nabla \cdot \left( \frac{\rho_l h_m}{12\eta} A \nabla p_0 - \rho_l h_m u_m \vec{b} (1 - \theta) \right)$$
 (3.11)

$$p_0\theta = 0, p_0 \ge 0, \theta \ge 0$$
 (3.12)

$$h_m(h_0) = \frac{1}{N_{\xi_{1,R}} N_{\xi_{2,R}}} \sum_{0}^{N_{\xi_{1,R}}} \sum_{0}^{N_{\xi_{1,R}}} h$$
 (3.13)

where,  $\eta$  = Viscosity, Pa·s

 $u_m$  = Mean velocity of two surfaces, mm/s

#### $h_m$ = Mean lubrication gap, mm

전단 연화 현상은 윤활유의 점도  $\eta$ 의 변화를 반영하여 Eq(3.14)와 같이고려할 수 있다[181]. 여기서  $\tau_0$ 는 기준 전단 응력으로, Eq(3.15)를 통해계산할 수 있다[168, 182-184].  $u_s$ 는 두 표면 사이의 미끄럼 속도를 나타낸다.

$$\eta = \frac{\tau_0 h_m}{u_s} sinh^{-1} \left[ \frac{\eta_0 u_s}{\tau_0 h_m} \right]$$
 (3.14)

$$\tau_{0} = \tau_{0a} + \frac{\tau_{e}}{2} \left\{ 1 + \tanh \left[ \Theta \left( \frac{2p_{fl}}{p_{s} + p_{e}} - 1 \right) \right] \right\}$$
 (3.15)

전단 파라미터  $\tau_e$ 와  $\Theta$ 는 Eq( 3.16 )과 Eq( 3.17 )을 통해 구할 수 있으며, 일반적인 기어에 사용되는 윤활유(75W90)를 고려하여  $\tau_{0a}=5\,MPa,\;p_s=0.5\,GPa,\;p_e=3.5\,GPa,\;$ 그리고  $\kappa=0.035$ 로 결정하였다[168].

$$\tau_e = \tau_{0a} + \kappa (p_e - p_s) \tag{3.16}$$

$$\Theta = \frac{\kappa(p_e - p_s)}{\tau_e} \tag{3.17}$$

압력에 따른 piezoviscosity 특성은 Eq( 3.18 )과 같이 Roeland 방정식을 사용해 고려하였다[185]. 여기서  $p_{0,R}$ 는 Roeland 상수를,  $\eta_0$ 는 주위 압력에서 윤활유의 동점도를 나타낸다. 압력-점도 인덱스는  $Z_R = \frac{\alpha_R p_{0,R}}{\ln(\eta_0 + 9.67)}$ 로계산된다.

$$\eta = \eta_0 \exp\left( (\ln(\eta_0) + 9.67) \left( -1 + \left( 1 + \frac{(p_{fl} - p_{cav})}{p_{0,R}} \right)^{Z_R} \right) \right)$$
(3.18)

이와 더불어 압력-밀도 관계는 Eq( 3.19 )와 같이 Dowson-Higginson 모델을 사용하였다[181, 185]. 여기서,  $C_1$ 과  $C_2$ 는 lubricant dependent 상수를 나

타낸다.

$$\rho_l = \rho_0 \frac{C_1 + C_2(p_{fl} - p_{cav})}{C_1 + (p_{fl} - p_{cav})}$$
(3.19)

#### 3.2.3.4 부하 동력손실 해석

접촉하는 두 물체에서 슬립이 발생하지 않는다고 가정하였을 때, 전단력은 전단 응력을 적분하여 계산할 수 있다. Homogenized Reynolds 방정식의 Poiseuille 및 Couette 흐름을 고려한 유체의 전단 응력은 Eq(3.20)과 같이 나타낼 수 있다[171]. 여기서, C와  $\vec{d}$ 는 homogenization factor로 Hansen 등[171]의 방법을 이용하여 계산하였다.

$$\vec{\tau}_{fl} = -\frac{h_m}{2} C \nabla p + \frac{\eta}{h_m} (-6u_m \vec{d} + u_s) (1 - \theta)$$
 (3.20)

where,  $\vec{\tau}_{fl}$  = Hydrodynamic shear stress, MPa

돌기 접촉 영역에서 발생하는 접촉 전단 응력은 Eq(3.21)과 같이 계산할수 있으며 돌기 접촉의 마찰계수는 Bowden and Tabor (2001)의 이론에 따라  $C_{f,b}$ 를  $1/(3\sqrt{3})$ 으로 결정하였다[186].

$$\vec{\tau}_{con} = C_{f,b} p_{con} \tag{3.21}$$

where,  $\vec{\tau}_{con}$  = Dry contact shear stress, MPa

 $C_{f,b}$  = Boundary friction coefficient

전체 전단 응력은 Eq(3.22)와 같이 계산할 수 있으며, 최종적으로 각 매크로 해석 셀에 대한 동력손실은 Eq(3.23)과 같이 계산할 수 있다. 여기서 a는 셀의 넓이를 나타낸다.

$$\vec{\tau}_{tot} = \vec{\tau}_{fl} + \vec{\tau}_{con} \tag{3.22}$$

$$P(x) = a \int_0^{h_m} \vec{\tau}_{tot} \left( \frac{\partial u}{\partial z} \right) dz$$
 (3.23)

where, a = Calculation cell area,  $mm^2$ 

#### 3.3 HMEHL 해석 모델 검증

본 연구에서는 기어 쌍의 부하 동력손실을 예측하기 위해 HMEHL 모델을 적용하였으며, 해석 솔버(solver)의 정확성을 검증하기 위해 선행 연구의 시험 결과와 비교하였다. 탄성유체윤활 해석에서 윤활막 내 압력 분포와 윤활유 두께 분포는 다중 물리 현상을 해석하기 위한 반복 계산 의 결과물로, 해석 솔버(solver)의 정확성을 검증하는 데 적합하다[187]. 본 연구에서는 Li와 Kahraman[187]이 개발한 탄성유체윤활 해석 솔버 (solver)의 검증에 사용된 시험 결과를 동일하게 사용하였다.

#### 3.3.1 윤활막 내 압력 분포 비교를 통한 모델 검증

윤활막 내 압력 분포를 검증하기 위해 Coulon 등[166]의 시험 결과를 이용하였다. 그는 Figure 3.3에 나타낸 것과 같이 볼에 임의의 표면 패임을 형성하고 Table 3.1에 나타낸 시험 조건에 대하여 표면 패임이 탄성유체윤활의 윤활막 압력 분포에 미치는 영향을 확인하였다. 표면 전단이크게 발생하도록 순수 미끄럼 운동을 모사하였으며, 25 mm/s와 75 mm/s의 두 가지 속도 조건에서 시험을 진행하였다.

![](_page_13_Figure_0.jpeg)

Figure 3.3 Ball geometry used in Coulon et al.[166]: (a) without dent; (b) with dent.

Table 3.1 Input parameters for EHL simulation for the comparison to Coulon et al.[166, 187]

| Parameter                             | Values                   |
|---------------------------------------|--------------------------|
| Dynamic viscosity at ambient pressure | 2.1 <i>Pa</i> · <i>s</i> |
| Pressure-viscosity coefficient        | $45.9 \ GPa^{-1}$        |
| Density at ambient pressure           | $1195 \text{ kg/m}^3$    |
| Normal load                           | 90 N                     |
| Sliding speed                         | 25,75 mm/s               |

본 연구에서는 Coulon 등[166]의 시험과 동일한 조건에서 윤활막 압력 분포 해석을 수행하였으며, 그 결과를 Figure 3.4에 나타냈다. 먼저, Figure 3.4(a)와 Figure 3.4(c)는 표면 패임이 없는 조건에서 두 속도 조건에 따른 윤활유의 압력 분포를 나타내며, 속도가 높아짐에 따라 윤활막의 최대 압력이 소폭 증가하는 것을 확인할 수 있었다. 또한, Figure 3.4(b)와 Figure 3.4(d)를 통해 표면 패임에 의한 압력 피크가 발생하는 것을 확인할 수 있었으며, 시험과 해석에서 매우 유사한 결과를 확인할 수 있었다. 속도가 높아짐에 따라 표면 패임에 의한 압력 상승 효과가 더욱 커지는

것을 확인하였다.

![](_page_14_Figure_1.jpeg)

Figure 3.4 Comparison of predicted pressure distribution to measurements of Coulon et al.[166]: (a) at 25 mm/s without dent; (b) at 25 mm/s with a dent; (c) 75mm/s without dent; (d) at 75 mm/s with a dent.

## 3.3.2 율확막 두께 비교를 통한 모델 검증

다음으로 Kaneta 등[165]의 윤활막 두께 시험 결과와 비교를 통해 해석 솔버(solver)를 검증하였다. 그는 Figure 3.5에 나타낸 것과 같이 볼에표면 파형을 임의로 가공하였으며, Coulon 등[166]의 시험과 마찬가지로 Table 3.2에 나타낸 것과 같이 순수 미끄럼 조건에서 21.6 mm/s 및 98 mm/s 두 가지 속도 조건으로 시험을 수행하였다.

![](_page_15_Figure_0.jpeg)

Figure 3.5 Ball geometry used in Kaneta et al.[165].

Table 3.2 Input parameters for EHL simulation for the comparison to Kaneta et al.[165, 187]

| Parameter                             | Values                |
|---------------------------------------|-----------------------|
| Dynamic viscosity at ambient pressure | 1.2366 Pa·s           |
| Pressure-viscosity coefficient        | $18  GPa^{-1}$        |
| Density at ambient pressure           | $878 \mathrm{kg/m^3}$ |
| Normal load                           | 39.2 N                |
| Sliding speed                         | 21.6,98 mm/s          |

그 결과, Figure 3.6에 나타낸 것과 같이 표면 파형을 따라 윤활막 두께 분포가 형성되는 것을 확인하였다. 또한, 저속 조건인 Figure 3.6(a)보다 고속 조건인 Figure 3.6(b)에서 더 두꺼운 윤활막 두께 분포를 가지는 것을 확인하였다. 이는 윤활유의 유입 속도가 증가함에 따라 윤활유의 공급량이 많아진 것으로, 시험과 해석 모두 유사한 결과를 보였다.

![](_page_16_Figure_0.jpeg)

Figure 3.6 Comparison of predicted film thickness distributions along the center line of the contact to measurements of Kaneta et al.[165]: (a) at 21.6 mm/s; (b) at 98 mm/s.

# 3.4 머신러닝을 이용한 HMEHL 해석 가속화

#### 3.4.1 인공신경망의 이론적 배경

본 연구에서는 HMEHL 해석을 최적설계에 적용하기 위해 머신러닝을 활용하였다. HMEHL 해석은 수렴을 위해 많은 반복 계산이 필요하여시간이 많이 소요되며, 다양한 제원 검토가 필요한 최적설계에 직접 적용하기에는 제한이 있다. 다양한 머신러닝 기법 중 인공신경망은 비선형관계와 다차원 문제에 적합하다고 알려져 있으며, 시스템의 중요한 특성을 자동적으로 학습할 수 있다는 장점을 가지고 있다. 또한 멀티 모달딥러닝(multimodal deep learning)과 같이 모달리티(modality)를 분리하여 새로운 아키텍처를 유연하게 구성할 수 있다는 장점을 가지고 있다[188]. 인공신경망은 크게 입력층, 은닉층, 출력층으로 구성되며, 비선형성을 반영하기 위해 두 개 이상의 은닉층을 사용하는 경우 일반적으로 깊은 신경망이라고 한다. 각 층을 구성하는 뉴런은 가중치, 편차 값, 활성함수가결합되어 그 결과를 출력한다. 인공신경망을 통해 예측한 결과값과 실제결과값의 차이를 통해 오차를 계산할 수 있으며, 오차가 최소화되도록가중치와 편차 값을 조정하는 과정을 모델의 학습이라고 한다. 이는 오

차의 최소화를 목적 함수로 갖는 최적화 과정이며 최적화 알고리즘으로 경사 하강법을 가장 많이 사용한다[188].

#### 3.4.2 머신러닝 모델의 입력 변수

본 연구에서는 머신러닝 모델 학습을 위해 20,000번의 HMEHL 해석을 수행하였다. 개발된 머신러닝 모델을 기어 쌍의 최적설계에 적용하기위하여 일반적인 기어의 작동 조건을 포함하도록 입력 변수 범위를 충분히 넓게 결정하였다[101, 168]. 윤활유 특성의 범위 또한 일반적인 윤활유의 유변학적 특성(rheological property)을 포함하도록 충분히 넓게 고려하였다[101, 102]. Table 3.3은 각 입력 변수의 범위를 나타낸다. 머신러닝 모델 학습 시 변수의 수를 줄이기 위해, 접촉하는 두 물체를 탄성 반 무한체로 가정하고  $E_1, E_2, \nu_1, \nu_2$  값을 이용해 축소 영률을 계산하여 사용하였다. 비커스 경도는 Akchurin 등[172]이 제안한 방법을 이용하여  $p_{asp}$ 의상한 값으로 결정하였다[171]. 표면 조도의 경우, 주어진 제곱 평균 제곱 근 높이( $R_q$ )와 상관 길이( $R_{cl}$ )에 대해 Garcia과 Stoll[189]의 모델을 이용하여 랜덤 표면을 생성하였으며 이 때 공학 분야에 널리 사용되는 가우스높이 분포와 지수 함수 자기상관 함수를 적용하였다[190, 191]. 자세한 내용은 3.4.3항에 다루었다.

Table 3.3 List of input parameters for HMEHL simulations

| Parameters                    | Minimum              | Maximum                 |
|-------------------------------|----------------------|-------------------------|
| Reduced Young's modulus, $E'$ | $2 \cdot 10^{11} Pa$ | 3 · 10 <sup>11</sup> Pa |
| Effective radius, $R$         | $5 \cdot 10^{-4} m$  | $5\cdot 10^{-1}~m$      |
| Length of contact, l          | $1\cdot 10^{-4}~m$   | $1\cdot 10^{-3}~m$      |
| Normal load, $F_N$            | $1\cdot 10^{-9} N$   | 500 <i>N</i>            |

| Surface velocity 1, $u_1$                  | $1\cdot 10^{-9}m/s$          | 30 m/s                       |
|--------------------------------------------|------------------------------|------------------------------|
| Surface velocity 2, $u_2$                  | $1\cdot 10^{-9}\ m/s$        | 30 <i>m/s</i>                |
| Base density, $\rho_0$                     | $750~kg/m^3$                 | $1000\ kg/m^3$               |
| Base viscosity, $\eta_0$                   | $5 \cdot 10^{-3} Pa \cdot s$ | $5 \cdot 10^{-2} Pa \cdot s$ |
| Pressure-viscosity coefficient, $\alpha_R$ | $5 \cdot 10^{-9} Pa^{-1}$    | $25 \cdot 10^{-9} Pa^{-1}$   |
| Roughness rms height, $R_q$                | $5 \cdot 10^{-8} m$          | $5\cdot 10^{-6} m$           |
| Roughness correlation length, $R_{cl}$     | $1\cdot 10^{-6}~m$           | $1\cdot 10^{-4} m$           |
| Vickers hardness of material, H            | 100 HV                       | 700 HV                       |
|                                            |                              |                              |

그 외 Roeland's equation과 Dowson-Higginson equation에 사용되는 윤활유 파라미터는 Table 3.4에 나타낸 것과 같이 사용하였다[169].

Table 3.4 Lubricant parameters used in Roeland's and Dowson-Higgison equation

| Lubricant fixed parameters                                | Values                    |
|-----------------------------------------------------------|---------------------------|
| Constant in Roeland's equation, $p_{0,R}$                 | 1.96 · 10 <sup>8</sup> Pa |
| $1^{\rm st}$ constant of Dowson-Higginson equation, $C_1$ | $5.9 \cdot 10^8 Pa$       |
| $2^{\rm nd}$ constant of Dowson-Higginson equation, $C_2$ | 1.34                      |

#### 3.4.3 표면 조도

윤활 해석에 표면 거칠기를 고려하기 위해서는 거칠기 데이터를 포함한 표면 정보가 필요하다. 표면 데이터는 직접 측정하여 얻어지거나수치적으로 생성하는 방법이 있다[192]. 직접 측정한 표면을 사용할 경우, 실제 시스템의 특성을 반영할 수 있어 해석 정확도가 높아지며, 수치적으로 생성한 표면은 표면을 정의하는 매개변수의 영향을 파악하는 데 유리하다. 본 연구는 표면에 대한 정확한 정보를 미리 알 수 없는 설계 단계에 적용하는 것이 주 목적으로, 수치적인 방법을 이용하여 표면을 생

성하였다. 실제 기어 시스템의 표면은 제작 공정에 따라 다양한 형태를 가진다. 통계적 방법을 통해 표면을 대표하는 매개변수를 도출할 수 있고, 반대로 매개변수를 통해 표면을 생성할 수 있다. 표면 돌기를 나타내는 매개변수로 돌기 크기를 나타내는 제곱 평균 제곱근 거칠기, 돌기의 높이 확률 밀도 함수의 좌우 대칭도를 나타내는 왜도(skewness), 날카로운 정도를 나타내는 첨도(kurtosis)가 있다. Figure 3.7은 두 매개변수에 따른 높이 확률 밀도 함수의 형태를 보여준다.

![](_page_19_Figure_1.jpeg)

Figure 3.7 Skewness and kurtosis of topography height distribution[190].

돌기 높이의 확률 밀도 함수를 정의한 후, 자기상관 함수의 상관 길이를 이용하여 돌기 간의 간격을 결정할 수 있다. 본 연구에서는 Garcia와 Stoll[189]가 제안한 몬테카를로 기반의 표면 생성 알고리즘을 이용하였다. Figure 3.8은 표면 생성 예시를 나타낸다. 예시에서 가우스 높이 확률 밀도 함수를 적용하였으며, 이는 왜도는 0, 첨도는 3을 의미한다. 상관 길이는 0.01을 갖는 지수 함수 형태의 자기상관 함수를 적용하였다.

![](_page_20_Figure_0.jpeg)

![](_page_20_Figure_1.jpeg)

Figure 3.8 Roughness generator based on Garcia and Stoll[189] when RMS height is 0.05, and correlation length is 0.01 (height distribution function is Gaussian and auto covariance function is exponential).

#### 3.4.4 머신러닝 모델

본 연구에서는 HMEHL 기반 부하 동력손실 해석의 속도를 향상하기 위해 머신러닝 기법을 사용하였으며 모델을 구성하기 위해 MATLAT의 Statistics and Machine Learning Toolbox와 Deep Learning Toolbox를 사용하였다. Figure 3.9는 머신러닝 학습을 위한 작업 흐름을 나타낸다. 데이터학습을 위해 Table 3.3에 나타낸 설계 변수 범위 내에서 Latin hypercube샘플링을 따라 20,000개의 HMEHL 데이터를 생성하였으며, 모델에 따른학습 성능 비교를 위해 단순 다층 퍼셉트론(multi-layer perceptron), 덧셈(addition) 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning)을 이용하였다. Figure 3.10은 각 모델의 아키텍처를 나타낸다. 세 가지 모델은 모두 인공신경망 모델의 일종으로 비선형 시스템에 대한 학습 성능이 우수한 것으로 알려져 있다. 유사 선행 연구[101, 102]에서는 다층 퍼셉트론(multi-layer perceptron)에서 가장 좋은 성능을 나타내었으며, 본 연구에서는 회귀 성능을 한층 더 개선하기 위해 멀티 모달 딥러닝(multimodal deep learning)을 추가로 적용하였다.

멀티 모달 딥러닝(multimodal deep learning)은 여러 데이터 소스를 융합 학습하여, 장단기 메모리(long short-term memory)와 합성곱 신경망(convolutional Neural Network) 등 다양한 딥러닝 모델의 장점을 결합해 이미지 분류, 캡셔닝(captioning), 회귀에 널리 사용된다[105-109]. 멀티 모달딥러닝(multimodal deep learning)은 모달리티 내 상관관계(within-modality)를잘 학습할 수 있으나, 교차 모달리티 간(cross-modality) 관계 모델링에는 효과적이지 않을 수 있어 데이터 융합의 잠재력을 온전히 활용하지 못하는 경우가 발생할 수 있다. 그러나 이러한 단순화는 모델의 복잡성을 줄

여주고 과적합 위험을 낮추는 데 효과적이다. 따라서 각 모달리티 (modality)가 독립적으로 결과에 영향을 미치는 경우, 한계적 표현 (marginal representation)에 집중하는 것이 유리할 수 있다[109]. 융합 방식에는 초기 융합(early fusion), 중간 융합(intermediate fusion), 후기 융합(late fusion)이 있으며, 중간 융합을 적용하였을 때 각 모달리티(modality)의 한계적 표현을 학습하고, 결합 전 모달리티(modality) 내 상관관계를 효과적으로 학습할 수 있다[109].

![](_page_22_Figure_1.jpeg)

Figure 3.9 Workflow of the machine learning approach for HMEHL simulation.

![](_page_22_Figure_3.jpeg)

Figure 3.10 Architecture of machine learning model used in this study: (a) simple

모달리티(modality)의 분류는 각 모달리티(modality)가 독립적으로 결과에 영향을 미치도록 HMEHL 해석의 도메인 지식을 이용하여 12개의입력 변수를 표면 조도 모달리티(modal 1), 기하학적 모달리티(modal 2), 운전 조건 모달리티(modal 3), 윤활 모달리티(modal 4)로 구분하였다. 조도의 제곱 평균 제곱근 높이, 상관 길이, 비커스 경도, 축소 영률은 homogenization factor를 계산할 때 사용되며, 유효 반경과 접촉 길이는 유한체적법의 도메인을 결정할 때 사용된다. 또한 수직 하중과 속도는 Reynolds 방정식을 해석하기 위한 조건으로 사용되며 밀도, 점도, 압력-밀도 계수는 주어진 환경에 따라 달라지는 윤활유의 특성을 계산하기 위해 사용된다.

은닉층과 뉴런의 수는 정해진 규칙이 없어 과소적합과 과적합을 방지하기 위해 시행착오법을 통해 결정하였다. 단순 다층 퍼셉트론(multi-layer perceptron), 덧셈 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning), 결합 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning)에 대해 동일하게 4개의 은닉층을 구성하고 각 은닉층이 128, 128, 64, 64개의 뉴런을 가지도록 결정하였다. Figure 3.10에 나타낸 것과 같이 멀티모달 딥러닝(multimodal deep learning)의 각 모달리티(modality)에서 동일한은닉층과 뉴런수를 가지며 두 개의 은닉층 사이에 융합 층을 배치하여중간 융합을 적용하였다. 이때 단순 다층 퍼셉트론(multi-layer perceptron)은 30657, 덧셈 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning)은 11073개, 결합 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning)은 17217개의 모델 파라미터(weigh 및 bias)를 가진다. 동일한 은

닉층 수와 뉴런 수를 가지더라도 모델의 아키텍처 개선을 통해 복잡도를 낮출 수 있다[109].

모델 파라미터의 최적화 알고리즘은 확률적 경사 하강법를 이용하였으며[193], 데이터는 과적합을 방지하기 위해 학습, 검증, 테스트 세트를 각각 70%, 15%, 15%로 설정하였다. Table 3.3에 나타낸 것과 같이 입력 변수는 서로 다른 차원의 값을 가지며 학습 성능을 개선하기 위해 Eq(3.24)를 통해 입력 변수가 -1과 1 사이의 값을 가지도록 정규화하였다. 각 뉴런은 활성함수를 통해 결과 값을 도출하며 본 연구에서는 렐루 (ReLu) 활성함수를 사용하였다 Eq(3.25).

$$x_{scaled} = 2 \cdot \frac{x - x_{min}}{x_{max} - x_{min}} - 1 \tag{3.24}$$

$$f(x) = \begin{cases} x \ (x \ge 0) \\ 0 \ (x < 0) \end{cases}$$
 (3.25)

## 3.5 결과 및 고찰

## 3.5.1 머신러닝 모델에 따른 결정계수 비교

본 연구에서는 학습, 검증, 시험 세트를 각각 70%, 15%, 15%로 분류하여 학습을 수행하였으며 결과는 다음과 같다. 먼저 Figure 3.11은 다층 퍼셉트론(multi-layer perceptron)의 결정계수를 나타낸 것이다. 학습 데이터에 대해서는 0.99975, 검증 데이터에 대해서는 0.99370, 테스트 데이터에 대해서는 0.98758의 결정계수를 보였으며 전체 데이터에 대해서는 0.99673의 결정계수를 가진다. Figure 3.12는 덧셈 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning)의 결정계수를 나타낸 것이며 학습, 검증, 시험, 전체 데이터에 대해 0.99761, 0.99401, 0.99458, 0.99658을 가진다. 마지막으로, 결합 계층을 적용한 멀티 모달 딥러닝(multimodal deep

learning)은 Figure 3.13과 같이 학습, 검증, 시험, 전체 데이터에 대해 0.99832, 0.99642, 0.99488, 0.99748의 결정계수를 가지는 것을 확인하였다.

Table 3.5는 각 모델의 성능을 나타낸 것이다. 결합 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning), 덧셈 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning), 단순 다층 퍼셉트론(multi-layer perceptron) 순으로 테스트 샘플에 대한 높은 회귀 성능을 나타냈다. 기존의 문헌들에서는 단순 다층 퍼셉트론(multi-layer perceptron)이 가장 높은 성능을 보였지만, 본 연구에서는 멀티 모달 딥러닝(multimodal deep learning)을 적용하여 결정계수를 약 0.0073 개선할 수 있었다[101, 102]. 또한 단순 다층 퍼셉트론(multi-layer perceptron)과 비교하였을 때 덧셈 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning)은 약 36%, 결합 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning)은 56%의 파라미터수를 가지며 과적합을 예방하고 계산 비용을 절약할 수 있었다.

![](_page_26_Figure_0.jpeg)

Figure 3.11 Predicted values versus true values for training, validation, test and overall data set for simple MLP.

![](_page_27_Figure_0.jpeg)

Figure 3.12 Predicted values versus true values for training, validation, test and overall data set for MMDL-Add.

![](_page_28_Figure_0.jpeg)

Figure 3.13 Predicted values versus true values for training, validation, test and overall data set for MMDL- Concat.

Table 3.5 Comparison of machine learning model performance for the prediction of load dependent power loss

|                      | Machine learning model |          |             |  |
|----------------------|------------------------|----------|-------------|--|
|                      | MLP                    | MMDL-Add | MMDL-Concat |  |
| $R^2$                | 0.98758                | 0.99458  | 0.99488     |  |
| Number of parameters | 30657                  | 11073    | 17217       |  |

## 3.5.2 머신러닝 모델의 강건성 검토

머신러닝 모델의 학습을 위해서는 HMEHL 해석을 통한 데이터 생성 과정이 필요하며, 이는 시간이 많이 소요되어 머신러닝 모델 개발 시고려되어야 한다. 목표하는 학습 성능에 필요한 데이터 수를 사전에 가

능할 수 있다면 효율적인 머신러닝 모델을 개발할 수 있다. 따라서 본연구에서는 학습 데이터의 수에 따른 결정계수를 확인하였으며 Table 3.6은 그 결과를 보여준다. 데이터 수를 50%로 줄였을 때 다층 퍼셉트론 (multi-layer perceptron), 덧셈 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning), 결합 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning)은 각각 0.98272, 0.99223, 0.99245의 결정계수를 보이며 전체 데이터를 사용했을 때와 비교하였을 때 성능 차이가 거의 나타나지 않는 것으로 판단된다. 25%이하의 데이터를 사용할 때 모델 성능이 급격하게 악화되는 것을 확인하였으며, 이는 Marian 등[101]의 결과와 유사하다.

Table 3.6 Coefficients of determination for the prediction of power loss according to the number of training data

|                           | Machine learning model |         |         |         |
|---------------------------|------------------------|---------|---------|---------|
|                           | Percentage of the      | MLP     | MMDL-   | MMDL-   |
|                           | training data used     |         | Add     | Concat  |
| Training $R^2$            | 10 %                   | 0.99996 | 0.99939 | 0.99972 |
|                           | 25 %                   | 0.99983 | 0.99793 | 0.99887 |
|                           | 50%                    | 0.99982 | 0.99715 | 0.99914 |
|                           | 75 %                   | 0.99974 | 0.99830 | 0.99800 |
|                           | 100 %                  | 0.99975 | 0.99761 | 0.99832 |
| Validation R <sup>2</sup> | 10 %                   | 0.93087 | 0.96743 | 0.98040 |
|                           | 25 %                   | 0.96562 | 0.98236 | 0.98046 |
|                           | 50%                    | 0.98301 | 0.99154 | 0.99372 |
|                           | 75 %                   | 0.98770 | 0.99240 | 0.99415 |
|                           | 100 %                  | 0.99370 | 0.99401 | 0.99642 |
| Test R <sup>2</sup>       | 10 %                   | 0.93496 | 0.96710 | 0.97822 |
|                           | 25 %                   | 0.96386 | 0.98463 | 0.98346 |

| 50%   | 0.98272 | 0.99223 | 0.99245 |
|-------|---------|---------|---------|
| 75 %  | 0.98771 | 0.99405 | 0.99442 |
| 100 % | 0.98758 | 0.99458 | 0.99488 |

각 모델의 과적합 정도를 확인하기 위해 학습 데이터 수에 따른 학습 세트와 테스트 세트 결정계수의 차이를 확인하였다(Figure 3.14). 모든 모델과 학습 데이터 수에서 학습 세트의 결정계수는 1에 가까운 값을 나타냈으며, 테스트 세트와의 결정계수 차이를 통해 각 모델의 강건성을 확인할 수 있었다. 다층 퍼셉트론(multi-layer perceptron)은 모든 학습 데이터 수에서 가장 큰 결정계수 차이를 보이며 높은 분산 특성을 나타냈다. 반면, 덧셈 계층과 결합 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning) 모델은 전체 데이터 범위에서 유사하게 낮은 분산 특성을 보여더 강건함을 확인할 수 있었다.

![](_page_30_Figure_2.jpeg)

Figure 3.14 R-square differences between training and test results according to the percentage of training data used.

결합 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning)은 1에 가까운 0.99488의 결정계수를 나타내며 높은 회귀 성능을 보였으나, 약 0.005 정도의 정보 손실이 발생하였다. 이는 조도 표면의 랜덤성에 의한 것으로 추측된다. HMEHL 모델에 고려한 조도 표면의 경우 입력되는 제곱 평균 제곱근 높이 $(R_q)$ 와 상관 길이 $(R_{cl})$ 를 바탕으로, 가우스 높이 분포와 지수 함수 자기상관 함수를 이용하여 무작위 조도 표면을 생성하므로 확률에 의존하게 된다. 따라서 동일한 입력 조건을 가지더라도  $p_{asp}$ 와  $p_{con}$ 에 차이가 발생할 수 있다.

#### 3.5.3 머신러닝 모델을 이용한 기어 쌍 부하 동력손실 해석

앞서 기술한 모델의 강건성과 예측성능을 고려하여 결합 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning)을 최종 모델로 선정하였으며, 모델의 신뢰성을 검증하기 위해 미리 학습된 결합 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning)을 활용하여 기어 쌍의 부하 동력손실을 해석하였다. 이후 Petry-Jonhson 등[167]의 시험 결과와 비교하여 모델을 검증하였다.

Table 3.7은 대상 기어 쌍의 제원 정보를 나타낸 것이다. 해당 기어 쌍에 대해 2000, 4000, 6000, 8000, 10000 rpm의 속도 조건과 413 Nm의 하중조건에서 Full HMEHL, 결합 계층 적용 멀티 모달 딥러닝(multimodal deep learning), 시험 결과의 평균 부하 동력손실을 비교하였다. 평균 부하 동력손실은 기초 피치에 해당하는 부하 동력손실의 평균 값을 나타내며, 기어 쌍의 피치, 런아웃 오차의 영향을 무시할 경우 기초 피치 간격으로 동일한 부하 동력손실이 반복된다. 윤활유는 기어 시스템에 널리 사용되는 75W90을 적용했으며, 탄성유체윤활 해석에 필요한 윤활유 특성은

Table 3.8의 값을 사용하였다.

Table 3.7 Basic design parameters of the spur gear considered[167]

| Parameter                  | Values         |  |  |
|----------------------------|----------------|--|--|
| Module                     | 2.32 mm        |  |  |
| Pressure angle             | 28.0 °         |  |  |
| Pitch diameter             | 92.74 mm       |  |  |
| Base diameter              | 81.89 mm       |  |  |
| Outside diameter           | 95.95 mm       |  |  |
| Root diameter              | 85.90 mm       |  |  |
| Start of active profile    | 87.73 mm       |  |  |
| Circular tooth thickness   | 2.925 mm       |  |  |
| Root fillet                | 0.83 mm        |  |  |
| Face width                 | 26.7 mm        |  |  |
| Center distance            | 91.5 <i>mm</i> |  |  |
| Measured roughness $(R_q)$ | $0.07~\mu m$   |  |  |

Table 3.8 Basic parameters of the 75W90 gear oil[168]

| Parameter                                  | Values                       |
|--------------------------------------------|------------------------------|
| Pressure-viscosity coefficient, $\alpha_p$ | $9.68 \cdot 10^{-9} Pa^{-1}$ |
| Base density, $\rho_0$                     | 799.30 $kg/m^3$              |
| Base viscosity, $\eta_0$                   | 0.0106 <i>Pa</i> · <i>s</i>  |

다섯 개의 속도 조건에 대해 총 510개의 탄성유체윤활 해석 데이터를 얻었으며, Figure 3.15는 결합 계층을 적용한 멀티 모달 딥러닝 (multimodal deep learning)과 Full HMEHL 간의 부하 동력손실 결과를 비교한 것이다. 그 결과, 결정계수 0.97777를 보였으며 결합 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning) 모델이 Full HMEHL과 유사한

결과를 나타내는 것을 확인하였다. Figure 3.16은 각 속도 조건에 대해 시험, 결합 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning), Full HMEHL, Li 등[168]의 탄성유체윤활 해석 결과, Xu [16]가 제안한 다중 선형 회귀법을 통해 해석한 부하 동력손실을 나타낸 것이다. 속도가 증가함에 따라 부하 동력손실이 증가하는 경향을 보였으며, 멀티 모달 딥러닝(multimodal deep learning)과 Full HMEHL 모두 시험 결과와 유사한 경향을 보였다.

시험 결과에서 8000~10000 rpm 구간에서 부하 동력손실의 증가율이 감소하는 것을 확인할 수 있었으며, 이는 열 연화에 의한 것으로 보고되었다[168]. 기어의 회전 속도가 높아짐에 따라 접촉면의 온도가 상승하게되고, 그에 따라 윤활유의 점도가 감소하게 된다. 본 연구에서 적용한 HMEHL 모델은 3.2.3.3항에 나타낸 것과 같이 등온 조건으로 가정함에따라 열 연화 현상을 반영하지 못해 약간의 차이가 발생하였다. 이는 Figure 3.16에 나타낸 Li 등[168]과 유사한 결과로 합리적인 수준의 오차로 판단하였다.

추가적인 비교를 위해 Xu [16]가 제안한 다중 선형 회귀법을 적용하여 부하 동력손실을 해석하였으며, 마찬가지로 Figure 3.16에 나타낸 것과 같이 본 연구에서 제안하는 두 가지 방법과 매우 유사한 결과를 확인할수 있었다. Xu [16]는 다중 선형 회귀식을 개발하기 위해 Petry-Jonhson 등 [167]이 시험에서 사용한 것과 동일한 윤활유(75W90)를 사용해 회귀 계수를 도출하였으며, 그에 따라 높은 정확성을 보인 것으로 판단하였다. 하지만 다중 선형 회귀식을 사용해 다른 윤활유에 대한 부하 동력손실을 예측하기 위해서는 새로운 회귀 계수를 도출해야 하는 한계가 존재한다. 해당 방법은 3.5.4.4항에 자세하게 기술하였다.

Table 3.9는 기어 쌍의 부하 동력손실을, Table 3.10은 효율에 대해 시험 결과와 상대 오차를 비교한 것이다. 대상 기어 쌍의 효율에 대한 멀티 모달 딥러닝(multimodal deep learning)의 최대 오차는 6000 rpm 조건에서 0.08%로 실제 기어 쌍의 동력 효율을 잘 예측하였다. 동력손실의 경우, 6000 rpm 조건에서 0.2 kW 차이로 43.02 %의 상대 오차를 보였다. 해석 모델에서 실제 시험에서 발생하는 열 연화 현상을 고려하지 못한점과전달 동력이 약 260 kW인 점을 고려할 때, 합리적인 수준의 오차로 판단하였다.

![](_page_34_Figure_1.jpeg)

Figure 3.15 MMDL-Concat versus Full HMEHL for gear power loss calculation.

![](_page_34_Figure_3.jpeg)

Figure 3.16 Comparison of predicted mean power loss to the measurements of

Table 3.9 Relative error to the test results of Petry-Johnson et al. in load dependent power loss[167]

| Rotational | Full HMEHL | MMDL    | Li's results[168] | Xu's method[16] |
|------------|------------|---------|-------------------|-----------------|
| speed      |            |         |                   |                 |
| 2000 rpm   | 27.10 %    | 34.99 % | 17.39 %           | 33.92 %         |
| 4000 rpm   | 28.07 %    | 39.36 % | 0.719 %           | 29.71 %         |
| 6000 rpm   | 31.28 %    | 43.02 % | 3.84 %            | 33.80 %         |
| 8000 rpm   | 31.42 %    | 34.96 % | 6.91 %            | 35.51 %         |
| 10000 rpm  | 16.69 %    | 15.76 % | 13.76 %           | 23.66 %         |

Table 3.10 Relative error to the test results of Petry-Johnson et al. in efficiency[167]

| Rotational | Full HMEHL | MMDL   | Li's method[168] | Xu's       |
|------------|------------|--------|------------------|------------|
| speed      |            |        |                  | method[16] |
| 2000 rpm   | 0.05 %     | 0.07 % | 0.03 %           | 0.07 %     |
| 4000 rpm   | 0.05 %     | 0.07 % | 0.00 %           | 0.05 %     |
| 6000 rpm   | 0.06 %     | 0.08 % | 0.01 %           | 0.06 %     |
| 8000 rpm   | 0.06 %     | 0.06 % | 0.01 %           | 0.06 %     |
| 10000 rpm  | 0.02 %     | 0.02 % | 0.02 %           | 0.03 %     |

Figure 3.17은 두 방법을 통해 해석한 기어 쌍의 물림 위치에 따른 동력손실을 나타낸다. 결합 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning) 모델이 Full HMEHL에 비해 부하 동력손실을 과소평가하는 경향이 나타났으나, 허용할 만한 차이로 전 운전 영역에서 경향성을 잘예측하는 것으로 판단하였다. 물림 치의 개수에 따라 LPSTC와 HPSTC에서 하중 변화가 발생하고 그에 따라 부하 동력손실에 급격한 변화가 발

생하는 것을 확인할 수 있으며, 피치 점 부근에서 미끄럼 속도가 감소하여 부하 동력손실이 급격하게 낮아지는 것을 확인하였다. 머신러닝을 통해 평균 부하 동력손실뿐만 아니라 기어 쌍의 물림 위치에 따른 부하 동력손실도 예측할 수 있었다. 마지막으로, Table 3.11은 2000 rpm의 속도 조건에서 기어 쌍 동력손실 해석에 필요한 시간을 멀티 모달 딥러닝 (multimodal deep learning)과 Full HMEHL로 비교한 결과를 나타낸 것으로, 멀티 모달 딥러닝(multimodal deep learning)을 적용했을 때 해석 시간을 약0.05%로 단축시킬 수 있다.

![](_page_36_Figure_1.jpeg)

Figure 3.17 Comparison of the local power loss between MMDL-Concat and Full HMEHL as a function of mesh cycle.

Table 3.11 Computational time for calculating the power loss of spur gear

|                    | Full HMEHL | MMDL   |
|--------------------|------------|--------|
| Computational time | 21,037 sec | 10 sec |

## 3.5.4 기존 기법과의 비교

1.3.2항에서 언급한 것처럼, 지금까지 선행 연구를 통해 부하 동력손 실 해석을 위한 다양한 방법이 제안되었으며, 본 항에서는 각 방법에 따 른 부하 동력손실 해석 결과를 다루었다. 본 연구에서 제안하는 머신러 닝 기반 탄성유체윤활 해석법과 더불어 가장 많이 사용되는 경험식 기반 의 방법과 다중 선형 회귀를 통한 탄성유체윤활 해석법을 소개하였다.

#### 3.5.4.1 Schlenk의 평균 마찰계수 해석

손실계수를 이용하여 부하 동력손실을 해석하기 전에 맞물리는 두기어 쌍의 평균 마찰계수에 대한 정보가 필요하다. 그중 Schlenk[194]가 제안한 경험식은 가장 널리 사용되는 방법 중 하나로 Eq(3.26)과 같이계산할 수 있다.

$$\mu_{mz} = 0.048 \left(\frac{F_{bt}}{b} \frac{1}{\nu_{\Sigma} \rho_{redC}}\right)^{0.2} \eta_{oil}^{-0.05} Ra^{0.25} X_L$$
 (3.26)

where,  $\mu_{mz}$  = Average friction coefficient

 $F_{bt}$  = Circumferential force at base circle, N

b = Face width, mm

 $\nu_{\Sigma}$  = Sum speed at operating pitch circle, m/s

 $\rho_{redC}$  = Reduced radius of curvature at pitch point, mm

 $\eta_{oil}$  = Lubricant viscosity, mPa·s

Ra = Mean surface roughness,  $\mu m$ 

 $X_L$  = Factor for oil type

## 3.5.4.2 Niemann의 손실계수 해석

손실계수를 해석하기 위한 방법은 오랜 시간 동안 다양한 수식이 제안되어 왔으며, 그중 Niemann과 Winter[68]가 제안한 방법은 가장 기초적인 방법으로 ISO 14179-2[195] 규격으로 채택되어 다양한 최적설계에 사용되어 왔다[6, 127]. Eq(3.27)은 손실계수를, Eq(3.28)은 손실계수와 평균마찰계수를 통해 부하 동력손실을 해석하는 방법을 나타낸다.

$$H_V = \frac{(u+1)\pi}{z_1 u cos(\beta_b)} (1 - \varepsilon_\alpha + \varepsilon_1^2 + \varepsilon_2^2)$$
 (3.27)

$$P_{VZP} = \mu_{mz} H_V P_A \tag{3.28}$$

where,  $H_V$  = Gear loss factor

u = Gear ratio

 $z_1$  = Number of teeth in pinion

 $\beta_b$  = Base helix angle, deg

 $\varepsilon_{\alpha}$  = Transverse contact ratio

 $\epsilon_1$  = Addendum contact ratio of pinion

 $\varepsilon_2$  = Addendum contact ratio of wheel

 $P_{VZP}$  = Load dependent gear losses, kW

 $P_A$  = Transmitted power, kW

#### 3.5.4.3 Wimmer의 손실계수 해석

앞서 언급한 Niemann과 Winter[68]가 제안한 방법은 기어 쌍의 제원에 의해 결정되는 매개변수로만 손실계수를 해석하며, 하중 분할 특성을 고려하지 못한다. 이로 인해 치형 수정, 치형오차 등을 고려할 수 없으며, 헬리컬 기어에 대한 정확성이 낮은 것으로 알려져 있다[73]. 이를 개선하기 위해 Wimmer[71]는 부하 접촉 해석을 통해 얻어지는 하중 분할 특성을 고려할 수 있도록 새로운 손실계수 수식을 제안하였다. Eq(3.29)는 이를 나타낸다.

$$H_{VL} = \frac{1}{p_{et}} \int_0^b \int_A^E \frac{f_N(x, y)}{F_{bt}} \frac{v_g(x, y)}{v_t} dx dy$$
 (3.29)

$$P_{VZP} = \mu_{mz} H_{VL} P_A \tag{3.30}$$

where,  $H_{VL}$  = Local gear loss factor

 $p_{et}$  = Transverse path of contact, mm

 $f_N$  = Nominal load, N

 $F_{bt}$  = Circumferential force at base circle, N

 $v_g$  = Sliding velocity, m/s

v<sub>t</sub> = Circumferential speed at pitch point, m/s

#### 3.5.4.4 Xu의 EHL 다중 선형 회귀 방법

마지막으로, Xu 등[16]은 탄성유체윤활 해석을 가속화하기 위해 다중 선형 회귀식을 제안하였다. Eq(3.31)~Eq(3.32)를 통해 기어 쌍의 위치에 따른 마찰계수를 도출할 수 있으며, Eq(3.33)을 통해 최종적으로 부하 동 력손실을 해석할 수 있다. 이는 Wimmer[71]의 방법과 마찬가지로, 부하 접촉 해석을 통해 얻어지는 하중 분할 특성을 고려할 수 있다. Table 3.12 은 해석에 필요한 계수들을 나타낸다.

$$\mu^{Hai} = e^{f(SR, P_h, \nu_0, S)} p_h^{b_2} |SR|^{b_3} V_e^{b_6} \nu_0^{b_7} R^{b_8}$$
 (3.31)

$$f(SR, P_h, \nu_0, S) = b_1 + b_4 |SR| P_h \log_{10}(\nu_0) + b_5 e^{-|SR| P_h \log_{10}(\nu_0)} + b_9 e^{S}$$
(3.32)

$$P_{VZP} = \mu^{Hai} W \omega \tag{3.33}$$

where,  $\mu^{Hai}$  = Friction coefficient using Xu's method

SR = Slide to roll ratio

 $v_0$  = Viscosity at oil inlet under ambient pressure, mPa·s

 $V_e$  = Entraining velocity, m/s

S = Surface roughness,  $\mu m$ 

P<sub>h</sub> = Maximum Hertzian pressure, MPa

W = Normal load, N

#### = Rotational speed, rad/s

ω

Table 3.12 Coefficients for the EHL based formula[16]

| Coefficients | Values    |
|--------------|-----------|
| $b_1$        | -8.916465 |
| $b_2$        | 1.03303   |
| $b_3$        | 1.036077  |
| $b_4$        | -0.354068 |
| $b_5$        | 2.812084  |
| $b_6$        | -0.100601 |
| $b_7$        | 0.752755  |
| $b_8$        | -0.390958 |
| $b_9$        | 0.620305  |

Figure 3.18은 앞서 언급한 각 방법을 이용하여 Petry-Jonhson 등[167]의 시험과 동일한 조건에서 기어 쌍의 부하 동력손실을 해석한 결과이다. 대상 기어 쌍은 치형 수정이 없는 간단한 형태의 스퍼 기어로, 네 가지방법 모두 큰 오차 없이 부하 동력손실을 예측할 수 있었다. 또한, 경험식 기반의 마찰계수와 손실계수를 이용하는 Niemann과 Wimmer의 방법은 서로 매우 유사한 결과를 나타냈으며, 마찬가지로 탄성유체윤활 해석기반의 두 해석(Xu, MMDL-Concat) 또한 유사한 결과를 보였다. 스퍼 기어에 대해서는 해석 방법에 따른 정확도의 차이가 크지 않았으나, 경험식 기반의 방법은 윤활유의 종류에 따라 XL과 같은 계수를 사용자가 정의해 주어야 하는 단점이 있으며, 다중 선형 회귀식을 이용한 방법 역시윤활유에 따라 새로운 계수를 도출해야 하는 번거로움이 존재한다. 해석의 자유도와 정확성을 고려하였을 때 본 연구에서 제안하는 머신러닝 기

반의 탄성유체윤활 해석 기법은 최적설계를 위한 솔버(solver)로 적합하다고 판단하였다.

![](_page_41_Figure_1.jpeg)

Figure 3.18 Comparison of the mean power loss with the traditional methods.

## 3.6 결론

본 연구에서는 기어 쌍 부하 동력손실 해석을 위해 기존 연구에서 사용한 탄성유체윤활 해석을 가속화하기 위해 머신러닝 모델을 적용하였다. 표면 조도와 캐비테이션을 고려한 HMEHL를 적용하여 데이터를 생성하고 기존 연구에서 사용한 다층 퍼셉트론(multi-layer perceptron)뿐만아니라 멀티 모달 입력에 대해 회귀 성능이 우수하다고 알려진 멀티 모달 딥러닝(multimodal deep learning) 모델을 사용하여 각 모델의 결정계수를 확인하였다. 본 연구의 결론을 요약하면 다음과 같다.

- 1) 표면 조도를 고려하여 부하 동력손실을 예측하기 위해 HMEHL 혼합 탄성유체윤활 해석 모델을 개발하였으며, 선행 연구의 시험 결과와 윤활유의 압력 구배 및 두께 분포를 비교하여 모델을 검증하였다.
- 2) 단순 다층 퍼셉트론(multi-layer perceptron), 덧셈 계층을 적용한

멀티 모달 딥러닝(multimodal deep learning), 결합 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning)의 결정계수는 각각 0.98758, 0.99458, 0.99488로 나타났으며, 그중 결정계수가 가장 높 은 결합 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning) 을 최종 모델로 선정하였다.

- 3) 개발된 머신러닝 모델을 이용하여 실제 기어 쌍의 부하 동력손실을 해석하고, 시험 결과와 비교하여 개발된 머신러닝 모델을 검증하였다. 그 결과, 결정계수 0.97777을 나타내며 Full HMEHL결과를 잘 추종하였으며, 시험 결과와 비교하였을 때, 기어의 효율에서 최대 0.08%의 오차를 나타내며 기어 쌍의 동력손실과 효율을 정확하게 예측하였다.
- 4) 결합 계층을 적용한 멀티 모달 딥러닝(multimodal deep learning)을 사용하였을 때, 기존 Full HMEHL 대비 해석 시간을 약 0.05%로 단축시킬 수 있었다.

본 연구에서 제안하는 방법은 스퍼 기어뿐만 아니라, 조각 이론을 통해 헬리컬 기어의 부하 동력손실 해석에 적용할 수 있다. 헬리컬 기어의 경우 치폭 방향으로 접촉 파라미터가 달라짐에 따라, 조각 이론을 이용하여 많은 수의 탄성유체윤활 해석이 요구되어 기존의 연구에서는 해석이 제한되었으나 머신러닝 기법을 이용하여 이를 극복할 수 있다. 또한 이를 기어 쌍의 강건 최적설계에 적용하여 넓은 해 공간에 대한 부하동력손실 해석을 수행할 수 있다.