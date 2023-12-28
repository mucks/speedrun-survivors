use crate::data::abilities::AbilityType;
use crate::menu::{BTN_BORDER_DEFAULT, BTN_BORDER_HOVER};
use crate::plugins::assets::UiAssets;
use crate::plugins::gameplay_effects::GameplayEffectEvent;
use crate::state::{AppState, ForState};
use crate::{COLOR_SOL_OCEAN, COLOR_SOL_SURGE};
use bevy::prelude::*;

pub fn menu_level_up(mut commands: Commands, assets: ResMut<UiAssets>) {
    let img_buff = assets.buff_1.clone();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            ForState {
                states: vec![AppState::GameLevelUp],
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(60.),
                        height: Val::Percent(80.),
                        flex_direction: FlexDirection::Column,
                        border: UiRect::all(Val::Px(5.)),
                        padding: UiRect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    border_color: BorderColor(Color::INDIGO),
                    background_color: Color::GOLD.into(),
                    ..Default::default()
                })
                .with_children(|parent| inner_wrapper(parent, &assets));
        });
}

fn inner_wrapper(parent: &mut ChildBuilder, assets: &UiAssets) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(5.)),
                padding: UiRect::all(Val::Px(50.)),
                align_items: AlignItems::Center,
                ..Default::default()
            },
            border_color: BorderColor(Color::PURPLE),
            background_color: Color::OLIVE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "- Level Up -",
                TextStyle {
                    font: assets.font_secondary.clone(),
                    font_size: 40.0,
                    color: COLOR_SOL_OCEAN,
                    ..default()
                },
            ));

            for ability in AbilityType::select_randomly(4) {
                ability_selector(parent, &assets, ability);
            }
        });
}

fn ability_selector(parent: &mut ChildBuilder, assets: &UiAssets, ability: AbilityType) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(20.),
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(Val::Px(5.)),
                    padding: UiRect::all(Val::Px(5.)),
                    margin: UiRect::top(Val::Px(15.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                border_color: BorderColor(BTN_BORDER_DEFAULT),
                background_color: Color::DARK_GREEN.into(),
                ..default()
            },
            LevelUpChoice { option: ability },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                ability.to_string(),
                TextStyle {
                    font: assets.font_secondary.clone(),
                    font_size: 30.0,
                    color: COLOR_SOL_SURGE,
                    ..default()
                },
            ));
        });
}

pub fn on_level_up_menu_button_action(
    mut button_interaction: Query<
        (&Interaction, &LevelUpChoice, &mut BorderColor),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut tx_gameplay: EventWriter<GameplayEffectEvent>,
) {
    // Check for button interaction
    for (interaction, choice, mut border) in button_interaction.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Resume the game
                next_state.set(AppState::GameRunning);
                // TODO actual implementation
                eprintln!("Level up selection: {choice:?}");
            }
            Interaction::Hovered => {
                border.0 = BTN_BORDER_HOVER;
            }
            Interaction::None => border.0 = BTN_BORDER_DEFAULT,
        }
    }
}

#[derive(Debug, Component)]
pub struct LevelUpChoice {
    option: AbilityType,
}
