use bevy::prelude::*;

use crate::{GameState, PlayerTurn, PlayingState, UiTheme};

#[derive(Component)]
struct ReloadButton;

pub struct NewGamePlugin;

impl Plugin for NewGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(PlayingState::Local).with_system(setup_restart_button),
        )
        .add_system(reload_button_interactions)
        .add_system_set(SystemSet::on_enter(PlayingState::NotPlaying).with_system(reload_game));
    }
}

fn root() -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::FlexEnd,
            padding: Rect {
                left: Val::Px(0.),
                right: Val::Px(20.),
                top: Val::Px(20.),
                bottom: Val::Px(0.),
            },
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    }
}

pub fn button(theme: &Res<UiTheme>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            padding: Rect::all(Val::Px(5.)),
            size: Size::new(Val::Auto, Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: theme.button.clone(),
        ..Default::default()
    }
}

pub fn button_text(
    asset_server: &Res<AssetServer>,
    theme: &Res<UiTheme>,
    label: &str,
) -> TextBundle {
    return TextBundle {
        text: Text::with_section(
            label,
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 20.0,
                color: theme.button_text.clone(),
            },
            Default::default(),
        ),
        ..Default::default()
    };
}

fn setup_restart_button(
    mut commands: Commands,
    theme: Res<UiTheme>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(root()).with_children(|parent| {
        parent
            .spawn_bundle(button(&theme))
            .with_children(|parent| {
                parent.spawn_bundle(button_text(&asset_server, &theme, "New Game"));
            })
            .insert(ReloadButton);
    });
}

fn reload_button_interactions(
    theme: Res<UiTheme>,
    mut buttons: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, With<ReloadButton>),
    >,
    mut game_state: ResMut<State<PlayingState>>,
) {
    for (interaction, mut color) in buttons.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = theme.button;
                game_state
                    .set(PlayingState::NotPlaying)
                    .expect("Could not set game state.");
            }
            Interaction::Hovered => *color = theme.button_hovered,
            Interaction::None => *color = theme.button,
        }
    }
}

fn reload_game(
    mut commands: Commands,
    query: Query<Entity>,
    mut playing_state: ResMut<State<PlayingState>>,
    mut game_state: ResMut<State<GameState>>,
    mut player_turn: ResMut<State<PlayerTurn>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    playing_state
        .set(PlayingState::Local)
        .expect("Could not set game state.");

    if game_state.current() != &GameState::GameOngoing {
        game_state
            .set(GameState::GameOngoing)
            .expect("Could not set game state.");
    }

    if player_turn.current() != &PlayerTurn::X {
        player_turn
            .set(PlayerTurn::X)
            .expect("Could not set game state.");
    }
}
