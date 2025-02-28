use serde::{Serialize, Deserialize};

pub type GearLevel = f32;
pub type EntityId = u64;
pub type PartyInstanceId = u32;
pub type RaidInstanceId = u32;
pub type CharacterId = u64;
pub type SkillId = u32;
pub type ClassId = u32;

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Packet {
    None,
    NewLocalPlayer {
        object_id: EntityId,
        name: String,
        class_id: ClassId,
        character_id: CharacterId,
        gear_score: GearLevel
    },
    NewPlayer {
        object_id: EntityId,
        name: String,
        class_id: ClassId,
        character_id: CharacterId,
        gear_score: GearLevel
    },
    NewNpc {
        object_id: EntityId,
        max_hp: i64,
    },
    PartyInfo {
        party_instance_id: PartyInstanceId,
        raid_instance_id: RaidInstanceId,
        members: Vec<PartyMember>
    },
    ZoneChange {
        zone_id: u32,
        zone_level: u32,
    },
    Damage {
        source_id: EntityId,
        damage: i64,
        skill_id: SkillId,
        max_hp: i64,
        current_hp: i64,
        target_id: EntityId
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartyMember {
    pub name: String,
    pub class_id: ClassId,
    pub character_id: CharacterId,
    pub gear_level: GearLevel,
}

pub struct Player {
    
}

pub struct Skill {
    
}