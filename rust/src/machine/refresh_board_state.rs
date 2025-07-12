use super::state::*;
use crate::board::Board;
use crate::machine::resolve_matches_state::ResolveMatchesState;
use crate::tile::Tile;
use crate::tile_node::TileNode;
use godot::classes::Tween;
use godot::prelude::*;

#[derive(Default)]
pub struct RefreshBoardState {
    tweens: Vec<Gd<Tween>>,
}

// refactor to require less iterations!
impl State for RefreshBoardState {
    fn start(&mut self, board: &mut Gd<Board>) {
        // For top row
        // col = 0
        // Note: due to borrow rules i am refactoring this to use standard for loops so
        // values can be copied on the stack
        let rows = board.bind().grid.rows();
        let cols = board.bind().grid.cols();

        for row in 0..rows {
            // Number empty in row
            let mut count = 0;
            for col in (0..cols).rev() {
                let index = (row, col);
                let tile = board.bind().get_tile(index).clone();

                match tile {
                    Some(mut tile_node) => {
                        if count > 0 {
                            let new_index = (row, col + count);
                            board
                                .bind_mut()
                                .set_tile(new_index, Some(tile_node.clone()));
                            board.bind_mut().set_tile(tile_node.bind().index, None);

                            self.tweens
                                .push(tile_node.bind_mut().tween_move(board, new_index));
                        }
                    }
                    None => {
                        count += 1;
                    }
                }
            }
            for offset in 0..count {
                let to_index = (row, offset);

                let mut tile_node = TileNode::instance_new(Tile::rand());
                // set position to tile position offseted up by one
                tile_node.set_position(
                    board.bind().index_to_vec2((row, 0))
                        - Vector2::new(0.0, board.bind().spacing * (count - offset) as f32),
                );
                // Add to tree for godot
                board.add_child(&tile_node);
                // Add move tween
                self.tweens
                    .push(tile_node.bind_mut().tween_move(board, to_index));
                // Set tile in board
                board.bind_mut().set_tile(to_index, Some(tile_node));

                //TODO
            }
            // spawn count tiles on top
        }
    }

    fn process(&mut self, board: &mut Gd<Board>, _delta: f64) -> Instruction {
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
