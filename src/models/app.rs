use docker_api::models::ContainerSummary;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct App{
    pub name: String,
    pub description: String,
    pub url: String,
    pub icon: String,
    pub new_tab: bool,
    pub container_id: Option<String>,
    pub container_name: Option<Vec<String>>,
    pub image_name: Option<String>,
    pub image_id: Option<String>,
    pub state: Option<String>,
}

impl App{
    pub fn from_summary(summary: ContainerSummary) -> Self{
        let labels = summary.labels.unwrap();

        let name =  labels.get("es.atareao.board.name").unwrap().to_string();
        let description = match &labels.get("es.atareao.board.description"){
            Some(description) => description.to_string(),
            None => "".to_string(),
        };
        let url = match &labels.get("es.atareao.board.url"){
            Some(url) => url.to_string(),
            None => "".to_string(),
        };
        let icon = match &labels.get("es.atareao.board.icon"){
            Some(icon) => icon.to_string(),
            None => "".to_string(),
        };
        let new_tab = match &labels.get("es.atareao.board.new_tab"){
            Some(new_tab) => *new_tab == "true",
            None => true,
        };
        let container_id = summary.id;
        let container_name = summary.names;
        let image_name = summary.image;
        let image_id = summary.image_id;
        let state = summary.state;
        Self{
            name,
            description,
            url,
            icon,
            new_tab,
            container_id,
            container_name,
            image_name,
            image_id,
            state
        }
    }
}
