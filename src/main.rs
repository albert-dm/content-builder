mod button;

use button::*;

use serde::{Deserialize, Serialize};
use yew::{html, Callback, Component, Context, Html};

enum Msg {
    SaveContent(&'static str),
    AddButton(&'static str),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(tag = "type")]
enum ComponentMods {
    Button(ButtonProps),
}

// #[derive(Serialize, Deserialize, Debug)]
struct Model {
    components: Vec<ComponentMods>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let data = r#" [
                {
                    type: "Button",
                    props: {
                        label: "Add One"
                    }
                }
            ]"#;

        // Parse the string of data into a Person object. This is exactly the
        // same function as the one that produced serde_json::Value above, but
        // now we are asking it for a Person as output.
        // let components: Vec<ComponentMods> = serde_json::from_str(data).unwrap();
        Self {
            components: vec![ComponentMods::Button(ButtonProps {
                label: String::from("Add one"),
                on_click: Callback::from(|_| ()),
            })],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = _ctx.link();
        match msg {
            Msg::SaveContent(message) => {
                self.components.push(ComponentMods::Button(ButtonProps {
                    label: String::from(message),
                    on_click: link.callback(|_| Self::Message::SaveContent("teste")),
                }));
                // the value has changed so we need to
                // re-render for it to appear on the page

                true
            }
            Msg::AddButton(label) => {
                log::info!("mensagem recebida");
                self.components.push(ComponentMods::Button(ButtonProps {
                    label: String::from(label),
                    on_click: link.callback(|_| Self::Message::SaveContent("teste")),
                }));

                //  let componentsJson = serde_json::to_string(&self.components).unwrap();

                // let desserializedComponents: Vec<ComponentMods> =
                    // serde_json::from_str(&componentsJson).unwrap();

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
            <p>{format!("Total Buttons {}", self.components.iter().len() )}</p>
                {
                    for self.components.iter().map(|c| match c {
                        ComponentMods::Button(ref props) =>
                        {
                            let props = props.clone();
                            html! {
                                <Button label={String::from(props.label)} on_click={link.callback(|_| Self::Message::AddButton("teste"))} />
                            }
                        }
                    })
                }
            </div>

        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
