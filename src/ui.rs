use bevy::prelude::*;

#[derive(Component)]
enum UiButton {
  SelectCell { cell: u8 },
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
    fn from_world(world: &mut World) -> Self {
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

pub fn button_system(
    theme: Res<UiTheme>,
    mut buttons: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material) in buttons.iter_mut() {
        match *interaction {
            Interaction::Clicked => *material = theme.button_pressed,
            Interaction::Hovered => *material = theme.button_hovered,
            Interaction::None => *material = theme.button,
        }
    }
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

pub fn border(theme: &Res<UiTheme>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(400.0), Val::Auto),
            border: Rect::all(Val::Px(8.0)),
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
        style: Style {
            margin: Rect::all(Val::Px(10.0)),
            ..Default::default()
        },
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

pub fn setup_ui(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  theme: Res<UiTheme>,
) {
  commands.spawn_bundle(UiCameraBundle::default());

  commands
      .spawn_bundle(root(&theme))
      .with_children(|parent| {
          parent
              .spawn_bundle(border(&theme))
              .with_children(|parent| {
                  parent
                      .spawn_bundle(menu_background(&theme))
                      .with_children(|parent| {
                          parent
                              .spawn_bundle(button(&theme))
                              .with_children(|parent| {
                                  parent.spawn_bundle(button_text(
                                      &asset_server,
                                      &theme,
                                      "New Game",
                                  ));
                              })
                              .insert(UiButton::SelectCell { cell: 1 });
                      });
              });
      });
}
