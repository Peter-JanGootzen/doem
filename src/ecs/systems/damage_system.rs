use crate::ecs::components::damage::Damage;
use crate::ecs::components::health::Health;
use crate::ecs::resources::collisions::Collisions;
use specs::prelude::*;

pub struct DamageSystem;

impl<'a> System<'a> for DamageSystem { 
    type SystemData = (
        Entities<'a>,
        Write<'a, Collisions>,
        ReadStorage<'a, Damage>,
        WriteStorage<'a, Health>,
    );

    fn run(&mut self, (mut entities, mut collisions, damage, mut health): Self::SystemData) {
        let mut to_kill: Vec<Entity> = Vec::new();
        for (ent1, ent2) in collisions.0.iter() {
            if let Some(damage_1) = damage.get(*ent1) {
                if damage_1.damage_dealer != *ent1 {
                    // We are now "damaging"
                    if let Some(health_2) = health.get_mut(*ent2) {
                        health_2.health -= damage_1.damage;
                        if health_2.health <= 0.0 {
                            to_kill.push(*ent2);
                        }
                    }
                    if damage_1.despawn_entity_on_impact {
                        to_kill.push(*ent1);
                    }
                }
            }
            if let Some(damage_2) = damage.get(*ent2) {
                if damage_2.damage_dealer != *ent2 {
                    if let Some(health_1) = health.get_mut(*ent1) {
                        health_1.health -= damage_2.damage;
                        if health_1.health <= 0.0 {
                            to_kill.push(*ent1);
                        }
                    }
                    if damage_2.despawn_entity_on_impact {
                        to_kill.push(*ent2);
                    }
                }
            }
        }
        for e in &to_kill {
            entities.delete(*e);
        }
    }

}
