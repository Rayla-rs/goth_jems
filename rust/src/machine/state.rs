use crate::board::Board;
use godot::{classes::InputEvent, obj::Gd};

/// State machine instruction to allow states to update the machine.
pub enum Instruction {
    Continue,
    Next,
    Push(Box<dyn State>),
    DropPush(Box<dyn State>),
}

/// State trait to impliment behaviour for the state machine
pub trait State {
    fn process(&mut self, _board: &Gd<Board>, _delta: f64) -> Instruction;
    fn input(&mut self, _board: &Gd<Board>, _input: Gd<InputEvent>) -> Instruction {
        Instruction::Continue
    }
    fn start(&mut self, _board: &Gd<Board>) {}
    fn end(&mut self, _board: &Gd<Board>) {}
    fn pause(&mut self, _board: &Gd<Board>) {}
    fn resume(&mut self, _board: &Gd<Board>) {}
}
