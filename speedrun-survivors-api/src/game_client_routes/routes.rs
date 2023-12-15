use actix_web::web::Data;
use actix_web::{post, web::Json, HttpResponse, Responder};
use std::sync::RwLock;

use crate::game_client_routes::model::*;
use crate::helius_rpc::test_api;
use crate::utils::{unixtime, weak_random_base64_string};
use crate::{Session, SessionStatus, Sessions};

#[post("/nft_list")]
async fn nft_list(req_data: Json<NftListRequest>) -> impl Responder {
    println!("{req_data:?}");
    HttpResponse::Ok().json(NftListResponse { nft_list: None })
}

#[post("/session_get")]
async fn session_get(
    state_sessions: Data<RwLock<Sessions>>,
    req_data: Json<SessionGetRequest>,
) -> impl Responder {
    println!("session_get request data: {req_data:?}");

    // Construct default response
    let mut response = SessionGetResponse::default();

    // Current time
    let now = unixtime();

    // Attempt to lock and read the database
    if let Ok(mut sessions) = state_sessions.read() {
        if let Some(entry) = sessions.data.get(&req_data.pubkey) {
            match entry.is_expired(now) {
                true => {
                    response.state = SessionStateClient::Expired;
                }
                false => {
                    response.state = SessionStateClient::Active;
                    response.entropy = entry.entropy.clone().into();
                }
            };
        }
    }

    HttpResponse::Ok().json(response)
}

#[post("/session_init")]
async fn session_init(
    state_sessions: Data<RwLock<Sessions>>,
    req_data: Json<SessionInitRequest>,
) -> impl Responder {
    println!("session_init request data: {req_data:?}");

    // Construct default response
    let mut response = SessionInitResponse::default();

    // Current time
    let now = unixtime();

    // New entropy
    let new_entropy = weak_random_base64_string(44);

    // Attempt to lock and update the database
    if let Ok(mut sessions) = state_sessions.write() {
        sessions
            .data
            .entry(req_data.pubkey.clone())
            .and_modify(|entry| {
                // Return if the session is not expired, otherwise overwrite the data if it timed out, or the game was never started
                if !entry.is_expired(now) {
                    response.result = SessionInitResult::ErrorGameActive;
                    return;
                }

                entry.entropy = new_entropy.clone();
                entry.state = SessionStatus::AwaitingSignature;
                entry.unixtime = now;
            })
            .or_insert(Session {
                entropy: new_entropy.clone(),
                state: SessionStatus::AwaitingSignature,
                unixtime: now,
            });

        // Did we find an active game?
        if response.result == SessionInitResult::ErrorGameActive {
            return HttpResponse::Ok().json(response);
        }

        // The response is positive if we got here
        response.result = SessionInitResult::Success;
        response.entropy = Some(new_entropy);
    }

    HttpResponse::Ok().json(response)
}

#[post("/session_cancel")]
async fn session_cancel(
    state_sessions: Data<RwLock<Sessions>>,
    req_data: Json<SessionCancelRequest>,
) -> impl Responder {
    println!("session_cancel request data: {req_data:?}");

    // Construct default response
    let mut response = SessionCancelResponse::default();

    // Check the signature TODO

    // Delete session data from the DB TODO

    //TEST helius RPC API
    let res = test_api().await;
    match res {
        Ok(itm) => {
            println!("item {itm:?}");
        }
        Err(e) => {
            println!("error {e:?}");
        }
    }

    HttpResponse::Ok().json(response)
}

#[post("/game_start")]
async fn game_start(
    state_sessions: Data<RwLock<Sessions>>,
    req_data: Json<GameStartRequest>,
) -> impl Responder {
    println!("game_start request data: {req_data:?}");

    // Construct default response
    let mut response = GameStartResponse::default();

    // Current time
    let now = unixtime();

    // Find the data
    if let Ok(mut sessions) = state_sessions.write() {
        if let Some(entry) = sessions.data.get_mut(&req_data.pubkey) {
            // Compare given entropy and expected state
            if entry.entropy != req_data.entropy
                || entry.state != SessionStatus::AwaitingSignature
                || entry.unixtime + 600 < now
            {
                response.result = GameStartResult::ErrorRequestDataDoesNotMatch;
                return HttpResponse::Ok().json(response);
            }

            // Create expected signed message
            let signed_message = format!("START GAME {}:{}", &req_data.pubkey, &req_data.entropy);

            // Verify the given signature
            // TODO IMPLEMENT ME must match req_data.signature
            if false {
                response.result = GameStartResult::ErrorSignatureInvalid;
                return HttpResponse::Ok().json(response);
            }

            // Update state to GameStarted
            entry.state = SessionStatus::GameStarted;
            entry.unixtime = now;

            // The response will be success
            response.result = GameStartResult::Success;
        } else {
            response.result = GameStartResult::ErrorNoSuchSession;
        }
    }

    HttpResponse::Ok().json(response)
}

#[post("/game_complete")]
async fn game_complete(
    state_sessions: Data<RwLock<Sessions>>,
    req_data: Json<GameCompleteRequest>,
) -> impl Responder {
    println!("game_complete request data: {req_data:?}");

    // Construct default response
    let mut response = GameCompleteResponse::default();

    // Current time
    let now = unixtime();

    if let Ok(mut sessions) = state_sessions.write() {
        if let Some(entry) = sessions.data.get_mut(&req_data.pubkey) {
            // Compare given entropy and expected state
            if entry.entropy != req_data.entropy
                || entry.state != SessionStatus::GameStarted
                || entry.unixtime + 3600 < now
            {
                response.result = GameCompleteResult::ErrorRequestDataDoesNotMatch;
                return HttpResponse::Ok().json(response);
            }

            // Create expected signed message
            let signed_message = format!(
                "COMPLETE GAME {}:{}:{}",
                &req_data.pubkey,
                &req_data.entropy,
                &req_data.nft_list.clone().unwrap_or_default().join("")
            );

            // Verify the given signature
            // TODO IMPLEMENT ME must match req_data.signature
            if false {
                response.result = GameCompleteResult::ErrorSignatureInvalid;
                return HttpResponse::Ok().json(response);
            }

            // Reset state
            entry.state = SessionStatus::AwaitingSignature;
            entry.unixtime = 0;

            // The response will be success
            response.result = GameCompleteResult::Success;
        } else {
            response.result = GameCompleteResult::ErrorNoSuchSession;
        }
    }

    HttpResponse::Ok().json(response)
}
