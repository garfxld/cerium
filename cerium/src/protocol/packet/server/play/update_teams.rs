use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    scoreboard::team::Team,
};

#[derive(Debug, Clone)]
pub struct UpdateTeamsPacket {
    pub team_name: String,
    pub method: TeamUpdateMethod,
}

impl Packet for UpdateTeamsPacket {}
impl ServerPacket for UpdateTeamsPacket {}

impl Encode for UpdateTeamsPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_string(&this.team_name)?;
        TeamUpdateMethod::encode(w, &this.method)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum TeamUpdateMethod {
    CreateTeam { team: Team, entities: Vec<String> },
    RemoveTeam,
    UpdateTeam { team: Team, entities: Vec<String> },
    AddEntities { entities: Vec<String> },
    RemoveEntities { entities: Vec<String> },
}

impl TeamUpdateMethod {
    pub fn id(&self) -> i32 {
        match self {
            Self::CreateTeam { .. } => 0,
            Self::RemoveTeam => 1,
            Self::UpdateTeam { .. } => 2,
            Self::AddEntities { .. } => 3,
            Self::RemoveEntities { .. } => 4,
        }
    }
}

impl Encode for TeamUpdateMethod {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_u8(this.id() as u8)?;
        match this {
            Self::CreateTeam { team, entities } => {
                Team::encode(w, team)?;
                w.write_array(entities, |w, v| w.write_string(v))?;
            }
            Self::RemoveTeam => {}
            Self::UpdateTeam { team, entities } => {
                Team::encode(w, team)?;
                w.write_array(entities, |w, v| w.write_string(v))?;
            }
            Self::AddEntities { entities } => {
                w.write_array(entities, |w, v| w.write_string(v))?;
            }
            Self::RemoveEntities { entities } => {
                w.write_array(entities, |w, v| w.write_string(v))?;
            }
        }
        Ok(())
    }
}

impl Encode for Team {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_component(this.name())?;
        w.write_u8(this.friendly_flags())?;
        w.write_varint(this.nametag_visibility() as i32)?;
        w.write_varint(this.collision_rule() as i32)?;
        w.write_varint(this.color())?;
        w.write_component(this.prefix())?;
        w.write_component(this.suffix())?;
        Ok(())
    }
}
