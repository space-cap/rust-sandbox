use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CounterProps {
    pub count: i32,
    pub step: i32,
    pub badge_class: &'static str,
    pub badge_label: &'static str,
    pub on_increment: Callback<()>,
    pub on_decrement: Callback<()>,
    pub on_reset: Callback<()>,
    pub on_set_step: Callback<i32>,
}

#[function_component(Counter)]
pub fn counter(props: &CounterProps) -> Html {
    let on_inc = {
        let cb = props.on_increment.clone();
        Callback::from(move |_| cb.emit(()))
    };

    let on_dec = {
        let cb = props.on_decrement.clone();
        Callback::from(move |_| cb.emit(()))
    };

    let on_rst = {
        let cb = props.on_reset.clone();
        Callback::from(move |_| cb.emit(()))
    };

    let count_class = if props.count > 0 {
        "counter-value text-positive"
    } else if props.count < 0 {
        "counter-value text-negative"
    } else {
        "counter-value text-zero"
    };

    let step_options = [1, 5, 10, 50];

    html! {
        <div class="card counter-card">
            <div class="counter-header">
                <span class={classes!("status-pill", props.badge_class)}>
                    { props.badge_label }
                </span>
                <span class="step-indicator">
                    { format!("현재 증감 단위: ±{}", props.step) }
                </span>
            </div>

            <div class="display-area">
                <div class={count_class}>
                    { props.count }
                </div>
            </div>

            <div class="controls-area">
                <div class="main-buttons">
                    <button class="btn btn-action btn-decrement" onclick={on_dec}>
                        { format!("- {}", props.step) }
                    </button>
                    <button class="btn btn-action btn-reset" onclick={on_rst} disabled={props.count == 0}>
                        { "초기화" }
                    </button>
                    <button class="btn btn-action btn-increment" onclick={on_inc}>
                        { format!("+ {}", props.step) }
                    </button>
                </div>

                <div class="step-selector">
                    <span class="step-label">{ "증감 단위(Step) 선택:" }</span>
                    <div class="step-btn-group">
                        {
                            for step_options.iter().map(|&s| {
                                let is_active = props.step == s;
                                let cb = props.on_set_step.clone();
                                let on_click = Callback::from(move |_| cb.emit(s));
                                html! {
                                    <button
                                        class={classes!("btn", "btn-step", is_active.then_some("active"))}
                                        onclick={on_click}
                                    >
                                        { format!("±{}", s) }
                                    </button>
                                }
                            })
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}
