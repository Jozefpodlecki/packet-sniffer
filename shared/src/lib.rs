use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PacketType {
    NewLocalPlayer,
    NewPlayer,
    NewNpc,
    ZoneInfo,
    Damage
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewLocalPlayerPacket {
    pub kind: PacketType,
    pub object_id: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPlayerPacket {
    pub kind: PacketType,
    pub object_id: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewNpcPacket {
    pub kind: PacketType,
    pub object_id: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZoneInfoPacket {
    pub kind: PacketType,
    pub zone_id: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamagePacket {
    pub kind: PacketType,
    pub skill_id: i32,
    pub source_id: i64,
    pub max_boss_hp: i32,
    pub boss_hp: i32,
    pub damage: i32,
    pub target_id: i64
}

