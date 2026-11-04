use yew::prelude::*;

use crate::components::canvas_view::CanvasView;
use crate::components::control_panel::ControlPanel;
use crate::particle_system::{InteractionMode, ParticleUniverse};

#[function_component(App)]
pub fn app() -> Html {
    // 캔버스 물리 세계 인스턴스를 단일 소유하는 Rc<RefCell<...>>
    let universe = use_mut_ref(|| ParticleUniverse::new(850.0, 580.0, 1000));

    let particle_count = use_state(|| 1000usize);
    let mode = use_state(|| InteractionMode::Attract);
    let connect_lines = use_state(|| true);
    let speed = use_state(|| 1.0f64);
    let fps = use_state(|| 60u32);

    // 파티클 수 변경
    let on_change_count = {
        let universe = universe.clone();
        let particle_count = particle_count.clone();
        Callback::from(move |count: usize| {
            universe.borrow_mut().set_particle_count(count);
            particle_count.set(count);
        })
    };

    // 인터랙션 모드 변경
    let on_change_mode = {
        let universe = universe.clone();
        let mode = mode.clone();
        Callback::from(move |m: InteractionMode| {
            universe.borrow_mut().set_mode(m);
            mode.set(m);
        })
    };

    // 성좌 연결선 토글
    let on_toggle_lines = {
        let universe = universe.clone();
        let connect_lines = connect_lines.clone();
        Callback::from(move |enabled: bool| {
            universe.borrow_mut().set_connect_lines(enabled);
            connect_lines.set(enabled);
        })
    };

    // 속도 변경
    let on_change_speed = {
        let universe = universe.clone();
        let speed = speed.clone();
        Callback::from(move |s: f64| {
            universe.borrow_mut().set_speed(s);
            speed.set(s);
        })
    };

    // 중앙 폭발 액션
    let on_burst = {
        let universe = universe.clone();
        Callback::from(move |()| {
            let mut u = universe.borrow_mut();
            let cx = u.width / 2.0;
            let cy = u.height / 2.0;
            u.burst(cx, cy, 100);
        })
    };

    // FPS 갱신
    let on_fps_update = {
        let fps = fps.clone();
        Callback::from(move |val: u32| {
            fps.set(val);
        })
    };

    html! {
        <div class="canvas-app-container">
            <header class="app-masthead">
                <div class="brand-badge">{ "🦀 Phase 5: High-Performance Canvas 2D" }</div>
                <h1 class="brand-title">{ "Rust WebAssembly 고성능 파티클 시뮬레이터" }</h1>
                <p class="brand-subtitle">
                    { "자바스크립트의 가비지 컬렉터(GC) 오버헤드 없이 수천 개의 물리 입자를 초당 60프레임으로 실시간 연산" }
                </p>
            </header>

            <main class="simulation-layout">
                <section class="canvas-container-section">
                    <CanvasView
                        universe={universe.clone()}
                        on_fps_update={on_fps_update}
                    />
                </section>

                <section class="controls-sidebar-section">
                    <ControlPanel
                        particle_count={*particle_count}
                        mode={*mode}
                        connect_lines={*connect_lines}
                        speed={*speed}
                        fps={*fps}
                        on_change_count={on_change_count}
                        on_change_mode={on_change_mode}
                        on_toggle_lines={on_toggle_lines}
                        on_change_speed={on_change_speed}
                        on_burst={on_burst}
                    />
                </section>
            </main>

            <section class="benchmark-info card">
                <h3>{ "⚡ 왜 WebAssembly가 그래픽/물리 연산에서 JavaScript보다 압도적인가?" }</h3>
                <div class="benchmark-grid">
                    <div class="bench-col">
                        <strong>{ "1. 예측 가능한 프레임 타임 (No Garbage Collection)" }</strong>
                        <p>{ "JS는 매 프레임 수천 개의 객체를 생성/파괴할 때 GC가 작동하여 뚝뚝 끊기는 스터터링(Stuttering)이 발생합니다. Rust Wasm은 연속된 메모리 버퍼(Vec)를 재사용하므로 GC 멈춤 현상이 원천 차단됩니다." }</p>
                    </div>
                    <div class="bench-col">
                        <strong>{ "2. SIMD & 네이티브 수치 연산 속도" }</strong>
                        <p>{ "삼각함수(sin, cos), 거리 제곱근(sqrt), 벡터 합산과 같은 부동소수점 연산이 브라우저 JIT 인터프리터를 거치지 않고 CPU 네이티브 기계어 명령어로 즉시 실행됩니다." }</p>
                    </div>
                    <div class="bench-col">
                        <strong>{ "3. 60FPS 논블로킹 렌더링 루프" }</strong>
                        <p>{ "`request_animation_frame`을 통해 브라우저 수직 동기화(V-Sync) 주기에 정확히 동기화되어 화면 주사율에 맞춘 부드러운 움직임을 보장합니다." }</p>
                    </div>
                </div>
            </section>

            <footer class="app-bottom">
                <p>{ "Rust WebAssembly Frontend Master Series • Sample 05: High-Performance Canvas 2D 🚀" }</p>
            </footer>
        </div>
    }
}
