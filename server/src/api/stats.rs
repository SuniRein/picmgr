use serde::Serialize;

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct CountResponse {
    pub count: i64,
}
