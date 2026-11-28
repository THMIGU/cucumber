use hecs::World;
use winit_input_map::InputMap;

use crate::{
	core::{input::Action, transform::Transform},
	game::components::player::{Player, PlayerMovement},
};

pub fn input_system(world: &mut World, input: &InputMap<Action>) {
	for movement in world.query_mut::<&mut PlayerMovement>() {
		let h_axis = input.axis(Action::Right, Action::Left);
		let v_axis = input.axis(Action::Down, Action::Up);

		movement.0.x = h_axis;
		movement.0.y = v_axis;
	}

	for (_, transform) in world.query_mut::<(&Player, &mut Transform)>() {
		let left_trigger = input.value(Action::LTrigger);
		let right_trigger = input.value(Action::RTrigger);

		transform.scale.x = 30.0 + left_trigger * 100.0;
		transform.scale.y = 30.0 + right_trigger * 100.0;
	}
}
