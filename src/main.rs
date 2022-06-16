mod button;
mod rich_text;
mod builder;
mod builder_display;
mod tool_bar;
mod custom_link;
mod display_box;

use button::*;
use builder::*;
use builder_display::*;
use tool_bar::*;
use yew::{html, Component, Context, Html};

enum Msg {
    SaveContent(&'static str),
    AddButton(&'static str),
}

struct Model {
    components: Vec<ComponentMods>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let data = r#" [
                {
                    "type": "Button",
                    "label": "Add One"
                },
                {
                    "type": "RichText",
                    "value": "<p>Testando <b>Negrito</b> e texto normal</p>"
                },
                {
                    "type": "Button",
                    "label": "Other One"
                },
                {
                    "type": "CustomLink",
                    "url": "https://google.com",
                    "display_type": "OnlyText",
                    "target": "Blank",
                    "label": "Teste link"
                },
                {
                    "type": "DisplayBox",
                    "flex_direction": "Row",
                    "children": [
                        {
                            "type": "RichText",
                            "value": "<h1>Texto dentro do box</h1>"
                        }
                    ]
                }
            ]"#;

        // Parse the string of data into a Person object. This is exactly the
        // same function as the one that produced serde_json::Value above, but
        // now we are asking it for a Person as output.
        let components: Vec<ComponentMods> = serde_json::from_str(data).unwrap();
        // Self { components }
        // let mut components = vec![ComponentMods::Button(ButtonProps {
        //     label: String::from("Add one"),
        //     on_click: Callback::from(|_| ()),
        // })];
        // let componentsString = serde_json::to_string(&components).unwrap();
        // components.push(ComponentMods::RichText(RichTextProps {value: componentsString}));
        Self {
            components,
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
        // let link = ctx.link();
        let components = self.components.clone();
        html! {
            <>
            <ToolBar/>
            <BuilderDisplay components={components} />
            </>

        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
