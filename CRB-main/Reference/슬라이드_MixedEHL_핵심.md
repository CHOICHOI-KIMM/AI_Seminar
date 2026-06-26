# [발표 슬라이드용] Mixed EHL 출력의 공학적 활용 — 핵심 요약

> 원본: `분석_MixedEHL_공학적활용.md`(11장 전문) 압축본. 발표용 2장 구성.

---

## ▣ Slide 1 — 핵심 논제: "관심사의 분리"

### 문제
조도 보정이 내장된 유막($h\times r_c(\sigma)$)으로 다시 $\Lambda=h/\sigma\to\kappa\to a_{\text{ISO}}$ 를 계산하면 **조도 이중 반영** → 수명 과소예측

### 원칙
| 도구 | 담당 영역 | 비고 |
|---|---|---|
| **ISO 281 $a_{\text{ISO}}$** | 아표면 개시 피로 (수명) | 조도 통계 **이미 내장**($\kappa\approx\lambda$ 대리) |
| **Mixed EHL 미시출력** | 표면개시 손상·마찰 (별개 물리) | $a_{\text{ISO}}$ 재보정 **금지** |

> **Mixed EHL 출력은 "수명을 다시 깎는 보정"이 아니라, $a_{\text{ISO}}$가 못 잡는 손상모드의 정량화 도구**

### 출력 → 활용 매핑
| Mixed EHL 출력 | → 공학적 활용 | 모델 |
|---|---|---|
| 하중분담비 $\zeta=W_a/W_t$ | 마찰·동력손실·발열 | $\mu=\mu_b\zeta+\mu_h(1-\zeta)$ |
| 돌기압 이력 $p_a$ | **마이크로피팅** | Dang Van + 마일드 Archard |
| $\mu\cdot p_a\cdot v_s$ | **스커핑/스미어링** | Blok flash 온도 / 트랙션 한계 |
| $p_a,\ v_s$ | **마모·런인** | 국부 Archard $\Delta h=k\,p_a\Delta s/H$ |
| 조합압 $(p_h+p_a)$ | **표면개시 수명** | SKF GBLM 표면항 |

*근거: Morales‑Espejel & Gabelli(2015), Blok(1963), Archard(1953), Ioannides–Harris(1985)*

---

## ▣ Slide 2 — 목적별 도구 선택 + 권고

### 의사결정 플로우
```
[목적]
 ├─ 표준 수명(인증·카탈로그) ─► ISO 281 a_ISO  ※Mixed EHL 미시출력 사용 금지
 ├─ 정밀 수명 ──────────────► SKF GBLM
 │     ├ 아표면항: 매끈 Hertz → Ioannides–Harris/L–P
 │     └ 표면항 : Mixed EHL 돌기응력 → Dang Van   ※κ 디레이팅 중복 금지
 ├─ 마찰·동력손실·발열 ──────► ζ → μ 가중합 → 토크 집계
 ├─ 마이크로피팅 위험 ───────► 돌기압 → Dang Van + 마일드 Archard
 ├─ 스커핑/스미어링 위험 ────► μ·p_a·v_s → Blok flash / 트랙션>τ_lim
 ├─ 마모·런인 ──────────────► p_a·Δs → 국부 Archard → 조도진화 → λ 재평가
 └─ 형상설계(크라우닝·에지로딩)► Mixed EHL 국부 p_a·λ 분포 → 프로파일 검증
```

### 비중복 핵심 규칙
> **하나의 물리효과, 하나의 모델항** — 조도/$\lambda$ 는 `a_ISO`(고전) **또는** GBLM 표면항(명시) 중 **한 곳에서만**

### 20MW+ 풍력 메인베어링 권고
- 저속·고하중·그리스 → **혼합윤활 비중 큼**(Hart 2022: 운전점 ~90% 혼합)
- 카탈로그 $a_{\text{ISO}}$ 수명 + **GBLM 표면항 + 마이크로피팅·스미어링 스크리닝** 병행
- Mixed EHL = **손상모드·형상설계 검증 도구**로 포지셔닝 (수명 재보정 아님)
- 정지·미소진동(false brinelling·WEC)은 EHL 밖 → 별도 기준

---

*상세·전체 출처: `분석_MixedEHL_공학적활용.md` / 가정·한계: `분석_EHL 윤활해석.md` 부록1·2*
