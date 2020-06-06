use custom_error::custom_error;
use serde::Serialize;
use warp::http::StatusCode;
use warp::reply::{json, Json, with_status, WithStatus};

custom_error! { pub Error
    InvalidData = "Invalid data",
    // ModelError {source: model::error::Error} = "[Model] {source}",
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    message: String,
}

impl From<Error> for WithStatus<Json> {
    fn from(error: Error) -> Self {
        // Log error
        log::error!("{}", error);

        let status = match error {
            Error::InvalidData => StatusCode::BAD_REQUEST,
            // Error::ModelError { source } => match source {
            //     Model::error::Error::ModelNotFound => StatusCode::NOT_FOUND,
            //     _ => StatusCode::INTERNAL_SERVER_ERROR,
            // },
        };

        let error_response = ErrorResponse {
            message: error.to_string(),
        };

        with_status(json(&error_response), status)
    }
}
