use hecs::World;

use crate::{core::transform::Transform, game::components::velocity::Velocity};

pub fn physics_system(world: &mut World) {
	for (transform, velocity) in world.query_mut::<(&mut Transform, &Velocity)>() {
		transform.position += velocity.0;
	}
}
