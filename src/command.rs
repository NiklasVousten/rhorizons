use std::fmt::Display;

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use thiserror::Error;

use crate::{
    client::{self, HorizonsQueryError},
    ephemeris::{self, EphemerisOrbitalElementsParser, EphemerisVectorParser},
    major_bodies, EphemerisVectorItem, MajorBody,
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
    // Target
    id: i32,

    // Coordinate body center
    center: i32,

    //Time
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

    pub fn from_id(id: i32) -> CommandBuilder {
        CommandBuilder {
            id,
            center: 10, //Default center is the sun
            start: None,
            end: None,
            time_step: None,
        }
    }

    pub fn build_with_type(
        &self,
        command_type: CommandType,
    ) -> Result<Box<dyn QueryCommand + Sync>, CommandTypeError> {
        match command_type {
            CommandType::Vector => Ok(Box::new(Command::<VectorCommand> {
                id: self.id,
                center: self.center,
                start: self.start,
                end: self.end,
                _time_step: self.time_step,
                command: VectorCommand::default(),
            })),
            _ => Err(CommandTypeError),
        }
    }

    pub fn with_id(&self, id: i32) -> Self {
        CommandBuilder {
            id,
            center: self.center,
            start: self.start,
            end: self.end,
            time_step: self.time_step,
        }
    }

    pub fn with_start(&self, start: DateTime<Utc>) -> Self {
        CommandBuilder {
            id: self.id,
            center: self.center,
            start: Some(start),
            end: self.end,
            time_step: self.time_step,
        }
    }

    pub fn with_end(&self, end: DateTime<Utc>) -> Self {
        CommandBuilder {
            id: self.id,
            center: self.center,
            start: self.start,
            end: Some(end),
            time_step: self.time_step,
        }
    }

    pub fn with_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        CommandBuilder {
            id: self.id,
            center: self.center,
            start: Some(start),
            end: Some(end),
            time_step: self.time_step,
        }
    }

    fn _with_time_step(&self, time_step: Duration) -> Self {
        CommandBuilder {
            id: self.id,
            center: self.center,
            start: self.start,
            end: self.end,
            time_step: Some(time_step),
        }
    }

    pub fn with_center(&self, center: i32) -> Self {
        CommandBuilder {
            id: self.id,
            center,
            start: self.start,
            end: self.end,
            time_step: self.time_step,
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

#[async_trait]
pub trait Parse {
    async fn parse(&self) -> ParseResultType;
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
    id: i32,

    center: i32,

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

#[async_trait]
impl Parse for MajorBodyCommand {
    async fn parse(&self) -> ParseResultType {
        let result = self.query_with_retries(10).await.unwrap();

        let items = result
            .iter()
            .filter_map(|s| MajorBody::try_from(s.as_str()).ok())
            .collect();

        ParseResultType::MajorBodies(items)
    }
}

impl<EC> QueryCommand for Command<EC>
where
    EC: EphemerisCommand,
    Self: Parse,
{
    fn get_parameters(&self) -> Vec<(&str, String)> {
        let mut parameters = vec![
            ("COMMAND", self.id.to_string()),
            //TODO: Make center dynamic
            ("CENTER", format!("500@{}", self.center)),
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

#[async_trait]
impl Parse for Command<VectorCommand> {
    async fn parse(&self) -> ParseResultType {
        let result = self.query_with_retries(10).await.unwrap();

        let items = EphemerisVectorParser::parse(result.iter().map(String::as_str)).collect();

        ParseResultType::Vector(items)
    }
}

#[async_trait]
impl Parse for Command<OrbitalElementCommand> {
    async fn parse(&self) -> ParseResultType {
        let result = self.query_with_retries(10).await.unwrap();

        let items =
            EphemerisOrbitalElementsParser::parse(result.iter().map(String::as_str)).collect();

        ParseResultType::Elements(items)
    }
}

impl EphemerisCommand for VectorCommand {
    fn get_parameters(&self) -> Vec<(&str, String)> {
        vec![
            ("EPHEM_TYPE", "VECTORS".to_string()),
            //()
        ]
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
        let c = cb.build_with_type(CommandType::MajorBody);
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
            .build_with_type(CommandType::Vector)
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
            .build_with_type(CommandType::Vector)
            .unwrap();
        let res: Result<Vec<String>, HorizonsQueryError> = c.query().await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn parse_center_command() {
        let cb = CommandBuilder::from_id(301).with_range(
            Utc.with_ymd_and_hms(2016, 10, 15, 12, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2016, 10, 15, 13, 0, 0).unwrap(),
        );
        
        let c_default_center = cb
            .build_with_type(CommandType::Vector)
            .unwrap();

        let c_sun_center = cb.with_center(10)
        .build_with_type(CommandType::Vector).unwrap();

        let res_default_center = c_default_center.parse().await;
        let res_sun_center = c_sun_center.parse().await;

        if let ParseResultType::Vector(default_center) = res_default_center {
            if let ParseResultType::Vector(center) = res_sun_center {
                assert_eq!(default_center[0].position, center[0].position);
                assert_eq!(default_center[0].velocity, center[0].velocity);
            }
        }

    }

    #[tokio::test]
    async fn parse_center_relative_command() {
        let cb = CommandBuilder::from_id(301).with_range(
            Utc.with_ymd_and_hms(2016, 10, 15, 12, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2016, 10, 15, 13, 0, 0).unwrap(),
        );

        let c_relative_moon = cb
            .with_center(399)
            .build_with_type(CommandType::Vector)
            .unwrap();

        let cb = cb.with_center(10);

        let c_static_moon = cb
            .with_id(301)
            .build_with_type(CommandType::Vector)
            .unwrap();

        let c_static_earth = cb
            .with_id(399)
            .build_with_type(CommandType::Vector)
            .unwrap();

        let res_relative = c_relative_moon.parse().await;

        let res_static_moon = c_static_moon.parse().await;
        let res_static_earth = c_static_earth.parse().await;

        let (pos, vel) = if let ParseResultType::Vector(static_moon) = res_static_moon {
            if let ParseResultType::Vector(static_earth) = res_static_earth {
                (
                    static_moon[0]
                        .position
                        .iter()
                        .enumerate()
                        .map(|(i, f)| f - static_earth[0].position[i])
                        .collect(),
                    static_moon[0]
                        .velocity
                        .iter()
                        .enumerate()
                        .map(|(i, f)| f - static_earth[0].velocity[i])
                        .collect(),
                )
            } else {
                assert!(false, "Static earth delivers wrong ParseResult Type");
                (vec![], vec![])
            }
        } else {
            assert!(false, "Static moon delivers wrong ParseResult Type");
            (vec![], vec![])
        };

        let error_delta = 1.5;

        if let ParseResultType::Vector(relative) = res_relative {
            for (i, &p) in relative[0].position.iter().enumerate() {
                assert!((p - pos[i]).abs() < error_delta, "{}", format!("Position difference to large at position {i} ({})", (p - pos[i])));
            }
            for (i, &v) in relative[0].velocity.iter().enumerate() {
                assert!((v - vel[i]).abs() < error_delta, "{}", format!("Position difference to large at position {i} ({})", (v - vel[i])));
            }
        } else {
            assert!(false, "Relative moon delivers wrong ParseResult Type");
        }
    }
}
