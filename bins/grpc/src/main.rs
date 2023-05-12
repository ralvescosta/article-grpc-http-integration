mod services;

use configs::{Configs, Empty};
use configs_builder::ConfigBuilder;
use opentelemetry::{global, Context};
use protos::v1::basic::basics_server::BasicsServer;
use services::basic::BasicsService;
use std::{error::Error, time::Duration};
use tonic::transport::Server;
use tracing::{debug, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = default_setup().await?;

    declare_health_meter()?;

    debug!("starting grpc server");
    Server::builder()
        .timeout(Duration::from_secs(60))
        .add_service(BasicsServer::new(BasicsService::new()))
        .serve_with_shutdown(cfg.app.app_addr().parse().unwrap(), async {
            debug!(addr = cfg.app.app_addr(), "grpc server is running");
            match tokio::spawn(tokio::signal::ctrl_c()).await {
                Err(e) => error!(
                    error = e.to_string(),
                    "server shutdown - something went wrong"
                ),
                _ => {}
            };
        })
        .await?;

    Ok(())
}

async fn default_setup() -> Result<Configs<Empty>, Box<dyn Error>> {
    let cfg = ConfigBuilder::new().otlp().build::<Empty>().await?;

    traces::otlp::setup(&cfg)?;
    metrics::otlp::setup(&cfg)?;

    Ok(cfg)
}

fn declare_health_meter() -> Result<(), Box<dyn Error>> {
    let meter = global::meter("grpc");
    let health_counter = meter
        .u64_observable_counter("grpc_health")
        .with_description("gRPC Server Health Counter")
        .init();

    let callback = move |ctx: &Context| {
        health_counter.observe(ctx, 1, &[]);
    };

    match meter.register_callback(callback) {
        Err(err) => {
            error!(error = err.to_string(), "error to register health counter");
            Err(err)
        }
        _ => Ok(()),
    }?;

    Ok(())
}
