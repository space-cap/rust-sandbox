use yew::prelude::*;
use crate::components::header::TodoHeader;
use crate::components::todo_filter::TodoFilterBar;
use crate::components::todo_item::TodoItem;
use crate::model::Filter;
use crate::storage::{load_todos, save_todos};

#[function_component(App)]
pub fn app() -> Html {
    let todos = use_state(load_todos);
    let filter = use_state(|| Filter::All);

    // 상태 변경 시 LocalStorage에 영구 저장 (React의 useEffect와 동일)
    {
        let todos = todos.clone();
        use_effect_with((*todos).clone(), move |todos_data| {
            save_todos(todos_data);
        });
    }

    // 1. 할 일 추가
    let on_add = {
        let todos = todos.clone();
        Callback::from(move |title: String| {
            let mut updated = (*todos).clone();
            if updated.add(&title) {
                todos.set(updated);
            }
        })
    };

    // 2. 완료 여부 토글
    let on_toggle = {
        let todos = todos.clone();
        Callback::from(move |id: u64| {
            let mut updated = (*todos).clone();
            updated.toggle(id);
            todos.set(updated);
        })
    };

    // 3. 할 일 삭제
    let on_remove = {
        let todos = todos.clone();
        Callback::from(move |id: u64| {
            let mut updated = (*todos).clone();
            updated.remove(id);
            todos.set(updated);
        })
    };

    // 4. 할 일 내용 수정
    let on_edit = {
        let todos = todos.clone();
        Callback::from(move |(id, new_title): (u64, String)| {
            let mut updated = (*todos).clone();
            if updated.edit(id, &new_title) {
                todos.set(updated);
            }
        })
    };

    // 5. 전체 일괄 토글
    let on_toggle_all = {
        let todos = todos.clone();
        Callback::from(move |completed: bool| {
            let mut updated = (*todos).clone();
            updated.toggle_all(completed);
            todos.set(updated);
        })
    };

    // 6. 완료 항목 일괄 삭제
    let on_clear_completed = {
        let todos = todos.clone();
        Callback::from(move |_| {
            let mut updated = (*todos).clone();
            updated.clear_completed();
            todos.set(updated);
        })
    };

    // 7. 필터 변경
    let on_change_filter = {
        let filter = filter.clone();
        Callback::from(move |new_filter: Filter| {
            filter.set(new_filter);
        })
    };

    let filtered_items = todos.filtered(*filter);
    let total_count = todos.total_count();
    let remaining_count = todos.remaining_count();
    let completed_count = todos.completed_count();
    let all_completed = total_count > 0 && remaining_count == 0;

    html! {
        <div class="todo-app-container">
            <header class="app-masthead">
                <div class="brand-badge">{ "🦀 Phase 3: Yew Todo Application" }</div>
                <h1 class="brand-title">{ "Rust Wasm 투두 매니저" }</h1>
                <p class="brand-subtitle">
                    { "양방향 컴포넌트 통신, 제어 컴포넌트 폼, 그리고 gloo-storage를 통한 브라우저 영구 상태 보존" }
                </p>
            </header>

            <main class="todo-main-section">
                <TodoHeader
                    on_add={on_add}
                    on_toggle_all={on_toggle_all}
                    all_completed={all_completed}
                    has_items={total_count > 0}
                />

                <div class="todo-list-card card">
                    if total_count == 0 {
                        <div class="empty-state">
                            <span class="empty-icon">{ "📋" }</span>
                            <h3>{ "등록된 할 일이 없습니다!" }</h3>
                            <p>{ "위 입력창에 새로운 목표나 작업 항목을 입력해 보세요." }</p>
                        </div>
                    } else if filtered_items.is_empty() {
                        <div class="empty-state">
                            <span class="empty-icon">{ "🔍" }</span>
                            <h3>{ "선택한 필터 조건에 맞는 항목이 없습니다." }</h3>
                            <p>{ "상단 또는 하단 필터(전체/진행 중/완료됨)를 변경해 보세요." }</p>
                        </div>
                    } else {
                        <ul class="todo-list">
                            {
                                for filtered_items.into_iter().map(|todo| {
                                    html! {
                                        <TodoItem
                                            key={todo.id}
                                            id={todo.id}
                                            title={todo.title.clone()}
                                            completed={todo.completed}
                                            on_toggle={on_toggle.clone()}
                                            on_remove={on_remove.clone()}
                                            on_edit={on_edit.clone()}
                                        />
                                    }
                                })
                            }
                        </ul>
                    }
                </div>

                if total_count > 0 {
                    <TodoFilterBar
                        current_filter={*filter}
                        remaining_count={remaining_count}
                        completed_count={completed_count}
                        on_change_filter={on_change_filter}
                        on_clear_completed={on_clear_completed}
                    />
                }

                <div class="guide-card card">
                    <h3>{ "💡 Phase 3 아키텍처 학습 포인트" }</h3>
                    <div class="guide-grid">
                        <div class="guide-col">
                            <strong>{ "1. 양방향 컴포넌트 이벤트 전달" }</strong>
                            <p>{ "부모는 Props로 Callback을 내려주고, 자식은 `cb.emit(data)`로 불변 상태 변경을 요청합니다." }</p>
                        </div>
                        <div class="guide-col">
                            <strong>{ "2. 제어 컴포넌트(Controlled Input)" }</strong>
                            <p>{ "`oninput`과 `onkeydown`으로 실시간 인풋 상태를 Rust 메모리에 동기화하고 Enter 제출을 처리합니다." }</p>
                        </div>
                        <div class="guide-col">
                            <strong>{ "3. gloo-storage 영구 보존" }</strong>
                            <p>{ "`use_effect_with`를 사용해 상태가 갱신될 때마다 자동으로 브라우저 LocalStorage에 JSON 직렬화 저장됩니다." }</p>
                        </div>
                    </div>
                </div>
            </main>

            <footer class="app-bottom">
                <p>{ "Rust WebAssembly Frontend Master Series • Sample 03: Yew Todo App 🚀" }</p>
            </footer>
        </div>
    }
}
