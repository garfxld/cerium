pub mod color;
mod component;
pub use component::*;

pub mod style;

mod components {
    mod keybind;
    mod object;
    mod score;
    mod selector;
    mod text;
    mod translatable;

    pub use keybind::KeybindComponent;
    pub use object::ObjectComponent;
    pub use score::ScoreComponent;
    pub use selector::SelectorComponent;
    pub use text::TextComponent;
    pub use translatable::TranslatableComponent;
}

pub use components::*;
