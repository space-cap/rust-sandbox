# 🚀 Sample 01: Axum 기본 API 서버

초고속 비동기 런타임 **Tokio**와 **Axum** 프레임워크를 기반으로 제작된 기본 REST API 서버 샘플입니다.

---

## 📌 주요 학습 내용
1. `Router`와 `get`, `post` 라우팅 등록
2. `Path` 추출기: URL 경로 변수 바인딩 (`/api/users/{id}`)
3. `Query` 추출기: 쿼리스트링 파싱 (`/api/search?keyword=...&limit=...`)
4. `Json` 추출기: JSON 본문 역직렬화 및 응답 직렬화 (`POST /api/users`)
5. `tracing`을 활용한 비동기 구조화 로깅
6. `Graceful Shutdown`: `Ctrl+C` 입력 시 진행 중인 요청을 처리한 후 안전 종료

---

## 🏃‍♂️ 실행 방법

프로젝트 루트 또는 본 폴더에서 다음 명령을 실행합니다:

```bash
cargo run -p sample-01-basic-api
```

서버가 구동되면 `http://127.0.0.1:3000` 주소로 접근할 수 있습니다.

---

## 🧪 API 테스트 (curl 명령어)

### 1. 루트 페이지 확인
```bash
curl http://127.0.0.1:3000/
```

### 2. 헬스체크 확인
```bash
curl http://127.0.0.1:3000/health
```

### 3. Path 파라미터 (유저 조회)
```bash
curl http://127.0.0.1:3000/api/users/7
```

### 4. Query 파라미터 (검색)
```bash
curl "http://127.0.0.1:3000/api/search?keyword=rust&limit=5"
```

### 5. JSON 요청 (새 유저 생성)
```bash
curl -X POST http://127.0.0.1:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"username": "ferris_coder", "email": "ferris@rust-lang.org"}'
```

