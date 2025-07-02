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
    pub fn new(x: usize, y: usize) -> Self {
        assert!(x < SIZE);
        assert!(y < SIZE);
        Self(x, y)
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
    pub fn tween_move(&mut self, board: &Board, new_pos: BoardPosition) -> Gd<Tween> {
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
}

// instance from tile
