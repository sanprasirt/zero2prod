use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Faild to read configuration!");
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await in our Server
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
