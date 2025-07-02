use super::state::*;
use crate::board::Board;
use godot::prelude::*;

pub struct DefaultState;

impl State for DefaultState {
    fn process(&mut self, _board: &Gd<Board>, _delta: f64) -> Instruction {
        Instruction::Continue
    }
}
