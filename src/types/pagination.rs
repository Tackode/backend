use serde::{Deserialize, Serialize};

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PaginationQuery {
    #[validate(range(min = 1, max = 10000))]
    pub page: i64,
    #[validate(range(min = 1, max = 10))]
    pub limit: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub page: i64,
    pub next_page: Option<i64>,
}
