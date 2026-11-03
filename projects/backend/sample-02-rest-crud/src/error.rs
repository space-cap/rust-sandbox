use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

/// 애플리케이션 전역 비즈니스 에러 열거형
#[derive(Debug)]
#[allow(dead_code)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    InternalServerError(String),
}

/// Axum 응답으로 변환하는 IntoResponse 구현체
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
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
