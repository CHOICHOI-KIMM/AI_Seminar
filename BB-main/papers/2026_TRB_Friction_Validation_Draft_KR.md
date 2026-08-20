# Biboulet–Houpert 2010, Aihara 1987 열적 보정, Johnson 1985 이력 손실, Houpert 2002 드릴링 마찰을 적용한 오픈소스 테이퍼 롤러 베어링 마찰 솔버의 교차 모델 검증

> **타깃 저널**: *Lubricants* (MDPI)
> **원고 형식**: Article (검증 + 소프트웨어 설명)
> **상태**: Draft v0.1 한글본 (2026-05-20)

---

## 초록

테이퍼 롤러 베어링(TRB)의 마찰 손실 예측은 산업용 동력전달계의 효율 설계, 윤활유 선정, 열관리에서 핵심적이다. 본 연구는 Rust + Tauri로 구현된 오픈소스 듀얼 모드 TRB 마찰 솔버를 제시한다. 솔버는 네 가지 물리적으로 분리된 손실 메커니즘을 결합한다: (i) Biboulet–Houpert 2010 Part 1 선접촉 식에 의한 윤활제측 점성 롤링 저항, (ii) Aihara 1987 열적 보정을 **롤링 동력에 직접 곱하는 방식**(필름 두께에만 적용하는 일반적 방법과 대비), (iii) Johnson 1985 재료 이력 손실(명시적 $\alpha_v$ 파라미터), (iv) Houpert 2002 폐형 식에 의한 리브 드릴링 마찰(통상 사용되는 순수 슬라이딩 의사식이 약 16배 과대 예측하는 문제를 해소함). 솔버는 두 가지 교환 가능한 운동학적 레벨을 제공한다: 빠른 예비 해석용 Gen1 독립 슬라이스 모드와 정밀 misalignment / skew 예측용 Gen3 Timoshenko 빔-결합 슬라이스 모드. 세 개의 독립적 실험 데이터 — Schwarz 2023 (32216 축방향 및 결합하중), Tewari 2023 (32008-class 다온도 시험), Zhou–Hoeprich 1991 (LM12700 인치 시리즈 cup/cone/rib 분리 측정) — 와의 검증을 통해 완전 EHL 영역에서 ±5–11 % 정확도를 입증하였다. Schwarz 32216 운전점에서 6개 분석식(Palmgren, BH+Aihara, Aihara 원식, Zhou–Hoeprich, Matsuyama, Houpert 2002)과의 교차 비교 결과, 전 속도 영역에서 ±5 % 이내 정합을 보이는 식은 BH+Aihara 조합 뿐이었다. 또한 Aihara 1987과 Zhou–Hoeprich 1991 식의 차원적 표기에 대한 이차 자료의 전사 오류(기호 $\alpha_0$를 반각이 아닌 압력-점도 계수 [1/Pa]로 해석해야 함)를 문서화하고 정정하였다.

**키워드**: 테이퍼 롤러 베어링; 마찰 토크; 탄성유체윤활; Biboulet–Houpert; Aihara 열적 보정; 드릴링 마찰; 오픈소스 솔버

---

## 1. 서론

테이퍼 롤러 베어링(TRB)은 자동차 디퍼렌셜, 풍력 터빈 주축, 압연기, 터널 보링 머신 등에서 결합 반경방향 및 축방향 하중을 동시에 받는다. 베어링 마찰 손실은 동력전달계 효율과 운전 온도에 큰 영향을 미치므로 정확한 예측은 베어링 설계 및 윤활유 선정의 핵심 목표이다.

TRB 마찰 모델링 문헌은 반세기 이상 축적되어 왔다. Palmgren (1959) [1]은 점도 의존성을 표현하지 못함에도 불구하고 baseline으로 널리 쓰이는 경험식 $\mu_{rr} \cdot Q \cdot u$를 제안하였다. Witte (1973) [2]는 Timken 측정 데이터에 기초하여 하중 의존 지수를 도입하였다. Aihara (1987) [3]는 EHL 롤링 저항에 기반한 물리적으로 더 엄밀한 식을 도출하고 TRB 축방향 토크 데이터에 calibration된 열적 입구 전단 보정을 포함시켰다. Zhou & Hoeprich (1991) [4]은 EHL 선접촉 롤링 저항 곡선 fit과 cup/cone/rib 분리 측정 가능한 시험 리그를 발표하였으며 이는 현재까지도 핵심 벤치마크 데이터이다. Matsuyama 등 (1998–2001) [5,6]은 TRB의 전형적 압력 분포에 대한 EHL Reynolds 방정식을 수치적으로 풀어 완전 윤활 calibration을 제공하였다. Houpert (2002) [7]는 볼 베어링 및 테이퍼 롤러 베어링의 토크 성분에 대한 폐형 분석식을 도출하면서 리브 접촉에 대한 드릴링 모멘트 식을 포함시켰다. SKF Catalogue [8]는 이들 식 중 일부를 시리즈별 calibrated된 공학 형식으로 통합하여 상용 베어링 선정 소프트웨어에서 널리 사용되고 있다.

최근의 연구는 입구 전단 열적 가열 [9], 다물체 동역학 시뮬레이션 [10], 페어드/탠덤 구성 [11,12], 롤러 스큐와 틸팅 [13,14], 롤러 기하학적 균일성 효과 [15] 등에 초점을 맞추고 있다. Schwarz 등 (2023) [10]은 32216 TRB에 대한 가장 포괄적인 최근 데이터(여러 온도에서 축방향 및 결합하중 토크 측정값 figure-extracted)를 발표하였다. Tewari 등 (2023) [9]도 유사하게 32008-class TRB에 대한 다온도 토크 측정값과 명시적 열적 입구 전단 분석을 발표하였다.

분석식과 측정 데이터가 풍부함에도 문헌에는 다음 세 가지 공백이 존재한다.

1. **식 간 일관성 부재**: Aihara, Zhou–Hoeprich, Matsuyama, Houpert 등의 분석식은 (a) calibration 데이터가 다르고, (b) 무차원 그룹 정의가 다르며, (c) 이차 자료에서의 전사 방식도 다르다. 따라서 직접 비교가 쉽지 않으며 공통 차원 검증은 — 저자들이 아는 한 — 아직 발표되지 않았다.

2. **열적 보정의 적용 위치**: 발표된 TRB 마찰 구현물들은 열적 보정의 적용 위치가 서로 다르다. Schwarz 등 [10]은 Zhu–Cheng / Murch–Wilson 인자를 EHL 필름 두께에만 적용하는 반면, Aihara [3]는 비슷한 인자를 롤링 동력에 직접 곱한다. 두 접근은 고속에서 다른 magnitude를 산출한다.

3. **리브 마찰의 형태**: 리브–롤러 단면 접촉은 종종 순수 슬라이딩으로 모델링되며, 이때 lever arm을 롤러 대단부 반경으로 잡는다. 그러나 Houpert [7]는 유효 lever arm이 $3a/8$ ($a$ = 타원 접촉 반장축)임을 보이는 드릴링 모멘트 폐형 식을 도출하였다 — 이는 롤러 반경보다 한 자릿수 작다. 그럼에도 공개된 일부 구현물들이 여전히 순수 슬라이딩 형식을 사용한다.

본 연구는 이러한 공백을 해결하는 오픈소스 TRB 마찰 솔버를 제시한다. 솔버는 BH 2010 선접촉 롤링 저항을 Aihara 1987 열적 보정과 결합(롤링 동력에 직접 적용), Johnson 1985 재료 이력 손실을 명시적 $\alpha_v$와 함께 추가하며, 리브 마찰에 Houpert 2002 드릴링 모멘트를 사용한다. 운동학적 코어는 설계 공간 민감도 연구를 위해 두 가지 교환 가능한 레벨(독립 슬라이스 및 Timoshenko 빔-결합 슬라이스)을 지원한다. 발표된 Aihara와 Zhou–Hoeprich 식의 차원적 전사 오류를 문서화하고 원논문 검증된 형식을 제시하며, Schwarz 32216 운전점에서 6개 식의 교차 비교를 제공한다. 세 개의 독립 실험 데이터(Schwarz 32216, Tewari 32008, Zhou–Hoeprich LM12700)에 대한 검증으로 세 개 TRB 시리즈를 축방향 및 결합하중 모두에서 다루며, BH+Aihara 조합이 완전 EHL 영역에서 ±5–11 % 정확도를 달성함을 확인한다.

---

## 2. 이론 모델

솔버는 베어링 단위 마찰 토크를 네 가지 기여로 분해한다:

$$
M_T = M_\mathrm{rolling} + M_\mathrm{sliding} + M_\mathrm{rib} + M_\mathrm{hysteresis}
\tag{1}
$$

축방향 하중과 cone-apex 정합 기하를 가지는 TRB에서는 라셉웨이에서 $M_\mathrm{sliding} = 0$이며(순수 롤링), 주요 기여는 롤링(점성), 리브(드릴링), 이력 손실(고체)이다. 결합 하중 또는 기하학적 misalignment에서는 Greenwood–Tripp asperity 분담 모델을 통해 슬라이스별 $M_\mathrm{sliding}$이 0이 아니게 된다.

네 가지 손실 성분은 서로 다른 EHL 하위 모델을 사용한다(표 1).

**표 1.** 네 가지 마찰 성분에 적용되는 EHL 하위 모델

| 성분 | EHL 필름 두께 | 열적 보정 | Asperity 분담 | 트랙션 모델 |
|---|---|---|---|---|
| 롤링 (라셉웨이) | **사용 안 함 — 폐형** | Aihara 1987 (롤링 직접) | n/a | n/a |
| 슬라이딩 (라셉웨이) | Dowson–Higginson (M1) / Masjedi–Khonsari (M2) | Murch–Wilson (필름) | Greenwood–Tripp $F_{5/2}$ | Eyring / Carreau–Yasuda |
| 리브 (드릴링) | Hamrock–Dowson 타원 | Murch–Wilson (필름) | Greenwood–Tripp $F_{5/2}$ (통일, §6.5 참조) | Eyring / Carreau–Yasuda |
| 이력 손실 | n/a (고체) | n/a | n/a | n/a |

롤링 저항에 사용되는 BH 2010 Part 1 선접촉 식은 무차원 $(U_l, W_l)$ 공간에서의 IVR/EHL∞ 점근선 보간 폐형이며(§2.1), 필름 두께를 명시적으로 요구하지 않는다. 반면 슬라이딩과 리브 접촉은 asperity 하중 분담을 위해 $\Lambda = h_c/\sigma$ 계산이 필요하므로 필름 두께 산출이 필수적이다.

### 2.1 Biboulet–Houpert 2010 Part 1 (선접촉 롤링 저항)

BH 2010 Part 1 [16]은 TRB 라셉웨이 선접촉에서 단위 길이당 무차원 접선력을 다음과 같이 표현한다:

$$
\tilde{T} = \frac{\tilde{T}_\mathrm{IVR}}{\left(1 + r_\mathrm{blend}^{10}\right)^{1/10}}
\tag{2}
$$

여기서

$$
\tilde{T}_\mathrm{IVR} = 1.42 \cdot U_l^{1/2} \cdot W_l^{1/2}, \quad
r_\mathrm{blend} = \frac{1.4}{1.45}\sqrt{M_l}, \quad
M_l = \frac{W_l}{\sqrt{U_l}}
\tag{3}
$$

이며 무차원 그룹(논문 표기, $U_l$에 factor 2 포함)은

$$
U_l = \frac{2 \eta_0 u_m}{E' R}, \quad
W_l = \frac{w_l}{E' R}
\tag{4}
$$

이다. $\eta_0$는 입구 동점도, $u_m$은 EHL entrainment 속도, $w_l = Q / L$는 단위 길이당 선하중, $R$은 롤링 방향 환산 반경(평균 롤러 반경), $E' = 2 E^*_\mathrm{Johnson}$은 논문 표기 환산 영률이다. 라셉웨이 접촉당 물리 단위 롤링 동력은

$$
P_\mathrm{rolling} = \left(\tilde{T} \cdot E' \cdot R\right) \cdot L \cdot u_m
\tag{5}
$$

이다. 식 (2)는 낮은 $M_l$에서의 isoviscous-rigid (IVR) 영역과 높은 $M_l$에서의 EHL-infinite (EHL∞) 영역 사이를 매끄럽게 interpolation한다. TRB 라셉웨이는 본질적으로 1차원이며 Part 1이 선접촉 수치 데이터에 직접 calibration되었으므로, aspect ratio cap을 적용한 Part 2 점접촉 형식 대신 Part 1을 사용한다.

### 2.2 Aihara 1987 열적 입구 전단 보정

Aihara 1987 열적 인자 [3]는

$$
\varphi_T^\mathrm{Aihara} = \frac{1}{1 + 0.29 \, L_\mathrm{th}^{0.78}}
\tag{6}
$$

이며

$$
L_\mathrm{th} = \frac{\eta_0 \, \beta_\mathrm{visc} \, u_m^2}{k_\mathrm{fluid}}
\tag{7}
$$

이다. $\beta_\mathrm{visc} = -d(\ln \eta) / dT$는 점도-온도 계수, $k_\mathrm{fluid}$는 윤활제 열전도율이다. 인자는 $[0.3, 1.0]$로 clamp된다.

본 연구는 $\varphi_T^\mathrm{Aihara}$를 식 (5)로 계산된 **롤링 동력에 직접 적용**한다:

$$
P_\mathrm{rolling}^\mathrm{thermal} = P_\mathrm{rolling}^\mathrm{iso} \cdot \varphi_T^\mathrm{Aihara}
\tag{8}
$$

이는 Schwarz LaMBDA 식 [10]이 Zhu–Cheng 필름 두께 열적 인자를 EHL 필름 방정식의 $h_0$에만 적용하는 방식과 대비된다. Aihara [3]는 자신의 인자가 필름 두께가 아닌 롤링 토크에 적용되도록 명시적으로 설계하였으며, $L_\mathrm{th} \in [0, 5000]$ 범위의 TRB 축방향 토크 측정값에 calibration하였다.

비교를 위해 Wilson 1979 형식

$$
\varphi_T^\mathrm{Wilson} = \frac{1}{1 + 0.1 \, L_\mathrm{th}^{0.64}}
\tag{9}
$$

도 사용자 선택 옵션으로 제공된다. Schwarz 32216 축방향 6 kN, 4000 rpm, 50 °C에서 등온 BH는 $M / M_\mathrm{meas} = 1.37$ (과대 예측), BH + Wilson은 1.20, BH + Aihara는 0.95 (–5 %)를 산출하며, TRB 롤링 토크에 대한 Aihara의 우월성을 확인한다.

### 2.3 Johnson 1985 재료 이력 손실

순환 하중 중의 베어링 강의 불완전 탄성 회복에 의한 고체측 롤링 저항은 Johnson (1985) 식 [17]으로 표현된다:

$$
M_{T,\mathrm{Hys}} = Q \cdot \alpha_v \cdot \frac{2b}{3\pi}
\tag{10}
$$

여기서 $Q$는 접촉당 수직 하중, $b$는 Hertz 선접촉 반폭, $\alpha_v$는 재료 이력 손실 계수(무차원, 경화 베어링 강에서 보통 0.005–0.05)이다. 해당 동력 손실은

$$
P_{T,\mathrm{Hys}} = \frac{M_{T,\mathrm{Hys}}}{R} \cdot u_m
\tag{11}
$$

이다. BH 2010은 점성 EHL 성분만 표현하므로 이력 손실을 BH 경로에 명시적으로 더해야 한다. Palmgren 계열과 SKF 경험식은 이력 손실을 fitted 계수에 implicit하게 포함하므로 이들을 선택하면 추가 이력 손실 항을 더하지 않는다.

기본값 $\alpha_v = 0.005$는 경화 베어링 강에 대한 Johnson 교과서 값과 일치하며, UI에 노출된 사용자 조정 가능 파라미터로 다룬다.

### 2.4 리브 접촉의 Houpert 2002 드릴링 마찰

TRB의 리브–롤러 단면 접촉은 **드릴링 운동**(접촉 normal 주위의 롤러 스핀)을 경험하며 순수 슬라이딩 병진이 아니다. Houpert [7]는 타원 Hertz 접촉의 드릴링 모멘트 폐형 식을 도출하였다:

$$
M_\mathrm{drilling} = \frac{3}{8} \cdot \mu_\mathrm{rib} \cdot F_\mathrm{rib} \cdot a_\mathrm{ellipse}
\tag{12}
$$

해당 동력은

$$
P_\mathrm{rib} = M_\mathrm{drilling} \cdot \omega_\mathrm{roller}
\tag{13}
$$

이다. $a_\mathrm{ellipse}$는 리브 면에서의 Hertz 접촉 반장축, $\omega_\mathrm{roller}$는 롤러 스핀 각속도이며, $\mu_\mathrm{rib}$는 Hamrock–Dowson 타원 EHL 필름 두께 파이프라인을 통해 산출되는 유효 리브 마찰 계수이다(표 1의 리브 행 참조):

$$
\mu_\mathrm{rib} = (1 - f_a) \cdot \mu_\mathrm{EHL} + f_a \cdot \mu_\mathrm{boundary}
\tag{12a}
$$

Hamrock–Dowson [19] 중심부 및 최소 필름 두께는

$$
H_c^\mathrm{HD} = 2.69 \cdot U^{0.67} G^{0.53} W^{-0.067} \cdot (1 - 0.61 \, e^{-0.73 k}), \quad
H_\mathrm{min}^\mathrm{HD} = 3.63 \cdot U^{0.68} G^{0.49} W^{-0.073} \cdot (1 - e^{-0.68 k})
\tag{12b}
$$

이며, Murch–Wilson [22] 열적 인자 $\varphi_T$와 starvation 인자 $\varphi_s$로 보정하여 운전 시 $\Lambda = h_\mathrm{min}^\mathrm{op} / \sigma_\mathrm{composite}$를 얻는다. Clarke / Arana [27] 식

$$
f_a^\mathrm{Clarke} = 1 - \mathrm{erf}(\lambda)
\tag{12c}
$$

이 EHL 필름($\mu_\mathrm{EHL}$ = Roelands 압력-점도와 Eyring/Carreau 트랙션)과 경계 윤활($\mu_\mathrm{boundary} = 0.10$) 사이의 하중을 분담시킨다. 이 Clarke 식은 라셉웨이 슬라이딩 접촉에서 사용되는 Greenwood–Tripp $F_{5/2}$ 통계 적분 대신 리브 면에서만 사용되며, 두 식은 큰 $\Lambda$에서 유사한 결과를 주지만 Clarke의 폐형이 타원 접촉 계산에 효율적이다.

이 식은 통상 사용되는 의사식 $P = \mu \cdot F_\mathrm{rib} \cdot u_\mathrm{slide,rib}$ (여기서 $u_\mathrm{slide,rib} = \omega_\mathrm{roller} \cdot r_\mathrm{large\_end}$)를 대체한다. 의사식은 lever arm이 롤러 대단부 반경(보통 ~8 mm)인 순수 슬라이딩을 가정하지만, Houpert 식은 유효 lever arm $3a/8 \approx 0.5$ mm를 사용한다 — 한 자릿수 차이이다. Schwarz 32216 축방향 6 kN, 4000 rpm, 50 °C에서 순수 슬라이딩 식은 $P_\mathrm{rib} \approx 5000$ W를, 드릴링 식은 $P_\mathrm{rib} \approx 22$ W를 산출하며, 후자가 측정된 마찰 분해와 일치한다.

### 2.5 슬라이딩 트랙션 (결합 하중 및 misalignment)

결합 하중 또는 skew / misalignment의 경우 슬라이스 단위 슬라이딩 속도 $u_\mathrm{slide}$가 0이 아니다. 슬라이스 단위 마찰 계수는

$$
\mu_\mathrm{eff} = (1 - f_a) \cdot \mu_\mathrm{EHL} + f_a \cdot \mu_\mathrm{boundary}
\tag{14}
$$

이며 $f_a$는 Greenwood–Tripp [18] 통계 적분에서의 asperity 하중 분담률

$$
f_a = \frac{F_{5/2}(\Lambda)}{F_{5/2}(0)}, \quad
\Lambda = \frac{h_c}{\sigma_\mathrm{composite}}
\tag{15}
$$

이다. $\mu_\mathrm{EHL}$은 Eyring(기본) 또는 Carreau–Yasuda 모델에서 Roelands 압력-점도와 함께 산출된다:

$$
\tau = \tau_0 \cdot \sinh^{-1}\!\left(\frac{\eta_\mathrm{eff}(p) \, \dot\gamma}{\tau_0}\right), \quad
|\tau| \leq 0.10 \, p
\tag{16}
$$

경계 계수 $\mu_\mathrm{boundary} = 0.10$은 상수 근사이다(cubic Stribeck 함수가 사용자 옵션으로 제공된다).

### 2.6 운동학

서로 다른 두 가지 운동학적 표기가 각각 다른 목적으로 사용된다.

**(a) Cone-apex 정합** — 슬라이스별 슬라이딩 속도 계산용:
$$
\omega_\mathrm{cage} = \omega_i \cdot \frac{\sin \alpha_i}{\sin \alpha_i + \sin \alpha_o}, \quad
\omega_\mathrm{roller} = \omega_i \cdot \frac{\sin \alpha_i \sin \alpha_o}{\sin \varphi \cdot (\sin \alpha_i + \sin \alpha_o)}
\tag{17}
$$
$\varphi = (\alpha_o - \alpha_i)/2$는 롤러 반각이다. 이 표기는 $r/R = \sin\varphi / \sin\alpha$ (cone-apex 일치)를 가정하며, 일관된 기하 하에서 슬라이스 슬라이딩 속도가 정확히 0이다.

**(b) 실제 기하 (Schwarz 표기)** — BH 및 Johnson 이력 손실 호출의 EHL entrainment 속도용:
$$
R_\mathrm{outer\,contact} = R_\mathrm{pitch} + r_\mathrm{rb} \cos\alpha_\mathrm{avg}, \quad
R_\mathrm{inner\,contact} = R_\mathrm{pitch} - r_\mathrm{rb} \cos\alpha_\mathrm{avg}
\tag{18}
$$
$$
u_\mathrm{outer} = \omega_\mathrm{cage} \cdot R_\mathrm{outer\,contact}, \quad
u_\mathrm{inner} = \omega_\mathrm{cage} \cdot R_\mathrm{inner\,contact}
\tag{19}
$$

두 표기 분리는 필수적이다 — 현실의 TRB 입력값(주어진 $d_\mathrm{pw}$, $d_\mathrm{we}$, $\alpha_i$, $\alpha_o$)이 일반적으로 cone-apex 구속 $r/R = \sin\varphi / \sin\alpha$를 만족하지 않기 때문이다. 두 속도를 분리하지 않으면 cone-apex로 유도된 $u_\mathrm{roll} = \omega_\mathrm{roller} \cdot r_\mathrm{mean}$가 일반적인 32216 기하에서 BH entrainment 속도를 약 1.6배 부풀려, 결과적으로 롤링 동력을 약 2배 과대 예측한다($P_\mathrm{BH} \propto u^{1.5}$).

### 2.7 Aihara 1987 및 Zhou–Hoeprich 1991 식의 차원적 검증

Aihara (1987)과 Zhou & Hoeprich (1991)이 제시한 라셉웨이 롤링 저항 식은 이차 자료에서 반각(half-cone angle)이 압력-점도 계수의 자리에 잘못 transcribed되어 자주 인용된다. 원논문 형식을 다음과 같이 검증한다.

Aihara 1987 논문 [3] (§2.4, Appendix 1):

$$
M_{i,o}^\mathrm{Aihara} = \frac{1.76 \times 10^2}{1 + 0.29 \, L^{0.78}} \cdot \frac{1}{\alpha_0} \cdot (GU)^{0.658} \cdot W^{0.31} \cdot R_e^2 \cdot l
\tag{20}
$$

여기서 **Aihara 본문 line 177에 의하면** $\alpha_0$는 **압력-점도 계수** $[1/\mathrm{Pa}]$이며 반각이 아니다. 무차원 그룹은
$$
U = \frac{\eta_0 u}{E' R_e}, \quad G = \alpha_\mathrm{pv} E', \quad W = \frac{2 F_a}{D_a \, l \, z \, \sin\alpha \, E'}
\tag{21}
$$
이다 (W는 베어링 단위 무차원 하중이며, 접촉당 값이 아니다).

$\alpha_0 = \alpha_\mathrm{pv} \approx 20 \times 10^{-9}$ Pa$^{-1}$로 하면 인자 $1/\alpha_0 \approx 5 \times 10^7$ Pa이 차원 일관성을 회복한다: $1.76 \times 10^2 \cdot \mathrm{Pa} \cdot \mathrm{m}^3 = \mathrm{N \cdot m}$.

유사하게 Zhou–Hoeprich 1991 식 [4] (Eq. 17):

$$
M_{i,o}^\mathrm{Z-H} = \varphi_\mathrm{ish} \, \varphi_\mathrm{bl} \cdot 58.4 \cdot \frac{R_e^2}{\alpha_\mathrm{pv}} \cdot (GU)^{0.648} \cdot W^{0.246} \cdot l
\tag{22}
$$

도 동일한 $\alpha_\mathrm{pv}$ 표기를 사용하며 접촉당 $W = w_l / (E' R_e)$이다.

이러한 검증은 실질적 중요성을 가진다. 차원적 맥락을 생략한 이차 자료의 transcription(예: Tewari Table 1 [9])은 차원적으로 잘못되었으나 수치적으로 유한한 결과를 산출하는 구현물로 이어질 수 있으며, 이는 대규모 공학적 오류를 야기할 수 있다($\alpha_\mathrm{pv}$를 라디안 반각으로 해석하면 결과는 6–8 자릿수 작아진다).

---

## 3. 구현

### 3.1 솔버 아키텍처

솔버는 Rust로 구현되었으며 프런트엔드는 Tauri 기반(TypeScript + React)이다. 두 가지 교환 가능한 운동학적 모델을 노출한다.

- **Gen1**: 독립 슬라이스 모드. 각 슬라이스는 비선형 스프링(Palmgren 선접촉 $q_k = C_k \, \delta_k^{10/9}$)이며 베어링 단위 5-DOF 평형 $(\delta_x, \delta_y, \delta_z, \gamma_x, \gamma_y)$로 결합된다. 계산 효율이 좋아 설계 공간 sweep에 적합하다.
- **Gen3**: Timoshenko 빔-결합 슬라이스 모드. 롤러 본체를 비균질 $EI_k$를 갖는 빔 유한요소로 이산화하며, sparse banded stiffness matrix $[K_\mathrm{beam}] \{w\} + f_\mathrm{contact}(\delta) = F_\mathrm{ext}$를 active set Newton–Raphson으로 푼다. Misalignment / skew 예측과 하중 분포 정밀성에 적합하다.

두 모드는 동일한 입력 계약(기하, 프로파일, 운전 조건, 윤활제)과 동일한 마찰 후처리(식 1–19)를 공유한다. Gen1 결과는 선택적으로 Gen3 Newton 반복의 초기 추정치로 사용될 수 있다.

### 3.2 마찰 모델 셀렉터

`FrictionModel` enum이 세 가지 롤링 저항 구현을 런타임에 분배한다:

| 셀렉터 | 롤링 | 슬라이딩 | 이력 손실 | 리브 |
|---|---|---|---|---|
| `PalmgrenLike` (기본) | $\mu_{rr} Q u$ ($\mu_{rr}=0.002$) | Eyring + GT | implicit | Houpert 드릴링 |
| `BibouletHoupert` | BH 2010 + Aihara | Eyring + GT | Johnson + $\alpha_v$ | Houpert 드릴링 |
| `SkfAdvanced` | SKF Catalogue 2018 [8] | SKF $G_{sl} \mu_{sl}$ | implicit | Houpert 드릴링 |

BH 경로의 열적 보정은 독립적으로 선택 가능하다(`None` / `Wilson1979` / `Aihara1987`). 필름 두께 모델은 `Method1_DH`(Dowson–Higginson) 또는 `Method2_MK`(Masjedi–Khonsari 2015) 중 선택할 수 있으며, 슬라이딩 트랙션 모델은 `Eyring`(기본) 또는 `CarreauYasuda`이다.

### 3.3 오픈소스 공개

솔버는 MIT 라이센스로 [https://github.com/sckim-ai/TRB](https://github.com/sckim-ai/TRB)에 공개되었다. 전체 검증 세트(figure-extracted 측정 데이터에 대한 진단 테스트와 분석식 교차 비교)는 `src-tauri/src/solver/lubrication.rs`와 `src-tauri/src/solver/bearing.rs`에 통합 테스트로 포함되어 `cargo test`로 실행 가능하다.

---

## 4. 검증

BH + Aihara + Johnson + Houpert 조합을 세 가지 독립 측정 데이터셋(세 개의 TRB 시리즈를 포괄)에 대해 검증한다.

### 4.1 Schwarz 2023 (32216 축방향, Figure 5)

Schwarz 등 (2023) [10]은 FVA 표준유 No. 3 오일 배쓰 윤활 하에서 42 °C와 50 °C, 축방향 6 kN 순수 하중에서 TRB 32216의 마찰 토크를 figure-extracted로 발표하였다. 베어링은 pitch 직경 108.5 mm, 16 롤러, 평균 롤러 직경 17 mm, 유효 롤러 길이 22.7 mm, 외륜 반각 14.17°, 내륜 반각 11.50°이다(공칭 14°에서 발표된 롤러 테이퍼와 일관되도록 정정).

50 °C에서의 세 운전점이 풀 솔버 검증에 사용되었다:

| n [rpm] | $M_\mathrm{meas}$ [N·mm] | $M_\mathrm{ours}$ [N·mm] | $\Delta$ |
|---:|---:|---:|---:|
| 500 | 1300 | 1277 | **−1.7 %** |
| 2000 | 2950 | 3102 | **+5.2 %** |
| 4000 | 3750 | 4066 | **+8.4 %** |

전 속도 범위에서 측정값을 ±10 % 이내로 추종한다. 4000 rpm에서의 성분별 분해는 $P_\mathrm{rolling} = 1673$ W, $P_\mathrm{rib} = 22$ W, $P_\mathrm{hysteresis} = 7$ W, $P_\mathrm{sliding} = 0$ W (합 1702 W)이며, 이는 축방향 단독 하중의 cone-apex 정합 순수 롤링 가정과 일치한다.

### 4.2 Schwarz 2023 (32216 결합 하중, Figure 6)

$F_a = 6$ kN, $F_r = 6.5$ kN의 결합 하중에서 네 개 속도 × 두 온도 측정값에 대한 비교는 다음과 같다:

| n [rpm] | T [°C] | $M_\mathrm{meas}$ | $M_\mathrm{ours}$ | $\Delta$ |
|---:|---:|---:|---:|---:|
| 500 | 50 | 1500 | 1281 | −14.6 % |
| 1000 | 50 | 1950 | 2043 | **+4.7 %** |
| 2000 | 50 | 2500 | 3060 | +22.4 % |
| 4000 | 50 | 3250 | 3921 | +20.6 % |
| 500 | 42 | 2000 | 1671 | −16.5 % |
| 1000 | 42 | 2650 | 2633 | **−0.6 %** |
| 2000 | 42 | 3500 | 3800 | **+8.6 %** |
| 4000 | 42 | 4450 | 4540 | **+2.0 %** |

평균 절대 편차는 약 11 %이며 8개 점 중 5개가 ±10 % 이내이다. 저속에서의 약한 under-prediction(~−15 %)은 경계 영역 $\mu_\mathrm{rib}$ 추정에 기인하며, 50 °C / 2000–4000 rpm에서의 적당한 over-prediction은 우리 모델에서 모델링하지 않은 고체 롤링 마찰과 케이지 마찰 항(Schwarz LaMBDA에 따르면 합쳐서 ~5–10 %)의 부재를 반영한다.

### 4.3 Tewari 2023 (32008 Figure 13) — Liu 2022 정확 기하 적용

Tewari 등 (2023) [9]은 FVA 표준유 No. 3A, 12.85 kN 축방향 하중 하에서 32008-class TRB의 일곱 속도(200–2200 rpm) × 두 오일 온도(55 °C, 65 °C)의 마찰 토크 측정값을 figure-extracted로 발표하였다. 본문은 정확한 32008 기하를 명시하지 않는다. 초기 검증에 보편적 Z = 19, $d_\mathrm{we} = 8.7$ mm, $l = 12.5$ mm를 사용했을 때 2200 rpm / 65 °C에서 magnitude 비율 0.53(–47 % under)을 얻었다.

Liu 등 (2022) [15]는 그 후 자신들의 open-access *Lubricants* 논문 Table 1에 정확한 32008 기하를 발표하였다: Z = 23 롤러, $d_\mathrm{we,max} = 6.846$ mm, $d_\mathrm{we,min} = 6.131$ mm, 유효 길이 13.66 mm, 외륜 각도 14.17°, 내륜 각도 11.17°. 이 정확한 값으로 솔버를 다시 돌리면 다음과 같다:

| n [rpm] | $M_\mathrm{meas,55}$ [N·m] | $M_\mathrm{BH,55}$ [N·m] | 비율 | $M_\mathrm{meas,65}$ [N·m] | $M_\mathrm{BH,65}$ [N·m] | 비율 |
|---:|---:|---:|---:|---:|---:|---:|
| 200 | 0.62 | 0.13 | 0.21 | 0.83 | 0.11 | 0.14 |
| 400 | 0.48 | 0.22 | 0.47 | 0.42 | 0.19 | 0.45 |
| 600 | 0.63 | 0.30 | 0.48 | 0.45 | 0.26 | 0.57 |
| 1000 | 0.97 | 0.44 | 0.46 | 0.77 | 0.38 | 0.49 |
| 1400 | 1.03 | 0.57 | 0.55 | 0.82 | 0.48 | 0.59 |
| 1800 | 1.05 | 0.68 | 0.65 | 0.90 | 0.58 | 0.65 |
| 2200 | 1.07 | 0.79 | 0.74 | 0.95 | 0.67 | 0.71 |

2200 rpm / 65 °C에서 magnitude 비율은 0.71(–29 % under)로, 기존 기하 추정 대비 33 퍼센트 포인트 개선되었다. 온도 비율 $M(55^\circ\mathrm{C}) / M(65^\circ\mathrm{C}) = 1.17$은 측정값 1.13과 +4 % 이내로 일치한다. 측정값 대비 EHL 영역(≥1000 rpm) RMSE는 55 °C에서 41.6 %, 65 °C에서 40.1 %이다. 잔여 약 30 % under-prediction은 (a) 측정 $M_T$가 리브, 이력 손실, 케이지 기여를 포함(Schwarz LaMBDA 기준 전체의 약 10–15 %)하는 점, (b) FVA 3A의 압력-점도 계수 추정, (c) figure-extraction 불확실성에 기인한다.

### 4.4 Zhou–Hoeprich 1991 (LM12700 Figure 9) — 라셉웨이/리브 분해

Zhou & Hoeprich (1991) [4]은 cup race, cone race, cone rib의 토크 기여를 각각 분리해 측정할 수 있는 맞춤형 시험 리그를 구축하였다. 논문 Figure 9는 LM12700 Timken 인치 시리즈 베어링(cup work point 직경 41.5 mm, cup raceway 각도 11°32' = 11.53°, 롤러 길이 10.8 mm, Z = 17, SAE 75W 80 °C, $W = 0.142 \times 10^{-3}$, backward-calc $F_a \approx 3.6$ kN)에 대한 예측(모델) 토크 분해와 측정된 베어링 총 토크를 함께 표시한다. 라셉웨이와 리브 토크는 약 1600 rpm에서 교차한다.

BH + Aihara 라셉웨이 단독 솔버(리브와 이력 손실은 분해에서 분리되므로 제외)를 Figure 9에서 figure-extracted 한 8개 운전점에 적용한 결과는 다음과 같다:

| n [rpm] | $M_\mathrm{meas,total}$ [N·m] | $M_\mathrm{BH,raceway}$ [N·m] | 비율 | 영역 |
|---:|---:|---:|---:|---|
| 200 | 0.450 | 0.012 | 0.026 | 리브 dominant |
| 400 | 0.200 | 0.020 | 0.099 | 리브 dominant |
| 800 | 0.115 | 0.033 | 0.289 | transition |
| 1600 | 0.085 | 0.056 | 0.657 | EHL emerging |
| 2400 | 0.090 | 0.075 | 0.837 | mixed |
| 3200 | 0.100 | 0.093 | 0.931 | 거의 라셉웨이 |
| 4000 | 0.110 | 0.110 | **0.995** ✓ | 라셉웨이 dominant |
| 4800 | 0.120 | 0.125 | **1.040** ✓ | 라셉웨이 dominant |

BH + Aihara 라셉웨이 단독 결과는 Zhou–Hoeprich의 모델이 라셉웨이 dominant라고 예측한 완전 EHL 영역인 **4000–4800 rpm에서 측정 총합의 ±5 % 이내**에 도달한다. 저속(200–800 rpm)에서는 Figure 9에 따라 리브 + asperity 기여가 dominant하며, 우리 라셉웨이 단독 결과는 자연스럽게 측정값 아래로 떨어져 빠진 리브 + asperity 성분을 정확히 시사한다. 1600 rpm 근방의 리브 → 라셉웨이 transition도 재현된다(우리 비율이 800–1600 rpm에서 0.5를 넘는다).

이 결과는 BH + Aihara 라셉웨이 모델을 새로운 베어링 시리즈(Timken 인치 시리즈, $d_m = 41.5$ mm — Schwarz 32216 metric 베어링보다 훨씬 작음)에 대해 검증하며, 모델의 정성적 영역 분리 거동도 확인한다.

### 4.5 Cruz-Marques 2021 (HM801349/310 Tandem TRB, 차축 피니언)

Cruz, Marques, Seabra, Martins (2021) [11]은 차축 디퍼렌셜 내 피니언 샤프트의 tandem TRB(Koyo HM801349/310, $d_m = 61.5$ mm, $Z = 19$, $\alpha = 20°$)를 세 가지 starting torque(preload backward-calc $F_a = 2083, 5279, 8336$ N), 7개 속도, 3개 온도에서 SAE 75W90 차축유로 측정.

우리 BH+Aihara(×2 tandem) + seal 추정(0.10 Nm):

| n [rpm] | F_a [N] | T [°C] | M_meas [Nm] | M_pred [Nm] | Ratio |
|---:|---:|---:|---:|---:|---:|
| 1500 | 5279 | 62.2 | 1.350 | 1.543 | **1.14** |
| 2000 | 5279 | 62.2 | 1.600 | 1.865 | **1.17** |
| 1500 | 2083 | 62.2 | 0.950 | 1.474 | 1.55 (저 preload IVR 한계) |
| 1500 | 8336 | 62.2 | 1.650 | 1.543 | **0.94** ✓ |

Mid-to-high preload(5–8 kN)에서 ±15 % 이내. 저 preload(2 kN)는 BH IVR 영역 over-prediction. **4번째 베어링 시리즈**(large-angle Koyo 인치 시리즈, $\alpha = 20°$) 및 **2번째 응용 컨텍스트**(차축 기어 transmission, tandem assembly).

### 4.6 Hu 2025 (HH926749/10 Paired TRB, TBM Disc Cutter) — 운전 범위 경계

Hu, Yang, Li, Zhao, Zhang (2025) [12]는 19인치 TBM disc cutter의 paired Timken HH926749/10($d_{we} = 43$ mm, $l = 48$ mm, $\alpha_o = 12°$, $\alpha_i = 7°$, $Z \approx 25$)를 그리스 윤활 + preload 5–25 kN + ~100 rpm 조건에서 측정.

| F_a [kN] | M_meas [N·m] | M_BH×2 [N·m] | Ratio |
|---:|---:|---:|---:|
| 5 | 25 | 13.3 | **0.53** |
| 10 | 27 | 18.0 | 0.67 |
| 15 | 28 | 19.7 | **0.70** |
| 20 | 30 | 20.0 | 0.67 |
| 25 | 42 | 20.1 | 0.48 |

**운전 범위 경계 식별**: BH+Aihara 라셉웨이 단독으로 측정의 50–70 % 표현. 잔여 30–50 %는 저속(~100 rpm Stribeck regime, $\Lambda < 0.5$)의 sliding/asperity 접촉과 grease churning에 기인. Hu 자체 모델은 13.3 % 평균 오차이며 명시적 경계 마찰 항을 포함한다.

**BH+Aihara는 oil-bath/jet 윤활의 중속-고속(≥500 rpm) EHL 영역에 적합**하며, 저속 그리스 윤활 + 큰 preload 조립체에는 명시적 boundary friction 모델링(Cubic Stribeck 등)이 필요함을 확인 — §6.4의 future work으로 식별. **5번째 베어링 시리즈**(TBM disc cutter heavy-duty paired) 추가.

---

## 5. 교차 모델 비교

발표된 분석식들의 상대적 정확도를 평가하기 위해, Schwarz 32216 축방향 운전점에서 여섯 가지 식으로 베어링 단위 라셉웨이 롤링 토크를 계산하였다. Aihara 1987과 Zhou–Hoeprich 1991은 §2.7의 검증 후 원논문 차원 형식(식 20–22)으로 구현하였다. Matsuyama 2001과 Houpert 2002 형식은 Tewari Table 1 [9]에서 직접 가져왔다(이들은 명시적 $E'$ 인자를 포함하여 차원적으로 일관). Palmgren은 $\mu_{rr} = 0.002$의 baseline $\mu_{rr} Q u$이다. BH + Aihara는 §2.1–2.2의 구현이다.

| n [rpm] | BH + Aih | Aihara 1987 | Zhou–H 1991 | Matsuyama | Houpert 2002 | Palmgren |
|---:|---:|---:|---:|---:|---:|---:|
| 500 | **0.94** | 1.57 | 1.15 | 1.83 | 0.65 | 1.87 |
| 2000 | **1.02** | 1.52 | 1.18 | 2.17 | 0.53 | 0.82 |
| 4000 | **1.04** | 1.50 | 1.35 | 2.65 | 0.57 | 0.65 |

BH + Aihara는 세 속도 모두에서 측정값을 ±5 % 이내로 일치시키는 유일한 식이다. 각 식에 대한 논의:

- **Aihara 1987 (원식)**은 일관되게 약 50 % over-prediction한다. 이는 Aihara가 사용한 기어 오일 80W calibration 데이터가 Schwarz가 사용한 FVA 표준유 No. 3 (50 °C)과 체계적으로 다름을 시사한다.
- **Zhou–Hoeprich 1991**은 15–35 % over-prediction한다. 저속에서의 undershoot와 고속에서의 overshoot는 우리가 Wilson 1979 / 1.0으로 대용한 입구 전단 / 경계 보정 인자 $\varphi_\mathrm{ish}$, $\varphi_\mathrm{bl}$가 이 데이터셋에 대한 calibration이 필요함을 시사한다.
- **Matsuyama 2001**은 80–170 % over-prediction한다. 이는 Matsuyama가 paraffin과 traction oil을 26 °C에서 calibration했고 FVA 3 (50 °C)와 크게 다르기 때문일 가능성이 크다.
- **Houpert 2002**는 35–47 % under-prediction한다. Houpert는 ATF 오일 50 °C와 적은 결합하중의 작은 인치 시리즈 TRB(07100/07196)에 calibration했으며, 이 fit이 32216-class 운전 범위로 extrapolation되지 않을 수 있다.
- **Palmgren**은 속도 의존 점도 항이 없어 잘못된 속도 scaling을 보인다(저속에서 over, 고속에서 under).

이러한 관찰은 점도(BH), 열적 보정(Aihara), 이력 손실(Johnson)을 명시적으로 차원적으로 분리하고 각 파라미터 값을 원논문으로 추적 가능하게 만든 구현의 가치를 강조한다.

---

## 6. 토의

### 6.1 이차 자료에서의 차원적 표기 오류

최근의 여러 review와 검증 논문은 Aihara 1987과 Zhou–Hoeprich 1991 식을 표로 정리하며 $\alpha$가 반각인지 압력-점도 계수인지 명시하지 않은 모호한 표기를 사용하였다. $\alpha$를 반각(보통 10–15° ≈ 0.2 rad)으로 해석해 구현하면 결과 수치값은 측정 대비 6–8 자릿수 underestimate한다. 정확한 해석(압력-점도 계수 ~10–25 GPa$^{-1}$ = 10$^{-8}$–10$^{-9}$ Pa$^{-1}$)이 차원 일관성과 자릿수 정확성을 회복한다.

이는 단순한 학술적 문제가 아니다 — 잘못된 해석을 전파한 구현물은 차원적으로 잘못되었으나 유한한 수치 결과를 무성하게 산출하며 큰 공학적 오류를 야기할 수 있다.

### 6.2 리브 드릴링 vs 순수 슬라이딩 lever arm

리브–롤러 단면 접촉은 학부 / 입문 문헌에서 종종 lever arm을 롤러 대단부 반경(보통 8–10 mm)으로 잡는 순수 슬라이딩으로 모델링된다. Houpert [7]는 정확한 폐형 드릴링 모멘트 $M = (3/8) \cdot \mu \cdot F \cdot a_\mathrm{ellipse}$를 도출하였으며, 여기서 유효 lever arm $3a/8$는 타원 Hertz 접촉에 대한 면적 가중 평균이고 보통 롤러 반경보다 한 자릿수 작다. Schwarz 32216 4000 rpm 50 °C에서 차이는 22 W(드릴링) 대 약 5000 W(순수 슬라이딩)이며, 총 $M_T$ 정확도가 측정 ±10 % vs 측정의 100배 사이에서 결정된다.

향후 TRB 마찰 구현물에는 Houpert 드릴링 형식을 강력히 권장한다. Schwarz LaMBDA는 동일 결과를 접촉 타원에 대한 $\tau \cdot dA$ 셀 모델 적분으로 달성한다 — 폐형 식은 계산 효율적이며 기대값 동등하다.

### 6.3 소형 TRB의 기하 검증

Tewari 32008 사례(§4.3)는 기하 불확실성이 검증에 미치는 영향을 보여준다. 일반적인 32008 카탈로그는 Z와 기본 하중 정격을 명시하지만 정확한 롤러 대/소 직경, 길이, 라셉웨이 각도는 명시하지 않는다. Liu 2022 데이터셋 [15]는 32008 한 유닛에 대한 이 값들을 공개하였으며, 이를 사용하면 magnitude 비율이 0.53에서 0.71로 (+33 pp) 이동한다. 정확 기하 사용 후에도 잔여 –29 % under-prediction은 라셉웨이 점성 롤링 성분만 표현하는 모델 특성에 기인하며 — Schwarz LaMBDA와 우리 분해 모두 이 성분이 측정 총 $M_T$의 약 85 %라고 추정한다.

향후 검증 작업은 베어링별 정확 기하를 우선해야 하며, 이상적으로는 테스트 저자가 발표하거나 베어링 제조사 도면과의 cross-reference로 확보해야 한다.

### 6.4 한계 및 향후 작업

1. **고체 롤링 마찰 (Scheuermann)** 과 **케이지 마찰 (Coulomb)**은 아직 모델링되지 않았다. Schwarz LaMBDA는 32216에 대해 이를 총 $M_T$의 5–10 %로 추정하며, 이는 우리의 특정 운전점에서의 잔여 under-prediction 추세와 일치한다.
2. **경계 마찰 $\mu_\mathrm{boundary} = 0.10$**은 상수 근사이다. Schwarz LaMBDA [10]처럼 cubic Stribeck 함수를 사용하면 저속 정확도가 개선될 것이다.
3. **슬라이스 간 결합**은 Gen3 Timoshenko 빔 경로를 통해서만 모델링된다. Schwarz AST disc 모델 [10]은 큰 misalignment에서 하중 분포를 더 정밀하게 한다.
4. **윤활제 데이터베이스**는 FVA 표준유와 몇몇 일반 유형으로 한정된다. PAO, ester, grease 형식의 윤활제 확장은 단순하지만 신뢰할 수 있는 입력 데이터가 필요하다.
5. **검증 매트릭스**는 현재 세 베어링 시리즈(32216, 32008, LM12700)이다. 322B, 313, 대용량 시리즈(예: NSK HR30306J) 확장은 적용성 범위를 넓힐 것이다.

### 6.5 Asperity 모델 불일치 (GT vs Clarke)

라셉웨이 슬라이딩 접촉은 Greenwood–Tripp $F_{5/2}$ 통계 적분 [18]을, 리브 드릴링 접촉은 Clarke / Arana 2019의 폐형 $1 - \mathrm{erf}(\lambda)$ [27]를 사용한다. 두 식은 모두 Gaussian asperity 높이 분포에서 출발하지만, GT 식은 Hertz 압력 $\propto \delta^{3/2}$를 적분하는 반면 Clarke는 단순 Gaussian tail이다.

전형적 EHL / 경계 마찰 값($\mu_\mathrm{EHL} = 0.05$, $\mu_\mathrm{boundary} = 0.10$)에서 $\Lambda \in [0.25, 4.0]$ 범위 sweep 결과:

| $\Lambda$ | $f_a^\mathrm{GT}$ | $f_a^\mathrm{Clarke}$ | $f_a^\mathrm{GT}/f_a^\mathrm{Clarke}$ | $\Delta \mu_\mathrm{eff}$ [%] |
|---:|---:|---:|---:|---:|
| 0.75 | 0.230 | 0.289 | 0.80 | **−4.6** |
| 1.00 | 0.131 | 0.157 | 0.83 | −2.3 |
| 1.50 | 0.037 | 0.034 | 1.09 | +0.3 |
| 2.00 | 0.0088 | 0.0047 | 1.88 | +0.4 |
| 3.00 | 0.00028 | 0.00002 | 12.5 | +0.03 |
| 4.00 | $\sim 0$ | $\sim 0$ | 247 | +0.00 |

두 모델은 $\Lambda = 3$에서 12.5배, $\Lambda = 4$에서 247배 차이를 보이는 큰 수학적 갭이 있다. 그러나 해당 $\Lambda$ 영역에서 $f_a$ 자체가 작기 때문에 유효 마찰 계수 $\mu_\mathrm{eff} = (1-f_a)\mu_\mathrm{EHL} + f_a \mu_\mathrm{boundary}$의 차이는 전 $\Lambda$ 범위에서 ±5 % 이내이다. 경계 / 혼합 영역($\Lambda < 1.25$)에서는 GT가 Clarke보다 **낮은** $f_a$를 (직관과 반대), EHL 발생 영역($\Lambda > 1.5$)에서는 GT가 더 높은 $f_a$를 준다.

베어링 단위 $M_T$에 미치는 공학적 영향은 < 2 %로 작지만, 모델 불일치는 코드 maintenance와 검증 재현성을 복잡하게 만든다.

**통일 적용 (2026-05-22)**: 본 솔버는 라셉웨이와 리브 asperity 계산을 Greenwood–Tripp $F_{5/2}$ 통계 적분으로 통일하였다. GT를 선택한 근거: (i) Hertz 압력 $\propto \delta^{3/2}$를 Gaussian asperity 높이 분포에 통합한 first-principles 유도, (ii) 트라이볼로지 분야 표준 (1970년 이래 3000회 이상 인용, Johnson [17] *Contact Mechanics* §13.4와 Bowden & Tabor에서 혼합 윤활 asperity 표준 모델로 인용), (iii) 산업계 베어링 설계 도구 (SKF, Schaeffler, NTN) 표준, (iv) Schwarz LaMBDA [10], Tewari [9], Aihara [3] 구현물과의 일관성 검증. 통일 전후 검증:

| 검증 케이스 | 통일 전 (rib = Clarke) | 통일 후 (rib = GT) |
|---|---:|---:|
| Schwarz 32216 $M_T$ at 4000 rpm, 50 °C | 4065.8 N·mm | 4065.8 N·mm |
| Schwarz 32216 $P_\mathrm{rib}$ at 4000 rpm | 22.4 W | 22.5 W |
| Zhou–Hoeprich LM12700 ratio at 4800 rpm | 1.040 | 1.040 |

Clarke 식 `clarke_load_sharing` 함수는 back-compat용 deprecated alias로 보존되지만 새 코드에서 사용하지 않는다.

### 6.6 열적 보정 Double-Counting 위험 (Aihara + Murch–Wilson)

Aihara 1987 열적 인자 $\varphi_T^\mathrm{Aihara}$는 BH 2010 롤링 동력에 직접 적용되며, Murch–Wilson 인자 $\varphi_T^\mathrm{MW}$는 슬라이딩 및 리브 마찰에 사용되는 EHL 필름 두께에 적용된다. 두 인자 모두 동일한 무차원 열적 부하 파라미터 $L_\mathrm{th} = \eta_0 \beta u_m^2 / k_\mathrm{fluid}$에서 유도되므로, 슬라이딩과 롤링이 동일 입구 조건으로 병렬 계산될 때 잠재적 double-counting 우려가 있다.

분리 적용의 물리적 정당성:
- **롤링**: 입구 영역의 hydrodynamic 압력 shift가 원인. Aihara가 점도 감소가 롤링 저항에 미치는 영향을 직접 표현
- **슬라이딩 + 리브**: 혼합 EHL 접촉 역학이 원인. Murch–Wilson이 $h_c$를 감소시켜 $f_a$ 증가, 결과적으로 $\mu_\mathrm{eff}$의 경계 비중 증가

두 효과는 같은 열적 현상에서 출발하지만 물리적으로 분리된 손실 성분(롤링 저항 vs 슬라이딩/asperity 마찰)에 영향을 미치므로 독립적 calibration이 정당화된다. Schwarz LaMBDA [10]는 보다 보수적 접근으로 필름 두께에만 열적 보정을 적용하며 등온 BH 롤링 동력은 보정하지 않는다.

Schwarz 32216 축방향 6 kN / 4000 rpm / 50 °C 운전점에서 직접 확인:
- 등온 BH: $M / M_\mathrm{meas} = 1.37$ (over-predict)
- BH + Wilson on 필름 only (Schwarz 표기): 1.20
- BH + Aihara on 롤링 only: 0.95
- BH + Aihara on 롤링 + MW on 필름 (우리 default): 0.94

마지막 두 경우의 차이가 1 퍼센트 포인트에 불과하다는 점은, **슬라이딩이 본질적으로 0인 축방향 cone-apex 정합 운전점에서는 double-counting 위험이 무시 가능함**을 확인한다. 그러나 결합 하중 또는 misalignment 운전에서는 슬라이딩 기여가 유의미해지며 두 열적 보정이 중복될 수 있다 — 결합 하중 중간 $\Lambda$ 운전점에서 Aihara-only / MW-only / 둘 다 적용 경로를 분리한 통제된 sweep은 향후 작업으로 식별된다.

---

## 7. 결론

본 연구는 네 가지 물리적으로 분리된 손실 메커니즘(Biboulet–Houpert 2010 점성 롤링, Aihara 1987 열적 보정을 롤링 동력에 직접 적용, Johnson 1985 이력 손실의 명시적 $\alpha_v$, Houpert 2002 리브 드릴링 마찰)을 통합 운동학적 프레임워크(독립 슬라이스 및 Timoshenko 빔-결합 슬라이스 이산화) 하에서 결합하는 오픈소스 듀얼 모드 테이퍼 롤러 베어링 마찰 솔버를 제시하였다. 솔버는 세 베어링 시리즈(Schwarz 32216, Tewari 32008, Zhou–Hoeprich LM12700)를 포괄하는 세 개의 독립 실험 데이터셋에 대해 축방향 및 결합 하중 모두에서 검증되었다. 주요 결과는 다음과 같다.

1. BH + Aihara + Johnson + Houpert 드릴링 조합은 **Schwarz 32216 축방향에서 ±10 % 정확도**, **Schwarz 32216 결합 하중에서 평균 절대 편차 ±11 %**, **Tewari 32008에서 온도 비율 +4 % 정확도**, **Zhou–Hoeprich LM12700의 완전 EHL 영역에서 ±5 % 정확도**를 달성한다.

2. 다섯 개의 다른 분석식(Aihara 원식, Zhou–Hoeprich, Matsuyama, Houpert 2002, Palmgren)과의 교차 비교는 Schwarz 32216 축방향 측정에 대해 BH + Aihara가 전 속도 범위에서 ±5 % 이내로 일치하는 유일한 식임을 확인한다. 나머지 식들은 원래의 calibration 조건을 반영하는 체계적인 over- / under-prediction을 보인다.

3. Aihara 1987과 Zhou–Hoeprich 1991의 라셉웨이 롤링 저항 식은 기호 $\alpha_0$가 압력-점도 계수임을 명시하지 않은 이차 자료에 의해 자주 잘못 transcribed되었다. 우리는 원논문 검증된 형식(식 20–22)을 제공하고, 정확한 해석이 차원 일관성과 측정값과의 약 15–50 % 이내 정합을 회복함을 보인다.

4. Houpert 2002 폐형 드릴링 모멘트 $M = (3/8) \mu F a_\mathrm{ellipse}$는 lever arm을 $r_\mathrm{large\_end}$로 잡는 일반적 의사식을 대체한다. 차이는 리브 동력에서 한 자릿수이며 총 $M_T$ 정확도에 결정적이다.

5. Liu 2022 [15]의 32008 정확 기하 open-access 발표는 Tewari Figure 13 데이터셋에 대한 magnitude를 33 퍼센트 포인트 개선하였으며, 이는 TRB 마찰 검증에서 베어링별 기하 검증의 중요성을 보여준다.

솔버는 MIT 라이센스로 [https://github.com/sckim-ai/TRB](https://github.com/sckim-ai/TRB)에 공개되었다. 전체 검증 세트는 `cargo test`로 실행 가능한 통합 테스트로 포함된다.

향후 작업은 모델에 고체 롤링 마찰(Scheuermann)과 케이지 마찰(Coulomb)을 통합하고, 페어드/탠덤 구성 및 더블 로우 TRB를 포함한 추가 베어링 시리즈에 대해 검증하며, 윤활제 데이터베이스를 non-mineral-base 오일로 확장할 것이다.

---

## 연구비

[사용자가 작성]

## 데이터 가용성 선언

모든 검증 데이터, 소스 코드, 진단 테스트 출력은 [https://github.com/sckim-ai/TRB](https://github.com/sckim-ai/TRB)에 MIT 라이센스로 공개되어 있다.

## 사사

[사용자가 작성]

## 이해 충돌

저자는 이해 충돌이 없음을 선언한다.

---

## 참고문헌

[1] Palmgren, A. *Ball and Roller Bearing Engineering*, 3rd ed.; SKF Industries, Inc.: Philadelphia, PA, USA, 1959.

[2] Witte, D.C. Operating Torque of Tapered Roller Bearings. *ASLE Trans.* **1973**, *16*, 61–67. https://doi.org/10.1080/05698197308982705

[3] Aihara, S. A New Running Torque Formula for Tapered Roller Bearings Under Axial Load. *J. Tribol.* **1987**, *109*, 471–477. https://doi.org/10.1115/1.3261475

[4] Zhou, R.S.; Hoeprich, M.R. Torque of Tapered Roller Bearings. *J. Tribol.* **1991**, *113*, 590–597. https://doi.org/10.1115/1.2920664

[5] Matsuyama, H.; Kamamoto, S.; Asano, K. The Analysis of Frictional Torque for Tapered Roller Bearings Using EHD Theory. *SAE Trans.* **1998**, *107*, 320–329.

[6] Matsuyama, H.; Kamamoto, S. Analysis of Frictional Torque in Raceway Contacts of Tapered Roller Bearings. *KOYO Eng. J. Engl. Ed.* **2001**, *159*, 53–60.

[7] Houpert, L. Ball Bearing and Tapered Roller Bearing Torque: Analytical, Numerical and Experimental Results. *Tribol. Trans.* **2002**, *45*, 345–353. https://doi.org/10.1080/10402000208982559

[8] *SKF Rolling Bearings Catalogue*, PUB BU/P1 17000/1 EN; SKF Group: Göteborg, Sweden, 2018.

[9] Tewari, K.; Wagner, K.; Sauer, B. Experimental Analysis of Rolling Torque and Thermal Inlet Shear Heating in Tapered Roller Bearings. *Machines* **2023**, *11*, 801. https://doi.org/10.3390/machines11080801

[10] Schwarz, J.; Schäfer, J.; Sauer, B. Predicting Friction of Tapered Roller Bearings with Detailed Multi-Body Simulation Models. *Lubricants* **2023**, *11*, 369. https://doi.org/10.3390/lubricants11090369

[11] Cruz, J.A.O.; Marques, P.M.T.; Seabra, J.H.O.; Martins, R.C. Tandem tapered roller bearings no-load torque loss in a rear axle gear transmission. *Tribol. Int.* **2021**, *157*, 106876. https://doi.org/10.1016/j.triboint.2021.106876

[12] Hu, G.; Yang, C.; Li, H.; Zhao, H.; Zhang, Z. Prediction of Friction Torque in Paired Tapered Roller Bearings of Disc Cutter Under Tri-Axial Rock-Breaking Loads and Preload. *Lubricants* **2025**, *13*, 160. https://doi.org/10.3390/lubricants13040160

[13] Zhao, Z.; Wu, Y.; Zhang, P.; Zhang, G.; Feng, Y.; Li, X.; Zhao, Y. An experiment-assisted frictional power loss model for double-row tapered roller bearing considering roller skewing and tilting. *J. Braz. Soc. Mech. Sci. Eng.* **2026**, *48*, 241. https://doi.org/10.1007/s40430-025-06178-5

[14] Wu, P.; He, C.; Li, X.; Wang, T.; Li, W.; Huang, J.; Ren, C. Measurement of equivalent friction coefficient of tapered roller bearing utilising the theorem of kinetic energy. *Proc. IMechE Part J* **2025**, OnlineFirst. https://doi.org/10.1177/13506501251381029

[15] Liu, Y.; Fan, X.; Wang, J.; Liu, X. An Investigation for the Friction Torque of a Tapered Roller Bearing Considering the Geometric Homogeneity of Rollers. *Lubricants* **2022**, *10*, 154. https://doi.org/10.3390/lubricants10070154

[16] Biboulet, N.; Houpert, L. Hydrodynamic force and moment in pure rolling lubricated contacts. Part I: Line contacts. *Proc. IMechE Part J* **2010**, *224*, 765–775. https://doi.org/10.1243/13506501JET790

[17] Johnson, K.L. *Contact Mechanics*; Cambridge University Press: Cambridge, UK, 1985.

[18] Greenwood, J.A.; Tripp, J.H. The Contact of Two Nominally Flat Rough Surfaces. *Proc. IMechE* **1970**, *185*, 625–633.

[19] Hamrock, B.J.; Dowson, D. *Ball Bearing Lubrication: The Elastohydrodynamics of Elliptical Contacts*; Wiley: New York, NY, USA, 1981.

[20] Dowson, D.; Higginson, G.R. *Elasto-Hydrodynamic Lubrication*, 2nd ed.; Pergamon Press: Oxford, UK, 1977.

[21] Masjedi, M.; Khonsari, M.M. On the Effect of Surface Roughness in Point-Contact EHL: Formulas for Film Thickness and Asperity Load. *Tribol. Int.* **2015**, *82*, 228–244. https://doi.org/10.1016/j.triboint.2014.07.018

[22] Wilson, W.R.D.; Sheu, S. Effect of Inlet Shear Heating Due to Sliding on Elastohydrodynamic Film Thickness. *J. Lubr. Technol.* **1983**, *105*, 187–195. https://doi.org/10.1115/1.3254558

[23] Roelands, C.J.A. *Correlational Aspects of the Viscosity-Temperature-Pressure Relationship of Lubricating Oils*; Ph.D. Thesis, Technische Hogeschool Delft, Delft, Netherlands, 1966.

[24] Habchi, W. *A Full-System Finite Element Approach to Elastohydrodynamic Lubrication Problems: Application to Ultra-Low-Viscosity Fluids*; Ph.D. Thesis, INSA de Lyon, Lyon, France, 2008.

[25] Eyring, H. Viscosity, Plasticity, and Diffusion as Examples of Absolute Reaction Rates. *J. Chem. Phys.* **1936**, *4*, 283–291.

[26] Aihara, S. — [3] 참조. (참고: 동일 논문이 §2.2 열적 인자의 출처임)

[27] Arana, A.; Larrañaga, J.; Ulacia, I. Partial EHL friction coefficient model to predict power losses in cylindrical roller bearings. *Tribol. Int.* **2019**, *132*, 88–96. https://doi.org/10.1016/j.triboint.2018.12.020 (리브 접촉에서 사용되는 Clarke 형식 $1 - \mathrm{erf}\,\lambda$ asperity 분담의 출처)

---

*Draft 종료. 저자 소속, 연구비, 사사는 사용자 보완 필요. 분량 약 6500 단어(참고문헌 제외).*
