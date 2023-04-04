use bevy::prelude::Component;
use uuid::Uuid;

pub mod constants;

#[derive(Component)]
pub struct Creature {
    health: u32,
    _name: String,
    tag: Uuid,
}

#[derive(Component)]
pub struct CreatureTag {
    _tag: Uuid,
}

impl Creature {
    pub fn new(name: String) -> Self {
        Self {
            health: 100,
            _name: name,
            tag: Uuid::new_v4(),
        }
    }

    pub fn get_creature_tag(&self) -> CreatureTag {
        CreatureTag {
            _tag: self.tag,
        }
    }

    pub fn take_damage(&mut self, damage_points: u32) {
        if self.health <= damage_points {
            self.health = 0
        } else {
            self.health -= damage_points
        }
    }

    pub fn _is_dead(&self) -> bool {
        self.health == 0
    }

    pub fn get_health(&self) -> u32 {
        self.health
    }
}
