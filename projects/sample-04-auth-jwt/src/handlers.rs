use crate::auth::{create_jwt, hash_password, verify_password, Claims, DEFAULT_JWT_SECRET};
use crate::error::AppError;
use crate::models::{AuthResponse, LoginRequest, RegisterRequest, User, UserResponse};
use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
use tokio::sync::RwLock;
use tracing::info;

/// 멀티스레드 안전한 인메모리 유저 저장소 및 JWT 시크릿
#[derive(Clone)]
pub struct AppState {
    pub users: Arc<RwLock<HashMap<String, User>>>,
    pub next_id: Arc<AtomicU64>,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new() -> Self {
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| DEFAULT_JWT_SECRET.into());
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(AtomicU64::new(1)),
            jwt_secret: secret,
        }
    }
}

/// 1. 회원가입: POST /api/auth/register
pub async fn register_handler(
    State(state): State<AppState>,
    Json(dto): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    let username = dto.username.trim();
    if username.len() < 3 {
        return Err(AppError::BadRequest("아이디는 최소 3자 이상이어야 합니다.".into()));
    }
    if dto.password.len() < 6 {
        return Err(AppError::BadRequest("비밀번호는 최소 6자 이상이어야 합니다.".into()));
    }

    // 아이디 중복 검사
    {
        let users = state.users.read().await;
        if users.contains_key(username) {
            return Err(AppError::Conflict("이미 사용 중인 아이디입니다.".into()));
        }
    }

    // Argon2id 암호화
    let password_hash = hash_password(&dto.password)?;
    let user_id = state.next_id.fetch_add(1, Ordering::SeqCst);
    let created_at = Utc::now().to_rfc3339();

    let new_user = User {
        id: user_id,
        username: username.to_string(),
        password_hash,
        created_at: created_at.clone(),
    };

    // 저장소에 등록
    {
        let mut users = state.users.write().await;
        users.insert(username.to_string(), new_user);
    }

    info!("신규 회원가입 완료: ID={}, username='{}'", user_id, username);

    Ok((
        StatusCode::CREATED,
        Json(UserResponse {
            id: user_id,
            username: username.to_string(),
            created_at,
        }),
    ))
}

/// 2. 로그인: POST /api/auth/login
pub async fn login_handler(
    State(state): State<AppState>,
    Json(dto): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let users = state.users.read().await;
    let user = users
        .get(dto.username.trim())
        .ok_or_else(|| AppError::Unauthorized("아이디 또는 비밀번호가 올바르지 않습니다.".into()))?;

    // Argon2 비밀번호 일치 검증
    let is_valid = verify_password(&dto.password, &user.password_hash)?;
    if !is_valid {
        return Err(AppError::Unauthorized("아이디 또는 비밀번호가 올바르지 않습니다.".into()));
    }

    // JWT 토큰 발급
    let (token, expires_in) = create_jwt(user.id, &user.username, &state.jwt_secret)?;

    info!("로그인 성공: username='{}', 토큰 발급 완료", user.username);

    Ok(Json(AuthResponse {
        token,
        token_type: "Bearer".into(),
        expires_in,
    }))
}

/// 3. 내 프로필 조회: GET /api/auth/me (보호된 라우트 🔒)
/// Claims 추출기 덕분에 유효한 JWT 토큰이 없으면 이 함수는 호출조차 되지 않습니다!
pub async fn me_handler(
    claims: Claims, // 👈 마법의 Auth Guard!
    State(state): State<AppState>,
) -> Result<Json<UserResponse>, AppError> {
    let users = state.users.read().await;
    let user = users
        .get(&claims.username)
        .ok_or_else(|| AppError::Unauthorized("사용자 정보를 찾을 수 없습니다.".into()))?;

    info!("내 프로필 조회 성공: ID={}, username='{}'", user.id, user.username);

    Ok(Json(UserResponse {
        id: user.id,
        username: user.username.clone(),
        created_at: user.created_at.clone(),
    }))
}

/// 4. 공개 API: GET /api/public/hello
pub async fn public_hello_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "message": "누구나 접근 가능한 공개 엔드포인트입니다! 🌐"
    }))
}

