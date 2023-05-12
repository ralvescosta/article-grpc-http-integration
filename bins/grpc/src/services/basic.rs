use async_trait::async_trait;
use opentelemetry::{
    global,
    trace::{Status, TraceContextExt},
};
use protos::v1::basic::{basics_server::Basics, CreateRequest, ListPagedRequest, ListResponse};
use traces::grpc::GRPCExtractor;
use tracing::info;

pub struct BasicsService {}

impl BasicsService {
    pub fn new() -> BasicsService {
        return BasicsService {};
    }
}

#[async_trait]
impl Basics for BasicsService {
    async fn create(
        &self,
        request: tonic::Request<CreateRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let ctx = global::get_text_map_propagator(|propagator| {
            propagator.extract(&GRPCExtractor::new(request.metadata()))
        });

        info!("request received");

        ctx.span().set_status(Status::Ok);

        Ok(tonic::Response::new(()))
    }

    async fn list_paged(
        &self,
        request: tonic::Request<ListPagedRequest>,
    ) -> Result<tonic::Response<ListResponse>, tonic::Status> {
        let ctx = global::get_text_map_propagator(|propagator| {
            propagator.extract(&GRPCExtractor::new(request.metadata()))
        });

        ctx.span().set_status(Status::Ok);

        Ok(tonic::Response::new(ListResponse::default()))
    }
}
