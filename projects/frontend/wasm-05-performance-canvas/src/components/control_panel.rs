use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::particle_system::InteractionMode;

#[derive(Properties, PartialEq)]
pub struct ControlPanelProps {
    pub particle_count: usize,
    pub mode: InteractionMode,
    pub connect_lines: bool,
    pub speed: f64,
    pub fps: u32,
    pub on_change_count: Callback<usize>,
    pub on_change_mode: Callback<InteractionMode>,
    pub on_toggle_lines: Callback<bool>,
    pub on_change_speed: Callback<f64>,
    pub on_burst: Callback<()>,
}

#[function_component(ControlPanel)]
pub fn control_panel(props: &ControlPanelProps) -> Html {
    let modes = [
        InteractionMode::Attract,
        InteractionMode::Repel,
        InteractionMode::Vortex,
        InteractionMode::Floating,
    ];

    let count_presets = [500, 1000, 2000, 4000];

    let on_speed_input = {
        let cb = props.on_change_speed.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                if let Ok(val) = target.value().parse::<f64>() {
                    cb.emit(val);
                }
            }
        })
    };

    let on_toggle_lines_click = {
        let cb = props.on_toggle_lines.clone();
        let next_val = !props.connect_lines;
        Callback::from(move |_| cb.emit(next_val))
    };

    let on_burst_click = {
        let cb = props.on_burst.clone();
        Callback::from(move |_| cb.emit(()))
    };

    let fps_class = if props.fps >= 55 {
        "fps-pill fps-smooth"
    } else if props.fps >= 30 {
        "fps-pill fps-warn"
    } else {
        "fps-pill fps-drop"
    };

    html! {
        <aside class="card control-panel-card">
            <div class="panel-header">
                <h3>{ "⚙️ 시뮬레이션 제어" }</h3>
                <span class={fps_class}>
                    { format!("⚡ {} FPS", props.fps) }
                </span>
            </div>

            <div class="control-section">
                <div class="section-title">
                    <span>{ "파티클 개수" }</span>
                    <strong class="count-val">{ format!("{} 개", props.particle_count) }</strong>
                </div>
                <div class="preset-btn-group">
                    {
                        for count_presets.iter().map(|&cnt| {
                            let is_active = props.particle_count == cnt;
                            let cb = props.on_change_count.clone();
                            let on_click = Callback::from(move |_| cb.emit(cnt));
                            html! {
                                <button
                                    class={classes!("btn", "btn-preset", is_active.then_some("active"))}
                                    onclick={on_click}
                                >
                                    { format!("{}", cnt) }
                                </button>
                            }
                        })
                    }
                </div>
            </div>

            <div class="control-section">
                <div class="section-title">
                    <span>{ "인터랙션 모드 (마우스/터치)" }</span>
                </div>
                <div class="mode-grid">
                    {
                        for modes.iter().map(|&m| {
                            let is_active = props.mode == m;
                            let cb = props.on_change_mode.clone();
                            let on_click = Callback::from(move |_| cb.emit(m));
                            html! {
                                <button
                                    class={classes!("btn", "btn-mode", is_active.then_some("active"))}
                                    onclick={on_click}
                                >
                                    { m.label() }
                                </button>
                            }
                        })
                    }
                </div>
            </div>

            <div class="control-section">
                <div class="section-title">
                    <span>{ "물리 연산 속도" }</span>
                    <span class="speed-val">{ format!("{:.1}x", props.speed) }</span>
                </div>
                <input
                    type="range"
                    class="range-slider"
                    min="0.2"
                    max="3.0"
                    step="0.1"
                    value={props.speed.to_string()}
                    oninput={on_speed_input}
                />
            </div>

            <div class="control-section actions-section">
                <button
                    class={classes!("btn", "btn-toggle", props.connect_lines.then_some("active"))}
                    onclick={on_toggle_lines_click}
                >
                    { if props.connect_lines { "🌐 성좌 연결선: 켜짐" } else { "🌐 성좌 연결선: 꺼짐" } }
                </button>

                <button class="btn btn-burst" onclick={on_burst_click}>
                    { "💥 중앙 폭발 (Burst 100)" }
                </button>
            </div>

            <div class="panel-tip">
                <p>{ "💡 캔버스 위를 마우스로 드래그하거나 클릭하여 실시간 물리 반응을 관찰해 보세요!" }</p>
            </div>
        </aside>
    }
}
