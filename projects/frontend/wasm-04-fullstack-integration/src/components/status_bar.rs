use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::dto::ConnectionStatus;

#[derive(Properties, PartialEq)]
pub struct StatusBarProps {
    pub status: ConnectionStatus,
    pub api_base: String,
    pub is_loading: bool,
    pub total_count: usize,
    pub on_refresh: Callback<()>,
    pub on_change_api_base: Callback<String>,
}

#[function_component(StatusBar)]
pub fn status_bar(props: &StatusBarProps) -> Html {
    let (badge_class, icon, status_text) = props.status.badge();

    let on_refresh_click = {
        let cb = props.on_refresh.clone();
        Callback::from(move |_| cb.emit(()))
    };

    let on_input_change = {
        let cb = props.on_change_api_base.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                cb.emit(target.value());
            }
        })
    };

    html! {
        <div class="card status-bar-card">
            <div class="status-left">
                <span class={classes!("connection-badge", badge_class)}>
                    <span class="status-dot">{ icon }</span>
                    { status_text }
                </span>
                <span class="count-pill">
                    { format!("동기화된 항목: {}개", props.total_count) }
                </span>
            </div>

            <div class="status-right">
                <div class="api-url-box">
                    <span class="api-label">{ "API 주소:" }</span>
                    <input
                        type="text"
                        class="api-input"
                        value={props.api_base.clone()}
                        oninput={on_input_change}
                        placeholder="/api"
                        title="Axum 백엔드 엔드포인트 기본 경로 (예: /api 또는 http://127.0.0.1:3001/api)"
                    />
                </div>

                <button
                    class="btn btn-refresh"
                    onclick={on_refresh_click}
                    disabled={props.is_loading}
                    title="서버에서 최신 데이터 다시 불러오기"
                >
                    if props.is_loading {
                        <span class="spinner"></span>
                        { " 동기화 중..." }
                    } else {
                        { "🔄 새로고침" }
                    }
                </button>
            </div>
        </div>
    }
}
