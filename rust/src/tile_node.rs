use crate::{board::Board, tile::Tile};
use godot::{
    classes::{AnimatedSprite2D, ResourceLoader, Sprite2D, Texture2D, Tween},
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

/// Duration of move tween in seconds per tile.
const TWEEN_DURATION: f64 = 0.25f64;

impl TileNode {
    /// Moves node to new pos with a tween. Use tween.is_running() to check if
    /// it has finished executing
    pub fn tween_move(&mut self, board: &Gd<Board>, new_index: (usize, usize)) -> Gd<Tween> {
        let new_pos = board.bind().index_to_vec2(new_index);
        let duration = ((self.base().get_position().distance_to(new_pos) / board.bind().spacing)
            as f64)
            * TWEEN_DURATION;
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
            &new_pos.to_variant(),
            duration,
        );

        tween
    }

    /// Instance and setup TileNode from prefab.
    pub fn instance_new(tile: Tile) -> Gd<Self> {
        // Instance from prefab
        let mut node = ResourceLoader::singleton()
            .load("res://prefabs/tile_node.tscn")
            .unwrap()
            .cast::<PackedScene>()
            .instantiate_as::<TileNode>();

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

    /// Instance and attach shatter particle to parent of this node
    /// Particle is free'd when its animation finishes
    /// The animation is determined by the tile property of self
    pub fn instance_shatter_particle(&mut self) {
        let mut node = ResourceLoader::singleton()
            .load("res://prefabs/shatter.tscn")
            .unwrap()
            .cast::<PackedScene>()
            .instantiate_as::<AnimatedSprite2D>();

        self.base_mut()
            .get_parent()
            .as_mut()
            .unwrap()
            .add_child(&node);

        // Play anim for tile
        node.play_ex().name(self.tile.to_str()).done();

        godot::task::spawn(async move {
            node.signals().animation_finished().to_future().await;
            node.queue_free();
        });
    }
}
