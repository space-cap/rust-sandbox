# 🦀 Sample 02: Yew 컴포넌트 & 상태 관리 (wasm-02-yew-counter)

React 개발자에게 친숙한 가상 DOM(Virtual DOM) 기반의 현대적 Rust 웹 프론트엔드 프레임워크 **Yew**를 학습하는 프로젝트입니다.

---

## 📌 주요 학습 내용

1. **`html!` 매크로**:
   - React의 JSX 문법과 거의 일치하는 선언적 컴포넌트 마크업 작성
   - 속성 바인딩, 동적 클래스(`classes!`), 조건부 렌더링(`if-else`), 반복 렌더링(`for iter`)
2. **함수형 컴포넌트 (`#[function_component]`)**:
   - React의 함수형 컴포넌트(`const MyComp = () => { ... }`)와 동일한 구조
   - Props 구조체(`#[derive(Properties, PartialEq)]`)를 통한 단방향 데이터 전달
3. **훅 기반 상태 관리 (`use_state`)**:
   - React의 `const [state, setState] = useState(0)`와 1:1로 대응되는 `use_state(|| 0)`
   - 불변성(Immutability) 보장 및 클론(Clone)을 통한 안전한 상태 전파
4. **이벤트 핸들러 (`Callback::from`)**:
   - `onclick`, `onchange` 등 브라우저 이벤트를 Rust 클로저로 캡처하여 안전하게 처리
5. **다크/라이트 모드 테마 스위처**:
   - Yew 상태와 CSS 변수(Custom Properties)를 결합한 부드러운 테마 전환
6. **실시간 변경 히스토리 로깅**:
   - 값 변경 내역을 실시간으로 추적하고 렌더링하는 반응형 UI 패턴

---

## 🏃‍♂️ 실행 방법 (로컬 개발 서버)

본 디렉터리(`projects/frontend/wasm-02-yew-counter`)에서 다음 명령어를 실행합니다:

```bash
trunk serve --port 9091
```

기본적으로 `http://127.0.0.1:9091` 포트로 로컬 웹 서버가 열리며, 소스 코드(`src/**/*.rs`, `index.html`, `styles.css`)를 수정하면 브라우저가 자동으로 핫 리로딩(Live Reload)됩니다.

---

## 🧪 비즈니스 로직 단위 테스트

UI 렌더링과 독립된 순수 Rust 카운터 및 히스토리 관리 로직을 검증합니다:

```bash
cargo test -p wasm-02-yew-counter
```

---

## 🏗️ 프로덕션 배포용 최적화 빌드

```bash
trunk build --release
```

- `dist/` 폴더에 최적화된 `.wasm` 바이너리와 정적 웹 번들이 생성됩니다.
