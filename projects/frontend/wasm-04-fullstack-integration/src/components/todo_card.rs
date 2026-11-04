use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use crate::dto::Todo;

#[derive(Properties, PartialEq)]
pub struct TodoCardProps {
    pub todo: Todo,
    pub on_toggle: Callback<(u64, bool)>,
    pub on_delete: Callback<u64>,
    pub on_update_content: Callback<(u64, String, Option<String>)>,
}

#[function_component(TodoCard)]
pub fn todo_card(props: &TodoCardProps) -> Html {
    let is_editing = use_state(|| false);
    let edit_title = use_state(|| props.todo.title.clone());
    let edit_desc = use_state(|| props.todo.description.clone().unwrap_or_default());

    let todo = &props.todo;

    // 완료 여부 토글 핸들러
    let on_toggle_click = {
        let cb = props.on_toggle.clone();
        let id = todo.id;
        let next_status = !todo.completed;
        Callback::from(move |_| cb.emit((id, next_status)))
    };

    // 삭제 핸들러
    let on_delete_click = {
        let cb = props.on_delete.clone();
        let id = todo.id;
        Callback::from(move |_| cb.emit(id))
    };

    // 편집 시작
    let on_start_edit = {
        let is_editing = is_editing.clone();
        let edit_title = edit_title.clone();
        let edit_desc = edit_desc.clone();
        let current_t = todo.title.clone();
        let current_d = todo.description.clone().unwrap_or_default();

        Callback::from(move |_| {
            edit_title.set(current_t.clone());
            edit_desc.set(current_d.clone());
            is_editing.set(true);
        })
    };

    // 편집 취소
    let on_cancel_edit = {
        let is_editing = is_editing.clone();
        Callback::from(move |_| is_editing.set(false))
    };

    // 편집 내용 저장
    let on_save_edit = {
        let is_editing = is_editing.clone();
        let edit_title = edit_title.clone();
        let edit_desc = edit_desc.clone();
        let cb = props.on_update_content.clone();
        let id = todo.id;

        Callback::from(move |_| {
            let t = (*edit_title).trim().to_string();
            if !t.is_empty() {
                let d = (*edit_desc).trim().to_string();
                let desc_opt = if d.is_empty() { None } else { Some(d) };
                cb.emit((id, t, desc_opt));
                is_editing.set(false);
            }
        })
    };

    let on_title_input = {
        let edit_title = edit_title.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                edit_title.set(target.value());
            }
        })
    };

    let on_desc_input = {
        let edit_desc = edit_desc.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlTextAreaElement>() {
                edit_desc.set(target.value());
            }
        })
    };

    let card_class = classes!(
        "card",
        "todo-item-card",
        todo.completed.then_some("is-completed")
    );

    html! {
        <div class={card_class}>
            if *is_editing {
                <div class="edit-panel">
                    <div class="form-group">
                        <label>{ "수정할 제목" }</label>
                        <input
                            type="text"
                            class="form-control"
                            value={(*edit_title).clone()}
                            oninput={on_title_input}
                        />
                    </div>
                    <div class="form-group">
                        <label>{ "수정할 설명" }</label>
                        <textarea
                            class="form-control textarea"
                            value={(*edit_desc).clone()}
                            oninput={on_desc_input}
                            rows="2"
                        />
                    </div>
                    <div class="edit-actions">
                        <button class="btn btn-sm btn-save" onclick={on_save_edit}>
                            { "💾 변경 저장" }
                        </button>
                        <button class="btn btn-sm btn-cancel" onclick={on_cancel_edit}>
                            { "취소" }
                        </button>
                    </div>
                </div>
            } else {
                <div class="card-content-view">
                    <div class="card-top">
                        <div class="id-and-status">
                            <span class="id-tag">{ format!("#{}", todo.id) }</span>
                            if todo.completed {
                                <span class="status-tag status-done">{ "✔ 완료됨" }</span>
                            } else {
                                <span class="status-tag status-active">{ "진행 중" }</span>
                            }
                        </div>
                        <div class="card-actions">
                            <button
                                class="btn-icon"
                                onclick={on_toggle_click}
                                title={if todo.completed { "진행 중으로 변경 (PUT)" } else { "완료 처리 (PUT)" }}
                            >
                                if todo.completed { { "↩" } } else { { "✔" } }
                            </button>
                            <button
                                class="btn-icon"
                                onclick={on_start_edit}
                                title="내용 수정하기"
                            >
                                { "✏️" }
                            </button>
                            <button
                                class="btn-icon btn-danger"
                                onclick={on_delete_click}
                                title="삭제하기 (DELETE)"
                            >
                                { "🗑️" }
                            </button>
                        </div>
                    </div>

                    <h4 class="todo-title">{ &todo.title }</h4>

                    if let Some(desc) = &todo.description {
                        if !desc.trim().is_empty() {
                            <p class="todo-desc">{ desc }</p>
                        }
                    }
                </div>
            }
        </div>
    }
}

