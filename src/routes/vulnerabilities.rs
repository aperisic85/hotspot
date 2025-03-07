use actix_web::{web, HttpResponse, Responder};
use log::info;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SqlInjectionPayload {
    param: String,
}

#[derive(Deserialize)]
pub struct XssPayload {
    input: String,
}

// Simulated SQL Injection Endpoint
pub async fn sql_injection(payload: web::Query<SqlInjectionPayload>) -> impl Responder {
    let query_param = &payload.param;

    // Log the SQL injection attempt
    info!(
        "Potential SQL Injection attempt detected! Query parameter: {}",
        query_param
    );

    // Respond with a fake database error to simulate vulnerability
    HttpResponse::InternalServerError().body("Database error: invalid syntax near 'DROP'")
}

// Simulated XSS Endpoint
pub async fn xss(payload: web::Query<XssPayload>) -> impl Responder {
    let user_input = &payload.input;

    // Log the XSS attempt
    info!(
        "Potential XSS attempt detected! User input: {}",
        user_input
    );

    // Respond with the input to simulate a reflected XSS vulnerability
    HttpResponse::Ok().body(format!(
        "<html><body>Your input was: {}</body></html>",
        user_input
    ))
}

pub async fn command_injection(cmd: web::Path<String>) -> impl Responder {
    info!("Potential command injection attempt detected! Command: {}", cmd);

    let fake_output = format!("Executing command: {}\nOutput: Command not found", cmd);
    HttpResponse::Ok().body(fake_output)
}