use serde::{Deserialize, Serialize};
use yew::{function_component, html, Callback, Properties};

#[derive(Serialize, Deserialize, Properties, Clone, PartialEq, Debug)]
pub struct ButtonProps {
    pub label: String,
    #[serde(skip)]
    pub on_click: Callback<()>,
}

#[function_component(Button)]
pub fn button(ButtonProps { label, on_click }: &ButtonProps) -> Html {
    let on_click = on_click.clone();
    let on_buttonn_click = {
        let on_click = on_click.clone();
        Callback::from(move |_| on_click.emit(()))
    };
    
    html! {
        <button onclick={on_buttonn_click}>
            {label}
        </button>
    }
}
