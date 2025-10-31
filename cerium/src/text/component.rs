use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{
    text::{
        color::{Rgb, Rgba},
        style::{ClickEvent, HoverEvent, TextStyle},
    },
    util::Identifier,
};

/// A wrapper around a [TextComponent](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component).
///
/// # Examples
/// ```
/// use cerium::text::*;
///
/// fn main() {
///     let component = TextComponent::new()
///         .on_hover(HoverEvent::show_text("You found me!"))
///         .child(
///             TextComponent::text("Line 1 (styled)")
///                 .bold()
///                 .color(NamedColor::Red),
///         )
///         .child(TextComponent::NEW_LINE)
///         .child("Line 2 (unstyled)");
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TextComponent {
    #[serde(flatten)]
    content: TextContent,
    #[serde(flatten)]
    style: TextStyle,
    #[serde(rename = "extra", skip_serializing_if = "Vec::is_empty", default)]
    children: Vec<TextComponent>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
enum TextContent {
    Text {
        text: Cow<'static, str>,
    },
    Translatable {
        translate: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        fallback: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        with: Vec<TextComponent>,
    },
    Scoreboard {
        name: String,
        objective: String,
    },
    Selector {
        selector: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        seperator: Box<Option<TextComponent>>,
    },
    Keybind {
        keybind: String,
    },
    Object {
        atlas: String,
        sprite: String,
    },
}

macro_rules! component {
    ($content:expr) => {
        TextComponent {
            content: TextContent::Text {
                text: Cow::Borrowed($content),
            },
            style: TextStyle::const_default(),
            children: Vec::new(),
        }
    };
}

impl TextComponent {
    /// An empty text component.
    pub const EMPTY: TextComponent = component!("");

    /// A new-line text component.
    pub const NEW_LINE: TextComponent = component!("\n");

    /// A space text component.
    pub const SPACE: TextComponent = component!(" ");

    /// Creates a new empty component. This is equivelent to [TextComponent::EMPTY].
    pub const fn new() -> Self {
        Self::EMPTY
    }

    /// Creates a new component with the type set to [TextContent::Text].
    pub fn text(text: impl Into<String>) -> TextComponent {
        Self::create(TextContent::Text {
            text: Cow::Owned(text.into()),
        })
    }

    /// Creates a new component with the type set to [TextContent::Translatable].
    pub fn translatable(
        translate: impl Into<String>,
        fallback: Option<impl Into<String>>,
        with: Vec<TextComponent>,
    ) -> TextComponent {
        Self::create(TextContent::Translatable {
            translate: translate.into(),
            fallback: fallback.map(Into::into),
            with,
        })
    }

    /// Creates a new component with the type set to [TextContent::Scoreboard].
    pub fn scoreboard(name: impl Into<String>, objective: impl Into<String>) -> TextComponent {
        Self::create(TextContent::Scoreboard {
            name: name.into(),
            objective: objective.into(),
        })
    }

    /// Creates a new component with the type set to [TextContent::Selector].
    pub fn selector(
        selector: impl Into<String>,
        seperator: Option<impl Into<TextComponent>>,
    ) -> TextComponent {
        Self::create(TextContent::Selector {
            selector: selector.into(),
            seperator: Box::new(seperator.map(Into::into)),
        })
    }

    /// Creates a new component with the type set to [TextContent::Keybind].
    pub fn keybind(keybind: impl Into<String>) -> TextComponent {
        Self::create(TextContent::Keybind {
            keybind: keybind.into(),
        })
    }

    // pub fn block_nbt() {
    //     todo!()
    // }

    // pub fn entity_nbt() {
    //     todo!()
    // }

    // pub fn storage_nbt() {
    //     todo!()
    // }

    /// Creates a new component with the type set to [TextContent::Object].
    pub fn object(atlas: impl Into<String>, sprite: impl Into<String>) -> TextComponent {
        Self::create(TextContent::Object {
            atlas: atlas.into(),
            sprite: sprite.into(),
        })
    }

    fn create(content: TextContent) -> Self {
        Self {
            content,
            style: TextStyle::default(),
            children: vec![],
        }
    }

    // ===== Style =====

    pub fn style(&self) -> &TextStyle {
        &self.style
    }

    pub fn style_mut(&mut self) -> &mut TextStyle {
        &mut self.style
    }

    /// Changes the color of the component.
    pub fn color(mut self, color: impl Into<Rgb>) -> Self {
        self.style_mut().set_color(color.into());
        self
    }

    /// The resource location of the font for this component in the resource pack within `assets/<namespace>/font`. Defaults to `minecraft:default`.
    pub fn font(mut self, font: impl Into<Identifier>) -> Self {
        self.style_mut().set_font(font.into().to_string());
        self
    }

    pub fn bold(mut self) -> Self {
        self.style_mut().set_bold(true);
        self
    }

    pub fn italic(mut self) -> Self {
        self.style_mut().set_italic(true);
        self
    }

    pub fn underlined(mut self) -> Self {
        self.style_mut().set_underlined(true);
        self
    }

    pub fn strikethrough(mut self) -> Self {
        self.style_mut().set_strikethrough(true);
        self
    }

    pub fn obfuscated(mut self) -> Self {
        self.style_mut().set_obfuscated(true);
        self
    }

    pub fn shadow_color(mut self, shadow_color: impl Into<Rgba>) -> Self {
        self.style_mut().set_shadow_color(shadow_color.into());
        self
    }

    pub fn insertion(mut self, insertion: String) -> Self {
        self.style_mut().set_insertion(insertion);
        self
    }

    pub fn on_hover(mut self, on_hover: HoverEvent) -> Self {
        self.style_mut().set_on_hover(on_hover);
        self
    }

    pub fn on_click(mut self, on_click: ClickEvent) -> Self {
        self.style_mut().set_on_click(on_click);
        self
    }

    // ===== Children =====

    fn extend(&mut self, components: impl IntoIterator<Item = TextComponent>) {
        self.children.extend(components);
    }

    pub fn child(mut self, child: impl Into<TextComponent>) -> Self {
        self.extend(std::iter::once(child.into()));
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<TextComponent>>,
    ) -> Self {
        self.extend(children.into_iter().map(|c| c.into()));
        self
    }
}

impl<S> From<S> for TextComponent
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        TextComponent::text(value.into()).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::NamedColor;

    #[test]
    fn test_json_deserialize() {
        let raw = r##"{"text":"","hover_event":{"action":"show_text","value":[{"text":"hover"}]},"extra":[{"text":"first line","color":"#ff5555","bold":true},{"text":"\n"},{"text":"second line"}]}"##;
        let component1: TextComponent = serde_json::from_str(raw).unwrap();

        let component2 = TextComponent::new()
            .on_hover(HoverEvent::show_text("hover"))
            .child(
                TextComponent::text("first line")
                    .bold()
                    .color(NamedColor::Red),
            )
            .child(TextComponent::NEW_LINE)
            .child("second line");

        assert_eq!(component1, component2)
    }
}
