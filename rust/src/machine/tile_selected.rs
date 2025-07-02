use super::state::{Instruction, State};
use crate::{board::Board, tile_node::TileNode};
use godot::{classes::InputEvent, prelude::*};

pub struct TileSelected(pub Gd<TileNode>);

impl State for TileSelected {
    fn process(&mut self, _board: &Gd<Board>, _delta: f64) -> Instruction {
        Instruction::Continue
    }

    fn input(&mut self, _board: &Gd<Board>, _input: Gd<InputEvent>) -> Instruction {
        // on deselct or select same deselect
        // on select other drop push swap

        // need to track current node being hovered over
        todo!()
        // look for select other tile
        // deselect
        // more..
        // DropPush swap
    }
}
