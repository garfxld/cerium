use crate::text::{Component, ParentComponent, StyledComponent, style::Style};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScoreComponent {
    name: String,
    objective: String,
    #[serde(flatten)]
    style: Style,
    #[serde(rename = "extra", skip_serializing_if = "Vec::is_empty", default)]
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
