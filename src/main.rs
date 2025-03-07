use actix_web::{web, App, HttpServer,HttpResponse, middleware};
use actix_web::error::JsonPayloadError;
use env_logger;
use log::error;
use log4rs::init_file;

mod routes;
mod logging;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    init_file("log4rs.yaml", Default::default()).unwrap();
   // Initialize logger
   // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting server at http://0.0.0.0:8080");

    // Start Actix-web server
    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Logger::default())
        .app_data(web::JsonConfig::default().limit(4096).error_handler(json_error_handler))
            .route("/", web::get().to(routes::index::index))
            .route("/login", web::post().to(routes::login::fake_login))
            .route("/vulnerable/sql", web::get().to(routes::vulnerabilities::sql_injection))
            .route("/vulnerable/xss", web::get().to(routes::vulnerabilities::xss))
            .route("/vulnerable/cmd/{command}", web::get().to(routes::vulnerabilities::command_injection))
            .route("/upload", web::post().to(routes::vulnerabilities::file_upload_trap))

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