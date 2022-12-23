use std::borrow::Borrow;
use hecs::{Entity, World};
use macroquad::math::{Vec2, vec2};
use macroquad::time::get_frame_time;
use crate::physics::actor::Actor;

use crate::physics::collider::Collider;
use crate::vec2i32;

pub mod collider;
pub mod actor;

/// An identifier for the physics engine to correctly query objects
#[derive(Clone, Debug)]
pub struct Solid {
    pub remainder: Vec2,
    pub velocity: Vec2,
}

impl Solid {
    pub fn new() -> Self {
        Self {
            remainder: Default::default(),
            velocity: Default::default()
        }
    }

    pub fn update(&mut self, my_entity: Entity, my_collider: &mut Collider, actors: Vec<(Entity, (&mut Actor, &mut Collider))>, solids: Vec<(Entity, (&Collider))>) {
        self.remainder.x += self.velocity.x * get_frame_time();
        self.remainder.y += self.velocity.y * get_frame_time();

        let move_x = self.remainder.x as i32;
        let move_y = self.remainder.y as i32;

        self.remainder -= vec2(move_x as f32, move_y as f32);

        my_collider.enabled = false;

        my_collider.x += move_x;
        my_collider.y += move_y;

        for (_, (mut actor, mut collider)) in actors {

            // Moving Right
            if my_collider.is_overlapping(vec2i32(move_x.signum(), 0), collider) {
                if move_x > 0 {
                    actor.move_x((my_collider.max_x() - collider.min_x()) as f32, collider, &solids);
                } else if move_x < 0 {
                    actor.move_x((my_collider.min_x() - collider.max_x()) as f32, collider, &solids);
                }
            }
            if my_collider.is_overlapping(vec2i32(0, move_y.signum()), collider) {
                if move_y > 0 {
                    actor.move_y((my_collider.max_y() - collider.min_y()) as f32, collider, &solids);
                } else if move_y < 0 {
                    actor.move_y((my_collider.min_y() - collider.max_y()) as f32, collider, &solids);
                }
            }

            else if actor.is_riding(my_entity) {
                actor.move_x(move_x as f32, collider, &solids);
                actor.move_y(move_y as f32, collider, &solids);
            }
        }

    }
}

pub fn update_physics(world: &mut World) {
    for (_, (actor, collider)) in world.query::<(&mut Actor, &mut Collider)>().iter() {
        actor.update(collider, &world.query::<&Collider>().with::<&Solid>().iter().collect::<Vec<_>>());
    }

    for (entity, (solid, collider)) in world.query::<(&mut Solid, &mut Collider)>().iter() {
        solid.update(entity,
                     collider,
                     world.query::<(&mut Actor, &mut Collider)>().iter().collect::<Vec<_>>(),
                     world.query::<(&Collider)>().with::<&Solid>().iter().collect::<Vec<_>>()
        );
    }
}