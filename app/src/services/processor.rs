use chrono::Utc;

use crate::models::Encounter;


pub struct Processor {
    encounter: Encounter
}

impl Processor {
    pub fn new() -> Self {
        let encounter = Encounter {
            id: 1,
            name: "Encounter 1".to_string(),
            started_on: Utc::now(),
            participants: vec![],
        };

        Processor { encounter }
    }

    pub fn tick(&self) -> &Encounter {
       &self.encounter
    }   
}