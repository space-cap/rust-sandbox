use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    ReactiveBench,
    DynamicFilter,
    StaticDeployment,
}

#[component]
pub fn Header(
    active_tab: ReadSignal<ActiveTab>,
    set_active_tab: WriteSignal<ActiveTab>,
) -> impl IntoView {
    view! {
        <header class="app-header">
            <div class="header-brand">
                <div class="logo-box">
                    <span class="logo-icon">"⚡"</span>
                </div>
                <div>
                    <h1 class="header-title">"Leptos 0.7 Showcase & Deploy"</h1>
                    <p class="header-subtitle">
                        "차세대 초고속 세분화 반응성(Fine-Grained Reactivity) & 프로덕션 배포 가이드"
                    </p>
                </div>
            </div>

            <div class="header-meta">
                <span class="meta-badge badge-leptos">"Leptos 0.7.8"</span>
                <span class="meta-badge badge-rust">"Zero Virtual DOM"</span>
                <span class="meta-badge badge-wasm">"Wasm CSR"</span>
            </div>

            <nav class="tab-navigation">
                <button
                    class=move || {
                        if active_tab.get() == ActiveTab::ReactiveBench {
                            "tab-btn active"
                        } else {
                            "tab-btn"
                        }
                    }
                    on:click=move |_| set_active_tab.set(ActiveTab::ReactiveBench)
                >
                    <span class="tab-icon">"⚡"</span>
                    <span class="tab-text">"1. 반응성 & 메모 (Signal)"</span>
                </button>

                <button
                    class=move || {
                        if active_tab.get() == ActiveTab::DynamicFilter {
                            "tab-btn active"
                        } else {
                            "tab-btn"
                        }
                    }
                    on:click=move |_| set_active_tab.set(ActiveTab::DynamicFilter)
                >
                    <span class="tab-icon">"📊"</span>
                    <span class="tab-text">"2. 고성능 리스트 (<For>)"</span>
                </button>

                <button
                    class=move || {
                        if active_tab.get() == ActiveTab::StaticDeployment {
                            "tab-btn active"
                        } else {
                            "tab-btn"
                        }
                    }
                    on:click=move |_| set_active_tab.set(ActiveTab::StaticDeployment)
                >
                    <span class="tab-icon">"🚀"</span>
                    <span class="tab-text">"3. 정적 배포 & CI/CD"</span>
                </button>
            </nav>
        </header>
    }
}

