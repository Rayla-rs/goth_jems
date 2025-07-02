use crate::{
    board::{Board, SIZE},
    tile::Tile,
};
use godot::{classes::Tween, prelude::*};

#[derive(Default)]
pub enum TileState {
    Matched,
    #[default]
    Default,
}

/// Postion of tile on board bounded by 0 and size.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct BoardPosition(pub usize, pub usize);

impl BoardPosition {
    /// Gets the position bellow itself if there is one
    pub fn bellow(self) -> Option<BoardPosition> {
        let BoardPosition(x, y) = self;
        x.checked_sub(1).map(|x| BoardPosition(x, y))
    }

    /// Gets the position above the board position.
    pub fn above(self) -> BoardPosition {
        BoardPosition(self.0 + 1, self.1)
    }
}

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct TileNode {
    pub pos: BoardPosition,
    // TODO set to random value on alloc
    pub tile: Tile,
    base: Base<Node2D>,
}

/// Duration of move tween in seconds.
const TWEEN_DURATION: f64 = 1f64;

impl TileNode {
    /// Moves node to new pos with a tween. Use tween.is_running() to check if
    /// it has finished executing
    pub fn tween_move(&mut self, board: &Gd<Board>, new_pos: BoardPosition) -> Gd<Tween> {
        self.pos = new_pos;

        // Create move tween
        let mut tween = self
            .base_mut()
            .create_tween()
            .expect("Tween should be created!");

        // Tween position to the new pos over TWEEN_DURATION.
        tween.tween_property(
            &self.base().clone(),
            "position",
            &board.board_position_to_vec2(new_pos).to_variant(),
            TWEEN_DURATION,
        );

        tween
    }

    pub fn instance_new_rand() -> Gd<Self> {
        todo!()
    }
}

// instance from tile
