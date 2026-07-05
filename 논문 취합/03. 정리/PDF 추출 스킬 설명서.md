# 논문 PDF 추출·정리 스킬 설명서

로컬 **MinerU CLI**를 기반으로 PDF 논문을 **Markdown + 이미지 통합본**으로 자동 변환하는 스킬 모음의 사용 설명서.
(작성·검증: 2026-07-04 / 환경: Windows, i5-10600K 6C, RAM 17GB, Python 3.13)

---

## 1. 개요

| 스킬 | 역할 | 호출어 |
|------|------|--------|
| **paper-pdf-pipeline** | PDF 1건 → 추출→감지→병합→검증 **원클릭** | "논문 PDF 자동추출" |
| **paper-md-merge** | MinerU MD + 분리 이미지 → **통합 MD** | "논문 md 통합" |
| **paper-summary** | 논문 → **전체 한국어** 9섹션 요약 | "논문 정리" |
| **paper-original-summary** | 본문은 **원문 영어 verbatim** + 앞부분 한국어 | "논문 원본 요약" |

**파이프라인 구조**:
```
[PDF] → ①MinerU 추출(md+이미지) → ②출력폴더 감지 → ③병합(정크제거·경로재작성) → ④검증 → [통합 MD]
        └─────────── paper-pdf-pipeline (오케스트레이터) ───────────┘
                              └ 내부에서 paper-md-merge 재사용 ┘
```

관련 스크립트:
- `~/.claude/skills/paper-pdf-pipeline/scripts/run_pdf_pipeline.py` — 단일 오케스트레이터
- `~/.claude/skills/paper-pdf-pipeline/scripts/batch_pdf_pipeline.py` — 다중 PDF 배치
- `~/.claude/skills/paper-md-merge/scripts/merge_mineru_md.py` — 병합 엔진

---

## 2. 사전 설치 (⚠️ 3가지 모두 필요)

이 PC 환경에서 실제로 필요했던 설치·설정:

| # | 항목 | 명령/설정 | 이유 |
|---|------|----------|------|
| 1 | MinerU 엔진 | `pip install "mineru[core]"` (v3.4.2) | 로컬 CLI 추출 엔진 (데스크톱 앱과 별개) |
| 2 | **SSL 우회** | `pip install pip-system-certs` | 회사망 TLS 검사(자체서명 CA)로 모델 다운로드 실패 방지 → Windows 신뢰저장소 사용 |
| 3 | 모델 소스 | 환경변수 `MINERU_MODEL_SOURCE=huggingface` | 최초 1회 pipeline 모델(~1GB) 다운로드 (스크립트가 기본 지정) |

> **CLI 호출 특이점**: 콘솔 exe가 PATH에 없어 모듈 진입점으로 호출:
> `python -c "from mineru.cli.client import main; main()" ...`

**백엔드/언어**:
- 백엔드: **`-b pipeline`** (이 PC는 torch가 CPU 전용 빌드 → GPU 미사용, pipeline이 정답)
- 방식: `-m auto`(디지털PDF=txt 경로, 빠름) / `-m ocr`(스캔·손상PDF)
- 언어: `-l`에 `en` **없음**. 영어 논문은 `-m auto`로 OCR 불필요(기본 `ch` 모델이 라틴 처리), 한글 스캔본만 `-l korean`

---

## 3. 사용법

### 3-1. 단일 PDF (권장 시작점)

Claude에게 **"이 PDF 논문 PDF 자동추출"** 이라고 지시하거나 직접 실행:

```bash
python ~/.claude/skills/paper-pdf-pipeline/scripts/run_pdf_pipeline.py \
  --pdf "<PDF경로>"
```

- 최종 MD: 상위 경로의 `03. 정리` 폴더에 `<논문명>.md`로 저장(없으면 PDF 폴더)
- 이미지: 통합 MD 옆 `<슬러그>/`에 `img_0001.jpg` 순번명으로 복사
- 중간 산출물: PDF 옆 `.mineru_out/`에 캐시(재실행 시 `--skip-extract`로 병합만 가능)

### 3-2. 다중 PDF 배치

```bash
python ~/.claude/skills/paper-pdf-pipeline/scripts/batch_pdf_pipeline.py \
  --pdf-dir "<폴더>" --final-dir "논문 취합/03. 정리" \
  --extract-workers 1 --workers 6
```

배치 처리 순서: ① auto 배치추출(모델 1회 로드) → ② 손상본만 OCR 2차 배치 → ③ 병합 CPU 병렬

### 3-3. 병합만 (이미 추출된 MinerU 결과)

```bash
python ~/.claude/skills/paper-md-merge/scripts/merge_mineru_md.py \
  --mineru-dir "<MinerU출력폴더(auto/ocr)>" \
  --out-md "<출력.md>" --img-slug "<ASCII슬러그>"
```

---

## 4. 자동 처리 기능 (무개입)

파이프라인이 자동으로 수행하는 품질 보정:

| 기능 | 동작 | 해제 옵션 |
|------|------|----------|
| **손상 텍스트레이어 감지→OCR** | 추출 전 pypdf 텍스트의 **비단어 비율** 측정. 폰트인코딩 손상 PDF는 `-m auto`가 깨지므로 `-m ocr` 자동전환 | `--no-auto-ocr` |
| **HTML표→Markdown 변환** | MinerU의 raw `<table>`을 MD 파이프표로 변환(셀 내 `$수식$` 렌더되도록) | `--keep-html-tables` |
| **정크 제거** | 표지 아이콘(page 0)·저널 보일러플레이트(To cite / Article views 등) 제거 | — |
| **이미지 순번 리네임** | 64자 해시 → `img_0001.jpg`(경로길이 단축) | `--keep-hash-names` |
| **파일명·경로 길이 캡** | MD명 80자, 슬러그 40자 → GitHub/Windows 260자 제한 대응 | `--max-name`/`--max-slug` |
| **병합 후 깨짐 경고** | 잔재 깨짐 문단 감지 시 `[warn]` 출력 | — |

---

## 5. 주요 옵션 레퍼런스

### run_pdf_pipeline.py
| 옵션 | 기본 | 설명 |
|------|------|------|
| `--pdf` | (필수) | 입력 PDF |
| `--final-dir` / `--final-md` | 03.정리 | 출력 폴더/경로 |
| `--backend` | pipeline | 추출 백엔드 |
| `--method` | auto | auto/txt/ocr |
| `--no-auto-ocr` | off | 손상 감지 OCR전환 비활성화 |
| `--skip-extract` | off | 추출 생략, 기존 출력만 병합 |
| `--max-name`/`--max-slug` | 80/40 | 파일명/슬러그 길이 캡 |

### batch_pdf_pipeline.py
| 옵션 | 기본 | 설명 |
|------|------|------|
| `--pdf-dir` / `--pdfs` | — | 폴더(재귀) 또는 개별 PDF들 |
| `--final-dir` | (필수) | 통합 MD 저장 폴더 |
| `--extract-workers` | **1** | 추출 동시 프로세스 수(§6 참조) |
| `--workers` | 4 | 병합 병렬 워커 |
| `--threads-per-worker` | 자동 | 추출 워커당 스레드 |

---

## 6. 성능·병렬화 (실측 기반)

**환경**: i5-10600K **6물리코어/12논리**, RAM 17GB(가용 8GB), **torch CPU 전용**(GTX 1650 미사용)

**추출 소요**:
- 정상 PDF(auto 경로): **~2–4분/편**
- 손상 PDF(전체 OCR): **~11–16분/편** (14p 기준, CPU OCR ≈ 60–70s/page)

**동시 추출 벤치마크** (OCR 3p×2편):

| 구성 | 벽시계 | 피크 RAM | 속도 |
|------|:---:|:---:|:---:|
| N=1 (6스레드 순차) | 439s | +5.8GB | 기준 |
| N=2 (3스레드 동시) | 409s | +7.2GB | **1.07×** |

> **결론: CPU 멀티워커 추출은 실익 없음** (단일 6스레드가 이미 6코어 포화 → 2×3스레드로 쪼개도 상쇄). N=2는 7% 이득에 RAM +1.4GB(8GB 한계 근접), N=3은 OOM. **`--extract-workers 1` 권장.**

**진짜 병렬화 이득** (배치 스크립트에 구현):
1. **배치모드(모델 1회 로드)** — N편을 한 프로세스로 처리, 편당 모델로드 중복 제거 (핵심)
2. **손상본만 OCR** — 정상본은 빠른 auto, 손상본만 느린 OCR
3. **병합 CPU 병렬** — merge는 초 단위, `--workers`로 동시

---

## 7. 문제 해결 (Troubleshooting)

| 증상 | 원인 | 해결 |
|------|------|------|
| `SSLCertVerificationError(self-signed cert)` | 회사망 TLS 검사 | `pip install pip-system-certs` |
| 모델 다운로드 느림/실패 | 소스 문제 | `MINERU_MODEL_SOURCE=huggingface` |
| `mineru: command not found` | 콘솔 exe 미등록 | `python -c "from mineru.cli.client import main; main()"` |
| **문단이 깨짐**(`llir pre\cni` 등) | PDF 임베디드 폰트레이어 손상 | `-m ocr` 재추출(자동 감지됨) |
| **표 안 수식이 `$...$` 문자로 보임** | HTML 표 안 수식은 Obsidian/GitHub 미렌더 | HTML표→MD 변환(기본 적용) |
| clone 시 `Filename too long` | 논문명 길어 경로 260자 초과 | 파일명·슬러그 캡·이미지 순번명(기본 적용) |

---

## 8. 출력 규약·주의

- 저장 위치: `논문 취합/03. 정리/`, 이미지는 ASCII 슬러그 하위폴더
- 중간 산출물 `.mineru_out/`은 `.gitignore` 제외(전체명 반복 깊은 경로 + 대용량)
- **검토 권장**: OCR 복구 문단은 경미한 OCR 흔적·수식 재구성이 있을 수 있어, 중요 수식·수치는 원문 PDF 대조 권장
- NOMENCLATURE·대형 부록표는 원본 OCR 품질·복잡 rowspan으로 변환 정렬이 완벽하지 않을 수 있음(수식은 렌더됨)
- 후속 한국어 정리는 `paper-summary` / `paper-original-summary`로 연계

---

## 9. 사례 (검증 완료)

| 논문 | 페이지 | 결과 |
|------|:---:|------|
| 2011 SKF Micropitting | 20p | 이미지 44개 누락0, Fig 품질 데스크톱 동급 |
| 2003 SKF Micro-Geometry | 14p | 폰트손상 감지→OCR 복구, 표 5개 MD변환, 깨짐0 (전자동 672초) |

---

**작성**: PDF 추출 스킬 개발 기록 기반 · **엔진**: MinerU 3.4.2 (pipeline, CPU)
