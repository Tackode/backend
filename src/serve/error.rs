use custom_error::custom_error;
use serde::Serialize;
use std::convert::Infallible;
use tracing::error;
use validator::ValidationErrors;
use warp::http::StatusCode;
use warp::{reject, Rejection, Reply};

custom_error! { pub Error
    InvalidData = "Invalid data",
    InvalidDataWithDetails {source: ValidationErrors} = "Invalid data: {source}",
    Unauthorized = "Unauthorized",
    ModelError {source: crate::model::error::Error} = "[Model] {source}",
}

impl reject::Reject for Error {}

impl From<crate::model::error::Error> for Rejection {
    fn from(error: crate::model::error::Error) -> Self {
        warp::reject::custom(Error::ModelError { source: error })
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

struct InternalErrorResponse {
    code: StatusCode,
    message: String,
}

/// Handle all rejctions and returns 400, 401, 404 and 500
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let response: InternalErrorResponse;

    if let Some(error) = err.find::<Error>() {
        error!("{:?}", error);

        response = InternalErrorResponse {
            code: match error {
                Error::InvalidData => StatusCode::BAD_REQUEST,
                Error::InvalidDataWithDetails { .. } => StatusCode::BAD_REQUEST,
                Error::Unauthorized => StatusCode::UNAUTHORIZED,
                Error::ModelError { source } => match source {
                    crate::model::error::Error::NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
            },
            message: error.to_string(),
        };
    } else if let Some(missing_header) = err.find::<warp::reject::MissingHeader>() {
        if missing_header.name() == "authorization" {
            response = InternalErrorResponse {
                code: StatusCode::UNAUTHORIZED,
                message: String::from("Unauthorized"),
            };
        } else {
            error!("Missing header: {:?}", missing_header);

            response = InternalErrorResponse {
                code: StatusCode::BAD_REQUEST,
                message: String::from("Bad request"),
            };
        }
    } else if let Some(body_error) = err.find::<warp::body::BodyDeserializeError>() {
        error!("Body error: {:?}", body_error);

        response = InternalErrorResponse {
            code: StatusCode::BAD_REQUEST,
            message: String::from("Bad request"),
        };
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        error!("Maybe not found error: {:?}", err);

        // Must be a not found response
        response = InternalErrorResponse {
            code: StatusCode::NOT_FOUND,
            message: String::from("Not found"),
        };
    } else {
        error!("Unhandled error: {:?}", err);

        response = InternalErrorResponse {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: String::from("Internal server error"),
        };
    }

    Ok(warp::reply::with_status(
        warp::reply::json(&ErrorResponse {
            code: response.code.as_u16(),
            message: response.message,
        }),
        response.code,
    ))
}
