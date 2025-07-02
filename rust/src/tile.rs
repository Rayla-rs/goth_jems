/// Tile enumeration used as a building block to represent the wider game board.
/// Actions involving tiles are handeled by the board and machine.
#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Diamond,
    B,
    C,
    D,
    #[default]
    Empty,
}
