//! src/lib.rs

pub mod configuration;
pub mod routes;
pub mod startup;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
// Include an import to run a server with actix-web
use actix_web::dev::Server;
// Include TcpListener to discover the port the OS has provided for the webserver
use std::net::TcpListener;

/*async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World!");
    format!("Hello {}", name)
}*/

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn health_check(_req: HttpRequest) -> HttpResponse {
    // .finish() sets an empty body & builds the HttpResponse
    // impl Responder enables us to drop the .finish()
    HttpResponse::Ok().finish()
}

// Extracts form data using serde
// This handler is only called if the content-type is *x-www-form-urlencoded*
// Content of the request could be deserialized to a 'FormData' struct
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// change the run() function to return a Result enum that contains a Server type or an Error
// TcpListener::local_addr will return a SocketAddr which exposes the bound port via .port()
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            /* // Add route for the root path registration
            .route("/", web::get().to(greet))
            // Add route for the greet handler registration
            .route("/{name}", web::get().to(greet))*/
            // Add route for the health check handler registration
            .route("/health_check", web::get().to(health_check))
            // new entry in the routing table for POST /subscription requests
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    // .await is dropped from the library

    Ok(server)
}
