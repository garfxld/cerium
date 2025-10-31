use crate::text::TextComponent;

#[derive(Debug, Clone)]
pub struct Team {
    name: TextComponent,
    friendly_flags: u8,
    nametag_visibility: NametagVisibility,
    collision_rule: CollisionRule,
    color: i32,
    prefix: TextComponent,
    suffix: TextComponent,
}

impl Team {
    pub fn builder() -> TeamBuilder {
        TeamBuilder::default()
    }

    pub fn name(&self) -> &TextComponent {
        &self.name
    }

    pub fn friendly_flags(&self) -> u8 {
        self.friendly_flags
    }

    pub fn nametag_visibility(&self) -> NametagVisibility {
        self.nametag_visibility
    }

    pub fn collision_rule(&self) -> CollisionRule {
        self.collision_rule
    }

    pub fn color(&self) -> i32 {
        self.color
    }

    pub fn prefix(&self) -> &TextComponent {
        &self.prefix
    }

    pub fn suffix(&self) -> &TextComponent {
        &self.suffix
    }
}

pub struct TeamBuilder {
    name: TextComponent,
    friendly_flags: u8,
    nametag_visibility: NametagVisibility,
    collision_rule: CollisionRule,
    prefix: TextComponent,
    suffix: TextComponent,
}

impl TeamBuilder {
    pub fn allow_friendly_fire(mut self) -> Self {
        self.friendly_flags |= 0x01;
        self
    }

    pub fn can_see_invisible_players(mut self) -> Self {
        self.friendly_flags |= 0x02;
        self
    }

    pub fn with_nametag_visibility(mut self, nametag_visibility: NametagVisibility) -> Self {
        self.nametag_visibility = nametag_visibility;
        self
    }

    pub fn with_collision_rule(mut self, collision_rule: CollisionRule) -> Self {
        self.collision_rule = collision_rule;
        self
    }

    pub fn with_prefix(mut self, prefix: TextComponent) -> Self {
        self.prefix = prefix;
        self
    }

    pub fn with_suffix(mut self, suffix: TextComponent) -> Self {
        self.suffix = suffix;
        self
    }

    pub fn build(self) -> Team {
        Team {
            name: self.name,
            friendly_flags: self.friendly_flags,
            nametag_visibility: self.nametag_visibility,
            collision_rule: self.collision_rule,
            color: 0,
            prefix: self.prefix,
            suffix: self.suffix,
        }
    }
}

impl Default for TeamBuilder {
    fn default() -> Self {
        Self {
            name: TextComponent::EMPTY,
            friendly_flags: 0,
            nametag_visibility: NametagVisibility::Always,
            collision_rule: CollisionRule::Always,
            prefix: TextComponent::EMPTY,
            suffix: TextComponent::EMPTY,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CollisionRule {
    Always,
    Never,
    PushOtherTeams,
    PushOwnTeam,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NametagVisibility {
    Always,
    Never,
    HideForOthersTeams,
    HideForOwnTeam,
}
