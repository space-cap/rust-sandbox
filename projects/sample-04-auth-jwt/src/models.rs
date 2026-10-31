use serde::{Deserialize, Serialize};

/// 데이터베이스에 저장되는 사용자 엔티티
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password_hash: String,
    pub created_at: String,
}

/// 회원가입 요청 DTO
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

/// 로그인 요청 DTO
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 로그인 성공 시 반환되는 토큰 응답
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub token_type: String,
    pub expires_in: usize,
}

/// 비밀번호를 제외한 안전한 사용자 프로필 응답
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: u64,
    pub username: String,
    pub created_at: String,
}

