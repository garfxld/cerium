use crate::{item::ItemStack, text::TextComponent, util::Identifier};

pub struct Advancement {
    parent_id: Option<Identifier>,
    display_data: Option<AdvancementDisplay>,
    nested_requirements: Vec<String>,
    sends_telemetry_data: bool,
}

impl Advancement {
    pub fn new(
        parent_id: Option<Identifier>,
        display_data: Option<AdvancementDisplay>,
        nested_requirements: Vec<String>,
        sends_telemetry_data: bool,
    ) -> Self {
        Self {
            parent_id,
            display_data,
            nested_requirements,
            sends_telemetry_data,
        }
    }
}

pub struct AdvancementDisplay {
    title: TextComponent,
    description: TextComponent,
    icon: ItemStack,
    frame: FrameType,
    flags: i32,
    background_texture: Option<Identifier>,
    x_coord: f32,
    y_coord: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FrameType {
    Task,
    Challenge,
    Goal,
}
