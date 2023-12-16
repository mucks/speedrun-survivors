use crate::plugins::assets::UiAssets;
use crate::state::{AppState, ForState};
use bevy::app::AppExit;
use bevy::prelude::*;

#[derive(Component)]
pub struct DrawBlinkTimer(pub Timer);

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::SplashScreen), start_menu)
            .add_systems(OnEnter(AppState::GameOver), gameover_menu)
            .add_systems(Update, (menu_input_system, menu_blink_system))
            .add_systems(Startup, setup);
    }
}

fn setup(mut _commands: Commands) {
    //TODO
}

fn start_menu(mut commands: Commands, assets: ResMut<UiAssets>) {
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

fn gameover_menu(mut commands: Commands, assets: ResMut<UiAssets>) {
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
                next_state.set(AppState::GameRunning);
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
