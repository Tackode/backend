#![feature(proc_macro_hygiene, decl_macro)]

extern crate openssl; // Should be before diesel
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod connectors;
mod models;
mod serve;

use connectors::ConnectorsBuilders;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    // Load configuration
    dotenv().ok();

    // Load logger
    pretty_env_logger::init();

    // Load database
    let connectors_builders = ConnectorsBuilders::new();

    // Run command
    serve::run(connectors_builders).await;
}
