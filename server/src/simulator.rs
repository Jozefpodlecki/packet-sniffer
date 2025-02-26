use log::info;
use shared::{DamagePacket, NewPlayerPacket};

pub struct SimulatorOptions {

}

pub struct Simulator {
    boss_hp: i32,
    max_boss_hp: i32
}

impl Simulator {
    pub fn new() -> Self {
        Self {
            boss_hp: 1000,
            max_boss_hp: 1000
        }
    }

    pub fn setup(&mut self, options: SimulatorOptions) {

    }

    pub fn generate_packet(&mut self) -> Vec<u8> {

        // let packet = NewPlayerPacket {
        //     kind: shared::PacketType::NewPlayer,
        //     object_id: 1
        // };

        let damage = 1i32;

        let packet = DamagePacket {
            kind: shared::PacketType::Damage,
            skill_id: 1,
            source_id: 1,
            boss_hp: self.boss_hp,
            max_boss_hp: self.max_boss_hp,
            damage,
            target_id: 1,
        };

        self.boss_hp -= damage;

        let encoded: Vec<u8> = bincode::serialize(&packet).unwrap();

        info!("Sending DamagePacket {:?}", encoded);

        encoded
    }
}