# P3 시각화 HTML — 타 컴퓨터 셋업 가이드

새 컴퓨터에서 이 저장소를 clone 하여 **본 컴퓨터와 동일한 2-워크트리 구조**(main / P3_HTML)로
작업하기 위한 절차. (관련 작업기록: `논문구현_P3_작업결과_시각화_HTML.md`)

---

## 개념 (먼저 읽기)

- 저장소는 **하나**(`github.com/CHOICHOI-KIMM/AI_Seminar`). 폴더를 둘로 나누는 것은 git **worktree** 기능이다.
- `AI_Seminar` = `main` 브랜치(Main bearing 등 비-P3), `AI_Seminar_P3` = `P3_HTML` 브랜치(P3 시각화).
- 두 폴더는 **같은 `.git` 을 공유**한다. 한쪽 커밋이 즉시 다른 쪽 `git log` 에 보인다.
- 폴더 이름(`AI_Seminar_P3`)은 **로컬에만 존재** — GitHub 에는 브랜치만 올라간다.

---

## 절차

### 1. Clone
```bash
cd /d/AI                       # 원하는 상위 폴더 (경로는 자유)
git clone https://github.com/CHOICHOI-KIMM/AI_Seminar.git
cd AI_Seminar                  # 이 폴더 = main (비-P3 작업)
```

### 2. worktree 로 P3_HTML 분리
```bash
git worktree add ../AI_Seminar_P3 P3_HTML
```
→ `../AI_Seminar_P3` 폴더 생성 = P3 작업 폴더.

### 3. ★ 훅 연결 (자립판 자동갱신 — clone 에 안 따라옴)
`core.hooksPath` 는 로컬 설정이라 clone 에 포함되지 않는다. **한 번만** 실행(`.git` 공유라 두 폴더 모두 적용):
```bash
git config core.hooksPath "논문 취합/03. 정리/논문구현_P3/hooks"
```
- 효과: `AI_Seminar_P3/논문 취합/03. 정리/논문구현_P3/viewer/` 소스를 고쳐 커밋하면
  `micropitting_viewer.html`(자립판)이 **자동 재생성·포함**된다.
- 생략해도 뷰어는 동작하나 자립판 자동갱신이 안 됨(3층 `--check` 로 수동 확인은 가능).

---

## clone 에 포함/미포함 (gitignore)

| 구분 | 항목 | 비고 |
|---|---|---|
| ✅ 포함(커밋됨) | `viewer/pkg/*.wasm`·glue, `viewer/micropitting_viewer.html`(자립판) | **뷰어는 툴체인 없이 바로 실행** |
| ✅ 포함 | `viewer/*.js`·`*.json`, `build_standalone.py`, `hooks/pre-commit`, 전 Rust 소스 | |
| ❌ 미포함(gitignore) | `micropitting-model/target`, `micropitting-wasm/target`·`pkg`·`pkg-node` | 빌드 시 재생성 |

---

## 용도별 필요 작업

### A. 뷰어 **열기만** — 툴체인 불필요
- **자립판(권장)**: `viewer/micropitting_viewer.html` 더블클릭 (서버·설치 불요).
- **서버판**: `viewer/뷰어실행.bat` 더블클릭 (또는 `viewer/` 에서 `python -m http.server 8741` → http://127.0.0.1:8741/index.html).

### B. P3 코드 **수정·재빌드** — Rust/WASM 툴체인 필요
```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.126   # ★ Cargo.lock 의 wasm-bindgen 과 정확히 일치
```
> `wasm-pack` 은 이 환경에서 OS 레벨 실행 차단된 이력이 있어 **2단계 직접 빌드**를 쓴다.
> 상세 절차는 `micropitting-wasm/Cargo.toml` 상단 주석 참조:
> ```
> cargo build --target wasm32-unknown-unknown --release
> wasm-bindgen --target web    --out-dir viewer/pkg  target/wasm32-unknown-unknown/release/micropitting_wasm.wasm   # 뷰어용
> wasm-bindgen --target nodejs --out-dir micropitting-wasm/pkg-node  …/micropitting_wasm.wasm                        # node 검증용
> ```
> CLI 버전(0.2.126)이 Cargo.lock 과 다르면 glue 가 깨진다 → 버전 고정 필수.

---

## 셋업 검증 (선택)
```bash
# AI_Seminar_P3 에서
cargo test --manifest-path "논문 취합/03. 정리/논문구현_P3/micropitting-model/Cargo.toml"   # 106단위+2통합 green
python "논문 취합/03. 정리/논문구현_P3/viewer/build_standalone.py" --check                    # [check] OK = 자립판 최신
```

---

## 지속 갱신 3중 체계 (참고)
1. **SSOT**: 자립판은 `build_standalone.py` 의 순수 생성물 — 수기 편집 금지.
2. **pre-commit 훅**(위 3): viewer 소스 변경 커밋 시 자립판 자동 재빌드·포함.
3. **`--check`**: 소스 내용해시 vs 자립판 내장 마커 대조, 불일치 시 exit 1(우회·타 PC 대비 그물).

---

## 요약 (최소 4줄)
```bash
git clone https://github.com/CHOICHOI-KIMM/AI_Seminar.git && cd AI_Seminar
git worktree add ../AI_Seminar_P3 P3_HTML
git config core.hooksPath "논문 취합/03. 정리/논문구현_P3/hooks"
# 코드 재빌드 시만: rustup target add wasm32-unknown-unknown; cargo install wasm-bindgen-cli --version 0.2.126
```
