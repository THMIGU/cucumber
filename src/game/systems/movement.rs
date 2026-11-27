use std::time::Duration;

use hecs::World;

use crate::game::components::{
	player::{PlayerMovement, PlayerSpeed},
	velocity::Velocity,
};

pub fn movement_system(world: &mut World, dt: Duration) {
	for (velocity, movement, speed) in
		world.query_mut::<(&mut Velocity, &PlayerMovement, &PlayerSpeed)>()
	{
		let delta = dt.as_secs_f32();

		velocity.0 *= (-10.0 * delta).exp();
		velocity.0 += movement
			.0
			.clamp_length_max(1.0)
			* speed.0 * delta;
	}
}
