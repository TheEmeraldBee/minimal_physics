use hecs::World;
use crate::physics::actor::Actor;

use crate::physics::collider::Collider;

pub mod collider;
pub mod actor;

/// An identifier for the physics engine to correctly query objects
#[derive(Clone, Debug)]
pub struct Solid;

pub fn update_physics(world: &mut World) {
    for (_, (actor, collider)) in &mut world.query::<(&mut Actor, &mut Collider)>() {
        actor.update(collider, &world.query::<&Collider>().with::<&Solid>().iter().collect::<Vec<_>>());
    }
}