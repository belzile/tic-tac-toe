use bevy::prelude::*;

mod ui;
pub use ui::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Tic Tac To!".to_string(),
            width: 640.0,
            height: 400.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .init_resource::<UiTheme>()
        .add_startup_system(setup_ui)
        .add_system(button_system.system())
        .run();
}
