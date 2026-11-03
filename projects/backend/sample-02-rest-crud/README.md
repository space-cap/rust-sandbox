# 🚀 Sample 02: 견고한 RESTful CRUD API 서버

스레드 안전한 인메모리 상태 관리(`Arc<RwLock<AppState>>`)와 관용적(Idiomatic) 에러 핸들링(`AppError`, `IntoResponse`)을 갖춘 모듈식 Todo REST API 서버입니다.

---

## 📌 주요 학습 내용
1. **스레드 안전한 동시성 제어**: `Arc<RwLock<T>>`를 이용해 읽기는 동시 다발적으로, 쓰기는 안전하게 독점 처리
2. **원자적 ID 생성**: `std::sync::atomic::AtomicU64`로 락 없이 고유 ID 부여
3. **관용적 에러 핸들링**: `AppError` enum 및 `IntoResponse` 구현으로 일관된 JSON 에러 응답 규격화
4. **계층형 모듈 아키텍처**: `models`, `error`, `handlers`, `main`으로 코드 관심사 분리
5. **Axum State 주입**: `with_state`와 `State<AppState>` 추출기를 통한 의존성 주입

---

## 🏃‍♂️ 실행 방법

```bash
cargo run -p sample-02-rest-crud
```

서버가 구동되면 `http://127.0.0.1:3001` 주소로 서비스됩니다.

---

## 🧪 실전 API 테스트 시나리오 (curl)

### 1. 신규 Todo 등록 (POST /api/todos)
```bash
curl -X POST http://127.0.0.1:3001/api/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Axum 상태 공유 마스터하기", "description": "Arc와 RwLock 활용법 익히기"}'
```
**응답:** `201 Created`와 함께 생성된 JSON 객체 반환 (`"id": 1`)

---

### 2. 전체 Todo 목록 조회 (GET /api/todos)
```bash
curl http://127.0.0.1:3001/api/todos
```
**응답:** `200 OK`와 함께 Todo 배열 반환

---

### 3. 단건 Todo 상세 조회 (GET /api/todos/1)
```bash
curl http://127.0.0.1:3001/api/todos/1
```
**응답:** `200 OK`와 함께 1번 Todo 반환

---

### 4. Todo 내용 및 완료 상태 수정 (PUT /api/todos/1)
```bash
curl -X PUT http://127.0.0.1:3001/api/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"completed": true}'
```
**응답:** `200 OK`, `completed: true`로 업데이트된 Todo 반환

---

### 5. 에러 처리 검증 (400 Bad Request & 404 Not Found)

#### 빈 제목으로 생성 시도 (400 Bad Request)
```bash
curl -X POST http://127.0.0.1:3001/api/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "   "}'
```
**응답:** `{"code": 400, "error": "할 일 제목(title)은 비어 있을 수 없습니다.", "success": false}`

#### 존재하지 않는 999번 조회 (404 Not Found)
```bash
curl http://127.0.0.1:3001/api/todos/999
```
**응답:** `{"code": 404, "error": "ID 999번 할 일을 찾을 수 없습니다.", "success": false}`

---

### 6. Todo 삭제 (DELETE /api/todos/1)
```bash
curl -X DELETE http://127.0.0.1:3001/api/todos/1
```
**응답:** `204 No Content`

---

## ⚡ 초고속 자동화 통합 테스트 (cargo test)

서버 포트를 열지 않고도 메모리 상에서 가상의 HTTP 요청을 전송해 전체 REST API의 동작을 0.01초 만에 검증합니다!

```bash
cargo test -p sample-02-rest-crud
```

### 테스트 항목 (`tests/api_tests.rs`)
- `test_create_and_get_todo`: 신규 생성 및 단건 조회 검증
- `test_empty_title_validation_fails`: 빈 제목 유효성 검증 실패(400 Bad Request) 확인
- `test_get_not_found_todo`: 존재하지 않는 ID 조회 시 404 Not Found 확인
- `test_update_and_delete_todo`: 상태 수정, 삭제(204 No Content), 삭제 후 재조회(404) 확인

