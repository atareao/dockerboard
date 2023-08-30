use serde::{Serialize, Deserialize};
use docker_api::{
    Docker,
    opts::{
        ContainerListOpts,
        ContainerFilter
    }, models::ContainerSummary,
};
use tracing::{info, error};
use super::{error::DBError, App};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category{
    pub name: String,
    pub icon: String,
    pub description: String,
    #[serde(default = "get_default_apps")]
    pub apps: Vec<App>,
}

fn get_default_apps() -> Vec<App>{
    Vec::new()
}

impl Category {
    pub async fn get_containers(&self, docker: &Docker) -> Vec<App>{
        info!("get_containers");
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
        match docker.containers().list(&opts).await{
            Err(e) => {
                error!("{:?}", e);
                Vec::new()
            },
            Ok(container_summaries) => {
                let mut containers: Vec<App> = Vec::new();
                for summary in container_summaries.into_iter(){
                    containers.push(App::from_summary(summary));
                }
                containers
            }
        }
    }
}
