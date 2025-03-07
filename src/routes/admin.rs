// src/routes/admin.rs
use actix_web::{web, HttpResponse, Responder};
use log::info;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AdminLoginForm {
    username: String,
    password: String,
}

pub async fn admin_login_page() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../templates/admin_login.html"))
}

pub async fn admin_login(form: web::Form<AdminLoginForm>) -> impl Responder {
    info!("Admin login attempt - Username: {}, Password: {}", form.username, form.password);
    HttpResponse::Found().header("Location", "/admin/dashboard").finish()
}

pub async fn admin_dashboard() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../templates/admin_dashboard.html"))
}
