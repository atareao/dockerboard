pub mod jwt_auth;
pub mod user;
pub mod estatic;
pub mod root;

use std::{sync::Arc, net::{SocketAddr, Ipv4Addr}};
use axum::{
    Router,
    http::{
        header::{
            ACCEPT,
            AUTHORIZATION,
            CONTENT_TYPE
        },
        HeaderValue,
        Method,
    },
};
use minijinja::{Environment, path_loader};
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer,
};
use once_cell::sync::Lazy;

use crate::models::config::Configuration;

pub static ENV: Lazy<Environment<'static>> = Lazy::new(|| {
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));
    env
});

#[derive(Clone)]
pub struct AppState {
    pub config: Configuration,
}

pub async fn serve(config: Configuration) -> anyhow::Result<()> {

    let cors = CorsLayer::new()
        .allow_origin(config.get_url().parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));

    let app = api_router(
            AppState {
                config: config.clone(),
            })
            .layer(TraceLayer::new_for_http())
            .layer(cors);

    axum::Server::bind(
        &SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.get_port()))
        .serve(app.into_make_service())
        .await
        .map_err(|_err| anyhow::anyhow!("Can't init"))
}

fn api_router(app_state: AppState) -> Router {
    estatic::router()
        .merge(root::router(Arc::new(app_state.clone())))
        .merge(user::router())
        .with_state(Arc::new(app_state.clone()))
}
