mod common;
mod error;
mod handler;

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

    // GET / -> HealthResponse
    let health = warp::get().and(warp::path::end()).map(handler::index);

    // GET /scan?<uuid> -> Place
    let scan = warp::path!("scan")
        .and(warp::query())
        .and(context_filter)
        .map(handler::scan);

    // POST /checkin {uuid, email, store_email, duration} -> 200
    // POST /validate-device {device_id, token} -> Credentials
    // GET /profile -> Profile(Organization)
    // POST /profile {email?} -> 200
    // POST /organization {name} -> 200
    // GET /checkins -> Checkin(Place)
    // POST /login {email, role, organization_name?} -> 200

    // Concatenate routes
    let routes = health.or(scan);

    log::info!("Configured for {}", env);
    log::info!("Listening on {}", addr);

    warp::serve(routes).run(addr).await;
}
