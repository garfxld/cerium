use crate::text::{Component, ParentComponent, StyledComponent, style::Style};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeybindComponent {
    keybind: String,
    #[serde(flatten)]
    style: Style,
    #[serde(rename = "extra", skip_serializing_if = "Vec::is_empty", default)]
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
