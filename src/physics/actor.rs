use hecs::{Entity};
use macroquad::math::Vec2;
use macroquad::prelude::get_frame_time;
use crate::physics::collider::Collider;
use crate::vec2i32;

#[derive(Debug, Clone)]
pub struct Actor {
    pub velocity: Vec2,
    pub remainder: Vec2
}

impl Actor {
    pub fn new() -> Self {
        Self {
            velocity: Default::default(),
            remainder: Default::default()
        }
    }

    pub fn update(&mut self, my_collider: &mut Collider, solids: &Vec<(Entity, &Collider)>) {
        let move_amount_x = self.velocity.x * get_frame_time();

        self.move_x(move_amount_x, my_collider, solids);

        let move_amount_y = self.velocity.y * get_frame_time();

        self.move_y(move_amount_y, my_collider, solids);
    }

    pub fn move_x(&mut self, distance: f32, my_collider: &mut Collider, solids: &Vec<(Entity, &Collider)>) {
        self.remainder.x += distance;
        let mut move_amount = self.remainder.x as i32;
        self.remainder.x -= move_amount as f32;

        while move_amount != 0 {
            let step = move_amount.signum();

            for (_, solid) in solids {
                if my_collider.is_overlapping(vec2i32(step, 0), solid) {
                    self.velocity.x = 0.0;
                    return;
                }
            }

            my_collider.x += step;
            move_amount -= step;
        }
    }

    pub fn move_y(&mut self, distance: f32, my_collider: &mut Collider, solids: &Vec<(Entity, &Collider)>) {
        self.remainder.y += distance;
        let mut move_amount = self.remainder.y as i32;
        self.remainder.y -= move_amount as f32;

        while move_amount != 0 {
            let step = move_amount.signum();

            for (_, solid) in solids {
                if my_collider.is_overlapping(vec2i32(0, step), solid) {
                    self.velocity.y = 0.0;
                    return;
                }
            }

            my_collider.y += step;
            move_amount -= step;
        }
    }
}