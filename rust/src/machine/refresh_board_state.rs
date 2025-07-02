use super::state::*;
use crate::board::Board;
use crate::machine::resolve_matches_state::ResolveMatchesState;
use crate::tile_node::{BoardPosition, TileNode};
use godot::classes::Tween;
use godot::prelude::*;

#[derive(Default)]
pub struct RefreshBoardState {
    tweens: Vec<Gd<Tween>>,
}

impl State for RefreshBoardState {
    fn start(&mut self, board: &Gd<Board>) {
        let mut iter = board.clone().bind_mut().grid.clone().into_iter();

        // For top row
        iter.next()
            .expect("Should have at least SIZE elems!")
            .iter()
            .enumerate()
            .for_each(|(colm, tile)| {
                if tile.is_none() {
                    // Tile is on the top row and is empty
                    // Spawn a new tile there and move it into place from above

                    let mut tile_node = TileNode::instance_new_rand();
                    let position = BoardPosition(0, colm);
                    // set position to tile position offseted up by one
                    tile_node.set_position(
                        board.bind().board_position_to_vec2(position)
                            - Vector2::new(0.0, board.bind().spacing),
                    );
                    // Add to tree for godot
                    board.clone().add_child(&tile_node);
                    // Add move tween
                    self.tweens
                        .push(tile_node.bind_mut().tween_move(board, position));
                    // Set tile in board
                    board.clone().bind_mut().set_tile(position, Some(tile_node));
                }
            });

        // For the rest
        iter.for_each(|tile| {
            tile.iter().for_each(|tile| {
                if let Some(tile) = tile.as_ref() {
                    if let Some(bellow) = tile.bind().pos.bellow() {
                        if board.bind().get_tile(bellow).is_none() {
                            // Tile is not the top or bottum row, is some, and is above a none space

                            // Move to space bellow
                            board
                                .clone()
                                .bind_mut()
                                .set_tile(bellow, Some(tile.clone()));
                            board.clone().bind_mut().set_tile(tile.bind().pos, None);

                            // Add tween to tweens
                            self.tweens
                                .push(tile.clone().bind_mut().tween_move(board, bellow));
                        }
                    }
                }
            });
        });
    }
    fn process(&mut self, board: &Gd<Board>, _delta: f64) -> Instruction {
        if self.tweens.iter_mut().all(|tween| !tween.is_running()) {
            if board.bind().needs_refresh() {
                // Needs anouther refresh
                Instruction::DropPush(Box::new(RefreshBoardState::default()))
            } else {
                let matches = board.bind().find_match_all();
                if matches.is_empty() {
                    // Board can yield control back to player
                    Instruction::Next
                } else {
                    // New matches found!
                    Instruction::DropPush(Box::new(ResolveMatchesState::new(matches)))
                }
            }
        } else {
            Instruction::Continue
        }
    }
}
