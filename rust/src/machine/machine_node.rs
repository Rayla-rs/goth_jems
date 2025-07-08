use crate::{board::Board, machine::machine::*};
use godot::{classes::InputEvent, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct MachineNode {
    #[export]
    board: Option<Gd<Board>>,
    machine: Machine,
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for MachineNode {
    fn process(&mut self, delta: f64) {
        self.machine
            .update(MachineUpdate::process(self.board.as_mut().unwrap(), delta));
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        self.machine
            .update(MachineUpdate::input(self.board.as_mut().unwrap(), &event));
    }
}
