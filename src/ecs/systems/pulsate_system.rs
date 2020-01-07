use specs::prelude::*;
use crate::ecs::components::pulsate::Pulsate;
use crate::ecs::components::transform::Transform;

pub struct PulsateSystem;

impl<'a> System<'a> for PulsateSystem {
    type SystemData = (
        WriteStorage<'a, Pulsate>,
        WriteStorage<'a, Transform>
    );

    fn run(&mut self, (mut pulsate, mut transform): Self::SystemData) {
        for (p, t) in (&mut pulsate, &mut transform).join() {
            // Growing bigger
            if p.current_direction {
                // It will become too big
                if t.scale.sign_length() + p.speed.length() > p.max_scale.length() {
                    t.scale = &p.max_scale + &(&p.max_scale - &(&t.scale + &p.speed));
                    p.current_direction = !p.current_direction;
                } else {
                    t.scale = &t.scale + &p.speed;
                }
            } else { // Shrinking
                // It will become too small
                if t.scale.sign_length() - p.speed.length() < p.min_scale.sign_length() {
                    t.scale = &p.min_scale - &(&p.min_scale - &(&t.scale - &p.speed));
                    p.current_direction = !p.current_direction;
                } else {
                    t.scale = &t.scale - &p.speed;
                }
            }
        }
    }
}