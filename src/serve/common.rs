use crate::connectors::ConnectorsBuilders;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone)]
pub struct Context {
    pub builders: ConnectorsBuilders,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub healthy: bool,
}

#[derive(Deserialize)]
pub struct ScanQuery {
    pub uuid: Uuid,
}
