use custom_error::custom_error;
use serde::Serialize;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::{reject, Rejection, Reply};

custom_error! { pub Error
    InvalidData = "Invalid data",
    Unauthorized = "Unauthorized",
    // ModelError {source: model::error::Error} = "[Model] {source}",
}

impl reject::Reject for Error {}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = String::from("NOT_FOUND");
    } else if let Some(error) = err.find::<Error>() {
        code = match error {
            Error::InvalidData => StatusCode::BAD_REQUEST,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            // Error::ModelError { source } => match source {
            //     Model::error::Error::ModelNotFound => StatusCode::NOT_FOUND,
            //     _ => StatusCode::INTERNAL_SERVER_ERROR,
            // },
        };

        message = error.to_string();
    } else if let Some(_) = err.find::<warp::reject::MissingHeader>() {
        code = StatusCode::UNAUTHORIZED;
        message = String::from("Unauthorized");
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = String::from("METHOD_NOT_ALLOWED");
    } else {
        log::error!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = String::from("UNHANDLED_REJECTION");
    }

    let json = warp::reply::json(&ErrorResponse {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
