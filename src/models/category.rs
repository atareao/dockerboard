use serde::{Serialize, Deserialize};
use super::app::App;
use docker_api::{
    Docker,
    opts::{
        ContainerListOpts,
        ContainerFilter
    }, models::ContainerSummary,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category{
    pub name: String,
    pub icon: String,
    pub description: String,
}

impl Category {
    pub async fn get_containers(&self, docker: &Docker) -> Result<Vec<ContainerSummary>, docker_api::Error>{
        let mut filters: Vec<ContainerFilter> = Vec::new();
        filters.push(ContainerFilter::Label(
            "es.atareao.board.active".to_string(),
            "true".to_string()));
        filters.push(ContainerFilter::Label(
            "es.atareao.board.category".to_string(),
            self.name.to_string()));
        filters.push(ContainerFilter::LabelKey(
            "es.atareao.board.name".to_string()));
        let opts = ContainerListOpts::builder()
            .filter(filters)
            .build();
        docker.containers().list(&opts).await
    }
}
