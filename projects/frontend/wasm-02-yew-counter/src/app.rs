use yew::prelude::*;
use crate::components::counter::Counter;
use crate::components::history::HistoryView;
use crate::components::theme_toggle::ThemeToggle;
use crate::logic::{CounterState, Theme};

#[function_component(App)]
pub fn app() -> Html {
    let counter_state = use_state(CounterState::new);
    let theme_state = use_state(|| Theme::Dark);

    // 테마 토글 콜백
    let on_toggle_theme = {
        let theme_state = theme_state.clone();
        Callback::from(move |_| {
            theme_state.set(theme_state.toggle());
        })
    };

    // 카운터 증가
    let on_increment = {
        let counter_state = counter_state.clone();
        Callback::from(move |_| {
            let mut state = (*counter_state).clone();
            state.increment();
            counter_state.set(state);
        })
    };

    // 카운터 감소
    let on_decrement = {
        let counter_state = counter_state.clone();
        Callback::from(move |_| {
            let mut state = (*counter_state).clone();
            state.decrement();
            counter_state.set(state);
        })
    };

    // 카운터 초기화
    let on_reset = {
        let counter_state = counter_state.clone();
        Callback::from(move |_| {
            let mut state = (*counter_state).clone();
            state.reset();
            counter_state.set(state);
        })
    };

    // 증감 단위(Step) 변경
    let on_set_step = {
        let counter_state = counter_state.clone();
        Callback::from(move |step: i32| {
            let mut state = (*counter_state).clone();
            state.set_step(step);
            counter_state.set(state);
        })
    };

    // 히스토리 초기화
    let on_clear_history = {
        let counter_state = counter_state.clone();
        Callback::from(move |_| {
            let mut state = (*counter_state).clone();
            state.clear_history();
            counter_state.set(state);
        })
    };

    let (badge_class, badge_label) = counter_state.status_badge();

    html! {
        <div class={classes!("app-container", theme_state.class_name())}>
            <header class="app-header">
                <div class="header-left">
                    <div class="badge-tech">{ "🦀 Phase 2: Yew Framework (v0.21)" }</div>
                    <h1>{ "React 개발자를 위한 Yew 카운터 & 상태 관리" }</h1>
                    <p class="header-desc">
                        { "가상 DOM(V-DOM), hooks(use_state), 그리고 html! 매크로를 이용한 모던 Rust 프론트엔드 실습" }
                    </p>
                </div>
                <div class="header-right">
                    <ThemeToggle
                        theme={*theme_state}
                        on_toggle={on_toggle_theme}
                    />
                </div>
            </header>

            <main class="app-main">
                <div class="grid-layout">
                    <div class="column-left">
                        <Counter
                            count={counter_state.count}
                            step={counter_state.step}
                            badge_class={badge_class}
                            badge_label={badge_label}
                            on_increment={on_increment}
                            on_decrement={on_decrement}
                            on_reset={on_reset}
                            on_set_step={on_set_step}
                        />

                        <div class="card comparison-card">
                            <h3>{ "💡 React vs Yew 1:1 비교 팁" }</h3>
                            <ul class="comparison-list">
                                <li>
                                    <strong>{ "JSX vs html!" }</strong>
                                    <code>{ "<div className=\"card\">" }</code> { " ➜ " } <code>{ "html! { <div class=\"card\"> }" }</code>
                                </li>
                                <li>
                                    <strong>{ "useState vs use_state" }</strong>
                                    <code>{ "const [c, setC] = useState(0)" }</code> { " ➜ " } <code>{ "let c = use_state(|| 0)" }</code>
                                </li>
                                <li>
                                    <strong>{ "onClick vs onclick" }</strong>
                                    <code>{ "onClick={() => setC(c + 1)}" }</code> { " ➜ " } <code>{ "onclick={Callback::from(...)}" }</code>
                                </li>
                            </ul>
                        </div>
                    </div>

                    <div class="column-right">
                        <HistoryView
                            items={counter_state.history.clone()}
                            on_clear={on_clear_history}
                        />
                    </div>
                </div>
            </main>

            <footer class="app-footer">
                <p>{ "Rust WebAssembly Frontend Master Series • Built with Trunk & Yew 🚀" }</p>
            </footer>
        </div>
    }
}
