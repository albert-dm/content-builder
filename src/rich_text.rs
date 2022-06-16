use serde::{Deserialize, Serialize};
use yew::{function_component, html, Properties};

#[derive(Serialize, Deserialize, Properties, Clone, PartialEq, Debug)]

pub struct RichTextProps {
    pub value: String,
}

#[function_component(RichText)]
pub fn button(RichTextProps { value }: &RichTextProps) -> Html {
    
    html! {
        <div>
            {value}
        </div>
    }
}
