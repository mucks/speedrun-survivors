use std::fmt::Display;

use anyhow::{anyhow, bail, Context, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use serde_repr::{Deserialize_repr, Serialize_repr};

const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const TOKEN_ACCOUNT_DATA_SIZE: usize = 165;

#[derive(Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Pubkey([u8; 32]);

impl TryFrom<&str> for Pubkey {
    type Error = anyhow::Error;
    fn try_from(s: &str) -> Result<Self> {
        let bytes = bs58::decode(s).into_vec()?;
        let mut pubkey = [0u8; 32];
        pubkey.copy_from_slice(&bytes);
        Ok(Self(pubkey))
    }
}

impl Display for Pubkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Debug for Pubkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pubkey = bs58::encode(self.0).into_string();
        write!(f, "{}", pubkey)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountQuery {
    pub account: Account,
    pub pubkey: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountInfo {
    pub context: Value,
    pub value: Account,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub data: [String; 2],
    pub executable: bool,
    pub lamports: u64,
    pub owner: String,
    pub rent_epoch: u128,
    pub space: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SplTokenAccount {
    /// The mint associated with this account
    pub mint: Pubkey,
    /// The owner of this account.
    pub owner: Pubkey,
    /// The amount of tokens this account holds.
    pub amount: u64,
    /// If `delegate` is `Some` then `delegated_amount` represents
    /// the amount authorized by the delegate
    pub delegate: Option<Pubkey>,
    /// The account's state
    pub state: SplTokenAccountState,
    /// If is_native.is_some, this is a native token, and the value logs the
    /// rent-exempt reserve. An Account is required to be rent-exempt, so
    /// the value is used by the Processor to ensure that wrapped SOL
    /// accounts do not drop below this threshold.
    pub is_native: Option<u64>,
    /// The amount delegated
    pub delegated_amount: u64,
    /// Optional authority to close the account.
    pub close_authority: Option<Pubkey>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SplTokenAccountState {
    /// Account is not yet initialized
    #[default]
    Uninitialized = 0,
    /// Account is initialized; the account owner and/or delegate may perform
    /// permitted operations on this account
    Initialized = 1,
    /// Account has been frozen by the mint freeze authority. Neither the
    /// account owner nor the delegate are able to perform operations on
    /// this account.
    Frozen = 2,
}

#[derive(serde::Serialize)]
struct RpcRequest<T> {
    jsonrpc: String,
    method: String,
    id: u32,
    params: T,
}

#[derive(serde::Deserialize)]
struct RpcResponse<T> {
    jsonrpc: String,
    result: Option<T>,
    error: Option<serde_json::Value>,
    id: u64,
}

impl<T> RpcRequest<T> {
    pub fn new(method: &str, params: T) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            id: 1,
            params,
        }
    }
}
pub struct SolanaRpcClient {
    pub url: String,
    #[cfg(feature = "local")]
    client: reqwest::Client,
}

impl SolanaRpcClient {
    pub fn devnet() -> Self {
        Self {
            url: "https://api.devnet.solana.com".to_string(),
            #[cfg(feature = "local")]
            client: reqwest::Client::new(),
        }
    }

    async fn get_token_accounts(&self, pubkey: &str) -> Result<Vec<AccountQuery>> {
        let params = json!([TOKEN_PROGRAM_ID, {
            "encoding": "base64",
            "filters": [
                {
                    "dataSize": TOKEN_ACCOUNT_DATA_SIZE,
                },
                {
                    "memcmp": {
                        "offset": 32,
                        "bytes": pubkey,
                    }
                }
            ]
        }]);

        let resp: Vec<AccountQuery> = self.rpc_post("getProgramAccounts", params).await?;

        Ok(resp)
    }

    async fn get_account_info(&self, pubkey: &Pubkey) -> Result<GetAccountInfo> {
        let params = json!([pubkey.to_string(), {
            "encoding": "base64",
        }]);

        self.rpc_post("getAccountInfo", params).await
    }

    async fn rpc_post<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: Value,
    ) -> Result<T> {
        let resp_val: Value = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&RpcRequest::new(method, params))
            .send()
            .await?
            .json()
            .await?;

        let resp_val_json_str = serde_json::to_string_pretty(&resp_val)?;

        let resp: RpcResponse<T> = serde_json::from_value(resp_val).map_err(|e| {
            anyhow!(
                "failed to deserialize rpc response: {}, {:?}",
                resp_val_json_str,
                e
            )
        })?;

        if let Some(e) = resp.error {
            bail!("rpc error: {:?}", e);
        }

        let result = resp.result.context("no result")?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use base64::prelude::*;

    use super::*;

    fn client() -> SolanaRpcClient {
        SolanaRpcClient::devnet()
    }

    fn pubkey() -> String {
        dotenvy::dotenv().ok();
        dotenvy::var("TEST_PUBKEY").expect("TEST_PUBKEY not set")
    }

    #[test]
    fn test_solana_rpc_client() {
        let client = SolanaRpcClient::devnet();
        assert_eq!(client.url, "https://api.devnet.solana.com");
    }

    #[tokio::test]
    async fn test_get_token_account() -> Result<()> {
        let client = SolanaRpcClient::devnet();
        let accounts = client.get_token_accounts(&pubkey()).await?;
        println!("accounts: {:?}", accounts);

        let account = &accounts[0];

        let data_bin = BASE64_STANDARD.decode(&account.account.data[0])?;

        let spl_token_account: SplTokenAccount = bincode::deserialize(&data_bin)?;

        println!("spl_token_account: {:?}", spl_token_account);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_account_info() -> Result<()> {
        let client = SolanaRpcClient::devnet();
        let account = client
            .get_account_info(&"43YENwBALvcSyRMy4pYx7QcpMwGXyyjrgyS2idxeoMSp".try_into()?)
            .await?;
        println!("account: {:?}", account);

        Ok(())
    }
}
