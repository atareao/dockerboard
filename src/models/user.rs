use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use super::config::Configuration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User{
    pub name: String,
    pub hashed_password: String,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct UserSchema {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub id: i64,
    pub name: String,
    pub role: String,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
