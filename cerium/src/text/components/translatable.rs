use serde::{Deserialize, Serialize};
use simdnbt::owned;

use crate::text::{Component, ParentComponent, StyledComponent, style::Style};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranslatableComponent {
    translate: String,
    fallback: Option<String>,
    with: Vec<Component>,
    style: Style,
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

impl simdnbt::Serialize for TranslatableComponent {
    fn to_compound(self) -> owned::NbtCompound {
        let mut compound = owned::NbtCompound::new();
        compound.insert("translate", self.translate);
        if let Some(fallback) = self.fallback {
            compound.insert("fallback", fallback);
        }
        if self.with.len() > 0 {
            compound.insert("with", self.with);
        }

        // Style + Children
        compound.extend(self.style.to_compound());
        if !self.children.is_empty() {
            compound.insert("extra", self.children);
        }

        compound
    }
}
