//! gloo-net을 이용한 브라우저 비동기 HTTP REST API 클라이언트
use gloo_net::http::Request;
use crate::dto::{CreateTodoDto, Todo, UpdateTodoDto};

/// GET /api/todos : 전체 할 일 목록 조회
pub async fn fetch_todos(api_base: &str) -> Result<Vec<Todo>, String> {
    let url = format!("{api_base}/todos");
    let resp = Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("네트워크 통신 실패: {e}"))?;

    if !resp.ok() {
        return Err(format!("서버 오류 응답 (HTTP {})", resp.status()));
    }

    resp.json::<Vec<Todo>>()
        .await
        .map_err(|e| format!("응답 데이터 파싱 실패: {e}"))
}

/// POST /api/todos : 새 할 일 등록
pub async fn create_todo(api_base: &str, dto: &CreateTodoDto) -> Result<Todo, String> {
    let url = format!("{api_base}/todos");
    let resp = Request::post(&url)
        .json(dto)
        .map_err(|e| format!("요청 바디 직렬화 실패: {e}"))?
        .send()
        .await
        .map_err(|e| format!("네트워크 전송 실패: {e}"))?;

    if !resp.ok() {
        return Err(format!("등록 실패 (HTTP {})", resp.status()));
    }

    resp.json::<Todo>()
        .await
        .map_err(|e| format!("생성된 항목 파싱 실패: {e}"))
}

/// PUT /api/todos/{id} : 할 일 수정 (제목/설명/완료여부)
pub async fn update_todo(api_base: &str, id: u64, dto: &UpdateTodoDto) -> Result<Todo, String> {
    let url = format!("{api_base}/todos/{id}");
    let resp = Request::put(&url)
        .json(dto)
        .map_err(|e| format!("요청 바디 직렬화 실패: {e}"))?
        .send()
        .await
        .map_err(|e| format!("네트워크 전송 실패: {e}"))?;

    if !resp.ok() {
        return Err(format!("수정 실패 (HTTP {})", resp.status()));
    }

    resp.json::<Todo>()
        .await
        .map_err(|e| format!("수정된 항목 파싱 실패: {e}"))
}

/// DELETE /api/todos/{id} : 할 일 삭제
pub async fn delete_todo(api_base: &str, id: u64) -> Result<(), String> {
    let url = format!("{api_base}/todos/{id}");
    let resp = Request::delete(&url)
        .send()
        .await
        .map_err(|e| format!("네트워크 전송 실패: {e}"))?;

    if !resp.ok() {
        return Err(format!("삭제 실패 (HTTP {})", resp.status()));
    }

    Ok(())
}

