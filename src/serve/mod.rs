mod common;
mod error;
mod handler;
mod types;

use crate::connectors::ConnectorsBuilders;
use common::Context;
use std::{env, net::SocketAddr};
use warp::Filter;

pub async fn run(builders: ConnectorsBuilders) {
    let env = env::var("BACKEND_ENV").expect("Missing BACKEND_ENV");

    let addr: SocketAddr = env::var("LISTEN")
        .expect("Missing LISTEN")
        .parse()
        .expect("Invalid LISTEN");

    let context = Context { builders };
    let context_filter = warp::any().map(move || context.clone());

    // Configure paths
    let health = warp::get().and(warp::path::end()).map(handler::index);

    log::info!("Configured for {}", env);
    log::info!("Listening on {}", addr);

    warp::serve(health).run(addr).await;
}
