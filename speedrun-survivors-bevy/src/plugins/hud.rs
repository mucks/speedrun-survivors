use crate::player::PlayerState;
use crate::plugins::coin_rewards::CoinAccumulator;
use crate::weapon::switch_weapon::SwitchWeaponEvent;
use crate::{plugins::assets::UiAssets, weapon::weapon_type::WeaponType};
use bevy::prelude::*;

use crate::state::{AppState, ForState};

#[derive(Debug, Component)]
pub struct WeaponButton {
    weapon_type: WeaponType,
}

pub struct HudPlugin;

const ITEMS_COLOR: Color = Color::BLACK;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameRunning), spawn_layout)
            .add_systems(
                Update,
                (on_update, on_weapon_button_click).run_if(in_state(AppState::GameRunning)),
            );
    }
}

fn on_weapon_button_click(
    mut query: Query<(&Interaction, &WeaponButton), (With<WeaponButton>, Changed<Interaction>)>,
    mut tx_switch: EventWriter<SwitchWeaponEvent>,
) {
    for (interaction, weapon_button) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                tx_switch.send(SwitchWeaponEvent {
                    weapon_type: weapon_button.weapon_type,
                });
            }
            _ => {}
        }
    }
}

fn on_update(
    mut query_coin: Query<&mut Text, (With<CoinText>, Without<ExpBar>)>,
    coin_accumulator: Res<CoinAccumulator>,
    mut query_exp: Query<&mut Style, (With<ExpBar>, Without<CoinText>)>,
    player_state: Res<PlayerState>,
) {
    let mut text = query_coin.single_mut();
    text.sections[0].value = format!("Coins: {}", coin_accumulator.total_coin);

    let mut text = query_exp.single_mut();
    text.width = Val::Percent(100. * player_state.level_progress);
}

#[derive(Component)]
pub struct CoinText {}

#[derive(Component)]
pub struct ExpBar {}

fn spawn_layout(mut commands: Commands, assets: ResMut<UiAssets>) {
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
                states: vec![AppState::GameRunning],
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

            parent
                .spawn(
                    TextBundle::from_section(
                        "Coins:",
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
                )
                .insert(CoinText {});

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
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(0.),
                                height: Val::Percent(100.),
                                ..Default::default()
                            },
                            background_color: Color::PURPLE.into(),
                            ..Default::default()
                        })
                        .insert(ExpBar {});
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.),
                        height: Val::Px(200.),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    // TODO refactor this
                    // Weapon slots
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|builder| {
                            for (weapon_type, img) in &assets.weapons {
                                spawn_child_node(builder, img.clone(), Some(*weapon_type));
                            }
                        });

                    // Buff slots
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|builder| {
                            spawn_child_node(builder, img_buff.clone(), None);
                            spawn_child_node(builder, img_buff.clone(), None);
                            spawn_child_node(builder, img_buff.clone(), None);
                        });
                });
        });
}

fn spawn_child_node(
    builder: &mut ChildBuilder,
    ui_img: UiImage,
    weapon_type: Option<WeaponType>,
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
        background_color: BackgroundColor(Color::DARK_GRAY),
        ..Default::default()
    });

    if let Some(weapon_type) = weapon_type {
        node.insert(WeaponButton { weapon_type });
    }
    node.with_children(|parent| {
        spawn_nested_icon(parent, ITEMS_COLOR, ui_img.clone());
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
