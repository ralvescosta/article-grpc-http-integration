use crate::{controllers::basic as basic_ctl, viewmodels::basic as basic_vm};
use http_components::viewmodels::HTTPError;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
  paths(
    basic_ctl::create, basic_ctl::list,
  ),
  components(
    schemas(
      HTTPError,
      basic_vm::Basic,
      basic_vm::CreateBasicRequest, basic_vm::CreateBasicResponse,
    ),
  ),
  tags(
    (name = "basic", description = "basic endpoints")
  ),
  info(
    title = "Basic HTTP API",
    version = "0.0.1",
    description = "Basic HTTP API"
  ),
  servers(
    (
      url = "http://localhost:8761", description = "local server"
    )
  )
)]
pub struct ApiDoc;
