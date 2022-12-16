extern crate macroquad;
extern crate hecs;
extern crate minimal_physics;

use std::error::Error;
use hecs::{ComponentError, Entity, World};
use macroquad::camera::{Camera2D, set_camera};
use macroquad::color::Color;
use macroquad::input::{is_key_down, is_key_pressed, KeyCode};
use macroquad::math::vec2;
use macroquad::prelude::get_frame_time;
use macroquad::window::{clear_background, next_frame};
use minimal_physics::prelude::*;

pub struct Player {
    pub entity: Entity
}

impl Player {
    pub fn update(&mut self, world: &mut World) -> Result<(), ComponentError> {

        // Try get the actor component and return error if no actor was added
        let mut actor = world.get::<&mut Actor>(self.entity)?;
        // Try get the collider component and return error if no collider was added
        let collider = world.get::<&mut Collider>(self.entity)?;

        let mut direction = 0.0;
        if is_key_down(KeyCode::D) {
            direction += 1.0;
        }
        if is_key_down(KeyCode::A) {
            direction -= 1.0;
        }

        actor.velocity.x = direction * 150.0;

        // Apply Gravity
        actor.velocity.y -= 500.0 * get_frame_time();

        if is_key_pressed(KeyCode::Space) {
            for (_, solid_collider) in world.query::<&Collider>().with::<&Solid>().iter() {
                // Check if we are grounded
                if collider.is_overlapping(vec2i32(0, -1), solid_collider) {
                    // Jump!
                    actor.velocity.y = 400.0;

                    // We've jumped, so we can leave the loop
                    break;
                }
            }
        }

        Ok(())
    }
}


#[macroquad::main("Testing")]
async fn main() -> Result<(), Box<dyn Error>> {

    // Initialize the world for hecs
    let mut world = World::new();

    // Spawn a player
    
    let player_entity = world.spawn((
        Actor::new(),
        Collider::new(0, 0, 25, 50)
    ));
    // Create a player class
    let mut player = Player {
        // Give it an entity to be able to find components on it
        entity: player_entity
    };

    // Spawn Some Solids
    world.spawn((
        // An identifier for a solid (MUST HAVE)
        Solid,
        Collider::new(0, -150, 800, 25)
    ));


    // Main run loop
    'running: loop {
        clear_background(Color::new(0.2, 0.2, 0.2, 1.0));

        set_camera(&Camera2D {
            zoom: vec2(1.0 / 1920.0, 1.0 / 1080.0),
            ..Default::default()
        });

        // Update player
        player.update(&mut world)?;

        // Update physics
        update_physics(&mut world);

        // Draw the colliders to the screen
        for (_, collider) in world.query::<&Collider>().iter() {
            collider.debug_draw(Color::new(0.0, 0.5, 0.5, 1.0));
        }

        // Close game if esc pressed
        if is_key_pressed(KeyCode::Escape) {
            break 'running;
        }

        // Wait for next frame
        next_frame().await
    }

    Ok(())
}