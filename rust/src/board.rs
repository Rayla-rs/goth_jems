use crate::{
    controller::Controller,
    tile_node::{BoardPosition, TileNode},
};
use godot::{classes::Tween, prelude::*};
use itertools::Itertools;

/// Board dimensions (nxn)
pub const SIZE: usize = 8;
/// Match threshold
const THRESH: usize = 3;

/// Game board for goth bejewled!
#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct Board {
    #[export]
    pub spacing: f32,
    pub controller: Option<Gd<Controller>>,
    // Changed to be nested vectors instead of strongly typed arrays for ease of use TODO
    pub grid: [[Option<Gd<TileNode>>; SIZE]; SIZE],
    base: Base<Node2D>,
}

// TODO build board

/// Fully defined match specifying weather it is a row or colm
pub enum Match {
    Row(GeneralMatch),
    Colm(GeneralMatch),
}

#[godot_api]
impl INode2D for Board {
    fn ready(&mut self) {
        // Initialize grid with new tiles
        // TODO: prevent matches in inital group
        (0..SIZE).for_each(|x| {
            (0..SIZE).for_each(|y| {
                let mut node = TileNode::instance_new_rand();
                self.base_mut().add_child(&node);
                let pos = BoardPosition(x, y);
                node.set_global_position(self.board_position_to_vec2(pos));
                node.bind_mut().pos = pos;
                self.grid[x][y] = Some(node);
            })
        });

        // Instance controller
        let controller = Controller::new_alloc();
        self.base_mut().add_child(&controller);
        self.controller = Some(controller);
    }
}

impl Match {
    /// Number of tiles matched
    fn len(&self) -> usize {
        match self {
            Match::Row(value) => value.len(),
            Match::Colm(value) => value.len(),
        }
    }
}

/// Used for capturing info of a match without specifying weather it is a colm or row match
#[derive(Clone, Copy)]
pub struct GeneralMatch {
    offset: usize,
    start: usize,
    end: usize,
}

impl GeneralMatch {
    /// Number of tiles matched
    fn len(&self) -> usize {
        self.end - self.start
    }
}

impl Board {
    /// Get the tile hovered over from the controller
    pub fn hovered_tile(&self) -> Option<Gd<TileNode>> {
        match self.controller.as_ref() {
            Some(controller) => controller.bind().hit.clone(),
            None => None,
        }
    }

    pub fn needs_refresh(&self) -> bool {
        !self.grid.iter().flat_map(|row| row.iter()).contains(&None)
    }

    /// Converts board position to tile_node position
    pub fn board_position_to_vec2(&self, board_position: BoardPosition) -> Vector2 {
        self.base().get_position()
            + Vector2::new(
                self.spacing * (board_position.0 as f32),
                self.spacing * (board_position.1 as f32),
            )
    }

    pub fn swap(board: &Gd<Board>, a: BoardPosition, b: BoardPosition) -> [Gd<Tween>; 2] {
        assert_ne!(a, b);
        let tmp = board.bind().get_tile(a).clone();
        let b_tile = board.bind().get_tile(b).clone();
        board.clone().bind_mut().set_tile(a, b_tile);
        board.clone().bind_mut().set_tile(b, tmp);

        // Create tweens
        // a moves to a and b to b because tiles were already swaped
        // and their positions are out of sink
        [
            board
                .bind()
                .get_tile(a)
                .unwrap()
                .bind_mut()
                .tween_move(board, a),
            board
                .bind()
                .get_tile(b)
                .unwrap()
                .bind_mut()
                .tween_move(board, b),
        ]
    }

    pub fn get_tile(&self, position: BoardPosition) -> Option<Gd<TileNode>> {
        self.grid[position.0][position.1].clone()
    }

    pub fn set_tile(&mut self, position: BoardPosition, tile: Option<Gd<TileNode>>) {
        self.grid[position.0][position.1] = tile;
    }

    pub fn iter_grid(&self) -> impl Iterator<Item = &Option<Gd<TileNode>>> {
        self.grid.iter().flat_map(|row| row.iter())
    }

    /// Returns the grip for easy row iteration
    pub fn get_grid(&self) -> Vec<Vec<Option<Gd<TileNode>>>> {
        self.grid
            .iter()
            .map(|row| row.iter().cloned().collect_vec())
            .collect_vec()
    }

    /// Returns a transpose of the grid allowing for easy iteration over colms
    pub fn get_grid_transpose(&self) -> Vec<Vec<Option<Gd<TileNode>>>> {
        (0..SIZE)
            .map(|x| (0..SIZE).map(|y| self.grid[y][x].clone()).collect_vec())
            .collect_vec()
    }

    /// Find general row matches in a grid. These are later maped to the correct Match.
    /// This allows for consilidation of row and colm match finding without duplicate code.
    fn find_general_match(grid: Vec<Vec<Option<Gd<TileNode>>>>) -> Vec<GeneralMatch> {
        let mut general_matches = Vec::new();
        grid.iter().enumerate().for_each(|(offset, row)| {
            // enumerated row iterator
            let mut iter = row.iter().enumerate();
            // inital state variables
            let (mut start, mut current) = iter
                .next()
                .expect("Iterator should have SIZE elements at this point!");

            // iterate throw the row and accumulate the number of
            // repetitions. If those repititions exceed 3 add to the finds vector.
            // Greedy implimentation!
            iter.for_each(|(index, tile)| {
                // Compare internal tile states
                // Note may need to add aditional none handeling!
                if tile.as_ref().map(|tile| tile.bind().tile)
                    != current.as_ref().map(|tile| tile.bind().tile)
                {
                    // TODO: map to Option<Tile>
                    if index - start > THRESH {
                        general_matches.push(GeneralMatch {
                            offset,
                            start,
                            end: index,
                        });
                    }
                    // Reset canidate and start
                    current = tile;
                    start = index;
                }
            });
            // make sure to check end
            if SIZE - start > THRESH {
                general_matches.push(GeneralMatch {
                    offset,
                    start,
                    end: SIZE,
                });
            }
        });
        general_matches
    }

    // Gets all matches from board.
    pub fn find_match_all(&self) -> Vec<Match> {
        // Find row matches and map to Match enum
        Board::find_general_match(self.get_grid())
            .iter()
            .map(|general_match| Match::Row(*general_match))
            .chain(
                // Find colm matches using the grid transpose, map them to the Match enum type and chain them
                // with the row results
                &mut Board::find_general_match(self.get_grid_transpose())
                    .iter()
                    .map(|general_match| Match::Colm(*general_match)),
            )
            .collect()
    }

    /// Score by number of matches by the accumulated number of tiles in matches.
    /// Can include the same tile up to twice if a row and colm match both contain it.
    fn score_matches(matches: &Vec<Match>) -> usize {
        matches.len() * matches.iter().fold(0, |acc, e| acc + e.len())
    }
}
