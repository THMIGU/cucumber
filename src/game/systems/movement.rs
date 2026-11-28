use std::time::Duration;

use hecs::World;

use crate::game::components::{
	player::{PlayerMovement, PlayerSpeed},
	velocity::Velocity,
};

pub fn movement_system(world: &mut World, dt: Duration) {
	let delta = dt.as_secs_f32();
	let damping = 10.0;
	let decay = (-damping * delta).exp();

	for (velocity, movement, speed) in
		world.query_mut::<(&mut Velocity, &PlayerMovement, &PlayerSpeed)>()
	{
		let input = movement
			.0
			.clamp_length_max(1.0);
		let acceleration = input * speed.0;

		velocity.0 = velocity.0 * decay + acceleration * ((1.0 - decay) / damping);
	}
}
