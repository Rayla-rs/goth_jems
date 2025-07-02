use super::state::*;
use crate::{
    board::Board, machine::resolve_matches_state::ResolveMatchesState, tile_node::BoardPosition,
};
use godot::{classes::Tween, prelude::*};

/// Swaps two tiles in the board
pub struct SwapState {
    pub a: BoardPosition,
    pub b: BoardPosition,
    tweens: Option<[Gd<Tween>; 2]>,
}

impl SwapState {
    pub fn new(a: BoardPosition, b: BoardPosition) -> Self {
        Self { a, b, tweens: None }
    }
}

impl State for SwapState {
    fn start(&mut self, board: &Gd<Board>) {
        self.tweens = Some(board.clone().bind_mut().swap(self.a, self.b));
    }

    fn process(&mut self, board: &Gd<Board>, _delta: f64) -> Instruction {
        match self
            .tweens
            .as_mut()
            .expect("Should have tween tuple!")
            .iter_mut()
            .all(|tween| !tween.is_running())
        {
            true => {
                let matches = board.bind().find_match_all();
                if !matches.is_empty() {
                    // Swap failed so undo action
                    Instruction::DropPush(Box::new(SwapState::new(self.b, self.a)))
                } else {
                    // Swap succeeded (board has match) so resolve matches
                    Instruction::DropPush(Box::new(ResolveMatchesState::new(matches)))
                }
            }
            // Not done so continue
            false => Instruction::Continue,
        }
    }
}
