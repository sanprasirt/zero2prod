use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use sqlx::{postgres::PgPoolOptions};
use zero2prod::telemetry::{get_subscriber, init_subscriber};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Faild to read configuration!");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        // .connect_lazy(&configuration.database.connection_string().expose_secret())
        .connect_lazy_with(configuration.database.with_db());
        // .expect("Failed to connect to Postgres connection pool.");
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await in our Server
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}
