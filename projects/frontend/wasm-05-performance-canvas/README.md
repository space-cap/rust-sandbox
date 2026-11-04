# 🦀 Sample 05: Canvas 2D 고성능 그래픽 연산 & 파티클 시뮬레이터 (wasm-05-performance-canvas)

자바스크립트의 고질적인 가비지 컬렉터(GC) 스터터링 없이, 브라우저 위에서 **수천 개의 입자 물리 연산과 성좌 연결선(Constellation)을 초당 60프레임(60FPS)**으로 부드럽게 렌더링하는 WebAssembly 그래픽 프로젝트입니다.

---

## 📌 주요 학습 내용

1. **HTML5 `<canvas>` 2D 직접 제어**:
   - `web-sys::CanvasRenderingContext2d`를 통해 브라우저 네이티브 2D 가속 엔진 조작
   - Motion Blur 잔상 효과, 호(Arc), 선(Stroke), 방사형 색상 렌더링
2. **60FPS 논블로킹 애니메이션 루프**:
   - `gloo-render`의 `request_animation_frame`을 활용한 브라우저 V-Sync 주기 동기화
   - 실시간 고정밀 `Performance.now()` 기반 프레임 레이트(FPS) 측정
3. **가비지 컬렉션(GC) 없는 연속 메모리 재사용**:
   - 자바스크립트는 매 프레임 수천 개의 객체를 생성/폐기하며 GC 멈춤(Stop-the-world)이 발생하지만, Rust Wasm은 연속된 `Vec<Particle>` 메모리 버퍼를 재사용하여 완벽한 프레임 균일성을 보장합니다.
4. **4가지 인터랙티브 물리 모드**:
   - 🧲 **인력 (Attract)**: 마우스/터치 위치로 입자들을 끌어당김
   - 💥 **척력 (Repel)**: 커서 접근 시 입자들을 밀어냄
   - 🌀 **소용돌이 (Vortex)**: 커서 주위로 접선 방향 나선 회전력 부여
   - ✨ **유영 (Float)**: 벽면 탄성 충돌 기반 자유 부유
5. **성좌 연결 효과 (Constellation Effect)**:
   - 근접한 두 입자 간의 거리를 계산하여 동적 투명도(Alpha)를 갖는 연결선 렌더링

---

## 🏃‍♂️ 실행 방법 (로컬 개발 서버)

본 디렉터리(`projects/frontend/wasm-05-performance-canvas`)에서 다음 명령어를 실행합니다:

```bash
trunk serve --port 9094
```

- 웹 브라우저에서 **`http://127.0.0.1:9094`**에 접속합니다.
- 캔버스 위를 마우스로 드래그하거나 클릭하여 파티클의 반응을 관찰해 보세요!

---

## 🧪 물리 엔진 순수 Rust 단위 테스트

브라우저 Canvas 없이도 입자 물리 충돌, 중력, 속도 계산 로직을 즉시 테스트합니다:

```bash
cargo test -p wasm-05-performance-canvas
```

---

## 🏗️ 프로덕션 배포용 빌드

```bash
trunk build --release
```
- `dist/` 폴더에 최적화된 `.wasm` 바이너리와 정적 웹 파일들이 생성됩니다.
