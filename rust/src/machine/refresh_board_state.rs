use super::state::*;
use crate::board::Board;
use crate::machine::resolve_matches_state::ResolveMatchesState;
use crate::tile_node::TileNode;
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
        // col = 0
        board.bind().grid.indexed_iter().for_each(|(index, tile)| {
            if index.1 == 0 {
                if tile.is_none() {
                    // Tile is on the top row and is empty
                    // Spawn a new tile there and move it into place from above

                    let mut tile_node = TileNode::instance_new_rand();
                    // set position to tile position offseted up by one
                    tile_node.set_position(
                        board.bind().index_to_vec2(index) - Vector2::new(0.0, board.bind().spacing),
                    );
                    // Add to tree for godot
                    board.clone().add_child(&tile_node);
                    // Add move tween
                    self.tweens
                        .push(tile_node.bind_mut().tween_move(board, index));
                    // Set tile in board
                    board.clone().bind_mut().set_tile(index, Some(tile_node));
                }
            } else {
                if let Some(tile) = tile.as_ref() {
                    let bellow_index = (tile.bind().index.0, tile.bind().index.1 + 1);
                    if let Some(bellow) = board.bind().grid.get(bellow_index.0, bellow_index.1) {
                        if bellow.is_none() {
                            // Tile is not the top or bottum row, is some, and is above a none space

                            // Move to space bellow
                            board
                                .clone()
                                .bind_mut()
                                .set_tile(bellow_index, Some(tile.clone()));
                            board.clone().bind_mut().set_tile(tile.bind().index, None);

                            // Add tween to tweens
                            self.tweens
                                .push(tile.clone().bind_mut().tween_move(board, bellow_index));
                        }
                    }
                }
            }
        });
    }

    fn process(&mut self, board: &Gd<Board>, _delta: f64) -> Instruction {
        if self.tweens.iter_mut().all(|tween| !tween.is_running()) {
            if board.bind().needs_refresh() {
                // Needs anouther refresh
                Instruction::DropPush(Box::new(RefreshBoardState::default()))
            } else {
                let matches = board.bind().find_matches_all();
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
