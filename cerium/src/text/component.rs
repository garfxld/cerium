use serde::{Deserialize, Serialize};

use crate::{
    text::{
        KeybindComponent, ObjectComponent, ScoreComponent, SelectorComponent, TextComponent,
        TranslatableComponent,
        color::{Rgb, Rgba},
        style::{ClickEvent, HoverEvent, Style},
    },
    util::Identifier,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Component {
    Text(TextComponent),
    Translatable(TranslatableComponent),
    Object(ObjectComponent),
    Score(ScoreComponent),
    Keybind(KeybindComponent),
    Selector(SelectorComponent),
}

impl Component {
    pub fn empty() -> TextComponent {
        Self::text("")
    }

    pub fn new_line() -> TextComponent {
        Self::text("\n")
    }

    pub fn space() -> TextComponent {
        Self::text(" ")
    }

    pub fn text(text: impl Into<String>) -> TextComponent {
        TextComponent::new(text.into())
    }

    pub fn translatable(
        translate: impl Into<String>,
        fallback: Option<impl Into<String>>,
        with: Vec<Component>,
    ) -> TranslatableComponent {
        TranslatableComponent::new(translate.into(), fallback.map(Into::into), with)
    }

    pub fn score(name: String, objective: String) -> ScoreComponent {
        ScoreComponent::new(name, objective)
    }

    pub fn selector(selector: String, seperator: Option<Component>) -> SelectorComponent {
        SelectorComponent::new(selector, seperator)
    }

    pub fn keybind(keybind: String) -> KeybindComponent {
        KeybindComponent::new(keybind)
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

    pub fn object(atlas: String, sprite: String) -> ObjectComponent {
        ObjectComponent::new(atlas, sprite)
    }
}

impl<S> From<S> for Component
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Component::text(value.into()).into()
    }
}

pub trait ParentComponent {
    fn extend(&mut self, components: impl IntoIterator<Item = Component>);

    fn child(mut self, child: impl Into<Component>) -> Self
    where
        Self: Sized,
    {
        self.extend(std::iter::once(child.into()));
        self
    }

    fn children(mut self, children: impl IntoIterator<Item = impl Into<Component>>) -> Self
    where
        Self: Sized,
    {
        self.extend(children.into_iter().map(|c| c.into()));
        self
    }
}

pub trait StyledComponent
where
    Self: Sized,
{
    fn style(&self) -> &Style;

    fn style_mut(&mut self) -> &mut Style;

    fn color(mut self, color: impl Into<Rgb>) -> Self {
        self.style_mut().set_color(color.into());
        self
    }

    /// The resource location of the font for this component in the resource pack within `assets/<namespace>/font`. Defaults to `minecraft:default`.
    fn font(mut self, font: impl Into<Identifier>) -> Self {
        self.style_mut().set_font(font.into().to_string());
        self
    }

    fn bold(mut self) -> Self {
        self.style_mut().set_bold(true);
        self
    }

    fn italic(mut self) -> Self {
        self.style_mut().set_italic(true);
        self
    }

    fn underlined(mut self) -> Self {
        self.style_mut().set_underlined(true);
        self
    }

    fn strikethrough(mut self) -> Self {
        self.style_mut().set_strikethrough(true);
        self
    }

    fn obfuscated(mut self) -> Self {
        self.style_mut().set_obfuscated(true);
        self
    }

    fn shadow_color(mut self, shadow_color: impl Into<Rgba>) -> Self {
        self.style_mut().set_shadow_color(shadow_color.into());
        self
    }

    fn insertion(mut self, insertion: String) -> Self {
        self.style_mut().set_insertion(insertion);
        self
    }

    fn on_hover(mut self, on_hover: HoverEvent) -> Self {
        self.style_mut().set_on_hover(on_hover);
        self
    }

    fn on_click(mut self, on_click: ClickEvent) -> Self {
        self.style_mut().set_on_click(on_click);
        self
    }
}

impl simdnbt::Serialize for Component {
    fn to_compound(self) -> simdnbt::owned::NbtCompound {
        match self {
            Component::Text(c) => c.to_compound(),
            Component::Translatable(c) => c.to_compound(),
            Component::Object(c) => c.to_compound(),
            Component::Score(c) => c.to_compound(),
            Component::Keybind(c) => c.to_compound(),
            Component::Selector(c) => c.to_compound(),
        }
    }
}
