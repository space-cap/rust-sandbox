# Rust Game 01: Bevy 시작하기 & ECS 핵심 원리 정복

Bevy 게임 엔진의 핵심 아키텍처인 **ECS(Entity-Component-System)** 패러다임을 이해하고, 2D 게임 윈도우 생성, 2D 카메라, 스프라이트 엔티티 생성, 키보드 입력 처리 및 경계 클램핑(Boundary Clamping)을 구현한 실습 프로젝트입니다.

---

## 🎮 주요 기능 및 학습 포인트

1. **Bevy 앱 구성 & 윈도우 설정 (`WindowPlugin`, `ClearColor`)**:
   - `960x600` 해상도의 2D 창 초기화 및 다크 테마 배경색 적용
2. **2D 카메라 및 개체 스폰 (`Commands`, `Camera2d`, `Sprite`)**:
   - `commands.spawn(Camera2d)`로 2D 직교 투영 카메라 생성
   - `Sprite::from_color(...)`와 `Transform::from_xyz(...)`로 플레이어 사각형 엔티티 생성
3. **컴포넌트 설계 (`Component`)**:
   - `Player`: 플레이어를 식별하는 마커 컴포넌트
   - `Speed(pub f32)`: 초당 이동 픽셀 속도
   - `Boundary`: 화면 밖 이탈 방지를 위한 경계 제한 구조체 및 `clamp()` 메서드
   - `HudInfoText`: 실시간 플레이어 상태를 렌더링하는 2D 텍스트 마커
4. **시스템 및 쿼리 (`Query`, `Res`, `ResMut`)**:
   - `player_movement_system`: `ButtonInput<KeyCode>`와 `Time::delta_secs()`를 활용한 프레임 레이트 독립적 이동
   - 대각선 이동 정규화(Vector Normalize) 및 조작 방향에 따른 미세 틸트(회전) 시각 효과
   - `diagnostics_system`: 하단 HUD 텍스트 실시간 갱신 및 콘솔 성능 진단 로깅
5. **독립 플러그인 모듈화 (`Plugin`)**:
   - `GameBasicsPlugin`으로 리소스, Startup/Update 시스템 묶음 캡슐화
6. **헤드리스(Headless) 단위 테스트**:
   - 실제 윈도우나 GPU 없이 메모리 상에서 ECS 월드를 구동하여 이동 로직 및 경계 클램핑 100% 자동화 테스트

---

## 🕹️ 조작 방법

| 입력 키 | 동작 |
| :--- | :--- |
| **W** / **↑** | 위쪽으로 이동 |
| **S** / **↓** | 아래쪽으로 이동 |
| **A** / **←** | 왼쪽으로 이동 (좌측 미세 틸트) |
| **D** / **→** | 오른쪽으로 이동 (우측 미세 틸트) |

---

## 🚀 실행 및 테스트 방법

### 1. 단위 테스트 실행 (GUI 창 없이 초고속 실행)
```bash
cargo test -p game-01-bevy-basics
```

### 2. 2D 게임 실행
```bash
cargo run -p game-01-bevy-basics
```

