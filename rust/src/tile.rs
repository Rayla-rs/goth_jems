use godot::builtin::GString;
use rand::Rng;

/// Tile enumeration used as a building block to represent the wider game board.
/// Actions involving tiles are handeled by the board and machine.
#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Dagger,
    Coffin,
    Spider,
    Bear,
    Diamond,
    Eye,
    #[default]
    Empty,
}

impl Tile {
    /// Function to get random tile varient (besides empty)
    pub fn rand() -> Tile {
        match rand::rng().random_range(0..6) {
            0 => Tile::Dagger,
            1 => Tile::Coffin,
            2 => Tile::Spider,
            3 => Tile::Bear,
            4 => Tile::Diamond,
            _ => Tile::Eye,
        }
    }

    /// Godot string representation for tile node dictionary access
    pub fn to_gstring(self) -> GString {
        GString::from(match self {
            Tile::Dagger => "dagger",
            Tile::Coffin => "coffin",
            Tile::Spider => "spider",
            Tile::Bear => "bear",
            Tile::Diamond => "diamond",
            _ => "eye",
        })
    }
}
