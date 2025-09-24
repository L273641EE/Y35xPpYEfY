use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use std::collections::HashMap;
use sysinfo::{System, SystemExt, ProcessExt, ProcessorExt};

// Handler function to analyze memory usage
async fn analyze_memory_usage() -> impl Responder {
    let mut system = System::new_all();
    system.refresh_all(); // Refresh the system information

    let mut memory_usage = HashMap::new();
    for (pid, process) in system.processes().iter().enumerate() {
        if let Some(name) = process.name() {
            memory_usage.insert(name.to_string(), process.memory());
        }
    }

    HttpResponse::Ok().json(memory_usage)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route(