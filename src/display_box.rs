use serde::{Deserialize, Serialize};
use yew::{function_component, html, Properties};
use crate::builder::*;
use crate::builder_display::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum DisplayBoxFlexDirection {
    Column,
    Row,
    ColumnReverse,
    RowReverse,
}

#[derive(Serialize, Deserialize, Properties, Clone, PartialEq, Debug)]
pub struct DisplayBoxProps {
    #[prop_or(DisplayBoxFlexDirection::Row)]
    pub flex_direction: DisplayBoxFlexDirection,
    pub children: Vec<ComponentMods>,
}

#[function_component(DisplayBox)]
pub fn display_box(
    DisplayBoxProps {
        flex_direction,
        children,
    }: &DisplayBoxProps,
) -> Html {
    let components = children.clone();
    html! {
        <div>
            <BuilderDisplay components={components} />
        </div>
    }
}
