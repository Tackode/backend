use custom_error::custom_error;
use serde::Serialize;
use std::convert::Infallible;
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
    let response: Result<InternalErrorResponse, Infallible>;

    if let Some(error) = err.find::<Error>() {
        log::error!("{:?}", error);

        response = Ok(InternalErrorResponse {
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
        });
    } else if let Some(missing_header) = err.find::<warp::reject::MissingHeader>() {
        if missing_header.name() == "authorization" {
            response = Ok(InternalErrorResponse {
                code: StatusCode::UNAUTHORIZED,
                message: String::from("Unauthorized"),
            })
        } else {
            log::error!("Missing header: {:?}", missing_header);

            response = Ok(InternalErrorResponse {
                code: StatusCode::BAD_REQUEST,
                message: String::from("Bad request"),
            })
        }
    } else if let Some(body_error) = err.find::<warp::body::BodyDeserializeError>() {
        log::error!("Body error: {:?}", body_error);

        response = Ok(InternalErrorResponse {
            code: StatusCode::BAD_REQUEST,
            message: String::from("Bad request"),
        });
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        log::error!("Maybe not found error: {:?}", err);

        // Must be a not found response
        response = Ok(InternalErrorResponse {
            code: StatusCode::NOT_FOUND,
            message: String::from("Not found"),
        })
    } else {
        log::error!("Unhandled error: {:?}", err);

        response = Ok(InternalErrorResponse {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: String::from("Internal server error"),
        })
    }

    response.map(|error| {
        warp::reply::with_status(
            warp::reply::json(&ErrorResponse {
                code: error.code.as_u16(),
                message: error.message,
            }),
            error.code,
        )
    })
}
