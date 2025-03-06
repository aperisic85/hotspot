use actix_web::{web,HttpResponse, Responder};
use crate::logging::log_login_attempt;

#[derive(serde::Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

pub async fn fake_login(info: web::Json<LoginInfo>) -> impl Responder {
    log_login_attempt(&info.username, &info.password);
    //HttpResponse::Ok().body("Login successful")
    // Always return a failure response
    HttpResponse::Unauthorized().json(serde_json::json!({
        "error": "Invalid username or password",
        "message": "Please check your credentials and try again."
    }))
}