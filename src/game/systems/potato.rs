use std::time::Duration;

use hecs::World;

use crate::{core::transform::Transform, game::components::potato::Potato};

pub fn update_potato(world: &mut World, dt: Duration) {
	for (_, transform) in world.query_mut::<(&Potato, &mut Transform)>() {
		transform.rotation += 60_f32.to_radians() * dt.as_secs_f32();
	}
}
