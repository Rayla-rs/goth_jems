use super::{refresh_board_state::RefreshBoardState, state::*};
use crate::board::{Board, Match};
use godot::{classes::Tween, prelude::*};

/// State for resolving actions related to a match occuring on the board.
pub struct ResolveMatchesState {
    matches: Vec<Match>,
    tweens: Vec<Tween>,
}

impl ResolveMatchesState {
    pub fn new(matches: Vec<Match>) -> Self {
        Self {
            matches,
            tweens: vec![],
        }
    }
}

impl State for ResolveMatchesState {
    fn start(&mut self, _board: &Gd<Board>) {
        //TODO create destroy tweens
        // create score tween
    }

    fn process(&mut self, _board: &Gd<Board>, _delta: f64) -> Instruction {
        if self.tweens.iter_mut().all(|tween| !tween.is_running()) {
            // Start refreshing board state to return to rest
            Instruction::DropPush(Box::new(RefreshBoardState::default()))
        } else {
            Instruction::Continue
        }
    }
}
