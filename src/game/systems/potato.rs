use std::time::Duration;

use hecs::World;

use crate::{core::transform::Transform, game::components::potato::Potato};

pub fn update_potato(world: &mut World, dt: Duration, accumulator: Duration) {
	for (_, transform) in world.query_mut::<(&Potato, &mut Transform)>() {
		transform.rotation += 2.0 * dt.as_secs_f32();
		transform.position.x = (accumulator.as_secs_f32() / 0.5).cos() * 50.0;
		transform.position.y = (accumulator.as_secs_f32() / 0.5).sin() * 50.0;
	}
}
