# TRB Contact Analysis - 프로젝트 구조

## 디렉토리 레이아웃

```
D:\SW\TRB\
├── src-tauri/                    # Rust 백엔드
│   ├── Cargo.toml               # 의존성 (nalgebra, sprs, rayon, serde 등)
│   ├── src/
│   │   ├── main.rs              # Tauri 진입점
│   │   ├── lib.rs               # 앱 빌더 + 커맨드 등록
│   │   ├── commands.rs          # Tauri #[command] 핸들러
│   │   ├── error.rs             # SolverError enum
│   │   └── solver/
│   │       ├── mod.rs           # 모듈 re-export
│   │       ├── types.rs         # 22개 데이터 구조 [구현]
│   │       ├── geometry.rs      # 슬라이싱 + 프로파일 보간 [구현]
│   │       ├── hertz.rs         # Hertz 접촉 + Weber 벌크 변형 [구현]
│   │       ├── gen1.rs          # 독립 슬라이스 솔버 [스텁]
│   │       ├── gen3.rs          # 빔 커플링 솔버 [스텁]
│   │       ├── beam.rs          # Timoshenko 빔 FE [스텁]
│   │       ├── rib_contact.rs   # 리브 접촉 [스텁]
│   │       ├── bearing.rs       # 5-DOF 평형 해석 [스텁]
│   │       └── life.rs          # ISO 16281 수명 계산 [스텁]
│   └── capabilities/
│       └── default.json         # Tauri 권한 설정
├── src/                         # React 프론트엔드
│   ├── App.tsx                  # 메인 레이아웃
│   ├── App.css                  # 스타일
│   ├── types/
│   │   └── bearing.ts           # Rust 타입 미러링 (TypeScript)
│   ├── hooks/
│   │   └── useSolver.ts         # Tauri invoke 래퍼
│   └── components/
│       ├── InputPanel/          # 입력 패널 [플레이스홀더]
│       ├── BearingView3D/       # Three.js 3D 뷰 [플레이스홀더]
│       ├── ResultCharts/        # Plotly 차트 [플레이스홀더]
│       ├── ContourMap/          # 압력/응력 히트맵 [플레이스홀더]
│       ├── ComparisonView/      # Gen1 vs Gen3 비교 [플레이스홀더]
│       └── AlertPanel/          # 경고 표시 [플레이스홀더]
├── CLAUDE.md                    # AI 개발 가이드
├── Master_plan.md               # 전체 시스템 설계 문서
├── History.md                   # 작업 히스토리
└── Manual/                      # 매뉴얼
```

## 빌드 방법

**주의**: VS 2022 개발자 환경이 필요합니다. VS 18의 `link.exe`만으로는 `msvcrt.lib`을 찾지 못합니다.

```bash
# 프론트엔드 빌드
npm run build

# Rust 솔버 빌드 (VS 2022 환경 필요)
# 방법 1: Developer Command Prompt for VS 2022에서 실행
cd src-tauri && cargo build

# 방법 2: 배치 파일 사용
vcvarsall.bat x64 && cd src-tauri && cargo build

# 테스트
cargo test

# 전체 앱 실행
npm run tauri dev
```

## Phase 현황

| Phase | 상태 | 내용 |
|-------|------|------|
| 1 | **완료** | 데이터 구조 + Hertz/Weber + 슬라이싱 |
| 2 | 예정 | Gen1 독립 슬라이스 솔버 |
| 3 | 예정 | Gen3 빔 커플링 솔버 |
| 4 | 예정 | 5-DOF 베어링 평형 + 리브 접촉 |
| 5 | 예정 | ISO 16281 수명 계산 |
| 6 | 예정 | Tauri UI (Three.js, Plotly) |
| 7 | 예정 | 검증 + 최적화 |
