use crate::menu::DrawBlinkTimer;
use crate::plugins::assets::UiAssets;
use crate::state::{AppState, ForState};
use crate::{COLOR_SOL_DINO, COLOR_SOL_OCEAN, GAME_NAME};
use bevy::prelude::*;

pub fn menu_splash_screen(mut commands: Commands, assets: ResMut<UiAssets>) {
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
                    GAME_NAME,
                    TextStyle {
                        font: assets.font_primary.clone(),
                        font_size: 100.0,
                        color: COLOR_SOL_DINO,
                    },
                ),
                ..default()
            },));
            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "Press Enter",
                        TextStyle {
                            font: assets.font_primary.clone(),
                            font_size: 50.0,
                            color: COLOR_SOL_OCEAN,
                        },
                    ),
                    ..default()
                },
                DrawBlinkTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
            ));
        });
}
