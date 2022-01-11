use bevy::prelude::*;

use crate::PlayerTurn;

pub struct CellClickedEvent {
    entity: Entity,
}

#[derive(PartialEq)]
pub enum CellState {
    Empty,
    X,
    O,
}

#[derive(Component)]
pub struct TicTacToeCell {
    pub cell_id: u8,
    pub state: CellState,
}

pub struct UiTheme {
    pub root: UiColor,
    pub border: UiColor,
    pub menu: UiColor,
    pub button: UiColor,
    pub button_hovered: UiColor,
    pub button_pressed: UiColor,
    pub button_text: Color,
}

impl FromWorld for UiTheme {
    fn from_world(_: &mut World) -> Self {
        UiTheme {
            root: Color::NONE.into(),
            border: Color::rgb(0.65, 0.65, 0.65).into(),
            menu: Color::rgb(0.15, 0.15, 0.15).into(),
            button: Color::rgb(0.15, 0.15, 0.15).into(),
            button_hovered: Color::rgb(0.35, 0.75, 0.35).into(),
            button_pressed: Color::rgb(0.35, 0.75, 0.35).into(),
            button_text: Color::WHITE,
        }
    }
}

pub fn board_cell_interaction_system(
    theme: Res<UiTheme>,
    mut send_cell_clicked: EventWriter<CellClickedEvent>,
    mut buttons: Query<
        (&Interaction, &mut UiColor, &TicTacToeCell, Entity),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, cell, entity) in buttons.iter_mut() {
        if cell.state != CellState::Empty {
            return;
        }

        match *interaction {
            Interaction::Clicked => {
                send_cell_clicked.send(CellClickedEvent { entity });
                *color = theme.button;
            }
            Interaction::Hovered => *color = theme.button_hovered,
            Interaction::None => *color = theme.button,
        }
    }
}

pub fn on_cell_clicked(
    mut events: EventReader<CellClickedEvent>,
    mut cell_query: Query<(&mut TicTacToeCell, &Children)>,
    mut cell_text_query: Query<&mut Text>,
    mut player_turn_state: ResMut<State<PlayerTurn>>,
) {
    let player_turn = player_turn_state.current().clone();

    for event in events.iter() {
        let (mut cell, children) = cell_query
            .get_mut(event.entity)
            .expect("on_cell_clicked: Cell not found.");

        update_cell_state(&mut cell, &player_turn);
        update_cell_text(&mut cell_text_query, children, &player_turn);
        update_player_turn(&mut player_turn_state);
    }
}

fn update_cell_state(cell: &mut Mut<TicTacToeCell>, player_turn: &PlayerTurn,) {
    cell.state = match player_turn {
        PlayerTurn::X => CellState::X,
        PlayerTurn::O => CellState::O,
    };
}

fn update_cell_text(
    cell_text_query: &mut Query<&mut Text>,
    children: &Children,
    player_turn: &PlayerTurn,
) {
    let text = match player_turn {
        PlayerTurn::X => "X",
        PlayerTurn::O => "O",
    };

    for child in children.iter() {
        if let Ok(mut cell_text) = cell_text_query.get_mut(*child) {
            cell_text.sections[0].value = text.to_string();
        }
    }
}

fn update_player_turn(player_turn_state: &mut ResMut<State<PlayerTurn>>,) {
    let player_turn = player_turn_state.current().clone();
    let next_state = match player_turn {
        PlayerTurn::X => PlayerTurn::O,
        PlayerTurn::O => PlayerTurn::X,
    };
    player_turn_state.set(next_state).unwrap()
}

pub fn root(theme: &Res<UiTheme>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: theme.root.clone(),
        ..Default::default()
    }
}

pub fn main_border(theme: &Res<UiTheme>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Auto, Val::Auto),
            border: Rect::all(Val::Px(2.0)),
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        color: theme.border.clone(),
        ..Default::default()
    }
}

pub fn square_row() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Auto, Val::Auto),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn square_border(theme: &Res<UiTheme>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
            border: Rect::all(Val::Px(2.0)),
            ..Default::default()
        },
        color: theme.border.clone(),
        ..Default::default()
    }
}

pub fn menu_background(theme: &Res<UiTheme>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            padding: Rect::all(Val::Px(5.0)),
            ..Default::default()
        },
        color: theme.menu.clone(),
        ..Default::default()
    }
}

pub fn button(theme: &Res<UiTheme>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: theme.button_text.clone(),
            },
            Default::default(),
        ),
        ..Default::default()
    };
}

pub fn setup_board(mut commands: Commands, theme: Res<UiTheme>, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(root(&theme)).with_children(|parent| {
        parent
            .spawn_bundle(main_border(&theme))
            .with_children(|parent| {
                for row_index in 0..3 {
                    parent.spawn_bundle(square_row()).with_children(|parent| {
                        for column_index in 1..=3 {
                            let cell_id = 3 * row_index + column_index;
                            parent
                                .spawn_bundle(square_border(&theme))
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(button(&theme))
                                        .with_children(|parent| {
                                            parent.spawn_bundle(button_text(
                                                &asset_server,
                                                &theme,
                                                "",
                                            ));
                                        })
                                        .insert(TicTacToeCell {
                                            cell_id,
                                            state: CellState::Empty,
                                        });
                                });
                        }
                    });
                }
            });
    });
}
