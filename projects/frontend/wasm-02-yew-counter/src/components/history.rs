use yew::prelude::*;
use crate::logic::HistoryItem;

#[derive(Properties, PartialEq)]
pub struct HistoryProps {
    pub items: Vec<HistoryItem>,
    pub on_clear: Callback<()>,
}

#[function_component(HistoryView)]
pub fn history_view(props: &HistoryProps) -> Html {
    let on_clear_click = {
        let cb = props.on_clear.clone();
        Callback::from(move |_| cb.emit(()))
    };

    html! {
        <div class="card history-card">
            <div class="history-header">
                <h3>{ "📜 변경 히스토리" }</h3>
                if !props.items.is_empty() {
                    <button class="btn btn-sm btn-clear" onclick={on_clear_click}>
                        { "기록 비우기" }
                    </button>
                }
            </div>

            if props.items.is_empty() {
                <div class="history-empty">
                    <p>{ "아직 카운터 조작 기록이 없습니다. 버튼을 눌러보세요!" }</p>
                </div>
            } else {
                <div class="history-list">
                    {
                        for props.items.iter().map(|item| {
                            let change_class = if item.after > item.before {
                                "history-tag tag-inc"
                            } else if item.after < item.before {
                                "history-tag tag-dec"
                            } else {
                                "history-tag tag-rst"
                            };

                            html! {
                                <div key={item.id} class="history-item">
                                    <span class={classes!("history-badge", change_class)}>
                                        { &item.description }
                                    </span>
                                    <span class="history-transition">
                                        <span class="val-before">{ item.before }</span>
                                        <span class="val-arrow">{ " ➜ " }</span>
                                        <span class="val-after">{ item.after }</span>
                                    </span>
                                    <span class="history-id">{ format!("#{}", item.id) }</span>
                                </div>
                            }
                        })
                    }
                </div>
            }
        </div>
    }
}
