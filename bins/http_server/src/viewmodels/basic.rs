use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Default, ToSchema)]
pub struct Basic {}

#[derive(Debug, Deserialize, Default, ToSchema, Validate)]
pub struct CreateBasicRequest {
    #[validate(length(min = 3))]
    pub name: String,
}

#[derive(Debug, Deserialize, Default, ToSchema)]
pub struct CreateBasicResponse {}
