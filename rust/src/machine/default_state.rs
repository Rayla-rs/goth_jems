use super::{state::*, tile_selected::TileSelected};
use crate::{SELECT_ACTION, board::Board};
use godot::{classes::InputEvent, prelude::*};

pub struct DefaultState;

/// Base state for state machine. Added by default.
impl State for DefaultState {
    fn resume(&mut self, board: &mut Gd<Board>) {
        // Update ui
        board.bind_mut().resolve_interum();
    }
    fn process(&mut self, _board: &mut Gd<Board>, _delta: f64) -> Instruction {
        Instruction::Continue
    }

    fn input(&mut self, board: &mut Gd<Board>, input: Gd<InputEvent>) -> Instruction {
        if input.is_action_pressed(SELECT_ACTION) {
            if let Some(tile) = board.bind().hovered_tile() {
                // Push selected state because tile node was selected
                Instruction::Push(Box::new(TileSelected(tile)))
            } else {
                Instruction::Continue
            }
        } else {
            Instruction::Continue
        }
    }
}
