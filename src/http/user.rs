use std::sync::Arc;

use axum::{
    body,
    Form,
    extract::State,
    Router,
    routing,
    http::{header, Response, StatusCode, },
    response::{IntoResponse, Html},
    Json,
    middleware,
};

use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use minijinja::context;

use crate::{
    models::user::{
            UserSchema,
            TokenClaims,
    },
    http::AppState,
    http::jwt_auth::auth,
};

use super::ENV;

pub fn router(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/login",
            routing::get(login).post(do_login)
        )
        //.route("/login",
        //    routing::post(do_login)
        //        //.route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
        //)
}


pub async fn get_token(
    app_state: &Arc<AppState>,
    body: UserSchema
) -> Result<String, (StatusCode, Json<serde_json::Value>)>{
//) -> Result<Json<serde_json::Value>,(StatusCode, Json<serde_json::Value>)>{
    tracing::info!("init login");
    let user = app_state.config.get_user(&body.name)
        .ok_or_else(|| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "Invalid name or password",
            });
            (StatusCode::BAD_REQUEST, Json(error_response))
        })?;
    if user.password != body.password{
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid name or password"
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.name.to_string(),
        exp,
        iat,
    };

    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(app_state.config.get_secret().as_ref()),
    ).map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Encoding JWT error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?)
}

pub async fn do_login(
    State(app_state): State<Arc<AppState>>,
    Form(user_data): Form<UserSchema>,
) -> impl IntoResponse{
    tracing::info!("Post data: {:?}", user_data);
    match get_token(&app_state, user_data).await {
        Ok(token) => {
            let cookie = Cookie::build("token", token.to_owned())
                .path("/")
                .max_age(cookie::time::Duration::hours(1))
                .same_site(SameSite::Lax)
                .http_only(true)
                .finish();
            tracing::info!("El token: {}", token.to_string());
            
            Ok(Response::builder()
                .status(StatusCode::SEE_OTHER)
                .header(header::LOCATION, "/")
                .header(header::SET_COOKIE, cookie.to_string())
                .body(body::Empty::new())
                .unwrap())
        },
        Err(e) => {
            let cookie = Cookie::build("token", "")
                .path("/")
                .max_age(cookie::time::Duration::hours(0))
                .same_site(SameSite::Lax)
                .http_only(true)
                .finish();

            //Err(Response::builder()
            //    .status(StatusCode::SEE_OTHER)
            //    .header(header::LOCATION, "/login?error=error")
            //    .header(header::SET_COOKIE, cookie.to_string())
            //    .body(Html("Hola"))
            //    .unwrap())
            tracing::info!("{:?}", e);
            let template = ENV.get_template("error.html").unwrap();
            let ctx = context! {
                title             => app_state.config.get_board_name(),
                error_title       => "Error",
                error_description => e.1.get("message"),
            };
            Err(Html(template.render(ctx).unwrap()))
        }
    }
}

pub async fn logout() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(cookie::time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response = Response::new(json!({"status": "success"}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    tracing::info!("Deleting cookie");
    Ok(response)
}

pub async fn login(
    State(app_state): State<Arc<AppState>>,
) -> impl IntoResponse{
    let template = ENV.get_template("login.html").unwrap();
    let ctx = context! {
        title => app_state.config.get_board_name(),
    };
    Html(template.render(ctx).unwrap())
}
