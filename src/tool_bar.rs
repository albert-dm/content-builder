use crate::builder::*;
use yew::{function_component, html};

#[function_component(ToolBar)]
pub fn toolBar() -> Html {
    // let toolbar_style = style!(
    //     // A CSS string literal
    //     r#"
    //         background-color: red;
     
    //         .nested {
    //             background-color: blue;
    //             width: 100px
    //         }
    //     "#
    // ).unwrap();
    // let class_name = toolbar_style.clone().get_class_name();
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
                }
            ]"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let components: Vec<ComponentMods> = serde_json::from_str(data).unwrap();
    html! {
        <aside>
            {serde_json::to_string(&components).unwrap()}
        </aside>
    }
}
