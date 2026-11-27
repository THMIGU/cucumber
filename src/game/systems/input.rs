use hecs::World;
use winit_input_map::InputMap;

use crate::{core::input::Action, game::components::player::PlayerMovement};

pub fn input_system(world: &mut World, input: &InputMap<Action>) {
	for movement in world.query_mut::<&mut PlayerMovement>() {
		let h_axis = input.axis(Action::Right, Action::Left);
		let v_axis = input.axis(Action::Down, Action::Up);

		movement.0.x = h_axis;
		movement.0.y = v_axis;
	}
}
