use godot::{classes::Control, prelude::*};

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct MainMenu {
    base: Base<Control>,
}

#[godot_api]
impl MainMenu {
    #[func]
    fn exit_game(&self) {
        self.base()
            .get_tree()
            .expect("Main menu should be in scene tree when exit_game is called!")
            .quit();
    }

    #[func]
    fn play_game() {
        todo!()
    }
}
