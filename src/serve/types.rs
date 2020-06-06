use serde::Serialize;
// use uuid::Uuid;

#[derive(Serialize)]
pub struct HealthResponse {
    pub healthy: bool,
}

// #[derive(Serialize)]
// pub struct ScanResponse {
//     pub uuid: Uuid,
//     pub name: String,
//     pub location: Option<String>,
//     /// Average duration in minutes
//     pub average_duration: i64,
// }

// // #[derive(FromForm)]
// pub struct CheckinForm {
//     pub uuid: Uuid,
//     /// Login is the email hashed
//     pub login: String,
//     pub email: Option<String>,
//     pub token: Option<String>,
// }

// #[derive(Serialize)]
// pub struct CheckinResponse {
//     pub token: String,
// }

// #[derive(Serialize)]
// pub struct ValidateDeviceResponse {
//     pub valid: bool,
// }
