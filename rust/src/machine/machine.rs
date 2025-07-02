use crate::board::Board;

use super::default_state::DefaultState;
use super::state::*;
use godot::{classes::InputEvent, obj::Gd};

/// Generic state machine for rust-godot I created for my personal game projects.
pub struct Machine {
    stack: Vec<Box<dyn State>>,
}

pub struct MachineUpdate<'a>(&'a Gd<Board>, MachineUpdateVarients);

impl<'a> MachineUpdate<'a> {
    pub fn process(board: &'a Gd<Board>, delta: f64) -> MachineUpdate<'a> {
        MachineUpdate(board, MachineUpdateVarients::Delta(delta))
    }
    pub fn input(board: &'a Gd<Board>, event: &Gd<InputEvent>) -> MachineUpdate<'a> {
        MachineUpdate(board, MachineUpdateVarients::Input(event.clone()))
    }
}

enum MachineUpdateVarients {
    Delta(f64),
    Input(Gd<InputEvent>),
}

impl Default for Machine {
    fn default() -> Self {
        Machine::new(Box::new(DefaultState))
    }
}

impl Machine {
    /// Create machine with initial state.
    pub fn new(initial_state: Box<dyn State>) -> Self {
        Self {
            stack: vec![initial_state],
        }
    }

    /// Handle machine updates
    pub fn update(&mut self, args: MachineUpdate) {
        let MachineUpdate(board, varient) = args;
        if !self.stack.is_empty() {
            // pop front state for mutable use and so we can use the stack
            let mut current = self.stack.remove(0);
            match match varient {
                MachineUpdateVarients::Delta(delta) => current.process(board, delta),
                MachineUpdateVarients::Input(event) => current.input(board, event),
            } {
                Instruction::Continue => {
                    // reinsert current state
                    self.stack.insert(0, current);
                }
                Instruction::Next => {
                    // handle current end
                    // current will be droped when is leaves scope
                    current.end(board);

                    // try to resume next state
                    if let Some(first) = self.stack.first_mut() {
                        first.resume(board);
                    }
                }
                Instruction::Push(mut state) => {
                    // pause current
                    current.pause(board);

                    // setup state
                    state.start(board);

                    // make sure to add current state back into stack
                    // append new state befor it
                    self.stack.insert(0, current);
                    self.stack.insert(0, state);
                }
                Instruction::DropPush(mut state) => {
                    // end current state
                    current.end(board);

                    //setup next state
                    state.start(board);

                    // add new state and let current be dropped
                    self.stack.insert(0, state);
                }
            }
        }
    }
}
