use chrono::{DateTime, Utc};
use thiserror::Error;

use async_trait::async_trait;

use crate::client::{self, HorizonsQueryError};

#[async_trait]
pub trait QueryCommand {
    async fn query(&self) -> Result<Vec<String>, HorizonsQueryError>;

    async fn query_with_retries(&self, retries: u8) -> Result<Vec<String>, HorizonsQueryError> {
        for n in 1..retries {
            log::trace!("try {}", n);
            if let Ok(result) = self.query().await {
                return Ok(result);
            }
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
        Err(HorizonsQueryError)
    }
}

pub enum CommandType {
    MajorBody,
    Vector,
    OrbitalElement,
}

pub struct CommandBuilder {}

impl CommandBuilder {
    pub fn new(command_type: CommandType) -> Box<dyn QueryCommand> {
        match command_type {
            CommandType::MajorBody => {Box::new(MajorBodyCommand {})},
            _ => {Box::new(MajorBodyCommand {})},
        }
    }
}

struct MajorBodyCommand {}

struct VectorCommand {
    id: i32,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}
struct OrbitalElementCommand {
    id: i32,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

#[async_trait]
impl QueryCommand for MajorBodyCommand {
    async fn query(&self) -> Result<Vec<String>, HorizonsQueryError> {
        client::query(&[("COMMAND", "MB")]).await
    }
}


