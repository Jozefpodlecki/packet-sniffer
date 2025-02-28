#[repr(C)]
#[derive(Debug)]
pub enum Packet {
    NewPlayer {
        object_id: i32,
    },
    NewNpc {
        object_id: i32,
    },
    ZoneChange {
        zone_id: i32,
        zone_level: i32,
    },
    Damage {
        object_id: i32,
    }
}