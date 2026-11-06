# Rust Game 05: Particle Effects & Juice (파티클 시스템 & 게임 감각 극대화)

Bevy 0.20 기반의 **게임 감각(Game Feel / Juice) 구현 프로젝트**입니다.  
단순한 렌더링을 넘어 게이머에게 폭발적인 타격감을 선사하는 4대 주스 기법(파티클 버스트, 트라우마 2차 곡선 카메라 진동, 히트스톱/임팩트 프리즈, 실시간 기즈모 시각화)을 실전 구현합니다.

---

## 🎮 주요 기능 및 주스(Juice) 메커니즘

1. **45-개체 방사형 파티클 버스트 (Particle Burst)**
   - 타깃 파괴 시 $2\pi$ 전방향으로 45개의 파편이 난수 속도로 비산
   - 골드, 화염 오렌지, 크림슨 레드, 퓨어 화이트 4단계 컬러 팔레트
   - 공기 저항(Drag, $v \leftarrow v \times 0.91$) 적용으로 자연스러운 감속
   - 잔여 수명($\text{lifetime fraction}$)에 연동된 실시간 크기 축소 및 알파 페이드아웃

2. **트라우마 2차 곡선 모델 카메라 쉐이크 (Trauma Camera Shake)**
   - $\text{Intensity} = \text{Trauma}^2$ 비선형 모델 채택 (약한 충격은 미세하고, 강한 충격은 폭발적)
   - 캐논 발사 시 미세한 반동(Recoil): $+0.12$ Trauma
   - 타깃 피격 시: $+0.18$ Trauma
   - 타깃 완전 폭발 시: $+0.55$ Trauma
   - 초당 1.6 속도로 원점($0, 0$) 수렴 감쇠

3. **히트스톱 / 임팩트 프리즈 (Hitstop / Impact Freeze)**
   - 타깃 파괴 순간 0.06초간 물리 시뮬레이션 및 파티클 이동을 동결
   - 순간 정지 동안 카메라는 진동하여 극적인 타격 충격감(Impact Punch) 연출

4. **Bevy 2D 디버그 기즈모 (Debug Gizmos)**
   - `[G]` 키를 눌러 실시간 온/오프 토글
   - 플레이어(민트), 타깃(레드), 탄환(옐로우) AABB 충돌 영역 와이어프레임 렌더링
   - 비산 중인 모든 파티클의 실시간 속도 벡터($v \times 0.04$) 선형 시각화

5. **실시간 주스 통계 HUD**
   - 파괴된 타깃 수, 누적 발사 수, 현재 활성 파티클 개수, 현재 카메라 Trauma 수치, 기즈모 상태 실시간 표시

---

## 🕹 조작 방법

| 키 입력 | 동작 |
| :--- | :--- |
| **`A` / `D`** 또는 **`←` / `→`** | 좌우 이동 |
| **`W` / `S`** 또는 **`↑` / `↓`** | 상하 이동 |
| **`Space`** | 쇼크웨이브 캐논 발사 (반동 쉐이크 동반) |
| **`G`** | 디버그 기즈모 (AABB 충돌체 & 속도 벡터) On/Off 토글 |

---

## 🚀 실행 및 테스트

### 1. 단위 테스트 실행 (Headless Unit Tests)
```bash
cargo test -p game-05-particle-effects
```
- AABB 충돌체 교차 판정 검증
- 카메라 쉐이크 트라우마 누적, 1.0 클램핑 및 감쇠 수렴 검증
- 히트스톱 타이머 라이프사이클 및 런 조건 검증
- 파티클 공기 저항 감속 및 수명 소멸(Despawn) 검증

### 2. 게임 실행
```bash
cargo run -p game-05-particle-effects
```

---

## 📂 파일 구조

```
game-05-particle-effects/
├── Cargo.toml
├── README.md
├── assets/
│   ├── audio/
│   │   ├── explosion.wav      # 폭발 사운드
│   │   └── laser.wav          # 발사 사운드
│   └── fonts/
│       └── malgun.ttf         # 맑은 고딕 한글 폰트
└── src/
    ├── main.rs                # 윈도우 초기화 및 단위 테스트
    ├── lib.rs                 # ParticleEffectsPlugin 및 Run Condition
    ├── components.rs          # 엔티티 컴포넌트 & 주스 리소스 정의
    └── systems/
        ├── mod.rs
        ├── setup.rs           # 카메라, 엔티티, UI 초기 스폰
        ├── player.rs          # 캐논 제어 & 반동 쉐이크
        ├── target.rs          # 충돌 판정, 파티클 버스트, 히트스톱, 리스폰
        ├── particle.rs        # 공기 저항, 이동, 페이드아웃
        ├── shake.rs           # 트라우마 카메라 쉐이크 & 히트스톱 타이머
        ├── gizmos.rs          # 기즈모 토글 & 2D 기즈모 렌더링
        └── hud.rs             # 실시간 주스 HUD 텍스트 갱신
```

