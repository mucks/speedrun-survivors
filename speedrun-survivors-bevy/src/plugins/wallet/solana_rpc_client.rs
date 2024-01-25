use anyhow::{bail, Context, Result};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const TOKEN_ACCOUNT_DATA_SIZE: usize = 165;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountQuery {
    pub account: Account,
    pub pubkey: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub data: String,
    pub executable: bool,
    pub lamports: i64,
    pub owner: String,
    pub rent_epoch: i64,
    pub space: i64,
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

    async fn rpc_post<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T> {
        let resp_str: String = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&RpcRequest::new(method, params))
            .send()
            .await?
            .text()
            .await?;

        debug!("resp_str: {:?}", resp_str);
        let resp: RpcResponse<T> = serde_json::from_str(&resp_str)?;

        if let Some(e) = resp.error {
            bail!("rpc error: {:?}", e);
        }

        let result = resp.result.context("no result")?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solana_rpc_client() {
        let client = SolanaRpcClient::devnet();
        assert_eq!(client.url, "https://api.devnet.solana.com");
    }

    #[tokio::test]
    async fn test_get_token_account() -> Result<()> {
        let client = SolanaRpcClient::devnet();
        let accounts = client
            .get_token_accounts("CmegLFqp2tT9jDn7ZHWUkUDLz2QoXEdnQn1mJeKCCRpF")
            .await?;
        println!("accounts: {:?}", accounts);

        Ok(())
    }
}
