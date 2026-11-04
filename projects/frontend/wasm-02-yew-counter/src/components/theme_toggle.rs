use yew::prelude::*;
use crate::logic::Theme;

#[derive(Properties, PartialEq)]
pub struct ThemeToggleProps {
    pub theme: Theme,
    pub on_toggle: Callback<()>,
}

#[function_component(ThemeToggle)]
pub fn theme_toggle(props: &ThemeToggleProps) -> Html {
    let on_click = {
        let cb = props.on_toggle.clone();
        Callback::from(move |_| cb.emit(()))
    };

    html! {
        <button class="theme-toggle-btn" onclick={on_click}>
            { props.theme.label() }
        </button>
    }
}

