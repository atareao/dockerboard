use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct App{
    pub name: String,
    pub url: String,
    pub icon: String,
    pub new_tab: bool,
    pub container_name: String,
    pub image_name: String,
    pub state: bool,
}
