use crate::menu::DrawBlinkTimer;
use crate::plugins::assets::UiAssets;
use crate::state::{AppState, ForState};
use bevy::prelude::*;

pub fn menu_splash_screen(mut commands: Commands, assets: ResMut<UiAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            ForState {
                states: vec![AppState::GamePaused],
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "pause",
                        TextStyle {
                            font: assets.font_expanse.clone(),
                            font_size: 100.0,
                            color: Color::rgb_u8(0xF8, 0xE4, 0x73),
                        },
                    ),
                    ..default()
                },
                DrawBlinkTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
            ));
        });
}
