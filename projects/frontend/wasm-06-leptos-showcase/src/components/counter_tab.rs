use leptos::prelude::*;
use crate::models::{compute_doubled, compute_squared, is_even};

#[component]
pub fn CounterTab() -> impl IntoView {
    // 1. 핵심 반응형 원자 (Signal)
    let (count, set_count) = signal(0);
    let (step, set_step) = signal(1);
    let (bench_result, set_bench_result) = signal(Option::<String>::None);
    let (update_count, set_update_count) = signal(0);

    // 2. 파생 상태 (Memo): count가 변경될 때만 캐시를 갱신
    let doubled = Memo::new(move |_| compute_doubled(count.get()));
    let squared = Memo::new(move |_| compute_squared(count.get()));
    let parity = Memo::new(move |_| {
        if is_even(count.get()) {
            "짝수 (Even) 🟢"
        } else {
            "홀수 (Odd) 🟡"
        }
    });

    // 3. 고속 벤치마크 함수
    let run_benchmark = move |_| {
        if let Some(window) = web_sys::window() {
            if let Some(perf) = window.performance() {
                let start = perf.now();
                // 1,000회 연속 상태 갱신
                for _ in 0..1000 {
                    set_count.update(|c| *c += 1);
                }
                let elapsed = perf.now() - start;
                set_update_count.update(|u| *u += 1000);
                set_bench_result.set(Some(format!(
                    "1,000회 연속 Signal 업데이트 완료: {:.3} ms (초당 약 {:.0} 회 연산 속도)",
                    elapsed,
                    1_000.0 / (elapsed / 1000.0)
                )));
            }
        }
    };

    let handle_increment = move |_| {
        set_count.update(|c| *c += step.get());
        set_update_count.update(|u| *u += 1);
    };

    let handle_decrement = move |_| {
        set_count.update(|c| *c -= step.get());
        set_update_count.update(|u| *u += 1);
    };

    let handle_reset = move |_| {
        set_count.set(0);
        set_update_count.update(|u| *u += 1);
        set_bench_result.set(None);
    };

    view! {
        <div class="tab-content counter-tab">
            <section class="card hero-counter-card">
                <div class="card-header">
                    <h2>"⚡ 세분화된 반응성 (Fine-Grained Signal) 실험실"</h2>
                    <span class="badge-status">"Leptos Reactive Graph"</span>
                </div>
                <p class="card-desc">
                    "버튼을 클릭하면 전체 컴포넌트가 다시 렌더링되지 않고, 바인딩된 DOM 텍스트 노드만 "
                    <strong>"즉시(Direct Mutation)"</strong> " 업데이트됩니다."
                </p>

                <div class="counter-display-box">
                    <div class="counter-label">"CURRENT SIGNAL VALUE"</div>
                    <div class="counter-value">{count}</div>
                    <div class="counter-stats">
                        <span class="stat-pill">
                            "누적 Signal 발생: " <strong>{update_count}</strong> " 회"
                        </span>
                        <span class="stat-pill">
                            "증감 간격: " <strong>"±" {step}</strong>
                        </span>
                    </div>
                </div>

                <div class="counter-controls">
                    <button class="btn btn-secondary" on:click=handle_decrement>
                        "− " {step}
                    </button>
                    <button class="btn btn-primary" on:click=handle_increment>
                        "+ " {step}
                    </button>
                    <button class="btn btn-outline" on:click=handle_reset>
                        "🔄 초기화"
                    </button>
                </div>

                <div class="step-selector">
                    <span class="step-label">"증감 단위 선택:"</span>
                    <button
                        class=move || if step.get() == 1 { "step-btn active" } else { "step-btn" }
                        on:click=move |_| set_step.set(1)
                    >
                        "1"
                    </button>
                    <button
                        class=move || if step.get() == 5 { "step-btn active" } else { "step-btn" }
                        on:click=move |_| set_step.set(5)
                    >
                        "5"
                    </button>
                    <button
                        class=move || if step.get() == 10 { "step-btn active" } else { "step-btn" }
                        on:click=move |_| set_step.set(10)
                    >
                        "10"
                    </button>
                    <button
                        class=move || if step.get() == 50 { "step-btn active" } else { "step-btn" }
                        on:click=move |_| set_step.set(50)
                    >
                        "50"
                    </button>
                </div>
            </section>

            <div class="grid-2-col">
                // 파생 상태 (Memo) 카드
                <section class="card memo-card">
                    <div class="card-header">
                        <h3>"🧠 파생 계산 캐싱 (Memo::new)"</h3>
                        <span class="badge-mini">"Cached Computation"</span>
                    </div>
                    <p class="card-desc">
                        "의존하는 Signal이 바뀔 때만 재연산되는 스마트 파생 상태입니다."
                    </p>
                    <div class="memo-list">
                        <div class="memo-item">
                            <span class="memo-key">"2배수 (count * 2):"</span>
                            <span class="memo-val val-blue">{doubled}</span>
                        </div>
                        <div class="memo-item">
                            <span class="memo-key">"제곱수 (count²):"</span>
                            <span class="memo-val val-purple">{squared}</span>
                        </div>
                        <div class="memo-item">
                            <span class="memo-key">"홀짝 판별:"</span>
                            <span class="memo-val val-green">{parity}</span>
                        </div>
                    </div>
                </section>

                // 고빈도 부하 테스트 카드
                <section class="card bench-card">
                    <div class="card-header">
                        <h3>"🚀 초고속 1,000회 연속 틱 벤치마크"</h3>
                        <span class="badge-mini">"Performance Test"</span>
                    </div>
                    <p class="card-desc">
                        "1,000번의 Signal 업데이트를 루프로 실행하여 Wasm 런타임의 극한 속도를 측정합니다."
                    </p>
                    <button class="btn btn-accent btn-block" on:click=run_benchmark>
                        "⚡ 1,000회 연속 갱신 실행"
                    </button>

                    {move || {
                        bench_result.get().map(|res| {
                            view! {
                                <div class="bench-result-box">
                                    <span class="bench-icon">"⏱️"</span>
                                    <span class="bench-text">{res}</span>
                                </div>
                            }
                        })
                    }}
                </section>
            </div>

            // 아키텍처 비교 섹션
            <section class="card architecture-card">
                <div class="card-header">
                    <h3>"🏛️ 아키텍처 혁신: Yew(Virtual DOM) vs Leptos(Signal)"</h3>
                </div>
                <div class="arch-compare-grid">
                    <div class="arch-box arch-vdom">
                        <div class="arch-tag">"기존 방식 (Yew / React)"</div>
                        <h4>"가상 DOM & 트리 Diffing"</h4>
                        <ul>
                            <li>"상태 변경 시 컴포넌트 함수를 매번 전체 재실행"</li>
                            <li>"메모리에 새 Virtual DOM 트리를 매 프레임 할당"</li>
                            <li>"이전 VDOM과 새 VDOM을 재귀적으로 비교(Diffing)"</li>
                            <li>"차이점(Diff)을 모아서 실제 브라우저 DOM에 패치"</li>
                        </ul>
                    </div>

                    <div class="arch-arrow">
                        <span>"진화 ➡️"</span>
                    </div>

                    <div class="arch-box arch-signal">
                        <div class="arch-tag tag-highlight">"차세대 혁신 (Leptos / SolidJS)"</div>
                        <h4>"세분화된 반응성 & 직접 DOM 바인딩"</h4>
                        <ul>
                            <li>"컴포넌트 함수는 최초 마운트 시 <strong>단 1번만 실행</strong>"</li>
                            <li>"가상 DOM 없음(Zero VDOM) → 메모리 할당 및 GC 오버헤드 0"</li>
                            <li>"Signal 클로저가 실제 DOM 텍스트 노드에 직접 결합"</li>
                            <li>"상태가 변하면 그 노드만 <strong>핀포인트(Pinpoint)</strong>로 즉시 갱신"</li>
                        </ul>
                    </div>
                </div>
            </section>
        </div>
    }
}

