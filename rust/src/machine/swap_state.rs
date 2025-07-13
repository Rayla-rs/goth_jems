use super::state::*;
use crate::{board::Board, machine::resolve_matches_state::ResolveMatchesState};
use godot::{classes::Tween, prelude::*};

/// Swaps two tiles in the board
pub struct SwapState {
    pub a: (usize, usize),
    pub b: (usize, usize),
    tweens: Option<[Gd<Tween>; 2]>,
    swaping_back: bool,
}

impl SwapState {
    pub fn new(a: (usize, usize), b: (usize, usize), swaping_back: bool) -> Self {
        Self {
            a,
            b,
            tweens: None,
            swaping_back,
        }
    }
}

impl State for SwapState {
    fn start(&mut self, board: &mut Gd<Board>) {
        self.tweens = Some(Board::swap(board, self.a, self.b));
    }

    fn process(&mut self, board: &mut Gd<Board>, _delta: f64) -> Instruction {
        match self
            .tweens
            .as_mut()
            .expect("Should have tween tuple!")
            .iter_mut()
            .all(|tween| !tween.is_running())
        {
            true => {
                // handle back swap case
                if self.swaping_back {
                    Instruction::Next
                } else {
                    let matches = board.bind().find_matches_all();
                    if matches.is_empty() {
                        // Swap failed so undo action
                        Instruction::DropPush(Box::new(SwapState::new(self.b, self.a, true)))
                    } else {
                        //TODO update board state for score

                        // Swap succeeded (board has match) so resolve matches
                        Instruction::DropPush(Box::new(ResolveMatchesState::new(matches)))
                    }
                }
            }
            // Not done so continue
            false => Instruction::Continue,
        }
    }
}
