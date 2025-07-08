use crate::board::Board;

use super::state::*;
use godot::{classes::Tween, prelude::*};

/// A non saveable state used for moving a unit given an edge that
/// refences that unit via the from vertex.
/// This class cannot be instanced by godot.
///
/// Currently tweening is only implimented for simple movements but
/// this class should be usable for more complex movements like
/// jumping and the like!
pub struct MoveState {
    // edge: Gd<Edge>,
    // tween: Gd<Tween>,
}
impl State for MoveState {
    fn start(&mut self, _board: &mut Gd<Board>) {
        // self.tween.play();
    }

    fn process(&mut self, _board: &mut Gd<Board>, _delta: f64) -> Instruction {
        todo!()
    }
}
