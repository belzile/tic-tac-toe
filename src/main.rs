use bevy::prelude::*;

mod states;
pub use states::*;
mod components;
pub use components::*;
mod board;
pub use board::*;
mod winning_logic;
pub use winning_logic::*;
mod game_instructions;
pub use game_instructions::*;
mod new_game;
pub use new_game::*;

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
        .add_state(PlayingState::Local)
        .add_state(PlayerTurn::X)
        .add_state(GameState::GameOngoing)
        .add_plugin(BoardPlugin)
        .add_plugin(WinningLogicPlugin)
        .add_plugin(GameInstructionsPlugin)
        .add_plugin(NewGamePlugin)
        .run();
}
