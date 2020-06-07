mod authorization;
mod common;
mod error;
mod handler;

use crate::connectors::ConnectorsBuilders;
use authorization::{professional_user_filter, public_user_filter};
use common::Context;
use error::handle_rejection;
use std::{env, net::SocketAddr};
use warp::Filter;

const CONTENT_LENGTH_LIMIT: u64 = 1024 * 16;

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

    // GET / -> HealthResponse
    let health = warp::get().and(warp::path::end()).map(handler::index);

    // GET /scan?<uuid> -> Place
    let scan = warp::get()
        .and(warp::path!("scan"))
        .and(warp::query())
        .and(context_filter.clone())
        .map(handler::scan);

    // POST /checkin {uuid, email, store_email, duration} -> 200
    let checkin = warp::post()
        .and(warp::path!("checkin"))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(handler::checkin);

    // POST /validate-device {device_id, confirmation_token} -> Credentials
    let validate_device = warp::post()
        .and(warp::path!("validate-device"))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(handler::validate_device);

    // GET /profile -> Profile(Organization)
    let get_profile = warp::get()
        .and(warp::path!("profile"))
        .and(public_user_filter(context.clone()))
        .and(context_filter.clone())
        .map(handler::get_profile);

    // POST /profile {email?} -> 200
    let set_profile = warp::post()
        .and(warp::path!("profile"))
        .and(public_user_filter(context.clone()))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(handler::set_profile);

    // POST /organization {name} -> 200
    let set_organization = warp::post()
        .and(warp::path!("organization"))
        .and(professional_user_filter(context.clone()))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(handler::set_organization);

    // GET /checkins -> Checkin(Place)
    let get_checkins = warp::get()
        .and(warp::path!("checkins"))
        .and(public_user_filter(context.clone()))
        .and(context_filter.clone())
        .map(handler::checkins);

    // POST /login {email, role, organization_name?} -> 200
    let login = warp::post()
        .and(warp::path!("login"))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(handler::login);

    // Concatenate routes
    let routes = health
        .or(scan)
        .or(checkin)
        .or(validate_device)
        .or(get_profile)
        .or(set_profile)
        .or(set_organization)
        .or(get_checkins)
        .or(login)
        .recover(handle_rejection);

    log::info!("Configured for {}", environment);
    log::info!("Listening on {}", addr);

    warp::serve(routes).run(addr).await;
}
