use std::fmt::Display;

use chrono::{DateTime, Utc, Duration};
use thiserror::Error;

use async_trait::async_trait;

use crate::client::{self, HorizonsQueryError};

pub enum CommandType {
    MajorBody,
    Observer,
    Vector,
    OrbitalElement,
}

#[derive(Debug, Error)]
pub struct CommandTypeError;

impl Display for CommandTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Incompatible command type")
    }
}

pub struct CommandBuilder {
    id: u32,

    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    time_step: Option<Duration>,
}

impl CommandBuilder {
    pub fn from_type(command_type: CommandType) -> Box<dyn QueryCommand> {
        match command_type {
            CommandType::MajorBody => Box::new(MajorBodyCommand {}),
            CommandType::Vector => Box::new(Command::<VectorCommand>::default()),
            CommandType::OrbitalElement => Box::new(Command::<OrbitalElementCommand>::default()),

            _ => Box::new(MajorBodyCommand {}),
        }
    }

    pub fn from_id(id: u32) -> CommandBuilder {
        CommandBuilder {
            id: id,
            start: None,
            end: None,
            time_step: None,
        }
    }

    pub fn with_type(
        self,
        command_type: CommandType,
    ) -> Result<Box<dyn QueryCommand>, CommandTypeError> {
        match command_type {
            CommandType::Vector => Ok(Box::new(Command::<VectorCommand> {
                id: self.id,
                start: self.start,
                end: self.end,
                time_step: self.time_step,
                command: VectorCommand::default(),
            })),
            _ => Err(CommandTypeError),
        }
    }

    pub fn with_start(&self, start: DateTime<Utc>) -> Self {
        CommandBuilder { id: self.id, start: Some(start), end: self.end, time_step: self.time_step }
    }

    pub fn with_end(&self, end: DateTime<Utc>) -> Self {
        CommandBuilder { id: self.id, start: self.start, end: Some(end), time_step: self.time_step }
    }

    pub fn with_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        CommandBuilder { id: self.id, start: Some(start), end: Some(end), time_step: self.time_step }
    }

    pub fn with_time_step(&self, time_step: Duration) -> Self {
        CommandBuilder { id: self.id, start: self.start, end: self.end, time_step: Some(time_step) }
    }
}

#[async_trait]
pub trait QueryCommand {
    fn get_parameters(&self) -> &[(&str, &str)];

    async fn query(&self) -> Result<Vec<String>, HorizonsQueryError> {
        client::query(&self.get_parameters()).await
    }

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

trait EphemerisCommand {
    fn get_parameters(&self) -> &[(&str, &str)];
}

#[derive(Debug)]
pub struct MajorBodyCommand {}

#[derive(Default)]
struct Command<EC>
where
    EC: EphemerisCommand,
{
    id: u32,

    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    time_step: Option<Duration>,

    command: EC,
}

struct ObserverCommand {}

#[derive(Default)]
struct VectorCommand {}

#[derive(Default)]
struct OrbitalElementCommand {}

struct SmallBodyCommand {}

impl QueryCommand for MajorBodyCommand {
    fn get_parameters(&self) -> &[(&str, &str)] {
        &[("COMMAND", "MB")]
    }
}

impl<EC> QueryCommand for Command<EC>
where
    EC: EphemerisCommand,
{
    fn get_parameters(&self) -> &[(&str, &str)] {
        todo!()
    }
}

impl EphemerisCommand for VectorCommand {
    fn get_parameters(&self) -> &[(&str, &str)] {
        todo!()
    }
}

impl EphemerisCommand for OrbitalElementCommand {
    fn get_parameters(&self) -> &[(&str, &str)] {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn major_bodies_with_id() {
        let cb = CommandBuilder::from_id(32);
        assert_eq!(cb.id, 32);
        let c = cb.with_type(CommandType::MajorBody);
        assert!(c.is_err());
    }
}