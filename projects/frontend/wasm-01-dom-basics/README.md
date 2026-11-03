# 🦀 Sample 01: WebAssembly DOM 조작 기초 & Trunk

브라우저 환경에서 JavaScript 없이 순수 **Rust와 WebAssembly(Wasm)**로 DOM을 직접 조작하고, 이벤트를 처리하며, JavaScript와 양방향 FFI(Foreign Function Interface)를 수행하는 기초 프로젝트입니다.

---

## 📌 주요 학습 내용

1. **`wasm-bindgen`**:
   - Rust 데이터 타입과 JavaScript 간의 안전하고 빠른 메모리 브릿지 구축
   - Rust 함수를 JS로 내보내기 (`#[wasm_bindgen]`) 및 JS API 가져오기
2. **`web-sys` & `js-sys`**:
   - 브라우저의 DOM(`Window`, `Document`, `HtmlElement`, `Event`)을 Rust 타입 시스템으로 다루기
   - `Closure::<dyn FnMut(...)>`를 이용한 브라우저 이벤트 리스너 등록
3. **`console_error_panic_hook`**:
   - 브라우저 Wasm에서 Rust panic 발생 시 콘솔에 친절한 스택 트레이스 출력
4. **Trunk 번들러**:
   - Webpack/Vite 없이 HTML 파일 하나로 Wasm 컴파일, 바인딩 코드 생성, 핫 리로딩(Live Reload)을 제공하는 현대적인 Rust Wasm 빌드 툴

---

## 🏃‍♂️ 실행 방법 (로컬 개발 서버)

본 디렉터리(`projects/frontend/wasm-01-dom-basics`)에서 다음 명령어를 실행합니다:

```bash
trunk serve
```

기본적으로 `http://127.0.0.1:8080` 포트로 로컬 웹 서버가 열리며, 소스 코드(`src/*.rs`, `index.html`, `styles.css`)를 수정하면 브라우저가 자동으로 핫 리로딩됩니다.

> 💡 **포트 변경 시**: `trunk serve --port 8081`

---

## 🏗️ 프로덕션 빌드 (배포용)

```bash
trunk build --release
```

- 최적화된 WebAssembly 바이너리(`.wasm`)와 정적 파일들이 `dist/` 폴더에 생성됩니다.
- Nginx, Vercel, Netlify, GitHub Pages 등 어떤 정적 웹 호스팅 환경에도 그대로 배포할 수 있습니다.

---

## 🧪 순수 Rust 로직 단위 테스트

비즈니스 로직(피보나치, 텍스트 분석 등)은 브라우저 없이도 즉시 테스트할 수 있습니다:

```bash
cargo test -p wasm-01-dom-basics
```

