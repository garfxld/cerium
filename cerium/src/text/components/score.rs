use serde::{Deserialize, Serialize};
use simdnbt::owned;

use crate::text::{Component, ParentComponent, StyledComponent, style::Style};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ScoreComponent {
    name: String,
    objective: String,
    style: Style,
    children: Vec<Component>,
}

impl ScoreComponent {
    pub(crate) fn new(name: String, objective: String) -> Self {
        Self {
            name,
            objective,
            ..Default::default()
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn objective(&self) -> &String {
        &self.objective
    }
}

impl StyledComponent for ScoreComponent {
    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl ParentComponent for ScoreComponent {
    fn extend(&mut self, components: impl IntoIterator<Item = Component>) {
        self.children.extend(components);
    }
}

impl From<ScoreComponent> for Component {
    fn from(value: ScoreComponent) -> Self {
        Component::Score(value)
    }
}

impl simdnbt::Serialize for ScoreComponent {
    fn to_compound(self) -> owned::NbtCompound {
        let mut compound = owned::NbtCompound::new();

        let mut score = owned::NbtCompound::new();
        score.insert("name", self.name);
        score.insert("objective", self.objective);

        compound.insert("score", score);

        // Style + Children
        compound.extend(self.style.to_compound());
        if !self.children.is_empty() {
            compound.insert("extra", self.children);
        }

        compound
    }
}
