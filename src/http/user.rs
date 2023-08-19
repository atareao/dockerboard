use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    body,
    Form,
    extract::{Query, State},
    Router,
    routing,
    http::{header, Response, StatusCode, },
    response::{IntoResponse, Html,},
    Extension,
    Json,
    middleware,
};

use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand_core::OsRng;
use serde_json::json;

use crate::{
    models::user::{
            UserSchema,
            TokenClaims,
            User,
            FilteredUser,
    },
    http::AppState,
    http::jwt_auth::auth,
};

pub fn router(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/v1/auth/register",
            routing::post(register)
        )
        //.route("/api/v1/auth/login",
        //    routing::post(login)
        //)
        .route("/api/v1/auth/logout",
            routing::get(logout)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
        )
        .route("/api/v1/users/me",
            routing::get(me)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
        )
}


pub async fn register(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let user_exists = User::exists(&app_state.pool, &body.email)
        .await
        .map_err(|e| {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Database error: {}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;

    if user_exists {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "User with that email already exists",
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Error while hashing password: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string())?;

    let user = User::create(&app_state.pool, &body.email, &hashed_password)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let user_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
        "user": FilteredUser::from_user(&user)
    })});

    Ok(Json(user_response))
}

pub async fn get_token(
    app_state: &Arc<AppState>,
    body: UserSchema
) -> Result<String, (StatusCode, Json<serde_json::Value>)>{
//) -> Result<Json<serde_json::Value>,(StatusCode, Json<serde_json::Value>)>{
    tracing::info!("init login");
    let user = User::read_from_email(&app_state.pool, &body.email)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?
        .ok_or_else(|| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "Invalid email or password",
            });
            (StatusCode::BAD_REQUEST, Json(error_response))
        })?;
    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };
    if !is_valid {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid email or password"
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
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
    t: Extension<Tera>,
    Form(user_data): Form<UserSchema>,
) -> impl IntoResponse{
    match get_token(&app_state, user_data).await {
        Ok(token) => {
            let cookie = Cookie::build("token", token.to_owned())
                .path("/")
                .max_age(time::Duration::hours(1))
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
                .max_age(time::Duration::hours(0))
                .same_site(SameSite::Lax)
                .http_only(true)
                .finish();

            //Err(Response::builder()
            //    .status(StatusCode::SEE_OTHER)
            //    .header(header::LOCATION, "/login?error=error")
            //    .header(header::SET_COOKIE, cookie.to_string())
            //    .body(Html("Hola"))
            //    .unwrap())
            let context = Context::new();
            Err(Html(t.render("podcast.html", &context).unwrap()))
        }
    }
}
//     let cookie = Cookie::build("token", token.to_owned())
//         .path("/")
//         .max_age(time::Duration::hours(1))
//         .same_site(SameSite::Lax)
//         .http_only(true)
//         .finish();
// 
//     let mut response = Response::new(json!({"status": "success", "token": token}).to_string());
//     response
//         .headers_mut()
//         .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
//     tracing::info!("Writing cookie");
//     tracing::info!("finish login");
//     Ok(axum::Json(json!({"status": "success", "token": token})))
// 
// }

// pub async fn login(
//         State(app_state): State<Arc<AppState>>,
//         Json(body): Json<UserSchema>,
//         ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
// 
//     match do_login(app_state, body).await{
//         Ok(body) => {
//             let response = Response::new(body.to_string());
//             tracing::info!("Writing cookie");
//             tracing::info!("finish login");
//             Ok(response)
//         },
//         Err(e) => Err(e),
//     }
// }

pub async fn logout() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(time::Duration::hours(-1))
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

pub async fn me(
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": FilteredUser::from_user(&user)
        })
    });

    Ok(Json(json_response))
}
