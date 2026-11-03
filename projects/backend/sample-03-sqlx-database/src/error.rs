use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

/// 애플리케이션 비즈니스 및 데이터베이스 에러
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    DatabaseError(String),
    InternalServerError(String),
}

/// sqlx::Error 발생 시 물음표(?) 연산자로 AppError::DatabaseError로 자동 승격!
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(format!("데이터베이스 오류: {err}"))
    }
}

/// 클라이언트에게 보낼 HTTP JSON 응답으로 변환
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(serde_json::json!({
            "success": false,
            "error": error_message,
            "code": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

