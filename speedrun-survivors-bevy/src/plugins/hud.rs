use crate::data::abilities::AbilityType;
use crate::player::PlayerState;
use crate::plugins::coin_rewards::CoinAccumulator;
use crate::weapon::switch_weapon::SwitchWeaponEvent;
use crate::{plugins::assets::UiAssets, weapon::weapon_type::WeaponType, COLOR_SOL_DINO};
use bevy::prelude::*;

use crate::state::{for_game_states, AppState};

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

fn on_enter_game_init(mut tx_hud: EventWriter<HudRedraw>) {
    tx_hud.send(HudRedraw::Root);
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
    mut query_coin: Query<&mut Text, (With<CoinText>, Without<KillsText>, Without<ExpBar>)>,
    mut query_kill: Query<&mut Text, (With<KillsText>, Without<CoinText>, Without<ExpBar>)>,
    mut query_exp: Query<&mut Style, (With<ExpBar>, Without<CoinText>, Without<KillsText>)>,
    coin_accumulator: Res<CoinAccumulator>,
    player_state: Res<PlayerState>,
) {
    if let Ok(mut text) = query_coin.get_single_mut() {
        text.sections[0].value = format!("Coins: {}", coin_accumulator.total_coin);
    }

    if let Ok(mut text) = query_kill.get_single_mut() {
        text.sections[0].value = format!("Kills: {}", player_state.total_kills);
    }

    if let Ok(mut text) = query_exp.get_single_mut() {
        text.width = Val::Percent(100. * player_state.level_progress);
    }
}

#[derive(Component)]
struct NodeRoot {}

#[derive(Component)]
struct NodeAbilitySlots {}

#[derive(Component)]
struct CoinText {}

#[derive(Component)]
struct KillsText {}

#[derive(Component)]
struct ExpBar {}

#[derive(Debug, Component)]
struct WeaponButton {
    weapon_type: WeaponType,
}

#[derive(Event)]
pub enum HudRedraw {
    Root,
    AbilitySlots,
}

enum SlotType {
    Weapon(WeaponType),
    Ability(AbilityType),
    Empty,
}

fn on_hud_redraw(
    mut commands: Commands,
    assets: Res<UiAssets>,
    player_state: Res<PlayerState>,
    node_root: Query<Entity, With<NodeRoot>>,
    node_abilities: Query<Entity, With<NodeAbilitySlots>>,
    mut tx_hud: EventReader<HudRedraw>,
) {
    let Some(event) = tx_hud.iter().last() else {
        return;
    };

    match event {
        HudRedraw::Root => {
            // Remove the existing hud
            if let Ok(entity) = node_root.get_single() {
                commands.entity(entity).despawn_recursive();
            }

            // Redraw the entire thing
            hud_full_redraw(&mut commands, &assets, &player_state);
        }
        HudRedraw::AbilitySlots => {
            if let Ok(entity) = node_abilities.get_single() {
                commands.entity(entity).despawn_descendants();

                commands
                    .spawn(NodeBundle::default())
                    .with_children(|parent| spawn_ability_slots(parent, &assets, &player_state))
                    .set_parent(entity);
            }
        }
    }
}

fn hud_full_redraw(
    commands: &mut Commands,
    assets: &Res<UiAssets>,
    player_state: &Res<PlayerState>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Start,
                    ..Default::default()
                },
                ..Default::default()
            },
            for_game_states(),
        ))
        .insert(NodeRoot {})
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(60.),
                        height: Val::Percent(100.),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Start,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| insert_wrapper_slots(parent, &assets, &player_state));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(40.),
                        height: Val::Percent(100.),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::End,
                        margin: UiRect::all(Val::Px(25.)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    insert_coin_counter(parent);
                    insert_kill_counter(parent);
                    insert_exp_bar(parent);
                });
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
                margin: UiRect::all(Val::Px(5.)),
                ..default()
            }),
        )
        .insert(CoinText {});
}

fn insert_kill_counter(parent: &mut ChildBuilder) {
    parent
        .spawn(
            TextBundle::from_section(
                "Kills:",
                TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.5, 0.5, 1.0),
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(5.)),
                ..default()
            }),
        )
        .insert(KillsText {});
}

fn insert_exp_bar(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(250.),
                height: Val::Px(40.),
                flex_direction: FlexDirection::Row,
                border: UiRect::all(Val::Px(3.)),
                margin: UiRect::all(Val::Px(5.)),
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
                .with_children(|parent| {
                    for (weapon_type, img) in &assets.weapons {
                        spawn_slot(parent, img.clone(), SlotType::Weapon(*weapon_type));
                    }
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(NodeAbilitySlots {})
                .with_children(|parent| {
                    spawn_ability_slots(parent, assets, player_state);
                });
        });
}

fn spawn_ability_slots(
    parent: &mut ChildBuilder,
    assets: &Res<UiAssets>,
    player_state: &Res<PlayerState>,
) {
    // Ability slots
    let mut ability_slots: Vec<SlotType> = player_state
        .ability_order
        .iter()
        .map(|ability| SlotType::Ability(*ability))
        .collect();

    ability_slots
        .extend((0..4i8.saturating_sub(ability_slots.len() as i8)).map(|_| SlotType::Empty));

    for slot in ability_slots {
        let img = match slot {
            SlotType::Ability(ability) => assets.abilities.get(&ability).unwrap().clone(),
            _ => assets.empty_slot.clone(),
        };
        spawn_slot(parent, img, slot);
    }
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
        _ => {}
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
