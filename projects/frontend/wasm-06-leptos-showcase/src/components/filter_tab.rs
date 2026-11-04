use leptos::prelude::*;
use crate::models::{filter_tech_items, TechCategory, TechItem};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    StarsDesc,
    NameAsc,
}

#[component]
pub fn FilterTab() -> impl IntoView {
    // 1. 상태 Signal 정의
    let (items, set_items) = signal(TechItem::sample_data());
    let (search_query, set_search_query) = signal(String::new());
    let (selected_category, set_selected_category) = signal(Option::<TechCategory>::None);
    let (sort_order, set_sort_order) = signal(SortOrder::StarsDesc);

    // 새 아이템 추가 폼 Signal
    let (new_name, set_new_name) = signal(String::new());
    let (new_desc, set_new_desc) = signal(String::new());
    let (new_cat, set_new_cat) = signal(TechCategory::RustWasm);
    let (next_id, set_next_id) = signal(100u32);

    // 2. 파생 계산 (Memo): 필터링 및 정렬이 적용된 최종 목록
    let processed_items = Memo::new(move |_| {
        let current_items = items.get();
        let query = search_query.get();
        let cat = selected_category.get();
        let order = sort_order.get();

        let mut list = filter_tech_items(&current_items, &query, cat);
        match order {
            SortOrder::StarsDesc => list.sort_by(|a, b| b.stars.cmp(&a.stars)),
            SortOrder::NameAsc => list.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
        }
        list
    });

    let total_count = Memo::new(move |_| items.get().len());
    let displayed_count = Memo::new(move |_| processed_items.get().len());

    // 3. 아이템 조작 핸들러
    let handle_star = move |id: u32| {
        set_items.update(|list| {
            if let Some(item) = list.iter_mut().find(|it| it.id == id) {
                item.stars += 100;
            }
        });
    };

    let handle_delete = move |id: u32| {
        set_items.update(|list| {
            list.retain(|it| it.id != id);
        });
    };

    let handle_add = move |e: web_sys::SubmitEvent| {
        e.prevent_default();
        let name = new_name.get().trim().to_string();
        let desc = new_desc.get().trim().to_string();
        if name.is_empty() {
            return;
        }

        let id = next_id.get();
        set_next_id.update(|i| *i += 1);

        let new_item = TechItem::new(
            id,
            name,
            new_cat.get(),
            if desc.is_empty() { "사용자 정의 기술 스택" } else { &desc },
            100,
            vec!["Custom", "UserAdded"],
        );

        set_items.update(|list| list.push(new_item));
        set_new_name.set(String::new());
        set_new_desc.set(String::new());
    };

    view! {
        <div class="tab-content filter-tab">
            <section class="card filter-controls-card">
                <div class="card-header">
                    <h2>"📊 고성능 컬렉션 렌더링 (<For>) & 실시간 필터"</h2>
                    <span class="badge-status">
                        {move || format!("{}/{} 항목 표시 중", displayed_count.get(), total_count.get())}
                    </span>
                </div>
                <p class="card-desc">
                    "Leptos의 " <code>"<For>"</code> " 컴포넌트는 고유 키(Key)를 기반으로 배열이 변경될 때 "
                    "전체 목록을 다시 그리지 않고 오직 추가/수정/삭제된 DOM 노드만 정밀하게 재활용합니다."
                </p>

                <div class="filter-bar">
                    // 검색 입력창
                    <div class="search-input-wrapper">
                        <span class="search-icon">"🔍"</span>
                        <input
                            type="text"
                            class="search-input"
                            placeholder="기술 스택 검색 (예: Leptos, VDOM, SSR)..."
                            prop:value=move || search_query.get()
                            on:input=move |ev| set_search_query.set(event_target_value(&ev))
                        />
                        {move || {
                            if !search_query.get().is_empty() {
                                view! {
                                    <button class="clear-btn" on:click=move |_| set_search_query.set(String::new())>
                                        "✕"
                                    </button>
                                }.into_any()
                            } else {
                                ().into_any()
                            }
                        }}
                    </div>

                    // 정렬 순서 토글
                    <div class="sort-controls">
                        <span class="control-label">"정렬:"</span>
                        <button
                            class=move || if sort_order.get() == SortOrder::StarsDesc { "btn-sort active" } else { "btn-sort" }
                            on:click=move |_| set_sort_order.set(SortOrder::StarsDesc)
                        >
                            "⭐ 추천순"
                        </button>
                        <button
                            class=move || if sort_order.get() == SortOrder::NameAsc { "btn-sort active" } else { "btn-sort" }
                            on:click=move |_| set_sort_order.set(SortOrder::NameAsc)
                        >
                            "🔤 이름순"
                        </button>
                    </div>
                </div>

                // 카테고리 칩 선택
                <div class="category-chips">
                    <span class="control-label">"카테고리:"</span>
                    <button
                        class=move || if selected_category.get().is_none() { "chip active" } else { "chip" }
                        on:click=move |_| set_selected_category.set(None)
                    >
                        "전체 보기"
                    </button>
                    <button
                        class=move || if selected_category.get() == Some(TechCategory::RustWasm) { "chip active" } else { "chip" }
                        on:click=move |_| set_selected_category.set(Some(TechCategory::RustWasm))
                    >
                        "🦀 Rust Wasm"
                    </button>
                    <button
                        class=move || if selected_category.get() == Some(TechCategory::Fullstack) { "chip active" } else { "chip" }
                        on:click=move |_| set_selected_category.set(Some(TechCategory::Fullstack))
                    >
                        "⚡ Fullstack"
                    </button>
                    <button
                        class=move || if selected_category.get() == Some(TechCategory::JavaScript) { "chip active" } else { "chip" }
                        on:click=move |_| set_selected_category.set(Some(TechCategory::JavaScript))
                    >
                        "🌐 JavaScript/TS"
                    </button>
                    <button
                        class=move || if selected_category.get() == Some(TechCategory::Tooling) { "chip active" } else { "chip" }
                        on:click=move |_| set_selected_category.set(Some(TechCategory::Tooling))
                    >
                        "🛠️ Tooling"
                    </button>
                </div>
            </section>

            // 새 항목 추가 아코디언/폼
            <section class="card add-form-card">
                <form on:submit=handle_add class="add-item-form">
                    <div class="form-grid">
                        <input
                            type="text"
                            class="form-input"
                            placeholder="기술 스택 이름 (예: Tokio, Tauri)"
                            prop:value=move || new_name.get()
                            on:input=move |ev| set_new_name.set(event_target_value(&ev))
                        />
                        <input
                            type="text"
                            class="form-input"
                            placeholder="간단한 설명"
                            prop:value=move || new_desc.get()
                            on:input=move |ev| set_new_desc.set(event_target_value(&ev))
                        />
                        <select
                            class="form-select"
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                match val.as_str() {
                                    "RustWasm" => set_new_cat.set(TechCategory::RustWasm),
                                    "Fullstack" => set_new_cat.set(TechCategory::Fullstack),
                                    "JavaScript" => set_new_cat.set(TechCategory::JavaScript),
                                    _ => set_new_cat.set(TechCategory::Tooling),
                                }
                            }
                        >
                            <option value="RustWasm">"🦀 Rust Wasm"</option>
                            <option value="Fullstack">"⚡ Fullstack"</option>
                            <option value="JavaScript">"🌐 JavaScript/TS"</option>
                            <option value="Tooling">"🛠️ Tooling"</option>
                        </select>
                        <button type="submit" class="btn btn-primary">
                            "➕ 항목 추가"
                        </button>
                    </div>
                </form>
            </section>

            // 기술 목록 그리드 (<For> 렌더링)
            <div class="tech-grid">
                <For
                    each=move || processed_items.get()
                    key=|item| item.id
                    let:item
                >
                    {
                        let id = item.id;
                        let category = item.category;
                        view! {
                            <article class="tech-card">
                                <div class="tech-card-header">
                                    <div class="tech-title-wrap">
                                        <h3 class="tech-name">{item.name}</h3>
                                        <span class=format!("tech-category-badge {}", category.color_class())>
                                            {category.as_str()}
                                        </span>
                                    </div>
                                    <button
                                        class="btn-star"
                                        title="+100 Stars 응원하기"
                                        on:click=move |_| handle_star(id)
                                    >
                                        <span class="star-icon">"⭐"</span>
                                        <span class="star-count">{item.stars}</span>
                                    </button>
                                </div>

                                <p class="tech-desc">{item.description}</p>

                                <div class="tech-tags">
                                    {item.tags.iter().map(|tag| {
                                        let tag_text = format!("#{tag}");
                                        view! {
                                            <span class="tech-tag">{tag_text}</span>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>

                                <div class="tech-card-footer">
                                    <span class="tech-id">"ID: #" {item.id}</span>
                                    <button
                                        class="btn-delete"
                                        title="삭제"
                                        on:click=move |_| handle_delete(id)
                                    >
                                        "🗑️ 삭제"
                                    </button>
                                </div>
                            </article>
                        }
                    }
                </For>
            </div>
        </div>
    }
}

