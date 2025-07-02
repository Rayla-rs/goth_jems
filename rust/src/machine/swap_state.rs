use super::state::*;
use crate::{
    board::Board,
    tile_node::{BoardPosition, TileNode},
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

    fn process(&mut self, _board: &Gd<Board>, _delta: f64) -> Instruction {
        match self
            .tweens
            .as_mut()
            .expect("Should have tween tuple!")
            .iter_mut()
            .all(|tween| !tween.is_running())
        {
            true => Instruction::Next,
            false => Instruction::Continue,
        }
    }
}
