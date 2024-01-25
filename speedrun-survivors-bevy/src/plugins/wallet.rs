use bevy::prelude::*;

mod solana_rpc_client;
#[cfg(feature = "wasm")]
mod wasm_wallet;

pub struct WalletPlugin;

impl Plugin for WalletPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "wasm")]
        app.add_plugins(wasm_wallet::WasmWalletPlugin);
    }
}
