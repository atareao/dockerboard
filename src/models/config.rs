use serde::{Serialize, Deserialize};
use serde_yaml::Error;

use super::{category::Category, user::User, app::App};

const DEFAULT_LOG_LEVEL: &'static str = "info";
const DEFAULT_PORT: u16 = 6969;
const DEFAULT_MAX_AGE: i32 = 60;
const DEFAULT_EXPIRES: &'static str = "60m";


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration{
    url: String,
    #[serde(default = "get_default_port")]
    port: u16,
    #[serde(default = "get_default_log_level")]
    log_level: String,
    jwt_secret: String,
    #[serde(default = "get_default_expires")]
    jwt_expires_in: String,
    #[serde(default = "get_default_maxage")]
    jwt_maxage: i32,
    board_name: String,
    users: Vec<User>,
    categories: Vec<Category>,
    #[serde(default = "get_default_docker_uri")]
    docker_uri: Option<String>,
}

fn get_default_log_level() -> String{
    DEFAULT_LOG_LEVEL.to_string()
}

fn get_default_port() -> u16{
    DEFAULT_PORT
}

fn get_default_expires() -> String{
    DEFAULT_EXPIRES.to_string()
}

fn get_default_maxage() -> i32{
    DEFAULT_MAX_AGE
}

fn get_default_docker_uri() -> Option<String>{
    None
}


impl Configuration {
    pub fn new(content: &str) -> Result<Configuration, Error>{
        serde_yaml::from_str(content)
    }

    pub fn get_log_level(&self) -> &str{
        &self.log_level
    }

    pub fn get_port(&self) -> u16{
        self.port
    }
    pub fn get_url(&self) -> &str{
        &self.url
    }

    pub fn get_secret(&self) -> &str{
        &self.jwt_secret
    }

    pub fn get_board_name(&self) -> &str{
        &self.board_name
    }

    pub fn get_categories(&self) -> &Vec<Category>{
        &self.categories
    }

    pub fn get_docker_uri(&self) -> Option<String>{
        self.docker_uri.to_owned()
    }

    pub fn get_user(&self, name: &str) -> Option<User>{
        for user in &self.users{
            if user.name == name{
                return Some(user.clone())
            }
        }
        None
    }

    pub fn get_category_by_name(&self, name: &str) -> Option<Category>{
        for category in &self.categories{
            if category.name == name{
                return Some(category.to_owned());
            }
        }
        None
    }

    pub fn category_has_app(category: &Category, app: &App) -> bool{
        for category_app in category.apps.iter(){
            if app.name == category_app.name || app.url == category_app.url{
                return true;
            }
        }
        false
    }

    pub fn add_app_in_category(&mut self, category_name: &str, app: App) -> Result<String, String>{
        let app_name = &app.name.clone();
        match self.get_category_by_name(category_name){
            Some(mut category) => {
                if Self::category_has_app(&category, &app){
                    Err(format!("Can not add. App exists: {}", app_name))
                }else{
                    category.apps.push(app);
                    Ok(format!("Added: {}", app_name))
                }
            },
            None => {
                tracing::error!("There is no category: {}", category_name);
                Err(format!("Can not add: {}", app_name))
            }
        }
    }
}
