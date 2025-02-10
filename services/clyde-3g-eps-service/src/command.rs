
use crate::error::{ServiceError, ServiceResult};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub cmd_id: u16,
    pub data: String,
}

impl Command {
    pub fn new(cmd_id: u16, data: &str) -> Self {
        Self {
            cmd_id,
            data: data.to_owned(),
        }
    }

    pub fn process(&self) -> ServiceResult<()> {
        if self.cmd_id == 0 {
            Err(ServiceError::InvalidCommand("cmd_id=0".to_string()))
        } else {
            println!("Processing cmd_id={}, data={}", self.cmd_id, self.data);
            Ok(())
        }
    }
}