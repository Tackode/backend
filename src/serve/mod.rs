mod authorization;
mod controller;
mod error;
mod query;
mod session;
mod types;

use crate::connector::ConnectorBuilder;
use error::handle_rejection;
use std::{env, net::SocketAddr};
use tracing::info;
use types::Context;
use warp::{http::header, http::Method, Filter};

pub async fn run(builder: ConnectorBuilder) {
    let environment = env::var("BACKEND_ENV").expect("Missing BACKEND_ENV");

    let addr: SocketAddr = env::var("LISTEN")
        .expect("Missing LISTEN")
        .parse()
        .expect("Invalid LISTEN");

    // Prepare Context
    let context = Context { builder };

    // CORS
    let mut cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION]);

    let allowed_origins_str = env::var("ALLOWED_ORIGINS").unwrap_or_default();

    if allowed_origins_str.is_empty() {
        cors = cors.allow_any_origin();
    } else {
        cors = cors.allow_origins(allowed_origins_str.split(','));
    }

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

    info!("Configured for {}", environment);
    info!("Listening on {}", addr);

    warp::serve(routes).run(addr).await;
}
