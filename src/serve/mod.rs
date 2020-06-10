mod authorization;
mod common;
mod controller;
mod error;

use crate::connectors::ConnectorsBuilders;
use common::Context;
use error::handle_rejection;
use std::{env, net::SocketAddr};
use warp::{http::header, http::Method, Filter};

pub async fn run(builders: ConnectorsBuilders) {
    let environment = env::var("BACKEND_ENV").expect("Missing BACKEND_ENV");

    let addr: SocketAddr = env::var("LISTEN")
        .expect("Missing LISTEN")
        .parse()
        .expect("Invalid LISTEN");

    // Prepare Context
    let context = Context { builders };
    let moved_context = context.clone();
    let context_filter = warp::any().map(move || moved_context.clone());

    // CORS
    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::DELETE])
        .allow_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_any_origin();

    // Concatenate routes
    let routes = controller::health::routes()
        .or(controller::authentication::routes(context.clone()))
        .or(controller::place::routes(context.clone()))
        .or(controller::profile::routes(context.clone()))
        .or(controller::organization::routes(context.clone()))
        .or(controller::checkin::routes(context.clone()))
        .or(controller::infection::routes(context.clone()))
        .recover(handle_rejection)
        .with(cors);

    log::info!("Configured for {}", environment);
    log::info!("Listening on {}", addr);

    warp::serve(routes).run(addr).await;
}
