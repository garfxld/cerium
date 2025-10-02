use serde::{Deserialize, Serialize};
use simdnbt::{
    borrow,
    owned::{self},
};

use crate::text::{Component, ParentComponent, StyledComponent, style::Style};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TextComponent {
    #[serde(rename = "text")]
    content: String,
    #[serde(flatten)]
    style: Style,
    #[serde(rename = "extra", skip_serializing_if = "Vec::is_empty")]
    children: Vec<Component>,
}

impl TextComponent {
    pub(crate) fn new(text: String) -> Self {
        Self {
            content: text,
            ..Default::default()
        }
    }

    pub fn text(&self) -> &String {
        &self.content
    }
}

impl StyledComponent for TextComponent {
    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl ParentComponent for TextComponent {
    fn extend(&mut self, components: impl IntoIterator<Item = Component>) {
        self.children.extend(components);
    }
}

impl From<TextComponent> for Component {
    fn from(value: TextComponent) -> Self {
        Component::Text(value)
    }
}

impl simdnbt::Serialize for TextComponent {
    fn to_compound(self) -> owned::NbtCompound {
        let mut compound = owned::NbtCompound::new();
        compound.insert("text", self.content);

        // Style + Children
        compound.extend(self.style.to_compound());
        if !self.children.is_empty() {
            compound.insert("extra", self.children);
        }

        compound
    }
}

impl simdnbt::Deserialize for TextComponent {
    fn from_compound(_compound: borrow::NbtCompound) -> Result<Self, simdnbt::DeserializeError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use simdnbt::ToNbtTag;

    use super::*;

    #[test]
    fn test_name() {
        let component = Component::text("some text").child(Component::text("text"));

        let text = serde_json::to_string(&component).unwrap();
        let nbt = simdnbt::Serialize::to_nbt(component.clone());
        let tag = component.to_nbt_tag();

        println!("text: {}", text);
        println!("nbt:  {:?}", nbt);
        println!("tag:  {:?}", tag);
    }
}
