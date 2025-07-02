use crate::tile_node::TileNode;
use godot::{
    classes::{InputEvent, InputEventMouseMotion, PhysicsPointQueryParameters2D},
    prelude::*,
};

/// Used for capturing the tile covered by the mouse.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct Controller {
    pub hit: Option<Gd<TileNode>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for Controller {
    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(res) = event.try_cast::<InputEventMouseMotion>() {
            // Query the world 2d for a intersection
            let mut query = PhysicsPointQueryParameters2D::new_gd();
            query.set_position(res.get_position());
            let hits = self
                .base()
                .get_viewport()
                .unwrap()
                .get_world_2d()
                .unwrap()
                .get_direct_space_state()
                .unwrap()
                .intersect_point(&query);

            // Update hit field
            self.hit = if let Some(hit) = hits.front() {
                // Extract from hit and get the parent node (assumed to be TileNode)
                hit.get("collider")
                    .map(|collider| collider.to::<Gd<Node>>())
                    .and_then(|node| node.get_parent())
                    .and_then(|parent| parent.try_cast::<TileNode>().ok())
            } else {
                None
            }
        }
    }
}
