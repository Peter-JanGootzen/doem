use doem_math::vector_space::{ Vector3, Vector4, Matrix4 };
use crate::ecs::resources::doem_events::DoemEvents;
use crate::ecs::components::shape::Shape;
use crate::ecs::components::collider::Collider;
use crate::ecs::components::physics::Physics;
use crate::ecs::components::transform::Transform;
use crate::ecs::components::damage::Damage;
use crate::ecs::components::gun::Gun;
use crate::consts;
use specs::prelude::*;
use luminance_glfw::{Action, Key, WindowEvent};

pub struct ShootSystem;

impl<'a> System<'a> for ShootSystem {
    type SystemData = (
        Read<'a, DoemEvents>,
        ReadStorage<'a, Gun>,
        Entities<'a>,
        WriteStorage<'a, Shape>,
        WriteStorage<'a, Damage>,
        WriteStorage<'a, Collider>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Physics>,
    );

    fn run(&mut self , (events, gun, entities, mut shape, mut damage, mut collider, mut transform, mut physics): Self::SystemData) {
        let mut to_add: Vec<(Shape, Damage, Collider, Transform, Physics)> = Vec::new();
        for (shooter_t, shooter_p, shooter_g) in (&transform, &physics, &gun).join() {
            for event in &events.0 {
               match event {
                    WindowEvent::Key(Key::Space, _, action, _)
                    if *action == Action::Press || *action == Action::Repeat => {
                        to_add.push((
                            Shape::Unit {
                                obj_path: consts::BULLET_OBJ_PATH.to_owned(),
                            },
                            Damage {
                                damage: shooter_g.damage
                            },
                            Collider {
                                half_size: Vector3::new_from_array([
                                    [0.0],
                                    [0.0],
                                    [0.0]
                                ]) 
                            },
                            Transform {
                                position: shooter_t.position.clone(),
                                scale: consts::BULLET_SCALE.clone(),
                                orientation: shooter_t.orientation.clone()
                            },
                            Physics {
                                velocity: &shooter_p.velocity + &shooter_g.velocity
                            }
                        ));
                    }
                    _ => () 
               }
            }
        }
        for comps in to_add {
            let bullet = entities.create();
            shape.insert(bullet, comps.0);
            damage.insert(bullet, comps.1);
            collider.insert(bullet, comps.2);
            transform.insert(bullet, comps.3);
            physics.insert(bullet, comps.4);
        }
    }
}
