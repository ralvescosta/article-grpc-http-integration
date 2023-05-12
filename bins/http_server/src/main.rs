mod controllers;
mod openapi;
mod routes;
mod viewmodels;

use crate::{openapi::ApiDoc, routes::basic};
use actix_web::web::{self, ServiceConfig};
use configs::{Configs, Empty};
use configs_builder::ConfigBuilder;
use http_components::CustomServiceConfigure;
use httpw::server::HTTPServer;
use std::error::Error;
use tonic::transport::Channel;
use tracing::error;
use utoipa::OpenApi;

const GRPC_ENDPOINT: &str = "http://0.0.0.0:50051";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut cfg = default_setup().await?;
    cfg.app.port = 8761;

    let channel = match tonic::transport::Endpoint::new(GRPC_ENDPOINT) {
        Err(err) => {
            error!(error = err.to_string(), "failure to create prost  endpoint");
            Err(err)
        }
        Ok(e) => match e.connect().await {
            Ok(c) => Ok(c),
            Err(err) => {
                error!(
                    error = err.to_string(),
                    "failure to stablish the connection"
                );
                Err(err)
            }
        },
    }?;

    let doc = ApiDoc::openapi();
    HTTPServer::new(&cfg.app)
        .custom_configure(container(channel))
        .custom_configure(basic::basic_route())
        .openapi(&doc)
        .start()
        .await?;

    Ok(())
}

fn container(channel: Channel) -> CustomServiceConfigure {
    CustomServiceConfigure::new(move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::<Channel>::new(channel.clone()));
    })
}

async fn default_setup() -> Result<Configs<Empty>, Box<dyn Error>> {
    let cfg = ConfigBuilder::new().otlp().build::<Empty>().await?;

    traces::otlp::setup(&cfg)?;
    metrics::otlp::setup(&cfg)?;

    Ok(cfg)
}
