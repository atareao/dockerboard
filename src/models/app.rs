use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct App{
    pub name: String,
    pub url: String,
    pub icon: String,
    pub description: String,
    pub new_tab: bool,
}
