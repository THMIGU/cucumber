use base_input_codes::*;
use winit_input_map::{InputCode, base_input_codes, binds};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
	Up,
	Down,
	Left,
	Right,
	LTrigger,
	RTrigger,
}

pub fn bindings() -> Vec<(Action, Vec<Vec<InputCode>>)> {
	binds!(
		(Action::Up, KeyW, ArrowUp, DPadUp, LeftStickUp),
		(Action::Down, KeyS, ArrowDown, DPadDown, LeftStickDown),
		(Action::Left, KeyA, ArrowLeft, DPadLeft, LeftStickLeft),
		(Action::Right, KeyD, ArrowRight, DPadRight, LeftStickRight),
		(Action::LTrigger, LeftTrigger),
		(Action::RTrigger, RightTrigger)
	)
}
