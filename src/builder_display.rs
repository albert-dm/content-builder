use crate::builder::*;
use crate::button::*;
use crate::rich_text::*;
use crate::custom_link::*;
use crate::display_box::*;
use yew::{function_component, html, Callback, Properties};

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct BuilderDisplayProps {
    pub components: Vec<ComponentMods>,
}

#[function_component(BuilderDisplay)]
pub fn button(BuilderDisplayProps { components}: &BuilderDisplayProps) -> Html {
        html! {
            <div>
                {
                    for components.iter().map(|c| match c {
                        ComponentMods::Button(ref props) =>
                        {
                            let props = props.clone();
                            html! {
                                <Button label={String::from(props.label)} on_click={Callback::from(|_| ())} />
                            }
                        }
                        ComponentMods::RichText(ref props) => 
                        {
                            let props = props.clone();
                            html! {
                                <RichText value={String::from(props.value)} />
                            }
                        }
                        ComponentMods::CustomLink(ref props) => 
                        {
                            let props = props.clone();
                            html! {
                                <CustomLink ..props />
                            }
                        }
                        ComponentMods::DisplayBox(ref props) => 
                        {
                            let props = props.clone();
                            html! {
                                <DisplayBox ..props />
                            }
                        }
                    })
                }
            </div>

        }
}
