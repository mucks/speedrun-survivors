use actix_web::{guard, web::scope, Scope};

mod model;
mod routes;

pub fn client_routes() -> Scope {
    scope("/play")
        .guard(guard::Header("content-type", "application/json"))
        .service(routes::nft_list)
        .service(routes::session_get)
        .service(routes::session_init)
        .service(routes::session_cancel)
        .service(routes::game_start)
        .service(routes::game_complete)
}
