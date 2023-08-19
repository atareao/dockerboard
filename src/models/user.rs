use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User{
    pub name: String,
    pub hashed_password: String,
    pub active: bool,
}

