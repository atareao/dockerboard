use serde::{Serialize, Deserialize};
use serde_yaml::Error;

use super::{category::Category, user::User};

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

    pub fn get_user(&self, name: &str) -> Option<User>{
        for user in &self.users{
            if user.name == name{
                return Some(user.clone())
            }
        }
        None
    }
}
