use crate::{board::Board, tile::Tile};
use godot::{
    classes::{ResourceLoader, Sprite2D, Texture2D, Tween},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct TileNode {
    #[export]
    image_dict: Dictionary,
    #[export]
    sprite: Option<Gd<Sprite2D>>,
    pub index: (usize, usize),
    // TODO set to random value on alloc
    pub tile: Tile,
    base: Base<Node2D>,
}

/// Duration of move tween in seconds.
const TWEEN_DURATION: f64 = 1f64;

impl TileNode {
    /// Moves node to new pos with a tween. Use tween.is_running() to check if
    /// it has finished executing
    pub fn tween_move(&mut self, board: &Gd<Board>, new_index: (usize, usize)) -> Gd<Tween> {
        self.index = new_index;

        // Create move tween
        let mut tween = self
            .base_mut()
            .create_tween()
            .expect("Tween should be created!");

        // Tween position to the new pos over TWEEN_DURATION.
        tween.tween_property(
            &self.base().clone(),
            "position",
            &board.bind().index_to_vec2(new_index).to_variant(),
            TWEEN_DURATION,
        );

        tween
    }

    /// Instance and setup TileNode from prefab.
    pub fn instance_new_rand() -> Gd<Self> {
        // Instance from prefab
        let mut node = ResourceLoader::singleton()
            .load("res://prefabs/tile_node.tscn")
            .unwrap()
            .cast::<PackedScene>()
            .instantiate_as::<TileNode>();

        // Get random tile
        let tile = Tile::rand();
        node.bind_mut().tile = tile;

        // Update sprite
        node.bind().sprite.as_ref().unwrap().clone().set_texture(
            &node
                .bind()
                .image_dict
                .get(tile.to_gstring())
                .unwrap()
                .to::<Gd<Texture2D>>(),
        );
        node
    }
}

// instance from tile
