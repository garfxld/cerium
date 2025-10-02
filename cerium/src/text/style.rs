use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use simdnbt::{
    ToNbtTag,
    owned::{self, NbtCompound, NbtTag},
};

use crate::text::{
    Component,
    color::{Rgb, Rgba},
};

// ===== Style ======

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Style {
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

impl Style {
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
}

impl simdnbt::Serialize for Style {
    fn to_compound(self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        if let Some(color) = self.color {
            compound.insert(
                "color",
                NbtTag::String(
                    format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b()).into(),
                ),
            );
        }
        if let Some(font) = self.font {
            compound.insert("font", font);
        }

        // I think sometimes some default styles are applied (e.g. italic text).
        // Therefore the property must be explicitly set to false.
        if let Some(bold) = self.bold {
            compound.insert("bold", bold);
        }
        if let Some(italic) = self.underlined {
            compound.insert("italic", italic);
        }
        if let Some(underlined) = self.underlined {
            compound.insert("underlined", underlined);
        }
        if let Some(strikethrough) = self.strikethrough {
            compound.insert("strikethrough", strikethrough);
        }
        if let Some(obfuscated) = self.obfuscated {
            compound.insert("obfuscated", obfuscated);
        }

        if let Some(shadow_color) = self.shadow_color {
            compound.insert(
                "shadow_color",
                NbtTag::String(
                    format!(
                        "#{:02X}{:02X}{:02X}{:02X}",
                        shadow_color.r(),
                        shadow_color.g(),
                        shadow_color.b(),
                        shadow_color.a()
                    )
                    .into(),
                ),
            );
        }

        if let Some(insertion) = self.insertion {
            compound.insert("insertion", insertion);
        }
        if let Some(hover_event) = self.hover_event {
            compound.insert("hover_event", hover_event);
        }
        if let Some(click_event) = self.click_event {
            compound.insert("click_event", click_event);
        }
        compound
    }
}

// ===== ClickEvent ======

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum ClickEvent {
    OpenUrl { url: Cow<'static, str> },
    // cannot be sent by the server.
    OpenFile { path: Cow<'static, str> },
    // doesnt need to be prefixed with `/`
    RunCommand { command: Cow<'static, str> },
    SuggestCommand { command: Cow<'static, str> },
    ChangePage { page: u32 },
    CopyToClipboard { value: Cow<'static, str> },
    // todo: show_dialog, custom
    // ShowDialog,
    // Custom,
}

impl simdnbt::Serialize for ClickEvent {
    fn to_compound(self) -> owned::NbtCompound {
        match self {
            Self::OpenUrl { url } => NbtCompound::from_values(vec![
                ("action".into(), "open_url".into()),
                ("url".into(), url.to_nbt_tag()),
            ]),
            Self::OpenFile { path } => NbtCompound::from_values(vec![
                ("action".into(), "open_file".into()),
                ("path".into(), path.to_nbt_tag()),
            ]),
            Self::RunCommand { command } => NbtCompound::from_values(vec![
                ("action".into(), "run_command".into()),
                ("command".into(), command.to_nbt_tag()),
            ]),
            Self::SuggestCommand { command } => NbtCompound::from_values(vec![
                ("action".into(), "suggest_command".into()),
                ("command".into(), command.to_nbt_tag()),
            ]),
            Self::ChangePage { page } => NbtCompound::from_values(vec![
                ("action".into(), "change_page".into()),
                ("page".into(), page.to_nbt_tag()),
            ]),
            Self::CopyToClipboard { value } => NbtCompound::from_values(vec![
                ("action".into(), "copy_to_clipboard".into()),
                ("value".into(), value.to_nbt_tag()),
            ]),
        }
    }
}

// ===== HoverEvent ======

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum HoverEvent {
    ShowText {
        // valid are: string, list, or object.
        value: Vec<Component>,
    },
    // todo: show_item, show_entity
    // ShowItem,
    // ShowEntity,
}

impl HoverEvent {
    pub fn show_text(text: impl Into<Component>) -> Self {
        Self::ShowText {
            value: vec![text.into()],
        }
    }
}

impl simdnbt::Serialize for HoverEvent {
    fn to_compound(self) -> owned::NbtCompound {
        match self {
            Self::ShowText { value } => NbtCompound::from_values(vec![
                ("action".into(), "show_text".into()),
                ("value".into(), value.to_nbt_tag()),
            ]),
        }
    }
}
