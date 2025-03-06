use actix_web::{web, App, HttpServer};
use env_logger;

mod routes;
mod logging;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting server at http://0.0.0.0:8080");

    // Start Actix-web server
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(routes::index::index))
            .route("/login", web::post().to(routes::login::fake_login))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}