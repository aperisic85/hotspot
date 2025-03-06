use actix_web::{web, App, HttpServer,HttpResponse, middleware};
use actix_web::error::JsonPayloadError;
use env_logger;
use log::error;

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
        .wrap(middleware::Logger::default())
        .app_data(web::JsonConfig::default().limit(4096).error_handler(json_error_handler))
            .route("/", web::get().to(routes::index::index))
            .route("/login", web::post().to(routes::login::fake_login))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}


fn json_error_handler(err: JsonPayloadError, _req: &actix_web::HttpRequest) -> actix_web::Error {
    error!("JSON deserialization error: {:?}", err);
    let error_response = HttpResponse::BadRequest().json(serde_json::json!({
        "error": "Invalid JSON payload",
        "details": format!("{}", err),
    }));
    actix_web::error::InternalError::from_response(err, error_response).into()
}