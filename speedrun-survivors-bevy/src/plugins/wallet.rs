use anyhow::{anyhow, Context, Result};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;
use wasm_bindgen::JsValue;

use crate::state::AppState;

use super::assets::UiAssets;

pub struct WalletPlugin;

impl Plugin for WalletPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WalletEvent>();
        app.add_systems(OnEnter(AppState::WalletMenu), setup_wallet_menu);
        app.add_systems(
            Update,
            (wallet_menu_system, wallet_event_system, wallet_task_system)
                .run_if(in_state(AppState::WalletMenu)),
        );
    }
}

fn reflect_get(target: &JsValue, key: &JsValue) -> Result<JsValue> {
    let result = js_sys::Reflect::get(target, key).map_err(|e| anyhow!("{:?}", e))?;
    debug!("reflect_get: {:?}", result);
    Ok(result)
}

#[derive(Debug, Component)]
pub struct Wallet {
    pub amount: u32,
    pub address: String,
}

#[derive(Debug, Event)]
pub enum WalletEvent {
    ConnectBtnClick,
}

#[derive(Debug, Component)]
pub enum WalletButtonType {
    Connect,
    Disconnect,
}

#[derive(Debug, Component)]
pub struct WalletMenu;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct WalletTask(Task<()>);

fn wallet_task_system(mut tasks: Query<&mut WalletTask>) {
    for mut task in &mut tasks {
        if let Some(_) = future::block_on(future::poll_once(&mut task.0)) {
            debug!("wallet task finished");
        }
    }
}

fn wallet_event_system(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    mut ev_reader: EventReader<WalletEvent>,
) {
    let thread_pool = AsyncComputeTaskPool::get();
    for event in ev_reader.iter() {
        match event {
            WalletEvent::ConnectBtnClick => {
                debug!("WalletEvent::ConnectBtnClick");

                wasm_bindgen_futures::spawn_local(async move {
                    let pubkey = connect_to_phantom().await;
                    println!("pubkey: {:?}", pubkey);
                });
            }
        }
    }
}

async fn connect_to_phantom() -> Result<String> {
    debug!("connect_to_wallet");
    let window = web_sys::window().context("could not get window")?;
    if let Some(solana) = window.get("solana") {
        let is_phantom = reflect_get(&*solana, &wasm_bindgen::JsValue::from_str("isPhantom"))?;

        if is_phantom == JsValue::from(true) {
            let connect_str = wasm_bindgen::JsValue::from_str("connect");
            let connect: js_sys::Function = reflect_get(&*solana, &connect_str)?.into();

            log::debug!("{:?}", connect.to_string());

            let resp = connect.call0(&solana).map_err(|err| anyhow!("{err:?}"))?;
            let promise = js_sys::Promise::resolve(&resp);

            let result = wasm_bindgen_futures::JsFuture::from(promise)
                .await
                .map_err(|err| anyhow!("{err:?}"))?;

            log::debug!("{:?}", result);

            let pubkey_str = wasm_bindgen::JsValue::from_str("publicKey");
            let pubkey_obj: js_sys::Object = reflect_get(&result, &pubkey_str)?.into();

            let bn_str = wasm_bindgen::JsValue::from_str("toString");
            let to_string_fn: js_sys::Function = reflect_get(&pubkey_obj, &bn_str)?.into();

            log::debug!("pubkey_obj: {:?}", to_string_fn.call0(&pubkey_obj));

            let pubkey = to_string_fn
                .call0(&pubkey_obj)
                .map_err(|err| anyhow!("{:?}", err))?;

            let public_key = pubkey
                .as_string()
                .context("could not convert pubkey to string")?;

            log::debug!("pubkey: {:?}", public_key);

            return Ok(public_key);
        }

        debug!("isPhantom: {:?}", is_phantom);
    }

    Err(anyhow!("could not connect to wallet"))
}

pub fn wallet_menu_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &WalletButtonType,
        ),
        (Changed<Interaction>, With<WalletButtonType>),
    >,
    mut ev_writer: EventWriter<WalletEvent>,
) {
    for (interaction, mut color, mut border_color, button_type) in &mut interaction_query {
        // styling

        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }

        match *interaction {
            Interaction::Pressed => match button_type {
                WalletButtonType::Connect => {
                    println!("Connect button clicked");
                    ev_writer.send(WalletEvent::ConnectBtnClick);
                }
                WalletButtonType::Disconnect => {
                    println!("Disconnect button clicked");
                }
            },
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            _ => {}
        }
    }
}

pub fn setup_wallet_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<UiAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // setup connect button
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(20.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Connect",
                        TextStyle {
                            font: assets.font_primary.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                })
                .insert(WalletButtonType::Connect);
        });

    // setup address display
    // setup balance display
}
