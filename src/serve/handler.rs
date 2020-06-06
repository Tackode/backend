use super::types::HealthResponse;
use warp::reply::Reply;

pub fn index() -> impl Reply {
    warp::reply::json(&HealthResponse { healthy: true})
}

// #[get("/scan?<uuid>")]
// fn scan(uuid: RUuid) -> Result<Json<ScanResponse>, Error> {
//     Ok(Json(ScanResponse {
//         uuid: uuid.into_inner(),
//         name: String::from("Nagano"),
//         location: Some(String::from("Salle 1")),
//         average_duration: 60,
//     }))
// }

// #[get("/checkins?<login>&<token>")]
// fn checkin(data: Form<CheckinForm>) -> Result<Json<CheckinResponse>, Error> {
//     Ok(Json(CheckinResponse {
//         token: data.login.clone(),
//     }))
// }

// #[post("/checkin", data = "<data>")]
// fn checkin(data: Form<CheckinForm>) -> Result<Json<CheckinResponse>, Error> {
//     Ok(Json(CheckinResponse {
//         token: data.login.clone(),
//     }))
// }

// #[get("/validate-device?<token>")]
// fn validate_device(token: String) -> Result<Json<ValidateDeviceResponse>, Error> {
//     Ok(Json(ValidateDeviceResponse {
//         valid: true
//     }))
// }
