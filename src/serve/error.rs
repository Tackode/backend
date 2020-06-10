use custom_error::custom_error;
use serde::Serialize;
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

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    let response: Result<InternalErrorResponse, Rejection>;

    if let Some(error) = err.find::<Error>() {
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
            response = Err(err);
        }
    } else {
        response = Err(err);
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
