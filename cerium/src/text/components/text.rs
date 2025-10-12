use crate::text::{Component, ParentComponent, StyledComponent, style::Style};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TextComponent {
    #[serde(rename = "text")]
    content: String,
    #[serde(flatten, default)]
    style: Style,
    #[serde(rename = "extra", skip_serializing_if = "Vec::is_empty", default)]
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
