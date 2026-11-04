use leptos::prelude::*;

pub mod components;
pub mod models;

use components::counter_tab::CounterTab;
use components::deployment_tab::DeploymentTab;
use components::filter_tab::FilterTab;
use components::header::{ActiveTab, Header};

#[component]
pub fn App() -> impl IntoView {
    let (active_tab, set_active_tab) = signal(ActiveTab::ReactiveBench);

    view! {
        <div class="app-container">
            // 헤더 및 탭 네비게이션
            <Header active_tab=active_tab set_active_tab=set_active_tab />

            // 본문 탭 영역 (반응형 뷰 전환)
            <main class="main-body">
                {move || match active_tab.get() {
                    ActiveTab::ReactiveBench => view! { <CounterTab /> }.into_any(),
                    ActiveTab::DynamicFilter => view! { <FilterTab /> }.into_any(),
                    ActiveTab::StaticDeployment => view! { <DeploymentTab /> }.into_any(),
                }}
            </main>

            // 푸터: Phase 1 ~ 6 로드맵 완주 기념 바
            <footer class="app-footer">
                <div class="footer-inner">
                    <div class="footer-brand">
                        <span class="rust-logo">"🦀"</span>
                        <span>"Rust WebAssembly Master Curriculum"</span>
                    </div>
                    <div class="footer-badges">
                        <span class="f-badge">"Phase 1: DOM & Trunk ✅"</span>
                        <span class="f-badge">"Phase 2: Yew Counter ✅"</span>
                        <span class="f-badge">"Phase 3: Yew Todo ✅"</span>
                        <span class="f-badge">"Phase 4: Axum Fullstack ✅"</span>
                        <span class="f-badge">"Phase 5: Canvas 2D 60FPS ✅"</span>
                        <span class="f-badge badge-active">"Phase 6: Leptos & Deploy 🏆"</span>
                    </div>
                    <p class="footer-copy">
                        "Built with Rust & Leptos 0.7 | Fine-Grained Reactivity with Zero Virtual DOM"
                    </p>
                </div>
            </footer>
        </div>
    }
}
