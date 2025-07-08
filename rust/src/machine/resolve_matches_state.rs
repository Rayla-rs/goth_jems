use super::{refresh_board_state::RefreshBoardState, state::*};
use crate::board::Board;
use godot::{classes::Tween, prelude::*};

/// State for resolving actions related to a match occuring on the board.
pub struct ResolveMatchesState {
    matches: Vec<Vec<(usize, usize)>>,
    tweens: Vec<Tween>,
}

impl ResolveMatchesState {
    pub fn new(matches: Vec<Vec<(usize, usize)>>) -> Self {
        Self {
            matches,
            tweens: vec![],
        }
    }
}

impl State for ResolveMatchesState {
    fn start(&mut self, board: &mut Gd<Board>) {
        //TODO create destroy tweens
        // create score ui tween
        self.matches
            .iter()
            .flat_map(|r#match| r#match.iter())
            .for_each(|index| {
                Board::remove_tile(board, *index);
            });
    }

    fn process(&mut self, board: &mut Gd<Board>, _delta: f64) -> Instruction {
        if self.tweens.iter_mut().all(|tween| !tween.is_running()) {
            // Start refreshing board state to return to rest
            if board.bind().needs_refresh() {
                Instruction::DropPush(Box::new(RefreshBoardState::default()))
            } else {
                // Refresh not needed
                Instruction::Next
            }
        } else {
            // Still tweening
            Instruction::Continue
        }
    }
}
