use crate::text::{Component, ParentComponent, StyledComponent, style::Style};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SelectorComponent {
    selector: String,
    seperator: Box<Option<Component>>,
    #[serde(flatten)]
    style: Style,
    #[serde(rename = "extra", skip_serializing_if = "Vec::is_empty", default)]
    children: Vec<Component>,
}

impl SelectorComponent {
    pub(crate) fn new(selector: String, seperator: Option<Component>) -> Self {
        Self {
            selector,
            seperator: Box::new(seperator),
            style: Default::default(),
            children: Default::default(),
        }
    }

    pub fn selector(&self) -> &String {
        &self.selector
    }

    pub fn seperator(&self) -> Option<&Component> {
        (*self.seperator).as_ref()
    }
}

impl StyledComponent for SelectorComponent {
    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl ParentComponent for SelectorComponent {
    fn extend(&mut self, components: impl IntoIterator<Item = Component>) {
        self.children.extend(components);
    }
}

impl From<SelectorComponent> for Component {
    fn from(value: SelectorComponent) -> Self {
        Component::Selector(value)
    }
}
