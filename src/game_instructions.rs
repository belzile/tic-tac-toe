use bevy::prelude::*;

use crate::{GameState, Player, PlayerTurn, PlayingState, UiTheme};

#[derive(Component)]
struct InstructionText;

pub struct GameInstructionsPlugin;

impl Plugin for GameInstructionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(PlayingState::Local).with_system(setup_instructions),
        )
        .add_system_set(
            SystemSet::on_update(PlayingState::Local)
                .with_system(update_instruction_on_state_change),
        );
    }
}

fn root() -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            padding: Rect {
                left: Val::Px(0.),
                right: Val::Px(0.),
                top: Val::Px(20.),
                bottom: Val::Px(0.),
            },
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    }
}

fn text(asset_server: &Res<AssetServer>, theme: &Res<UiTheme>, label: &str) -> TextBundle {
    return TextBundle {
        text: Text::with_section(
            label,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: theme.button_text.clone(),
            },
            Default::default(),
        ),
        ..Default::default()
    };
}

fn setup_instructions(mut commands: Commands, theme: Res<UiTheme>, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(root()).with_children(|parent| {
        parent
            .spawn_bundle(text(&asset_server, &theme, "Test"))
            .insert(InstructionText);
    });
}

fn update_instruction_on_state_change(
    player_turn_state: Res<State<PlayerTurn>>,
    game_state: Res<State<GameState>>,
    mut instructions: Query<&mut Text, With<InstructionText>>,
) {
    if player_turn_state.is_changed() {
        let next_text = match player_turn_state.current() {
            &PlayerTurn::X => "Player's turn: X",
            _ => "Player's turn: O",
        };
        let mut ui_text = instructions.single_mut();
        ui_text.sections[0].value = next_text.to_string();
    }

    if game_state.is_changed() {
        let mut ui_text = instructions.single_mut();

        match game_state.current() {
            &GameState::Won(Player::X) => ui_text.sections[0].value = "X Won!!!".to_string(),
            &GameState::Won(Player::O) => ui_text.sections[0].value = "O Won!!!".to_string(),
            &GameState::Draw => ui_text.sections[0].value = "Draw :-(".to_string(),
            &GameState::GameOngoing => (),
        }
    }
}
