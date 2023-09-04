use std::sync::Arc;
use axum::{
    Router,
    extract::State,
    routing,
    response::{
        IntoResponse,
        Html,
        Json
    },
    middleware,
    http::header::{self, HeaderValue},
};
use base64::{engine::general_purpose, Engine as _};
use minijinja::context;

use crate::http::{
    AppState,
    jwt_auth::auth,
};

use super::ENV;

//pub fn router(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
pub fn router(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/favicon.ico",
            routing::get(favicon)
        )
        .route("/healthcheck",
            routing::get(healthcheck)
        )
        .route("/",
            routing::get(get_root)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
        )
}

async fn favicon() -> impl IntoResponse {
    let one_pixel_favicon = "";
    let pixel_favicon= general_purpose::STANDARD.decode(one_pixel_favicon).unwrap();
    ([(header::CONTENT_TYPE, HeaderValue::from_static("image/png"))], pixel_favicon)
}

async fn get_root(
    State(app_state): State<Arc<AppState>>,
) -> impl IntoResponse{
    let template = ENV.get_template("index.html").unwrap();
    let ctx = context! {
        title => app_state.config.get_board_name(),
        categories => app_state.config.get_categories(),
    };
    Html(template.render(ctx).unwrap())
}

async fn healthcheck() -> impl IntoResponse{
    Json(serde_json::json!({
        "status": "success",
        "message": "Up and running"
    }))
}
