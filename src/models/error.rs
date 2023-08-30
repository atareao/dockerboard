use std::{fmt, error};

#[derive(Debug, Clone)]
pub struct CustomError {
    message: String,
}

impl CustomError {
    pub fn new(message: String) -> Self{
        Self{
            message
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl error::Error for CustomError {}

pub enum DBError {
    CE(CustomError),
    DE(docker_api::Error),
}

impl Into<DBError> for docker_api::Error{
    fn into(self) -> DBError{
        DBError::DE(self)
    }

}


