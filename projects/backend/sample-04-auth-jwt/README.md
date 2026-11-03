# 🔐 Sample 04: Argon2 & JWT 보안 인증 서버

현대 웹 표준 최고 보안 등급의 **Argon2id 비밀번호 암호화**, **JWT(JSON Web Token) 발급 및 서명 검증**, 그리고 Axum의 커스텀 익스트랙터(`Claims`)를 활용한 **선언적 인증 가드(Auth Guard)**를 갖춘 인증 서버 샘플입니다.

---

## 📌 주요 학습 내용
1. **Argon2id 패스워드 해싱**: GPU 무차별 대입 공격을 방어하는 메모리 강제(Memory-hard) 암호화
2. **JWT 토큰 발급 및 만료 시간 관리**: `jsonwebtoken` 크레이트를 이용한 HS256 서명 및 `exp` 검증
3. **Axum 커스텀 익스트랙터(Auth Guard)**: 핸들러 매개변수 선언만으로 `Authorization: Bearer <token>` 헤더를 파싱하고 미인증 요청을 401 차단
4. **Tower-HTTP 미들웨어**: 프론트엔드 연동을 위한 관대한 CORS 설정

---

## 🏃‍♂️ 실행 방법

```bash
cargo run -p sample-04-auth-jwt
```
- 서비스 포트는 **3003** (`http://127.0.0.1:3003`) 입니다.

---

## 🧪 실전 API 테스트 시나리오 (curl)

### 1. 신규 회원가입 (POST /api/auth/register)
```bash
curl -X POST http://127.0.0.1:3003/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username": "ferris_dev", "password": "secure_password_123"}'
```
**응답:** `201 Created`

---

### 2. 로그인 및 JWT 토큰 획득 (POST /api/auth/login)
```bash
curl -X POST http://127.0.0.1:3003/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "ferris_dev", "password": "secure_password_123"}'
```
**응답:**
```json
{
  "expires_in": 7200,
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer"
}
```

---

### 3. 보호된 라우트 접근 (GET /api/auth/me)

#### 토큰을 실어서 요청 (성공 200 OK)
```bash
curl http://127.0.0.1:3003/api/auth/me \
  -H "Authorization: Bearer <발급받은_토큰_붙여넣기>"
```
**응답:** `{"created_at":"...","id":1,"username":"ferris_dev"}`

#### 토큰 없이 접근 시도 (차단 401 Unauthorized)
```bash
curl http://127.0.0.1:3003/api/auth/me
```
**응답:** `{"code":401,"error":"인증 토큰(Authorization 헤더)이 누락되었습니다.","success":false}`

---

## ⚡ 자동화 테스트 실행 (cargo test)

```bash
cargo test -p sample-04-auth-jwt
```
회원가입, 로그인, 토큰 인증, 미인증 차단, 중복 가입 차단 등 4가지 보안 테스트를 0.05초 만에 검증합니다!

