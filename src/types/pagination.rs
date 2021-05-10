use serde::{Deserialize, Serialize};

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PaginationQuery {
    #[validate(range(min = 1, max = 10000))]
    pub page: usize,
    #[validate(range(min = 1, max = 10))]
    pub limit: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub page: usize,
    pub next_page: Option<usize>,
}
