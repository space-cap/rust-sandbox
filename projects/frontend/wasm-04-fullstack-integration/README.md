# 🦀 Sample 04: Axum 백엔드 & Yew 프론트엔드 풀스택 연동 (wasm-04-fullstack-integration)

Rust로 구축한 백엔드 REST API([`sample-02-rest-crud`](../../backend/sample-02-rest-crud))와 WebAssembly 프론트엔드([`wasm-04-fullstack-integration`](.))를 **동일한 Rust 구조체(DTO)를 공유**하여 엔드투엔드(End-to-End)로 연결하는 풀스택 실습 프로젝트입니다.

---

## 📌 주요 학습 내용

1. **단일 DTO 모델 공유 ("풀스택 Rust의 기적")**:
   - 백엔드의 `struct Todo`, `CreateTodoDto`, `UpdateTodoDto` 모델과 프론트엔드 모델이 **100% 동일한 Rust 코드**로 동작합니다.
   - TypeScript나 Swagger, OpenAPI 코드 생성기 없이도 컴파일 타임에 완벽한 타입 안전성이 보장됩니다.
2. **`gloo-net`을 활용한 비동기 HTTP 통신**:
   - `Request::get`, `Request::post`, `Request::put`, `Request::delete`
   - `wasm_bindgen_futures::spawn_local`을 통한 논블로킹 비동기 렌더링
3. **Trunk 리버스 프록시 (`Trunk.toml`)**:
   - 로컬 개발 환경에서 `/api/...` 요청을 Axum 서버(`http://127.0.0.1:3001`)로 자동 포워딩하여 브라우저 CORS 문제 없이 원활하게 통신
4. **실시간 서버 연결 상태 인디케이터**:
   - 🟢 연결됨 (`Connected`), 🔴 오프라인 (`Disconnected`), 🟡 확인 중 (`Connecting`)
   - 로딩 스피너 및 네트워크 예외 처리 배너 제공
5. **완전한 CRUD 웹 UI**:
   - 목록 조회 (`GET`), 신규 등록 (`POST`), 상태 토글 및 인라인 수정 (`PUT`), 삭제 (`DELETE`)

---

## 🏃‍♂️ 실행 방법 (풀스택 환경 구동)

본 풀스택 예제는 **1) 백엔드 Axum 서버**와 **2) 프론트엔드 Trunk 개발 서버**를 각각 실행합니다:

### 1단계: 백엔드 Axum REST API 서버 실행
새 터미널을 열고 다음을 실행합니다:
```bash
cd H:\lee\rust-sandbox\projects\backend\sample-02-rest-crud
cargo run
```
> 서버가 포트 `3001` (`http://127.0.0.1:3001`)에서 구동됩니다.

### 2단계: 프론트엔드 Yew Wasm 개발 서버 실행
또 다른 터미널에서 다음을 실행합니다:
```bash
cd H:\lee\rust-sandbox\projects\frontend\wasm-04-fullstack-integration
trunk serve --port 9093
```
- 브라우저에서 **`http://127.0.0.1:9093`**에 접속합니다.
- 상단 상태바에 `🟢 Axum 서버 연결됨 (3001)`이 표시되며, 실시간으로 Todo 데이터를 백엔드와 주고받을 수 있습니다.

---

## 🧪 DTO 직렬화 및 모델 단위 테스트

```bash
cargo test -p wasm-04-fullstack-integration
```

---

## 🏗️ 프로덕션 배포용 빌드

```bash
trunk build --release
```
- `dist/` 폴더에 번들링된 바이너리와 정적 파일이 생성됩니다.
