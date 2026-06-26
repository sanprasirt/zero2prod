// use actix_web::{App, HttpResponse, HttpServer, Responder, web};
// use actix_web::dev::Server;
// use std::net::TcpListener;

pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;

// async fn health_check() -> impl Responder {
//     HttpResponse::Ok().finish()
// }

// #[derive(serde::Deserialize)]
// pub struct FormData {
//     email: String,
//     name: String
// }

// Let's start simple
// async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

// Notice the different signature!
// Starts the Actix web server and exposes the health check endpoint.
// pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
//     let server = HttpServer::new(|| {
//         App::new()
//             .route("/health_check", web::get().to(health_check))
//             .route("/subscriptions", web::post().to(subscribe))
//         })
//         .listen(listener)?
//         .run();
//     // No. .await here!
//     Ok(server)
// }