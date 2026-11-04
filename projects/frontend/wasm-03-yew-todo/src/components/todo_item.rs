use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TodoItemProps {
    pub id: u64,
    pub title: String,
    pub completed: bool,
    pub on_toggle: Callback<u64>,
    pub on_remove: Callback<u64>,
    pub on_edit: Callback<(u64, String)>,
}

#[function_component(TodoItem)]
pub fn todo_item(props: &TodoItemProps) -> Html {
    let is_editing = use_state(|| false);
    let edit_value = use_state(|| props.title.clone());

    // 체크박스 클릭 핸들러
    let on_toggle_click = {
        let cb = props.on_toggle.clone();
        let id = props.id;
        Callback::from(move |_| cb.emit(id))
    };

    // 삭제 버튼 클릭 핸들러
    let on_remove_click = {
        let cb = props.on_remove.clone();
        let id = props.id;
        Callback::from(move |_| cb.emit(id))
    };

    // 더블클릭 시 편집 모드 진입
    let on_double_click = {
        let is_editing = is_editing.clone();
        let edit_value = edit_value.clone();
        let current_title = props.title.clone();
        Callback::from(move |_| {
            edit_value.set(current_title.clone());
            is_editing.set(true);
        })
    };

    // 편집 인풋 변경
    let on_edit_input = {
        let edit_value = edit_value.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                edit_value.set(target.value());
            }
        })
    };

    // 편집 완료 저장 로직
    let save_edit = {
        let is_editing = is_editing.clone();
        let edit_value = edit_value.clone();
        let on_edit = props.on_edit.clone();
        let id = props.id;
        Callback::from(move |()| {
            if *is_editing {
                on_edit.emit((id, (*edit_value).clone()));
                is_editing.set(false);
            }
        })
    };

    // 키보드 엔터/ESC 핸들러
    let on_edit_keydown = {
        let save_edit = save_edit.clone();
        let is_editing = is_editing.clone();
        let edit_value = edit_value.clone();
        let original_title = props.title.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                save_edit.emit(());
            } else if e.key() == "Escape" {
                edit_value.set(original_title.clone());
                is_editing.set(false);
            }
        })
    };

    // 포커스 아웃(blur) 시 자동 저장
    let on_edit_blur = {
        let save_edit = save_edit.clone();
        Callback::from(move |_| {
            save_edit.emit(());
        })
    };

    let item_class = classes!(
        "todo-item",
        props.completed.then_some("completed"),
        (*is_editing).then_some("editing"),
    );

    html! {
        <li class={item_class}>
            if *is_editing {
                <div class="edit-wrapper">
                    <input
                        type="text"
                        class="edit-input"
                        value={(*edit_value).clone()}
                        oninput={on_edit_input}
                        onkeydown={on_edit_keydown}
                        onblur={on_edit_blur}
                        autofocus=true
                    />
                    <span class="edit-hint">{ "Enter: 저장 • Esc: 취소" }</span>
                </div>
            } else {
                <div class="view-wrapper">
                    <label class="checkbox-container">
                        <input
                            type="checkbox"
                            checked={props.completed}
                            onclick={on_toggle_click}
                        />
                        <span class="checkmark"></span>
                    </label>

                    <span
                        class="todo-title"
                        ondblclick={on_double_click}
                        title="더블클릭하여 내용 수정"
                    >
                        { &props.title }
                    </span>

                    <div class="actions">
                        <button
                            class="btn-icon btn-edit"
                            onclick={let is_editing = is_editing.clone(); let edit_value = edit_value.clone(); let title = props.title.clone(); Callback::from(move |_| { edit_value.set(title.clone()); is_editing.set(true); })}
                            title="수정하기"
                        >
                            { "✏️" }
                        </button>
                        <button
                            class="btn-icon btn-delete"
                            onclick={on_remove_click}
                            title="삭제하기"
                        >
                            { "🗑️" }
                        </button>
                    </div>
                </div>
            }
        </li>
    }
}

