use godot::classes::{InputEvent, Sprite2D};
use godot::{classes::ISprite2D, prelude::*};

mod board;
mod controller;
mod machine;
mod main_menu;
mod patterns;
mod tile;
mod tile_node;
struct GothJems;

#[gdextension]
unsafe impl ExtensionLibrary for GothJems {}

pub const SELECT_ACTION: &str = "select";
pub const DESELECT_ACTION: &str = "deselect";
