use docker_api::{Docker, opts::ContainerListOpts};
use tokio;
use std::{process, str::FromStr};
use tracing_subscriber::{
    EnvFilter,
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

use models::Configuration;

mod http;
mod models;

#[tokio::main]
async fn main(){
    let configuration = read_configuration().await;

    tracing_subscriber::registry()
        .with(EnvFilter::from_str(configuration.get_log_level()).unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();
    http::serve(configuration).await.unwrap();
}

async fn read_configuration() -> Configuration{
    let content = match tokio::fs::read_to_string("config.yml")
        .await {
            Ok(value) => value,
            Err(e) => {
                println!("Error with config file `config.yml`: {}",
                    e.to_string());
                process::exit(0);
            }
        };
    match Configuration::new(&content){
        Ok(configuration) => configuration,
        Err(e) => {
            println!("Error with config file `config.yml`: {}",
                e.to_string());
            process::exit(0);
        }
    }
}

async fn read_containers(configuration: &mut Configuration){
    if configuration.get_docker_uri().is_none(){
        return;
    }
    let uri = configuration.get_docker_uri().unwrap();
    match Docker::new(uri) {
        Ok(docker) => {
            let mut categories = configuration.get_categories();
            let category_names:Vec<String> = configuration.get_categories().into_iter().map(|item| item.name.to_owned()).collect();
            let opts = ContainerListOpts::builder().build();
            let containers = docker.containers().list(&opts).await.unwrap_or_default();
            for container in containers {
                let labels = container.labels.unwrap_or_default();
                if !labels.is_empty() && 
                        labels.contains_key("board.enable") && 
                        labels.get("board.enable") == Some(&"true".to_string()) &&
                        labels.contains_key("board.category") && labels.get("board.category").is_some() &&
                        labels.contains_key("board.app.name") && labels.get("board.app.name").is_some() &&
                        labels.contains_key("board.app.url") && labels.get("board.app.url").is_some() {
                    let category = labels.get("board.category").unwrap();
                    if category_names.contains(category) {
                        let name = labels.get("board.app.name").unwrap();
                        let url = labels.get("board.app.url").unwrap();
                        let icon = match labels.get("board.app.icon"){
                                Some(icon) => icon,
                                None => ""
                        };
                        let description = match labels.get("board.app.description"){
                                Some(description) => description,
                                None => ""
                        };
                        let new_tab = match labels.get("board.app.name"){
                                Some(new_tab) => new_tab.to_lowercase() == "true" || new_tab.to_lowercase() == "yes",
                            None => false,
                        };
                    }
                }
            }
        },
        Err(e) => {
            tracing::error!("Error with docker: {}", e);
        },
    }
}
