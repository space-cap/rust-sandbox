use crate::error::AppError;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;
use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub const DEFAULT_JWT_SECRET: &str = "super-secret-rust-jwt-token-key-2026";

/// JWT 페이로드에 담기는 사용자 클레임
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: u64,           // 사용자 고유 ID
    pub username: String,   // 사용자명
    pub exp: usize,         // 만료 시각 (타임스탬프)
}

/// 1. 비밀번호 암호화 (Argon2id 해싱)
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| AppError::InternalServerError(format!("비밀번호 암호화 실패: {e}")))
}

/// 2. 비밀번호 검증
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| AppError::InternalServerError(format!("해시 파싱 실패: {e}")))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// 3. JWT 토큰 발급 (유효기간: 2시간)
pub fn create_jwt(user_id: u64, username: &str, secret: &str) -> Result<(String, usize), AppError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(2))
        .expect("만료 시각 계산 실패")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        username: username.to_string(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::InternalServerError(format!("JWT 토큰 생성 실패: {e}")))?;

    Ok((token, 7200)) // 7200초 = 2시간
}

/// 4. JWT 토큰 검증
pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized("유효하지 않거나 만료된 토큰입니다.".into()))?;

    Ok(token_data.claims)
}

/// 5. Axum 커스텀 익스트랙터 (인증 가드 / Auth Guard)
/// 핸들러 매개변수에 `claims: Claims`를 적으면 자동으로 Authorization 헤더를 검증!
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 1. Authorization 헤더 추출
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("인증 토큰(Authorization 헤더)이 누락되었습니다.".into()))?;

        // 2. "Bearer <token>" 형식 분리
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("올바른 Bearer 토큰 형식이 아닙니다.".into()))?;

        // 3. 서명 및 유효기간 검증
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| DEFAULT_JWT_SECRET.into());
        verify_jwt(token, &secret)
    }
}

