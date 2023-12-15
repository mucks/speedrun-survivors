use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

pub async fn test_api() -> Result<Value> {
    // let client = reqwest::Client::new();
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(8))
        .build()?;

    let req_data = GetAssetsByOwnerRequest::default_with_address(
        "86xCnPeV69n6t3DnyGvkKobf9FdN2H9oiVDdaMpo2MMY".to_string(),
    );
    let response = client
        .post("https://api.mainnet-beta.solana.com")
        .json(&req_data)
        .send()
        .await?
        .json::<Value>()
        .await?;
    Ok(response)

    //TODO process to essential data
}

#[derive(Deserialize, Debug)]
pub struct APIResponse {
    blah: String,
}

#[derive(Serialize)]
struct GetAssetsByOwnerRequest {
    #[serde(rename = "jsonrpc")]
    json_rpc: String,
    id: String,
    method: String,
    params: GetAssetsByOwnerRequestParams,
}

#[derive(Serialize)]
struct GetAssetsByOwnerRequestParams {
    #[serde(rename = "ownerAddress")]
    owner_address: String,
    page: u32,
    limit: u32,
}

impl GetAssetsByOwnerRequest {
    fn default_with_address(address: String) -> Self {
        Self {
            json_rpc: "2.0".to_string(),
            id: "unique_id_todo".to_string(), //TODO
            method: "getAssetsByOwner".to_string(),
            params: GetAssetsByOwnerRequestParams {
                owner_address: address,
                page: 1,
                limit: 1000,
            },
        }
    }
}
