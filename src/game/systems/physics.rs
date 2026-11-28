use std::time::Duration;

use hecs::World;

use crate::{core::transform::Transform, game::components::velocity::Velocity};

pub fn physics_system(world: &mut World, dt: Duration) {
	for (transform, velocity) in world.query_mut::<(&mut Transform, &Velocity)>() {
		transform.position += velocity.0 * dt.as_secs_f32();
	}
}
