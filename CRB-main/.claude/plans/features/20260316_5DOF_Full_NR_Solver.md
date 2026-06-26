# 5-DOF Full NR 베어링 평형 솔버

## TL;DR
- **Quick Summary**: TRB δz 자유변수 3-DOF force NR 솔버 (nalgebra LM-damped)
- **Deliverables**: `solve_bearing_equilibrium_5dof()` 함수 ✅
- **Status**: 구현 완료, 테스트 통과

## 구현 결과

### 완료 항목
- [x] nalgebra LM-damped NR로 δx/δy/δz 동시 풀이
- [x] 중심차분 야코비안 + DOF/잔차 스케일링
- [x] Line search + step limiting
- [x] Block solver warm-start (초기값)
- [x] 테스트 3개 통과

### 핵심 발견: 모멘트 자유도 제한
- γx/γy에 대한 My 감도(dMy/dγy)가 극히 낮음
- 독립 베어링 모델에서는 모멘트 평형이 물리적으로 불가 (축 구속 필요)
- MASTA의 My=-96 N·m는 축-베어링 시스템의 반력
- → 모멘트 잔차는 수렴 조건에서 제외, 반력으로 출력

### 남은 과제
- [ ] Tauri command 추가 (솔버 선택 옵션)
- [ ] UI에 솔버 선택 드롭다운 추가
- [ ] 축-베어링 시스템 모델 (모멘트 DOF 활성화 시 필요)
