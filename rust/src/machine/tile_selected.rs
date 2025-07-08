use super::state::{Instruction, State};
use crate::{
    DESELECT_ACTION, SELECT_ACTION, board::Board, machine::swap_state::SwapState,
    tile_node::TileNode,
};
use godot::{
    classes::{InputEvent, tween::TransitionType},
    prelude::*,
};

pub struct TileSelected(pub Gd<TileNode>);

impl State for TileSelected {
    fn start(&mut self, _board: &mut Gd<Board>) {
        // Select anim
        let mut tween = self.0.bind_mut().base_mut().create_tween().unwrap();
        tween.tween_property(
            &self.0.bind().base().clone(),
            "scale",
            &Vector2::new(0.75, 0.75).to_variant(),
            0.1,
        );
        tween.set_trans(TransitionType::EXPO);
    }

    fn end(&mut self, _board: &mut Gd<Board>) {
        // Deselect anim
        let mut tween = self.0.bind_mut().base_mut().create_tween().unwrap();
        tween.tween_property(
            &self.0.bind().base().clone(),
            "scale",
            &Vector2::new(1.0, 1.0).to_variant(),
            0.1,
        );
        tween.set_trans(TransitionType::EXPO);
    }

    fn process(&mut self, _board: &mut Gd<Board>, _delta: f64) -> Instruction {
        Instruction::Continue
    }

    fn input(&mut self, board: &mut Gd<Board>, input: Gd<InputEvent>) -> Instruction {
        if input.is_action_pressed(DESELECT_ACTION) {
            Instruction::Next
        } else if input.is_action_pressed(SELECT_ACTION) {
            if let Some(hover) = board.bind().hovered_tile() {
                let a = self.0.bind().index;
                let b = hover.bind().index;
                // Are a and b ajacent
                if neighbours(a, b) {
                    return Instruction::DropPush(Box::new(SwapState::new(a, b, false)));
                }
            }
            // On fail treat as deselect
            return Instruction::Next;
        } else {
            Instruction::Continue
        }
    }
}

fn neighbours(a: (usize, usize), b: (usize, usize)) -> bool {
    a != b
        && [
            (a.0 + 1, a.1),
            (a.0.saturating_sub(1), a.1),
            (a.0, a.1 + 1),
            (a.0, a.1.saturating_sub(1)),
        ]
        .contains(&b)
}
