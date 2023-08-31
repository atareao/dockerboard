use tokio;
use std::{process, str::FromStr};
use tracing_subscriber::{
    EnvFilter,
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use futures::StreamExt;
use docker_api::{Docker, models::EventMessage, opts::EventsOptsBuilder};
use tracing::{info, debug, error};

use models::Configuration;
use tokio::sync::broadcast;

mod http;
mod models;

#[derive(Clone)]
pub struct AppState {
    pub config: Configuration,
    pub tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main(){
    let mut configuration = read_configuration().await;

    tracing_subscriber::registry()
        .with(EnvFilter::from_str(configuration.get_log_level()).unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();
    if configuration.init().await.is_err(){
        println!("Can not init");
        process::exit(0);
    }

    let (tx, mut rx): (
        broadcast::Sender<String>,
        broadcast::Receiver<String>
    ) = broadcast::channel(2);

    let tx2 = tx.clone();
    let configuration2 = configuration.clone();
    let app_state = AppState{
        config: configuration,
        tx,
    };

    println!("Antes");
    tokio::spawn(async move {
        process_docker_events(&configuration2, &tx2).await;
    });
    http::serve(app_state).await.unwrap();
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

async fn process_docker_events(configuration: &Configuration, sender: &broadcast::Sender<String>){
    let hostname = "Este";
    let uri = configuration.get_docker_uri().unwrap();
    let docker = Docker::new(uri).unwrap();
    while let Some(event_result) = docker.events(&EventsOptsBuilder::default().build()).next().await {
        match event_result {
            Ok(event_message) => {
                debug!("event => {:?}", event_message);
                sender.send(format!("Message: {:?}", event_message));
                //process(event, &configuration, &hostname).await;
            },
            Err(e) => error!("Error: {}", e),
        };
    }
}

async fn process(event: EventMessage){
    debug!("event => {:?}", event);
}
