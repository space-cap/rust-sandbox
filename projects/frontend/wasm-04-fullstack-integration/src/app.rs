use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::api::{create_todo, delete_todo, fetch_todos, update_todo};
use crate::components::create_form::CreateForm;
use crate::components::error_banner::ErrorBanner;
use crate::components::status_bar::StatusBar;
use crate::components::todo_card::TodoCard;
use crate::dto::{ConnectionStatus, CreateTodoDto, Todo, UpdateTodoDto};

#[function_component(App)]
pub fn app() -> Html {
    let todos = use_state(Vec::<Todo>::new);
    let api_base = use_state(|| "/api".to_string());
    let connection_status = use_state(|| ConnectionStatus::Connecting);
    let is_loading = use_state(|| false);
    let is_submitting = use_state(|| false);
    let error_message = use_state(|| Option::<String>::None);

    // 공통 데이터 로드 함수 (새로고침 및 초기 마운트 시 호출)
    let load_data = {
        let todos = todos.clone();
        let api_base = api_base.clone();
        let connection_status = connection_status.clone();
        let is_loading = is_loading.clone();
        let error_message = error_message.clone();

        Callback::from(move |()| {
            let todos = todos.clone();
            let base = (*api_base).clone();
            let connection_status = connection_status.clone();
            let is_loading = is_loading.clone();
            let error_message = error_message.clone();

            is_loading.set(true);

            spawn_local(async move {
                match fetch_todos(&base).await {
                    Ok(data) => {
                        todos.set(data);
                        connection_status.set(ConnectionStatus::Connected);
                        error_message.set(None);
                    }
                    Err(err) => {
                        connection_status.set(ConnectionStatus::Disconnected);
                        error_message.set(Some(format!(
                            "백엔드 연결 실패: {err}. Axum 서버(포트 3001)가 켜져 있는지 확인하세요."
                        )));
                    }
                }
                is_loading.set(false);
            });
        })
    };

    // 컴포넌트 마운트 및 API 주소 변경 시 자동 로드
    {
        let load_data = load_data.clone();
        let base_val = (*api_base).clone();
        use_effect_with(base_val, move |_| {
            load_data.emit(());
        });
    }

    // 1. 할 일 생성 (POST /api/todos)
    let on_create = {
        let todos = todos.clone();
        let api_base = api_base.clone();
        let is_submitting = is_submitting.clone();
        let error_message = error_message.clone();
        let connection_status = connection_status.clone();

        Callback::from(move |dto: CreateTodoDto| {
            let todos = todos.clone();
            let base = (*api_base).clone();
            let is_submitting = is_submitting.clone();
            let error_message = error_message.clone();
            let connection_status = connection_status.clone();

            is_submitting.set(true);

            spawn_local(async move {
                match create_todo(&base, &dto).await {
                    Ok(created) => {
                        let mut list = (*todos).clone();
                        list.push(created);
                        todos.set(list);
                        connection_status.set(ConnectionStatus::Connected);
                        error_message.set(None);
                    }
                    Err(err) => {
                        error_message.set(Some(format!("항목 생성 실패: {err}")));
                    }
                }
                is_submitting.set(false);
            });
        })
    };

    // 2. 완료 여부 토글 (PUT /api/todos/{id})
    let on_toggle = {
        let todos = todos.clone();
        let api_base = api_base.clone();
        let error_message = error_message.clone();

        Callback::from(move |(id, next_status): (u64, bool)| {
            let todos = todos.clone();
            let base = (*api_base).clone();
            let error_message = error_message.clone();

            let update_dto = UpdateTodoDto {
                title: None,
                description: None,
                completed: Some(next_status),
            };

            spawn_local(async move {
                match update_todo(&base, id, &update_dto).await {
                    Ok(updated) => {
                        let mut list = (*todos).clone();
                        if let Some(item) = list.iter_mut().find(|t| t.id == id) {
                            *item = updated;
                        }
                        todos.set(list);
                    }
                    Err(err) => {
                        error_message.set(Some(format!("상태 업데이트 실패: {err}")));
                    }
                }
            });
        })
    };

    // 3. 내용 수정 (PUT /api/todos/{id})
    let on_update_content = {
        let todos = todos.clone();
        let api_base = api_base.clone();
        let error_message = error_message.clone();

        Callback::from(move |(id, title, desc): (u64, String, Option<String>)| {
            let todos = todos.clone();
            let base = (*api_base).clone();
            let error_message = error_message.clone();

            let update_dto = UpdateTodoDto {
                title: Some(title),
                description: desc,
                completed: None,
            };

            spawn_local(async move {
                match update_todo(&base, id, &update_dto).await {
                    Ok(updated) => {
                        let mut list = (*todos).clone();
                        if let Some(item) = list.iter_mut().find(|t| t.id == id) {
                            *item = updated;
                        }
                        todos.set(list);
                    }
                    Err(err) => {
                        error_message.set(Some(format!("내용 수정 실패: {err}")));
                    }
                }
            });
        })
    };

    // 4. 삭제 (DELETE /api/todos/{id})
    let on_delete = {
        let todos = todos.clone();
        let api_base = api_base.clone();
        let error_message = error_message.clone();

        Callback::from(move |id: u64| {
            let todos = todos.clone();
            let base = (*api_base).clone();
            let error_message = error_message.clone();

            spawn_local(async move {
                match delete_todo(&base, id).await {
                    Ok(()) => {
                        let mut list = (*todos).clone();
                        list.retain(|t| t.id != id);
                        todos.set(list);
                    }
                    Err(err) => {
                        error_message.set(Some(format!("항목 삭제 실패: {err}")));
                    }
                }
            });
        })
    };

    // API 주소 변경
    let on_change_api_base = {
        let api_base = api_base.clone();
        Callback::from(move |new_base: String| {
            api_base.set(new_base);
        })
    };

    // 에러 닫기
    let on_dismiss_error = {
        let error_message = error_message.clone();
        Callback::from(move |_| {
            error_message.set(None);
        })
    };

    let on_refresh = {
        let load_data = load_data.clone();
        Callback::from(move |_| {
            load_data.emit(());
        })
    };

    html! {
        <div class="fullstack-app-container">
            <header class="app-masthead">
                <div class="brand-badge">{ "🦀 Phase 4: Fullstack Rust (Axum + Yew)" }</div>
                <h1 class="brand-title">{ "Axum REST API & Yew 풀스택 연동" }</h1>
                <p class="brand-subtitle">
                    { "TypeScript/OpenAPI 변환 없는 100% 동일 Rust 모델 공유 & gloo-net 비동기 HTTP 통신" }
                </p>
            </header>

            <StatusBar
                status={*connection_status}
                api_base={(*api_base).clone()}
                is_loading={*is_loading}
                total_count={todos.len()}
                on_refresh={on_refresh}
                on_change_api_base={on_change_api_base}
            />

            <ErrorBanner
                error={(*error_message).clone()}
                on_dismiss={on_dismiss_error}
            />

            <main class="main-content-layout">
                <section class="left-pane">
                    <CreateForm
                        on_create={on_create}
                        is_submitting={*is_submitting}
                    />

                    <div class="card info-card">
                        <h3>{ "✨ 풀스택 Rust의 독보적 강점" }</h3>
                        <ul class="info-list">
                            <li>
                                <strong>{ "단일 모델 정의(Single Source of Truth)" }</strong>
                                <p>{ "백엔드의 struct Todo와 프론트엔드의 struct Todo가 100% 일치합니다." }</p>
                            </li>
                            <li>
                                <strong>{ "Trunk 리버스 프록시" }</strong>
                                <p>{ "Trunk.toml에 등록된 리버스 프록시 덕분에 브라우저 CORS 문제 없이 /api 요청을 처리합니다." }</p>
                            </li>
                            <li>
                                <strong>{ "비동기 Wasm 런타임" }</strong>
                                <p>{ "gloo-net과 spawn_local로 브라우저 이벤트 루프를 차단(Blocking)하지 않고 안전하게 통신합니다." }</p>
                            </li>
                        </ul>
                    </div>
                </section>

                <section class="right-pane">
                    <div class="list-header">
                        <h2>{ "서버 데이터베이스 동기화 목록" }</h2>
                        <span class="count-badge">{ format!("총 {}건", todos.len()) }</span>
                    </div>

                    if *is_loading && todos.is_empty() {
                        <div class="loading-state card">
                            <span class="big-spinner"></span>
                            <p>{ "Axum 서버에서 데이터를 불러오는 중입니다..." }</p>
                        </div>
                    } else if todos.is_empty() {
                        <div class="empty-state card">
                            <span class="empty-icon">{ "📦" }</span>
                            <h3>{ "등록된 Todo 항목이 없습니다" }</h3>
                            <p>{ "좌측 입력 폼을 통해 새로운 할 일을 백엔드 서버에 등록해 보세요!" }</p>
                        </div>
                    } else {
                        <div class="todo-cards-grid">
                            {
                                for todos.iter().cloned().map(|item| {
                                    let id = item.id;
                                    html! {
                                        <TodoCard
                                            key={id}
                                            todo={item}
                                            on_toggle={on_toggle.clone()}
                                            on_delete={on_delete.clone()}
                                            on_update_content={on_update_content.clone()}
                                        />
                                    }
                                })
                            }
                        </div>
                    }
                </section>
            </main>

            <footer class="app-bottom">
                <p>{ "Rust WebAssembly Frontend Master Series • Sample 04: Fullstack Axum Integration 🚀" }</p>
            </footer>
        </div>
    }
}
