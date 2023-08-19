use std::{sync::Arc, net::{SocketAddr, Ipv4Addr}};
use axum::{
    Router,
    extract::FromRequestParts,
    middleware::from_extractor,
    http::{
        header,
        StatusCode,
        request::Parts,
    },
    Extension,
    RequestPartsExt,
};
use tower_http::trace::TraceLayer;
use tower::ServiceBuilder;

use crate::models::config::Configuration;

#[derive(Clone)]
struct ApiContext {
    config: Arc<Configuration>,
}

pub async fn serve(config: Configuration) -> anyhow::Result<()> {

    let app = api_router().layer(
        ServiceBuilder::new()
            .layer(Extension(ApiContext {
                config: Arc::new(config.clone()),
            }))
            // Enables logging. Use `RUST_LOG=tower_http=debug`
            .layer(TraceLayer::new_for_http())
            .layer(from_extractor::<RequireAuth>())
    );

    axum::Server::bind(
        &SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.get_port()))
        .serve(app.into_make_service())
        .await
        .map_err(|_err| anyhow::anyhow!("Can't init"))
    
}
