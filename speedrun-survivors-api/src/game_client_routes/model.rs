use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct NftListRequest {
    pub pubkey: String,
}

#[derive(Serialize)]
pub enum NftKind {
    Hero,
    Weapon,
    PowerUp,
}

#[derive(Serialize)]
pub struct NftData {
    pub key: String,
    pub kind: NftKind,
    #[serde(rename = "imgUrl")]
    pub image_url: String,
}

#[derive(Serialize)]
pub struct NftListResponse {
    #[serde(rename = "nftList")]
    pub nft_list: Option<Vec<NftData>>,
}

#[derive(Deserialize, Debug)]
pub struct SessionGetRequest {
    pub pubkey: String,
}

#[derive(Default, Serialize)]
pub enum SessionStateClient {
    #[default]
    None,
    Expired,
    Active,
}

#[derive(Default, Serialize)]
pub struct SessionGetResponse {
    pub state: SessionStateClient,
    pub entropy: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SessionInitRequest {
    pub pubkey: String,
}

#[derive(Default, PartialEq, Serialize)]
pub enum SessionInitResult {
    Success,
    ErrorGameActive,
    #[default]
    ErrorUnexpected,
}

#[derive(Default, Serialize)]
pub struct SessionInitResponse {
    pub result: SessionInitResult,
    pub entropy: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SessionCancelRequest {
    pub pubkey: String,
    pub signature: String,
}

#[derive(Default, Serialize)]
pub enum SessionCancelResult {
    Success,
    ErrorSignatureInvalid,
    #[default]
    ErrorUnexpected,
}

#[derive(Default, Serialize)]
pub struct SessionCancelResponse {
    pub result: SessionCancelResult,
}

#[derive(Deserialize, Debug)]
pub struct GameStartRequest {
    pub pubkey: String,
    pub entropy: String,
    pub signature: String,
}

#[derive(Default, Serialize)]
pub enum GameStartResult {
    Success,
    ErrorNoSuchSession,
    ErrorRequestDataDoesNotMatch,
    ErrorSignatureInvalid,
    #[default]
    ErrorUnexpected,
}

#[derive(Default, Serialize)]
pub struct GameStartResponse {
    pub result: GameStartResult,
}

#[derive(Deserialize, Debug)]
pub struct GameCompleteRequest {
    pub pubkey: String,
    pub entropy: String,
    #[serde(rename = "nftList")]
    pub nft_list: Option<Vec<String>>,
    pub replay: String,
    pub signature: String,
}

#[derive(Default, Serialize)]
pub enum GameCompleteResult {
    Success,
    ErrorNoSuchSession,
    ErrorRequestDataDoesNotMatch,
    ErrorSignatureInvalid,
    #[default]
    ErrorUnexpected,
}

#[derive(Default, Serialize)]
pub struct GameCompleteResponse {
    pub result: GameCompleteResult,
    pub todo: String, //TODO rewarded things and stuff
}
