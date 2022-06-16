use crate::button::*;
use crate::rich_text::*;
use crate::custom_link::*;
use crate::display_box::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum ComponentMods {
    Button(ButtonProps),
    RichText(RichTextProps),
    CustomLink(CustomLinkProps),
    DisplayBox(DisplayBoxProps)
}