use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    item::ItemStack,
    text::{
        TextComponent,
        color::{Rgb, Rgba},
    },
    util::Identifier,
};

// ===== Style ======

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct TextStyle {
    // Formatting
    #[serde(default, skip_serializing_if = "Option::is_none")]
    color: Option<Rgb>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    font: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bold: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    italic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    underlined: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    strikethrough: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    obfuscated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    shadow_color: Option<Rgba>,

    // Interactivity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    insertion: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    click_event: Option<ClickEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hover_event: Option<HoverEvent>,
}

impl TextStyle {
    // Formatting

    pub fn color(&self) -> Option<Rgb> {
        self.color
    }

    pub fn font(&self) -> Option<&String> {
        self.font.as_ref()
    }

    pub fn bold(&self) -> bool {
        self.bold.unwrap_or_default()
    }

    pub fn italic(&self) -> bool {
        self.italic.unwrap_or_default()
    }

    pub fn underlined(&self) -> bool {
        self.underlined.unwrap_or_default()
    }

    pub fn strikethrough(&self) -> bool {
        self.strikethrough.unwrap_or_default()
    }

    pub fn obfuscated(&self) -> bool {
        self.obfuscated.unwrap_or_default()
    }

    pub fn shadow_color(&self) -> Option<Rgba> {
        self.shadow_color
    }

    pub fn set_color(&mut self, color: Rgb) {
        self.color = Some(color);
    }

    pub fn set_font(&mut self, font: String) {
        self.font = Some(font);
    }

    pub fn set_bold(&mut self, bold: bool) {
        self.bold = Some(bold);
    }

    pub fn set_italic(&mut self, italic: bool) {
        self.italic = Some(italic);
    }

    pub fn set_underlined(&mut self, underlined: bool) {
        self.underlined = Some(underlined);
    }

    pub fn set_strikethrough(&mut self, strikethrough: bool) {
        self.strikethrough = Some(strikethrough);
    }

    pub fn set_obfuscated(&mut self, obfuscated: bool) {
        self.obfuscated = Some(obfuscated);
    }

    pub fn set_shadow_color(&mut self, shadow_color: Rgba) {
        self.shadow_color = Some(shadow_color);
    }

    // Interactivity
    pub fn insertion(&self) -> Option<&String> {
        self.insertion.as_ref()
    }

    pub fn hover_event(&self) -> Option<&HoverEvent> {
        self.hover_event.as_ref()
    }

    pub fn click_event(&self) -> Option<&ClickEvent> {
        self.click_event.as_ref()
    }

    pub fn set_insertion(&mut self, insertion: String) {
        self.insertion = Some(insertion);
    }

    pub fn set_on_hover(&mut self, on_hover: HoverEvent) {
        self.hover_event = Some(on_hover);
    }

    pub fn set_on_click(&mut self, on_click: ClickEvent) {
        self.click_event = Some(on_click);
    }

    pub(crate) const fn const_default() -> Self {
        Self {
            color: None,
            font: None,
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            shadow_color: None,
            insertion: None,
            click_event: None,
            hover_event: None,
        }
    }
}

// ===== ClickEvent ======

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
#[serde(tag = "action", rename_all = "snake_case")]
enum ClickEventType {
    OpenUrl { url: String },
    RunCommand { command: String },
    SuggestCommand { command: String },
    ChangePage { page: u32 },
    CopyToClipboard { value: String },
    // todo: ShowDialog,
    // todo: Custom,
}

/// A wrapper around a [ClickEvent](https://minecraft.wiki/w/Text_component_format#Click_events).
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub struct ClickEvent {
    #[serde(flatten)]
    ty: ClickEventType,
}

impl ClickEvent {
    /// Opens the specified URL in the user's default web browser.
    pub fn open_url(url: impl Into<String>) -> Self {
        Self {
            ty: ClickEventType::OpenUrl { url: url.into() },
        }
    }

    /// Runs the specified command. This runs as if the player typed the specified command in chat and pressed enter.
    pub fn run_command(command: impl Into<String>) -> Self {
        Self {
            ty: ClickEventType::RunCommand {
                command: command.into(),
            },
        }
    }

    /// Opens chat and fills in the specified text or command. If a chat message was already being composed, it is overwritten.This does not work in books.
    pub fn suggest_command(command: impl Into<String>) -> Self {
        Self {
            ty: ClickEventType::SuggestCommand {
                command: command.into(),
            },
        }
    }

    /// Can only be used in written books. Changes to the specified page if that page exists.
    pub fn change_page(page: u32) -> Self {
        Self {
            ty: ClickEventType::ChangePage { page },
        }
    }

    /// Copies the specified text to the clipboard.
    pub fn copy_to_clipboard(value: impl Into<String>) -> Self {
        Self {
            ty: ClickEventType::CopyToClipboard {
                value: value.into(),
            },
        }
    }
}

// ===== HoverEvent ======

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
enum HoverEventType {
    ShowText {
        // valid are: string, list, or object.
        value: Vec<TextComponent>,
    },
    ShowItem {
        id: String,
        count: Option<i32>,
        components: Option<()>, // todo
    },
    ShowEntity {
        name: Box<Option<TextComponent>>,
        id: String,
        uuid: String,
    },
}

/// A wrapper around a [HoverEvent](https://minecraft.wiki/w/Text_component_format#Hover_events).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HoverEvent {
    #[serde(flatten)]
    ty: HoverEventType,
}

impl HoverEvent {
    /// Shows a text component.
    pub fn show_text(text: impl Into<TextComponent>) -> Self {
        Self {
            ty: HoverEventType::ShowText {
                value: vec![text.into()],
            },
        }
    }

    /// Shows the tooltip of an item as if it was being hovering over it in an inventory.
    pub fn show_item(stack: ItemStack, show_amount: bool, show_components: bool) -> Self {
        Self {
            ty: HoverEventType::ShowItem {
                id: stack.material().key().to_string(),
                count: show_amount.then_some(stack.amount()),
                components: show_components.then_some(()),
            },
        }
    }

    /// Shows an entity's name, type, and UUID.
    pub fn show_entity(name: Option<impl Into<TextComponent>>, id: Identifier, uuid: Uuid) -> Self {
        Self {
            ty: HoverEventType::ShowEntity {
                name: Box::new(name.map(Into::into)),
                id: id.to_string(),
                uuid: uuid.to_string(),
            },
        }
    }
}
