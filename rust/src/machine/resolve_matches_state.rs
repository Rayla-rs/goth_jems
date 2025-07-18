use super::{refresh_board_state::RefreshBoardState, state::*};
use crate::board::Board;
use godot::{classes::Tween, prelude::*};
use itertools::Itertools;

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
        self.matches
            .iter()
            .flat_map(|r#match| r#match.iter())
            .unique()
            .for_each(|index| {
                board
                    .bind_mut()
                    .get_tile(*index)
                    .unwrap()
                    .bind_mut()
                    .instance_shatter_particle();
                Board::remove_tile(board, *index);
            });

        // Update ui
        board.bind_mut().update_streak(self.matches.len() as u32);
        board.bind_mut().update_hits(
            self.matches
                .iter()
                .flat_map(|r#match| r#match.iter())
                .count() as u32,
        );
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
