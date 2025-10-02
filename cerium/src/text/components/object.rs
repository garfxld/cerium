use serde::{Deserialize, Serialize};
use simdnbt::owned;

use crate::text::{Component, ParentComponent, StyledComponent, style::Style};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ObjectComponent {
    atlas: String,
    sprite: String,
    style: Style,
    children: Vec<Component>,
}

impl ObjectComponent {
    pub(crate) fn new(atlas: String, sprite: String) -> Self {
        Self {
            atlas,
            sprite,
            ..Default::default()
        }
    }

    pub fn atlas(&self) -> &String {
        &self.atlas
    }

    pub fn sprite(&self) -> &String {
        &self.sprite
    }
}

impl StyledComponent for ObjectComponent {
    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl ParentComponent for ObjectComponent {
    fn extend(&mut self, components: impl IntoIterator<Item = Component>) {
        self.children.extend(components);
    }
}

impl From<ObjectComponent> for Component {
    fn from(value: ObjectComponent) -> Self {
        Component::Object(value)
    }
}

impl simdnbt::Serialize for ObjectComponent {
    fn to_compound(self) -> owned::NbtCompound {
        let mut compound = owned::NbtCompound::new();
        compound.insert("atlas", self.atlas);
        compound.insert("sprite", self.sprite);

        // Style + Children
        compound.extend(self.style.to_compound());
        if !self.children.is_empty() {
            compound.insert("extra", self.children);
        }

        compound
    }
}
