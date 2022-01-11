use bevy::prelude::*;

mod states;
pub use states::*;
mod board;
pub use board::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Tic Tac Toe!".to_string(),
            width: 640.0,
            height: 400.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .init_resource::<UiTheme>()
        .add_state(PlayerTurn::X)
        .add_event::<CellClickedEvent>()
        .add_startup_system(setup_board)
        .add_system(board_cell_interaction_system)
        .add_system(on_cell_clicked)
        .run();
}
