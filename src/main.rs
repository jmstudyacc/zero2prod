use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World!");
    format!("Hello {}", name)
}

/// Responder is a conversion trait into an HttpResponse type
async fn health_check(_req: HttpRequest) -> impl Responder {
    // .finish() sets an empty body & builds the HttpResponse
    // impl Responder enables us to drop the .finish()
    HttpResponse::Ok() //.finish()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Add route for the root path registration
            .route("/", web::get().to(greet))
            /* Add route for the greet handler registration
            .route("/{name}", web::get().to(greet))*/
            // Add route for the health check handler registration
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
