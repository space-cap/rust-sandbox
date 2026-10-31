# 🗄️ Sample 03: SQLx 영구 데이터베이스 API 서버

비동기 커넥션 풀(`SqlitePool`)과 자동 마이그레이션(`sqlx::migrate!`), 그리고 `FromRow` 기반의 타입 안전한 쿼리를 갖춘 SQLite 영구 저장소 REST API 서버입니다.

---

## 📌 주요 학습 내용
1. **비동기 커넥션 풀 관리**: `SqlitePool`을 Axum의 `State`로 공유하여 논블로킹 I/O 구현
2. **코드 레벨 자동 마이그레이션**: `migrations/` 내의 DDL 스크립트를 서버 기동 시 자동 적용
3. **타입 안전한 SQL 쿼리**: `sqlx::query_as`와 `FromRow`로 DB 레코드를 Rust 구조체로 자동 매핑
4. **영구 저장과 격리 테스트 분리**:
   - 운영/개발: 파일 기반 SQLite (`todos.db`)
   - 자동화 테스트: 초고속 인메모리 SQLite (`sqlite::memory:`)

---

## 🏃‍♂️ 실행 방법

```bash
cargo run -p sample-03-sqlx-database
```

- 서버가 구동되면 로컬 디렉토리에 `todos.db` 파일이 자동 생성됩니다.
- 서비스 주소는 **포트 3002** (`http://127.0.0.1:3002`) 입니다.

---

## 🧪 실전 API 테스트 (curl)

### 1. 새 Todo 데이터 등록 (POST)
```bash
curl -X POST http://127.0.0.1:3002/api/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "SQLx 마스터하기", "description": "SQLite에 영구 저장"}'
```

### 2. 전체 Todo 목록 조회 (GET)
```bash
curl http://127.0.0.1:3002/api/todos
```

### 3. 단건 조회 (GET)
```bash
curl http://127.0.0.1:3002/api/todos/1
```

### 4. 내용 및 상태 수정 (PUT)
```bash
curl -X PUT http://127.0.0.1:3002/api/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"completed": true}'
```

### 5. 삭제 (DELETE)
```bash
curl -X DELETE http://127.0.0.1:3002/api/todos/1
```

---

## ⚡ 자동화 테스트 실행 (cargo test)

```bash
cargo test -p sample-03-sqlx-database
```
인메모리 SQLite를 활용하여 0.01초 만에 전체 CRUD 라이프사이클을 독립적으로 검증합니다!

