use std::fmt::Display;

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use thiserror::Error;

use crate::{
    client::{self, HorizonsQueryError},
    ephemeris, major_bodies,
};

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
    pub fn from_type(command_type: CommandType) -> Box<dyn QueryCommand + Sync> {
        match command_type {
            CommandType::MajorBody => Box::new(MajorBodyCommand {}),
            CommandType::Vector => Box::<Command<VectorCommand>>::default(),
            CommandType::OrbitalElement => Box::<Command<OrbitalElementCommand>>::default(),

            _ => Box::new(MajorBodyCommand {}),
        }
    }

    pub fn from_id(id: u32) -> CommandBuilder {
        CommandBuilder {
            id,
            start: None,
            end: None,
            time_step: None,
        }
    }

    pub fn with_type(
        self,
        command_type: CommandType,
    ) -> Result<Box<dyn QueryCommand + Sync>, CommandTypeError> {
        match command_type {
            CommandType::Vector => Ok(Box::new(Command::<VectorCommand> {
                id: self.id,
                start: self.start,
                end: self.end,
                _time_step: self.time_step,
                command: VectorCommand::default(),
            })),
            _ => Err(CommandTypeError),
        }
    }

    pub fn with_start(&self, start: DateTime<Utc>) -> Self {
        CommandBuilder {
            id: self.id,
            start: Some(start),
            end: self.end,
            time_step: self.time_step,
        }
    }

    pub fn with_end(&self, end: DateTime<Utc>) -> Self {
        CommandBuilder {
            id: self.id,
            start: self.start,
            end: Some(end),
            time_step: self.time_step,
        }
    }

    pub fn with_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        CommandBuilder {
            id: self.id,
            start: Some(start),
            end: Some(end),
            time_step: self.time_step,
        }
    }

    fn _with_time_step(&self, time_step: Duration) -> Self {
        CommandBuilder {
            id: self.id,
            start: self.start,
            end: self.end,
            time_step: Some(time_step),
        }
    }
}

#[async_trait]
pub trait QueryCommand: Parse {
    fn get_parameters(&self) -> Vec<(&str, String)>;
}

#[async_trait]
trait Query {
    async fn query(&self) -> Result<Vec<String>, HorizonsQueryError>;

    async fn query_with_retries(&self, retries: u8) -> Result<Vec<String>, HorizonsQueryError>;
}

#[async_trait]
impl<T: QueryCommand + Sync + ?Sized> Query for T {
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

pub enum ParseResultType {
    MajorBodies(Vec<major_bodies::MajorBody>),
    Vector(Vec<ephemeris::EphemerisVectorItem>),
    Elements(Vec<ephemeris::EphemerisOrbitalElementsItem>),
}

pub trait Parse {
    fn parse(&self) -> ParseResultType;
}

pub trait EphemerisCommand {
    fn get_parameters(&self) -> Vec<(&str, String)>;
}

#[derive(Debug)]
pub struct MajorBodyCommand {}

#[derive(Default)]
pub struct Command<EC>
where
    EC: EphemerisCommand,
{
    id: u32,

    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    _time_step: Option<Duration>,

    command: EC,
}

struct _ObserverCommand {}

#[derive(Default)]
struct VectorCommand {}

#[derive(Default)]
struct OrbitalElementCommand {}

struct _SmallBodyCommand {}

impl QueryCommand for MajorBodyCommand {
    fn get_parameters(&self) -> Vec<(&str, String)> {
        vec![("COMMAND", "MB".to_string())]
    }
}

impl Parse for MajorBodyCommand {
    fn parse(&self) -> ParseResultType {
        todo!()
    }
}

impl<EC> QueryCommand for Command<EC>
where
    EC: EphemerisCommand,
{
    fn get_parameters(&self) -> Vec<(&str, String)> {
        let mut parameters = vec![
            ("COMMAND", self.id.to_string()),
            ("EPHEM_TYPE", "VECTORS".to_string()),
        ];

        if let Some(start) = self.start {
            parameters.push(("START_TIME", start.format("%Y-%b-%d-%T").to_string()))
        }

        if let Some(end) = self.end {
            parameters.push(("STOP_TIME", end.format("%Y-%b-%d-%T").to_string()))
        }

        parameters.extend(self.command.get_parameters());

        parameters
    }
}

impl<EC> Parse for Command<EC>
where
    EC: EphemerisCommand,
{
    fn parse(&self) -> ParseResultType {
        todo!()
    }
}

impl EphemerisCommand for VectorCommand {
    fn get_parameters(&self) -> Vec<(&str, String)> {
        //todo!()
        vec![]
    }
}

impl EphemerisCommand for OrbitalElementCommand {
    fn get_parameters(&self) -> Vec<(&str, String)> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn major_bodies_with_id() {
        let cb = CommandBuilder::from_id(32);
        assert_eq!(cb.id, 32);
        let c = cb.with_type(CommandType::MajorBody);
        assert!(c.is_err());
    }

    #[test]
    fn get_parameters() {
        let cb = CommandBuilder::from_id(399);
        let c = cb
            .with_range(
                Utc.with_ymd_and_hms(2016, 10, 15, 12, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2016, 10, 15, 13, 0, 0).unwrap(),
            )
            .with_type(CommandType::Vector)
            .unwrap();
        println!("{:?}", c.get_parameters());
    }

    #[tokio::test]
    async fn query_command() {
        let cb = CommandBuilder::from_id(399);
        let c = cb
            .with_range(
                Utc.with_ymd_and_hms(2016, 10, 15, 12, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2016, 10, 15, 13, 0, 0).unwrap(),
            )
            .with_type(CommandType::Vector)
            .unwrap();
        let res = c.query().await;
        println!("{:?}", res.unwrap());
    }
}
