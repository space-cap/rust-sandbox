use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ErrorBannerProps {
    pub error: Option<String>,
    pub on_dismiss: Callback<()>,
}

#[function_component(ErrorBanner)]
pub fn error_banner(props: &ErrorBannerProps) -> Html {
    if let Some(msg) = &props.error {
        let on_click = {
            let cb = props.on_dismiss.clone();
            Callback::from(move |_| cb.emit(()))
        };

        html! {
            <div class="error-banner">
                <div class="error-content">
                    <span class="error-icon">{ "⚠️" }</span>
                    <span class="error-text">{ msg }</span>
                </div>
                <button class="btn-dismiss" onclick={on_click} title="닫기">
                    { "✕" }
                </button>
            </div>
        }
    } else {
        html! {}
    }
}
