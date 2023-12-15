mod game_client_routes;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::{bail, Result};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::RwLock;
use std::time::SystemTime;

#[derive(Debug, PartialEq)]
enum SessionStatus {
    AwaitingSignature,
    GameStarted,
}

#[derive(Debug)]
struct Session {
    entropy: String,
    state: SessionStatus,
    unixtime: u64,
}

#[derive(Default)]
struct Sessions {
    data: HashMap<String, Session>,
}

fn is_session_expired(session: &Session, now: u64) -> bool {
    //TODO refactor move
    // In case of AwaitingSignature we must wait some time for the client to sign, otherwise there might be a DoS opportunity
    (session.state == SessionStatus::AwaitingSignature && session.unixtime + 30 < now)
        || session.unixtime + 3600 < now
}

fn unixtime() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let conf_ssl = configure_ssl()?;

    let state_sessions = Data::new(RwLock::new(Sessions::default()));

    log::info!("Starting HTTPS server at https://localhost:8443");
    HttpServer::new(move || {
        let conf_cors = configure_cors();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(conf_cors)
            .app_data(Data::clone(&state_sessions))
            .service(hello)
            .service(echo)
            .service(game_client_routes::client_routes())
            .default_service(web::route().to(version))
    })
    .bind_rustls_021(("127.0.0.1", 8443), conf_ssl)?
    .run()
    .await
    .map_err(anyhow::Error::from)
}

fn configure_ssl() -> Result<ServerConfig> {
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // Load the files
    let cert_file = &mut BufReader::new(File::open("cert/cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("cert/key.pem").unwrap());

    // Convert
    let cert_chain = certs(cert_file)?.into_iter().map(Certificate).collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)?
        .into_iter()
        .map(PrivateKey)
        .collect();

    // Must have the key files
    if keys.is_empty() {
        bail!("Error loading certificate or key files");
    }

    Ok(config.with_single_cert(cert_chain, keys.remove(0)).unwrap())
}

fn configure_cors() -> Cors {
    Cors::default()
        // .allowed_origin("*") //TODO restrict
        // .allowed_origin_fn(|origin, _req_head| {
        //     origin.as_bytes().ends_with(b".rust-lang.org")
        // })
        // .allowed_methods(vec!["GET", "POST"])
        // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        // .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn version() -> impl Responder {
    HttpResponse::Ok().body(format!(
        "{{\"app\":\"{}\",\"version\":\"{}\"}}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    ))
}
