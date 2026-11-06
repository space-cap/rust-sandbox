# Rust Game 02: 2D 스프라이트 시트 & 프레임 애니메이션 🏃🦀

Bevy 게임 엔진에서 **단일 텍스처 아틀라스(Texture Atlas)**를 사용하여 드로우 콜(Draw Call)을 최적화하고, `AnimationTimer`와 `AnimationIndices` 컴포넌트를 통해 키보드 입력에 반응하는 살아 숨 쉬는 픽셀 페리스(Ferris the Crab) 애니메이션을 구현한 프로젝트입니다.

---

## 🎮 주요 기능 및 학습 포인트

1. **텍스처 아틀라스 레이아웃 (`TextureAtlasLayout`)**:
   - 128x64 픽셀의 페리스 스프라이트 시트를 가로 4칸 x 세로 2칸(각 셀 32x32)으로 자동 그리드 분할
   - `TextureAtlas { layout, index }` 구조로 렌더링할 특정 프레임 제어
2. **Nearest 텍스처 필터링 (`ImagePlugin::default_nearest()`)**:
   - 픽셀 아트 그래픽을 2.5배 확대(32px -> 80px)해도 블러(흐림) 없이 레트로 도트 느낌을 선명하게 유지
3. **상태 머신 기반 애니메이션 전환 (`AnimationState`)**:
   - **대기 (Idle)**: 프레임 0 ~ 3 (숨쉬기 & 발 까딱이기)
   - **달리기 (Run)**: 프레임 4 ~ 7 (다리 걷기 & 집게발 펌핑)
   - 키보드 입력(WASD/방향키) 시 즉시 `Run`으로 전환되며, 입력 중지 시 즉시 `Idle`로 복귀
4. **수평 스프라이트 반전 (`Sprite::flip_x`)**:
   - 왼쪽 이동 시 `sprite.flip_x = true`, 오른쪽 이동 시 `sprite.flip_x = false`로 설정하여 단 하나의 스프라이트 시트로 좌우 시선 처리
5. **프레임 레이트 독립 타이머 (`AnimationTimer`, `TimerMode::Repeating`)**:
   - 가변 델타 타임(`time.delta()`)을 누적하여 모니터 주사율에 관계없이 항상 0.12초마다 정확하게 1프레임씩 순환
6. **실시간 HUD 텍스트**:
   - 화면 하단에 현재 애니메이션 상태(Idle/Run), 아틀라스 인덱스(#0~#7), 방향, 좌표, 속도 실시간 렌더링 (맑은 고딕 한글 폰트 적용)
7. **헤드리스(Headless) 단위 테스트**:
   - 창 없이 메모리 상에서 키 입력에 따른 Idle $\leftrightarrow$ Run 상태 전환 및 아틀라스 인덱스 틱 전진 100% 자동 검증

---

## 🕹️ 조작 방법

| 입력 키 | 동작 | 애니메이션 |
| :--- | :--- | :--- |
| **W, A, S, D** / **방향키** | 상하좌우 이동 | **달리기 (Run)** 애니메이션 + 이동 방향 반전 |
| **입력 없음 (손을 뗌)** | 정지 | **대기 (Idle)** 애니메이션으로 자동 전환 |

---

## 🚀 실행 및 테스트 방법

### 1. 단위 테스트 실행 (초고속 검증)
```bash
cargo test -p game-02-sprite-animation
```

### 2. 2D 애니메이션 게임 실행
```bash
cargo run -p game-02-sprite-animation
```

