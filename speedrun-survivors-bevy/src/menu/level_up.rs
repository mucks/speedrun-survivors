use crate::plugins::assets::UiAssets;
use crate::state::{AppState, ForState};
use bevy::prelude::*;

pub fn menu_level_up(mut commands: Commands, assets: ResMut<UiAssets>) {
    let img_buff = assets.buff_1.clone();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    ..Default::default()
                },
                ..Default::default()
            },
            ForState {
                states: vec![AppState::GameLevelUp],
            },
        ))
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    margin: UiRect::top(Val::Px(5.)),
                    ..Default::default()
                },
                ..Default::default()
            });

            parent.spawn(
                TextBundle::from_section(
                    "BLAH NBLALAD:",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(5.0),
                    right: Val::Px(100.0),
                    ..default()
                }),
            );

            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(250.),
                        height: Val::Px(40.),
                        flex_direction: FlexDirection::Row,
                        position_type: PositionType::Absolute,
                        border: UiRect::all(Val::Px(3.)),
                        top: Val::Px(55.0),
                        right: Val::Px(50.0),
                        ..Default::default()
                    },
                    border_color: BorderColor(Color::INDIGO),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(0.),
                            height: Val::Percent(100.),
                            ..Default::default()
                        },
                        background_color: Color::PURPLE.into(),
                        ..Default::default()
                    });
                });
        });
}
