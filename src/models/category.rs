use serde::{Serialize, Deserialize};
use super::app::App;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category{
    pub name: String,
    pub icon: String,
    pub description: String,
    pub apps: Vec<App>,
}

