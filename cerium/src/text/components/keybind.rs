use serde::{Deserialize, Serialize};
use simdnbt::owned;

use crate::text::{Component, ParentComponent, StyledComponent, style::Style};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct KeybindComponent {
    keybind: String,
    style: Style,
    children: Vec<Component>,
}

impl KeybindComponent {
    pub(crate) fn new(keybind: String) -> Self {
        Self {
            keybind,
            ..Default::default()
        }
    }

    pub fn keybind(&self) -> &String {
        &self.keybind
    }
}

impl StyledComponent for KeybindComponent {
    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl ParentComponent for KeybindComponent {
    fn extend(&mut self, components: impl IntoIterator<Item = Component>) {
        self.children.extend(components);
    }
}

impl From<KeybindComponent> for Component {
    fn from(value: KeybindComponent) -> Self {
        Component::Keybind(value)
    }
}

impl simdnbt::Serialize for KeybindComponent {
    fn to_compound(self) -> owned::NbtCompound {
        let mut compound = owned::NbtCompound::new();
        compound.insert("keybind", self.keybind);

        // Style + Children
        compound.extend(self.style.to_compound());
        if !self.children.is_empty() {
            compound.insert("extra", self.children);
        }

        compound
    }
}
