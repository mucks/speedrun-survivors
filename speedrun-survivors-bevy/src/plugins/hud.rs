use crate::data::abilities::AbilityType;
use crate::player::PlayerState;
use crate::plugins::coin_rewards::CoinAccumulator;
use crate::weapon::switch_weapon::SwitchWeaponEvent;
use crate::{plugins::assets::UiAssets, weapon::weapon_type::WeaponType, COLOR_SOL_DINO};
use bevy::prelude::*;

use crate::state::{for_game_states, AppState};

#[derive(Debug, Component)]
pub struct WeaponButton {
    weapon_type: WeaponType,
}

pub struct HudPlugin;

const ITEMS_COLOR: Color = Color::BLACK;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameInitializing), on_enter_game_init)
            .add_systems(
                Update,
                (on_update, on_weapon_button_click).run_if(in_state(AppState::GameRunning)),
            )
            .add_systems(Update, on_hud_redraw.run_if(on_event::<HudRedraw>()))
            .add_event::<HudRedraw>();
    }
}

pub fn on_enter_game_init(mut tx_hud: EventWriter<HudRedraw>) {
    tx_hud.send(HudRedraw {});
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
    if let Ok(mut text) = query_coin.get_single_mut() {
        text.sections[0].value = format!("Coins: {}", coin_accumulator.total_coin);
    }

    if let Ok(mut text) = query_exp.get_single_mut() {
        text.width = Val::Percent(100. * player_state.level_progress);
    }

    // Attemot to delete something
    //TODO could then have empty ability icons and delete them
    // or just "redraw" use         player_state: Res<PlayerState>,
}

#[derive(Component)]
pub struct HudRoot {}

#[derive(Component)]
pub struct CoinText {}

#[derive(Component)]
pub struct ExpBar {}

#[derive(Component)]
pub struct AbilityHudIcon {
    slot: u8,
    ability: Option<AbilityType>,
}

#[derive(Event)]
pub struct HudRedraw {}

enum SlotType {
    Weapon(WeaponType),
    Ability(AbilityType),
    Empty,
}

fn on_hud_redraw(
    mut commands: Commands,
    assets: Res<UiAssets>,
    player_state: Res<PlayerState>,
    hud: Query<Entity, With<HudRoot>>,
) {
    // Remove any existing hud
    if let Ok(entity) = hud.get_single() {
        commands.entity(entity).despawn_recursive();
        eprintln!("UNSPAWNH UD");
    }

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    ..Default::default()
                },
                ..Default::default()
            },
            for_game_states(),
        ))
        .insert(HudRoot {})
        .with_children(|parent| {
            insert_coin_counter(parent);
            insert_exp_bar(parent);

            insert_wrapper_slots(parent, &assets, &player_state);
        });
}

fn insert_coin_counter(parent: &mut ChildBuilder) {
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
}

fn insert_exp_bar(parent: &mut ChildBuilder) {
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
}

fn insert_wrapper_slots(
    parent: &mut ChildBuilder,
    assets: &Res<UiAssets>,
    player_state: &Res<PlayerState>,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(500.),
                height: Val::Px(300.),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
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
                        spawn_slot(builder, img.clone(), SlotType::Weapon(*weapon_type));
                    }
                });

            // Ability slots
            let mut ability_slots: Vec<SlotType> = player_state
                .abilities
                .iter()
                .map(|(ability, _level)| SlotType::Ability(*ability))
                .collect();
            ability_slots.extend(
                (0..4i8.saturating_sub(ability_slots.len() as i8)).map(|_| SlotType::Empty),
            );

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| {
                    for slot in ability_slots {
                        let mut img = match slot {
                            SlotType::Ability(ability) => {
                                assets.abilities.get(&ability).unwrap().clone()
                            }
                            _ => assets.buff_1.clone(),
                        };
                        spawn_slot(builder, img, slot);
                    }
                });
        });
}

fn spawn_slot(parent: &mut ChildBuilder, ui_img: UiImage, slot_type: SlotType) {
    let mut node = parent.spawn(ButtonBundle {
        style: Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Px(64.),
            height: Val::Px(64.),
            margin: UiRect::all(Val::Px(5.)),
            padding: UiRect::all(Val::Px(2.)),
            ..Default::default()
        },
        background_color: BackgroundColor(COLOR_SOL_DINO),
        ..Default::default()
    });

    match slot_type {
        SlotType::Weapon(weapon_type) => {
            node.insert(WeaponButton { weapon_type });
        }
        SlotType::Ability(ability_type) => {
            //TODO
        }
        SlotType::Empty => {}
    }

    node.with_children(|parent| {
        spawn_nested_icon(parent, ITEMS_COLOR, ui_img.clone());
    });
}

fn spawn_nested_icon(parent: &mut ChildBuilder, background_color: Color, ui_img: UiImage) {
    parent
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
