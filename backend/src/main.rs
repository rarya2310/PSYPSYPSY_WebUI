use actix_web::{web, App, HttpResponse, HttpServer, Result, middleware::Logger};
use actix_cors::Cors;
use serde_json::json;

async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "psypsypsy_backend",
        "version": "0.1.0"
    })))
}

async fn hello() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "message": "Hello from PSYPSYPSY Backend!",
        "framework": "Actix-web"
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    println!("🚀 Starting PSYPSYPSY Backend Server");
    println!("📍 Server running at: http://localhost:8080");
    println!("🏥 Health check: http://localhost:8080/health");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .route("/", web::get().to(hello))
            .route("/health", web::get().to(health))
            .service(
                web::scope("/api")
                    .route("/hello", web::get().to(hello))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
