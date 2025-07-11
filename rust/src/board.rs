use crate::{controller::Controller, tile::Tile, tile_node::TileNode};
use godot::{classes::Tween, prelude::*};
use grid::Grid;
use itertools::Itertools;
use rand::seq::IndexedRandom;

/// Board dimensions (nxn)
pub const SIZE: usize = 8;
/// Match threshold
pub const THRESH: usize = 2;

/// Game board for goth bejewled!
#[derive(GodotClass)]
#[class(base = Node2D)]
pub struct Board {
    #[export]
    pub spacing: f32,
    pub controller: Option<Gd<Controller>>,
    pub grid: Grid<Option<Gd<TileNode>>>, // setup size on ready
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Board {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            spacing: 64.0,
            controller: None,
            grid: Grid::with_capacity(SIZE, SIZE),
            base,
        }
    }

    fn ready(&mut self) {
        // Generate grid
        self.generate(SIZE);

        // Instance controller
        let controller = Controller::new_alloc();
        self.base_mut().add_child(&controller);
        self.controller = Some(controller);
    }
}

// type Matches = Vec<Vec<Coord>>;
// type Coord = (usize, usize);

impl Board {
    /// Get the tile hovered over from the controller
    pub fn hovered_tile(&self) -> Option<Gd<TileNode>> {
        match self.controller.as_ref() {
            Some(controller) => controller.bind().hit.clone(),
            None => None,
        }
    }

    pub fn needs_refresh(&self) -> bool {
        self.grid.iter().contains(&None)
    }

    /// Converts board position to tile_node position
    pub fn index_to_vec2(&self, index: (usize, usize)) -> Vector2 {
        Vector2::new(
            self.spacing * (index.0 as f32),
            self.spacing * (index.1 as f32),
        )
    }

    pub fn swap(board: &Gd<Board>, a: (usize, usize), b: (usize, usize)) -> [Gd<Tween>; 2] {
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

    pub fn remove_tile(board: &Gd<Board>, index: (usize, usize)) {
        board.bind().get_tile(index).unwrap().queue_free();
        board.clone().bind_mut().set_tile(index, None);
        // TODO -> return tween
    }

    pub fn get_tile(&self, index: (usize, usize)) -> Option<Gd<TileNode>> {
        self.grid[index].clone()
    }

    pub fn set_tile(&mut self, index: (usize, usize), tile: Option<Gd<TileNode>>) {
        self.grid[index] = tile;
    }

    // Get all matches
    pub fn find_matches_all(&self) -> Vec<Vec<(usize, usize)>> {
        // Col matches
        let mut matches = Self::find_matches(&self.grid);
        let mut transposed = self.grid.clone();
        transposed.transpose();

        // Row matches
        // Transpose to allow for reuse of find_matches
        // Then flip the x and y back into coords for original grid
        matches.append(
            &mut Self::find_matches(&transposed)
                .iter()
                .map(|r#match| r#match.iter().map(|(x, y)| (*y, *x)).collect_vec())
                .collect_vec(),
        );
        matches
    }

    // Gets all colm matches
    fn find_matches(grid: &Grid<Option<Gd<TileNode>>>) -> Vec<Vec<(usize, usize)>> {
        // TODO: rework to look ahead for patterns instead
        // if finds pattern take it and move forward equal to its len

        // Remap map to the tiles contained by the nodes
        let grid = grid.clone().map(|elem| elem.unwrap().bind().tile);
        let mut matches = Vec::new();
        for row in 0..grid.rows() {
            // Setup state vars
            let mut start_col = 0;
            let mut start_tile = grid[(row, start_col)];

            // Iterate through colm
            for col in 1..grid.cols() {
                let current_tile = grid[(row, col)];
                if start_tile != current_tile {
                    if col - start_col > THRESH {
                        // Broke condition
                        // Add match and reset state vars
                        matches.push((start_col..col).map(|col| (row, col)).collect_vec());
                    }
                    start_col = col;
                    start_tile = current_tile;
                }
            }

            // Check for match at end
            if grid.cols() - start_col > THRESH {
                matches.push((start_col..grid.cols()).map(|col| (row, col)).collect_vec());
            }
        }

        matches
    }

    /// Score by number of matches by the accumulated number of tiles in matches.
    /// Can include the same tile up to twice if a row and colm match both contain it.
    fn score_matches(matches: &Vec<Vec<(usize, usize)>>) -> usize {
        matches.len() * matches.iter().fold(0, |acc, e| acc + e.len())
    }

    fn generate(&mut self, size: usize) {
        let mut rng = rand::rng();
        self.grid = Grid::init(size, size, None);

        // Because of the order in witch we populate the grid, we only need to check for
        // matches above and to the left of a tile
        //
        // Generate from left to right and top to bottum
        // Cannot fail because their are more than 2 tile varients!
        for x in 0..size {
            for y in 0..size {
                let index = (x, y);
                let mut possible_tiles = Tile::get_vec();
                // Check left for match
                if x >= THRESH {
                    let _ = ((x - THRESH)..x)
                        .map(|x| {
                            self.grid[(x, y)]
                                .as_ref()
                                .expect("Should have already been processed and therfore be Some!")
                                .bind()
                                .tile
                        })
                        .all_equal_value()
                        .inspect(|value| possible_tiles.retain(|elem| elem != value));
                }
                // Check up for match
                if y >= THRESH {
                    let _ = ((y - THRESH)..y)
                        .map(|y| {
                            self.grid[(x, y)]
                                .as_ref()
                                .expect("Should have already been processed and therfore be Some!")
                                .bind()
                                .tile
                        })
                        .all_equal_value()
                        .inspect(|value| possible_tiles.retain(|elem| elem != value));
                }
                // Instance new tile node with random tile from possible tiles
                let mut node = TileNode::instance_new(
                    *possible_tiles.choose_weighted(&mut rng, |_| 1).unwrap(),
                );

                // Setup node in board and scene
                self.base_mut().add_child(&node);
                node.set_position(self.index_to_vec2(index));
                node.bind_mut().index = index;
                self.grid[index] = Some(node);
            }
        }
    }
}
