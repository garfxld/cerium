use serde::{Deserialize, Serialize};
use simdnbt::owned;

use crate::text::{Component, ParentComponent, StyledComponent, style::Style};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SelectorComponent {
    selector: String,
    seperator: Box<Option<Component>>,
    style: Style,
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

impl simdnbt::Serialize for SelectorComponent {
    fn to_compound(self) -> owned::NbtCompound {
        let mut compound = owned::NbtCompound::new();
        compound.insert("selector", self.selector);
        if let Some(seperator) = *self.seperator {
            compound.insert("seperator", seperator);
        }

        // Style + Children
        compound.extend(self.style.to_compound());
        if !self.children.is_empty() {
            compound.insert("extra", self.children);
        }

        compound
    }
}
