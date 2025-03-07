use actix_web::{web, HttpResponse, Responder};
use log::info;
use serde::Deserialize;
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};

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

pub async fn path_traversal(path: web::Path<String>) -> impl Responder {
    info!("Potential path traversal attempt detected! Path: {}", path);

    let fake_content = format!("Content of file: {}", path);
    HttpResponse::Ok().body(fake_content)
}

pub async fn file_upload_trap(mut payload: Multipart) -> impl Responder {
    while let Ok(Some(mut field)) = payload.try_next().await {
        // Extract content-disposition headers
        if let Some(content_disposition) = field.content_disposition() {
            if let Some(filename) = content_disposition.get_filename() {
                info!("File upload attempt detected - Filename: {}", filename);

                let mut size = 0;
                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(data) => size += data.len(),
                        Err(e) => {
                            info!("Error reading file chunk: {}", e);
                            return HttpResponse::InternalServerError().body("Error processing file");
                        }
                    }
                }

                info!("File size: {} bytes", size);
            } else {
                info!("File upload attempt detected but no filename provided.");
            }
        } else {
            info!("Missing content-disposition header in file upload.");
        }
    }

    HttpResponse::Ok().body("File uploaded successfully")
}
