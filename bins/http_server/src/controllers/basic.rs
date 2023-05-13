use crate::viewmodels::basic::{Basic, CreateBasicRequest};
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder,
};
use http_components::{middlewares::otel::HTTPExtractor, validate, viewmodels::HTTPError};
use opentelemetry::{
    global,
    trace::{Status, TraceContextExt},
};
use protos::v1::basic::{basics_client::BasicsClient, CreateRequest, ListPagedRequest};
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

    let mut grpc_request = tonic::Request::new(CreateRequest::default());
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

#[utoipa::path(
  get,
  path = "",
  context_path = "/v1/basic",
  tag = "basic",
  responses(
    (status = 200, description = "List of basics", body = Vec<Basic>),
    (status = 400, description = "Bad request", body = HTTPError ),
    (status = 409, description = "Conflict", body = HTTPError),
    (status = 500, description = "Internal server error", body = HTTPError)
  ),
)]
#[get("")]
pub async fn list(
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

    let mut grpc_request = tonic::Request::new(ListPagedRequest::default());
    traces::grpc::inject(&ctx, grpc_request.metadata_mut());

    let mut client = BasicsClient::new(channel.get_ref().to_owned());
    match client.list_paged(grpc_request).await {
        Ok(res) => {
            ctx.span().set_status(Status::Ok);
            HttpResponse::Ok().json(
                res.get_ref()
                    .data
                    .iter()
                    .map(|_| Basic::default())
                    .collect::<Vec<Basic>>(),
            )
        }
        Err(err) => {
            error!(error = err.to_string(), "failure to create");
            HttpResponse::BadRequest().json(HTTPError::bad_request("failure to create", ""))
        }
    }
}
