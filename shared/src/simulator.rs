use log::info;

use crate::{models::Packet};

pub struct SimulatorOptions {
    pub boss_name: String,
    pub max_boss_hp: i64
}

pub struct Simulator {
    boss_hp: i64,
    max_boss_hp: i64
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
}

impl Iterator for Simulator {
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {

        let damage = 1i64;
        
        let packet = Packet::Damage {
            skill_id: 1,
            source_id: 1,
            current_hp: self.boss_hp,
            max_hp: self.max_boss_hp,
            damage,
            target_id: 1,
        };

        self.boss_hp -= damage;

        Some(packet)
    }
}