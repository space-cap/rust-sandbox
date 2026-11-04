use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use crate::dto::CreateTodoDto;

#[derive(Properties, PartialEq)]
pub struct CreateFormProps {
    pub on_create: Callback<CreateTodoDto>,
    pub is_submitting: bool,
}

#[function_component(CreateForm)]
pub fn create_form(props: &CreateFormProps) -> Html {
    let title = use_state(String::new);
    let description = use_state(String::new);

    let on_title_input = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                title.set(target.value());
            }
        })
    };

    let on_desc_input = {
        let description = description.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlTextAreaElement>() {
                description.set(target.value());
            }
        })
    };

    let on_submit = {
        let title = title.clone();
        let description = description.clone();
        let on_create = props.on_create.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let t = (*title).trim().to_string();
            if !t.is_empty() {
                let d = (*description).trim().to_string();
                let desc_opt = if d.is_empty() { None } else { Some(d) };

                on_create.emit(CreateTodoDto {
                    title: t,
                    description: desc_opt,
                });

                title.set(String::new());
                description.set(String::new());
            }
        })
    };

    let is_empty = (*title).trim().is_empty();

    html! {
        <div class="card form-card">
            <h3 class="form-title">{ "📝 새 Todo 등록 (POST /api/todos)" }</h3>
            <form onsubmit={on_submit} class="todo-form">
                <div class="form-group">
                    <label>{ "제목 (Title) *" }</label>
                    <input
                        type="text"
                        class="form-control"
                        placeholder="예: REST API 엔드포인트 통합 테스트 작성"
                        value={(*title).clone()}
                        oninput={on_title_input}
                        required=true
                    />
                </div>

                <div class="form-group">
                    <label>{ "설명 (Description - 선택사항)" }</label>
                    <textarea
                        class="form-control textarea"
                        placeholder="상세 작업 메모 또는 관련 이슈 링크를 입력하세요..."
                        value={(*description).clone()}
                        oninput={on_desc_input}
                        rows="2"
                    />
                </div>

                <div class="form-actions">
                    <button
                        type="submit"
                        class="btn btn-submit"
                        disabled={is_empty || props.is_submitting}
                    >
                        if props.is_submitting {
                            <span class="spinner"></span>
                            { " 서버에 등록 중..." }
                        } else {
                            { "🚀 서버에 저장하기" }
                        }
                    </button>
                </div>
            </form>
        </div>
    }
}
