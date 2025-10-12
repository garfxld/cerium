use crate::text::{Component, ParentComponent, StyledComponent, style::Style};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TranslatableComponent {
    translate: String,
    fallback: Option<String>,
    with: Vec<Component>,
    #[serde(flatten)]
    style: Style,
    #[serde(rename = "extra", skip_serializing_if = "Vec::is_empty", default)]
    children: Vec<Component>,
}

impl TranslatableComponent {
    pub(crate) fn new(translate: String, fallback: Option<String>, with: Vec<Component>) -> Self {
        Self {
            translate,
            fallback,
            with,
            ..Default::default()
        }
    }

    pub fn translate(&self) -> &String {
        &self.translate
    }

    pub fn fallback(&self) -> Option<&String> {
        self.fallback.as_ref()
    }
}

impl StyledComponent for TranslatableComponent {
    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl ParentComponent for TranslatableComponent {
    fn extend(&mut self, components: impl IntoIterator<Item = Component>) {
        self.children.extend(components);
    }
}

impl From<TranslatableComponent> for Component {
    fn from(value: TranslatableComponent) -> Self {
        Component::Translatable(value)
    }
}
