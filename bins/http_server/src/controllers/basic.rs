use crate::viewmodels::basic::CreateBasicRequest;
use actix_web::{
    post,
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder,
};
use http_components::{middlewares::otel::HTTPExtractor, validate, viewmodels::HTTPError};
use opentelemetry::{
    global,
    trace::{Status, TraceContextExt},
};
use protos::v1::basic::{basics_client::BasicsClient, CreateRequest};
use tonic::transport::Channel;
use tracing::error;

#[utoipa::path(
  post,
  path = "",
  context_path = "/v1/basic",
  tag = "basic",
  request_body = CreateBasicRequest,
  responses(
    (status = 201, description = "Basic created"),
    (status = 400, description = "Bad request", body = HTTPError ),
    (status = 409, description = "Conflict", body = HTTPError),
    (status = 500, description = "Internal server error", body = HTTPError)
  ),
)]
#[post("")]
pub async fn create(
    req: HttpRequest,
    body: Json<CreateBasicRequest>,
    channel: Data<Channel>,
) -> impl Responder {
    let ctx = global::get_text_map_propagator(|propagator| {
        propagator.extract(&HTTPExtractor::new(req.headers()))
    });

    if let Err(err) = validate::body_validator(&ctx, &body.0) {
        return HttpResponse::BadRequest().json(err);
    };

    let mut grpc_request = tonic::Request::new(CreateRequest {});
    traces::grpc::inject(&ctx, grpc_request.metadata_mut());

    let mut client = BasicsClient::new(channel.get_ref().to_owned());
    match client.create(grpc_request).await {
        Ok(_) => {
            ctx.span().set_status(Status::Ok);
            HttpResponse::Ok().finish()
        }
        Err(err) => {
            error!(error = err.to_string(), "failure to create");
            HttpResponse::BadRequest().json(HTTPError::bad_request("failure to create", ""))
        }
    }
}
