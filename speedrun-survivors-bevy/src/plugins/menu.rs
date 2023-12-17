use crate::heroes::HeroType;
use crate::plugins::assets::UiAssets;
use crate::state::{AppState, ForState};
use bevy::app::AppExit;
use bevy::prelude::*;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct DrawBlinkTimer(pub Timer);

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::SplashScreen), menu_splash_screen)
            .add_systems(OnEnter(AppState::GameCreate), menu_game_create)
            .add_systems(OnEnter(AppState::GameOver), menu_game_over)
            .add_systems(Update, (menu_input_system, menu_blink_system))
            .add_systems(Startup, setup)
            .insert_resource(GameConfigState::default());
    }
}

fn setup(mut _commands: Commands) {
    //TODO
}

fn menu_splash_screen(mut commands: Commands, assets: ResMut<UiAssets>) {
    let font_color = Color::rgb_u8(0x92, 0xA6, 0x8A);

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            ForState {
                states: vec![AppState::SplashScreen],
            },
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                style: Style { ..default() },
                text: Text::from_section(
                    "Speedrun Survivors",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 100.0,
                        color: font_color,
                    },
                ),
                ..default()
            },));
            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "enter",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 50.0,
                            color: font_color,
                        },
                    ),
                    ..default()
                },
                DrawBlinkTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
            ));
        });
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    SettingsDisplay,
    SettingsSound,
    BackToMainMenu,
    BackToSettings,
    Quit,
}

fn menu_game_create(
    mut commands: Commands,
    assets: Res<UiAssets>,
    mut state: ResMut<GameConfigState>,
) {
    // TODO on this screen all kinds of stats should be modifiable, the level selected, and the hero selected
    state.hero = HeroType::Pepe;

    // Screen wrapper
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    ..default()
                },
                ..default()
            },
            ForState {
                states: vec![AppState::GameCreate],
            },
        ))
        .with_children(|parent| {
            // Wrapper for the main content
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(85.),
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| wrapper_content(builder, &assets));

            // Wrapper for the footer
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(15.),
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    background_color: Color::OLIVE.into(),
                    ..Default::default()
                })
                .with_children(|builder| wrapper_footer(builder, &assets));
        });
}

fn wrapper_content(parent: &mut ChildBuilder, assets: &UiAssets) {
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    // Wrapper for the left side
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background_color: Color::DARK_GREEN.into(),

            ..Default::default()
        })
        .with_children(|parent| {
            // Wrapper for hero selection
            wrapper_hero_selector(parent, assets);

            // Wrapper for level selection
            wrapper_level_selector(parent, assets);
        });

    // Wrapper for the right side
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background_color: Color::CRIMSON.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // Right side wrapper TODO
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "Equip your cNFTs",
                            TextStyle {
                                font_size: 30.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        }),
                    );

                    // Display three buttons for each action available from the main menu:
                    // - new game
                    // - settings
                    // - quit
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            let icon = assets.buff_1.clone();
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: icon,
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "New Game",
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Settings,
                        ))
                        .with_children(|parent| {
                            let icon = assets.buff_1.clone();
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: icon,
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "Settings",
                                button_text_style.clone(),
                            ));
                        });
                });
        });
}

fn wrapper_hero_selector(parent: &mut ChildBuilder, assets: &UiAssets) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(50.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Hero Selection",
                    TextStyle {
                        font_size: 30.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
            );

            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    for hero in HeroType::into_iter() {
                        let ui_img = assets.heroes.get(&hero).unwrap();
                        spawn_hero_select_box(parent, ui_img.clone(), &hero);
                    }
                });
        });
}

fn wrapper_level_selector(parent: &mut ChildBuilder, assets: &UiAssets) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(50.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Level Selection",
                    TextStyle {
                        font_size: 30.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
            );
        });
}

fn wrapper_footer(parent: &mut ChildBuilder, assets: &UiAssets) {
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MenuButtonAction::Quit,
                ))
                .with_children(|parent| {
                    let icon = assets.buff_1.clone();
                    parent.spawn(ImageBundle {
                        style: button_icon_style.clone(),
                        image: icon,
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section("Quit", button_text_style.clone()));
                });
        });

    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::End,
                ..Default::default()
            },
            background_color: Color::PINK.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MenuButtonAction::Quit,
                ))
                .with_children(|parent| {
                    let icon = assets.buff_1.clone();
                    parent.spawn(ImageBundle {
                        style: button_icon_style,
                        image: icon,
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section("Start", button_text_style));
                });
        });
}

fn spawn_hero_select_box(
    builder: &mut ChildBuilder,
    ui_img: UiImage,
    hero_type: &HeroType,
) -> Entity {
    let mut node = builder.spawn(ButtonBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Px(64f32),
            height: Val::Px(64f32),
            margin: UiRect::all(Val::Px(5.)),
            padding: UiRect::all(Val::Px(2.)),
            ..Default::default()
        },
        background_color: BackgroundColor(Color::CRIMSON),
        ..Default::default()
    });

    node.with_children(|parent| {
        spawn_nested_icon(parent, Color::GOLD, ui_img.clone());
    })
    .id()
}

fn spawn_nested_icon(builder: &mut ChildBuilder, background_color: Color, ui_img: UiImage) {
    builder
        .spawn(NodeBundle {
            background_color: BackgroundColor(background_color),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(56.0),
                        height: Val::Px(56.0),
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                },
                ui_img,
            ));
        });
}

fn menu_game_over(mut commands: Commands, assets: Res<UiAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            ForState {
                states: vec![AppState::GameOver],
            },
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                style: Style { ..default() },
                text: Text::from_section(
                    "Game Over",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 100.0,
                        color: Color::rgb_u8(0xAA, 0x22, 0x22),
                    },
                ),
                ..default()
            },));
            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "enter",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 50.0,
                            color: Color::rgb_u8(0x88, 0x22, 0x22),
                        },
                    ),
                    ..default()
                },
                DrawBlinkTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
            ));
        });
}

fn menu_blink_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DrawBlinkTimer, &Visibility)>,
) {
    for (entity, mut timer, visibility) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            let new_visibility = if visibility == Visibility::Visible {
                Visibility::Hidden
            } else {
                Visibility::Visible
            };
            commands.entity(entity).insert(new_visibility);
        }
    }
}

fn menu_input_system(
    state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    keys: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    //if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Comma) {
    if state.get() != &AppState::SplashScreen && keys.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::SplashScreen);
    } else {
        match state.get() {
            AppState::SplashScreen => {
                if keys.just_pressed(KeyCode::Return) {
                    next_state.set(AppState::GameCreate);
                }
                if keys.just_pressed(KeyCode::Escape) {
                    app_exit_events.send(AppExit);
                }
            }
            AppState::GameCreate => {
                //TODO should be choose loadout, equip NFT UI
                if keys.just_pressed(KeyCode::Return) {
                    next_state.set(AppState::GameRunning);
                }
            }
            AppState::GameOver => {
                if keys.just_pressed(KeyCode::Return) {
                    next_state.set(AppState::SplashScreen);
                }
            }
            AppState::GameRunning => {}
        }
    }
}

#[derive(Resource)]
pub struct GameConfigState {
    pub hero: HeroType,
    pub level: u64,
}

impl Default for GameConfigState {
    fn default() -> Self {
        Self {
            hero: HeroType::BonkInu,
            level: 0,
        }
    }
}
