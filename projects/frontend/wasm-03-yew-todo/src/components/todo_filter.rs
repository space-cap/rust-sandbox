use yew::prelude::*;
use crate::model::Filter;

#[derive(Properties, PartialEq)]
pub struct FilterProps {
    pub current_filter: Filter,
    pub remaining_count: usize,
    pub completed_count: usize,
    pub on_change_filter: Callback<Filter>,
    pub on_clear_completed: Callback<()>,
}

#[function_component(TodoFilterBar)]
pub fn todo_filter_bar(props: &FilterProps) -> Html {
    let filters = [Filter::All, Filter::Active, Filter::Completed];

    let on_clear = {
        let cb = props.on_clear_completed.clone();
        Callback::from(move |_| cb.emit(()))
    };

    html! {
        <footer class="filter-footer card">
            <span class="remaining-counter">
                <strong>{ props.remaining_count }</strong>
                { " 개의 미완료 항목" }
            </span>

            <div class="filter-btn-group">
                {
                    for filters.iter().map(|&f| {
                        let is_active = props.current_filter == f;
                        let cb = props.on_change_filter.clone();
                        let on_click = Callback::from(move |_| cb.emit(f));

                        html! {
                            <button
                                class={classes!("filter-btn", is_active.then_some("active"))}
                                onclick={on_click}
                            >
                                { f.label() }
                            </button>
                        }
                    })
                }
            </div>

            <div class="clear-wrapper">
                if props.completed_count > 0 {
                    <button class="btn-clear-completed" onclick={on_clear}>
                        { format!("완료 항목 삭제 ({}개)", props.completed_count) }
                    </button>
                }
            </div>
        </footer>
    }
}
