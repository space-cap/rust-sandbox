use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub on_add: Callback<String>,
    pub on_toggle_all: Callback<bool>,
    pub all_completed: bool,
    pub has_items: bool,
}

#[function_component(TodoHeader)]
pub fn todo_header(props: &HeaderProps) -> Html {
    let input_value = use_state(String::new);

    // 입력값 변경 핸들러
    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                input_value.set(target.value());
            }
        })
    };

    // 추가 실행 공통 함수
    let submit_todo = {
        let input_value = input_value.clone();
        let on_add = props.on_add.clone();
        Callback::from(move |()| {
            let val = (*input_value).trim().to_string();
            if !val.is_empty() {
                on_add.emit(val);
                input_value.set(String::new());
            }
        })
    };

    // 엔터 키 입력 시 추가
    let on_keydown = {
        let submit_todo = submit_todo.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                submit_todo.emit(());
            }
        })
    };

    // 버튼 클릭 시 추가
    let on_click_add = {
        let submit_todo = submit_todo.clone();
        Callback::from(move |_| {
            submit_todo.emit(());
        })
    };

    // 전체 선택/해제 토글
    let on_click_toggle_all = {
        let on_toggle_all = props.on_toggle_all.clone();
        let next_val = !props.all_completed;
        Callback::from(move |_| {
            on_toggle_all.emit(next_val);
        })
    };

    html! {
        <header class="todo-header-card card">
            <div class="header-top">
                <span class="badge-tag">{ "Phase 3 • Yew Todo WebApp" }</span>
                <span class="storage-indicator">{ "💾 LocalStorage 자동 저장 활성화" }</span>
            </div>

            <div class="input-container">
                if props.has_items {
                    <button
                        class={classes!("btn-toggle-all", props.all_completed.then_some("checked"))}
                        onclick={on_click_toggle_all}
                        title={if props.all_completed { "전체 완료 해제" } else { "모두 완료 처리" }}
                    >
                        { "✔" }
                    </button>
                }
                <input
                    type="text"
                    class="todo-input"
                    placeholder="오늘 어떤 작업을 완료할 예정인가요? (Enter로 추가)"
                    value={(*input_value).clone()}
                    oninput={on_input}
                    onkeydown={on_keydown}
                    autofocus=true
                />
                <button
                    class="btn btn-add"
                    onclick={on_click_add}
                    disabled={(*input_value).trim().is_empty()}
                >
                    { "추가" }
                </button>
            </div>
        </header>
    }
}
