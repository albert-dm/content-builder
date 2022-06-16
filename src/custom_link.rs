use serde::{Deserialize, Serialize};
use std::fmt;
use yew::{function_component, html, Properties, virtual_dom::AttrValue};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum CustomLinkDisplayType {
    OnlyText,
    Button,
    Hiperlink,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum CustomLinkTarget {
    Blank,
    Current,
}

impl fmt::Display for CustomLinkTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomLinkTarget::Blank => write!(f, "_blank"),
            CustomLinkTarget::Current => write!(f, "_self"),
        }
    }
}

#[derive(Serialize, Deserialize, Properties, Clone, PartialEq, Debug)]
pub struct CustomLinkProps {
    pub url: String,
    pub display_type: CustomLinkDisplayType,
    #[prop_or(CustomLinkTarget::Blank)]
    pub target: CustomLinkTarget,
    pub label: String,
}

#[function_component(CustomLink)]
pub fn custom_link(
    CustomLinkProps {
        url,
        display_type,
        target,
        label,
    }: &CustomLinkProps,
) -> Html {
    html! {
        <a href={AttrValue::from(url.clone())} target={AttrValue::from(target.to_string())}>
            {label}
        </a>
    }
}
