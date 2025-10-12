use crate::text::{Component, ParentComponent, StyledComponent, style::Style};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObjectComponent {
    atlas: String,
    sprite: String,
    #[serde(flatten)]
    style: Style,
    #[serde(rename = "extra", skip_serializing_if = "Vec::is_empty", default)]
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
