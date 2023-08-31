use serde::{Serialize, Deserialize};
use docker_api::{
    Docker,
    opts::{
        ContainerListOpts,
        ContainerFilter
    },
};
use tracing::{info, debug, error};
use super::App;

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
    pub async fn init(&mut self, docker: &Docker) {
        info!("init");
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
        debug!("Opts: {:?}", &opts);
        self.apps = Vec::new();
        match docker.containers().list(&opts).await{
            Err(e) => {
                error!("{:?}", e);
            },
            Ok(container_summaries) => {
                debug!("{:?}", container_summaries);
                for summary in container_summaries.into_iter(){
                    self.apps.push(App::from_summary(summary));
                }
            }
        }
    }
}
