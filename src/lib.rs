//! src/lib.rs

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
// Include an import to run a server with actix-web
use actix_web::dev::Server;
// Include TcpListener to discover the port the OS has provided for the webserver
use std::net::TcpListener;

/*async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World!");
    format!("Hello {}", name)
}*/

async fn health_check(_req: HttpRequest) -> HttpResponse {
    // .finish() sets an empty body & builds the HttpResponse
    // impl Responder enables us to drop the .finish()
    HttpResponse::Ok().finish()
}

// Start simple with a 200 return for the handler
async fn subscribe(_req: HttpRequest) -> HttpResponse {
    // this will not parse the body of the request, as such the input validation tests will not fail
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
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    // .await is dropped from the library

    Ok(server)
}
