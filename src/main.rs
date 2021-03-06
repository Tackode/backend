#[cfg(any(target_os = "unix", target_os = "linux"))]
extern crate openssl; // Should be before diesel
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate validator_derive;
extern crate validator;

mod connector;
mod model;
mod security;
mod serve;

use connector::ConnectorBuilder;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    // Load configuration
    dotenv().ok();

    // Load logger
    pretty_env_logger::init();

    // Load database
    let connector_builders = ConnectorBuilder::new();

    // Run command
    serve::run(connector_builders).await;
}
