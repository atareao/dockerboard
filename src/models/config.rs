use serde::{Serialize, Deserialize};
use serde_yaml::Error;

use super::{category::Category, user::User};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration{
    #[serde(default = "get_default_log_level")]
    log_level: String,
    categories: Vec<Category>,
    users: Vec<User>
}

fn get_default_log_level() -> String{
    "info".to_string()
}

impl Configuration {
    pub fn new(content: &str) -> Result<Configuration, Error>{
        serde_yaml::from_str(content)
    }

    pub fn get_log_level(&self) -> &str{
        &self.log_level
    }

    pub fn get_users(&self) -> &Vec<User>{
        &self.users
    }

    pub fn get_categories(&self) -> &Vec<Category>{
        &self.categories
    }

    pub fn get_user(&self, name: &str) -> Option<User>{
        for user in self.users{
            if user.name == name{
                return Some(user)
            }
        }
        None
    }
}
