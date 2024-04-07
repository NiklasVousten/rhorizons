use chrono::{DateTime, Utc};

use super::*;

//---------- Query Parameter Types ----------
#[derive(Default)]
pub enum QueryFormat {
    #[default]
    Json,
    Text,
}

#[derive(Default)]
pub(crate) struct CommandType(pub Option<Command>);

pub enum Command {
    MajorBody,
    Id(u32),
    Name(String),
}

pub(crate) struct ObjData(pub bool);

pub(crate) struct MakeEphem(pub bool);

pub enum EphemerisType {
    Observer(ObserverArgs),
    Vectors(VectorArgs),
    Elements(ElementArgs),
    Spk(SpkArgs),
    Approach(ApproachArgs),
}

#[derive(Default)]
pub(crate) struct EmailAddressType(Option<String>);

//---------- Parameter Types ----------
pub enum Center {
    Geocentric(Option<i32>),
    //More Variants missing
}

#[derive(Default)]
pub enum RefPlane {
    #[default]
    Ecliptic,
    Frame,
    BodyEquator,
}

pub enum Coord {
    Geodetic(f32, f32, f32),
    Cylindrical(f32, f32, f32),
}

#[derive(Default)]
pub struct StartTime(pub Option<DateTime<Utc>>);

#[derive(Default)]
pub struct StopTime(pub Option<DateTime<Utc>>);

pub enum StepSize {
    Days(u32),
    Hours(u32),
    Minutes(u32),
    Years(u32),
    Months(u32),
    Unitless(u32),
}

pub enum TListType {}

#[derive(Default)]
pub struct Quantities(pub Vec<Quantity>);

pub enum Quantity {
    Select(u8),
    Macro(char),
}

#[derive(Default)]
pub enum RefSystem {
    #[default]
    Icrf,
    B1950,
}

#[derive(Default)]
pub enum OutUnits {
    #[default]
    KmS,
    AuD,
    KmD,
}

//TODO: VecTable

#[derive(Default)]
pub enum VecCorr {
    #[default]
    None,
    Lt,
    LtS,
}

#[derive(Default)]
pub enum CalFormat {
    #[default]
    Cal,
    Jd,
    Both,
}

#[derive(Default)]
pub enum CalType {
    #[default]
    Mixed,
    Gregorian,
}

#[derive(Default)]
pub enum AngFormat {
    #[default]
    Hms,
    Deg,
}

#[derive(Default)]
pub enum Apparent {
    #[default]
    Airless,
    Refracted,
}

#[derive(Default)]
pub enum TimeDigits {
    #[default]
    Minutes,
    Seconds,
    Fracsec,
}

#[derive(Default)]
pub enum RangeUnit {
    #[default]
    Au,
    Km,
}

#[derive(Default)]
pub (crate) struct SuppressRangeRate(pub bool);

pub (crate) struct ElevCut(pub i8);

#[derive(Default)]
pub (crate) struct SkipDaylt(pub bool);

pub (crate) struct SolarElong(pub f32, pub f32);

pub (crate) struct Airmass(pub f32);

pub (crate) struct LhaCutoff(pub f32);

pub (crate) struct AngRangeCutoff(pub f32);

#[derive(Default)]
pub (crate) struct ExtraPrec(pub bool);

#[derive(Default)]
pub (crate) struct CsvFormat(pub bool);

pub (crate) struct VecLabels(pub bool);

#[derive(Default)]
pub (crate) struct VecDeltaT(pub bool);

pub (crate) struct ElmLabels(bool);

#[derive(Default)]
pub enum TpType {
    #[default]
    Absolute,
    Relative,
}

#[derive(Default)]
pub (crate) struct RTSOnly(pub bool);

//-------------------------------------
//                  Impl
//-------------------------------------

impl EmailAddressType {
    pub fn new(email_addr: String) -> Self {
        assert_eq!(
            email_addr.split("@").count(),
            2,
            "Email address does not contain @"
        );
        assert!(
            email_addr.split("@").last().unwrap().split(".").count() > 2,
            "Email address does not contain valid domain"
        );

        Self(Some(email_addr))
    }

    pub fn address(&self) -> &Option<String> {
        &self.0
    }
}

//-------------------------------------
//             Default impl
//-------------------------------------

//---------- Query Parameter Types ----------
impl Default for ObjData {
    fn default() -> Self {
        Self(true)
    }
}

impl Default for MakeEphem {
    fn default() -> Self {
        Self(true)
    }
}

impl Default for EphemerisType {
    fn default() -> Self {
        EphemerisType::Observer(Default::default())
    }
}

//---------- Parameter Types ----------
impl Default for Center {
    fn default() -> Self {
        Center::Geocentric(None)
    }
}

impl Default for Coord {
    fn default() -> Self {
        Coord::Geodetic(0.0, 0.0, 0.0)
    }
}

impl Default for StepSize {
    fn default() -> Self {
        StepSize::Minutes(60)
    }
}

impl Default for ElevCut {
    fn default() -> Self {
        Self(-90)
    }
}

impl Default for SolarElong {
    fn default() -> Self {
        Self(0.0, 180.0)
    }
}

impl Default for Airmass {
    fn default() -> Self {
        Self(38.0)
    }
}

impl Default for LhaCutoff {
    fn default() -> Self {
        Self(0.0)
    }
}

impl Default for AngRangeCutoff {
    fn default() -> Self {
        Self(0.0)
    }
}

//-------------------------------------
//        Query Parameter impl
//-------------------------------------
//---------- Query Parameter Types ----------
impl QueryParameter for QueryFormat {
    fn get_parameters(&self) -> Vec<(String, String)> {
        vec![(
            "format".to_owned(),
            match self {
                QueryFormat::Json => "json".to_owned(),
                QueryFormat::Text => "text".to_owned(),
            },
        )]
    }
}

impl QueryParameter for CommandType {
    fn get_parameters(&self) -> Vec<(String, String)> {
        if let Some(command) = &self.0 {
            command.get_parameters()
        } else {
            vec![]
        }
    }
}

impl QueryParameter for Command {
    fn get_parameters(&self) -> Vec<(String, String)> {
        vec![(
            "COMMAND".to_owned(),
            match self {
                Command::MajorBody => "MB".to_owned(),
                Command::Id(id) => id.to_string(),
                Command::Name(name) => name.clone(),
            },
        )]
    }
}

impl QueryParameter for ObjData {
    fn get_parameters(&self) -> Vec<(String, String)> {
        vec![(
            "OBJ_DATA".to_owned(),
            (if self.0 { "YES" } else { "NO" }).to_owned(),
        )]
    }
}

impl QueryParameter for MakeEphem {
    fn get_parameters(&self) -> Vec<(String, String)> {
        vec![(
            "MAKE_EPHEM".to_owned(),
            (if self.0 { "YES" } else { "NO" }).to_owned(),
        )]
    }
}

impl QueryParameter for EphemerisType {
    fn get_parameters(&self) -> Vec<(String, String)> {
        match self {
            EphemerisType::Observer(args) => {
                let mut params = vec![("EPHEM_TYPE".to_owned(), "OBSERVER".to_owned())];
                params.extend(args.get_parameters());
                params
            }
            EphemerisType::Vectors(args) => {
                let mut params = vec![("EPHEM_TYPE".to_owned(), "VECTORS".to_owned())];
                params.extend(args.get_parameters());
                params
            }
            EphemerisType::Elements(args) => {
                let mut params = vec![("EPHEM_TYPE".to_owned(), "ELEMENTS".to_owned())];
                params.extend(args.get_parameters());
                params
            }
            EphemerisType::Spk(args) => {
                let mut params = vec![("EPHEM_TYPE".to_owned(), "SPK".to_owned())];
                params.extend(args.get_parameters());
                params
            }
            EphemerisType::Approach(args) => {
                let mut params = vec![("EPHEM_TYPE".to_owned(), "APPROACH".to_owned())];
                params.extend(args.get_parameters());
                params
            }
        }
    }
}

impl QueryParameter for EmailAddressType {
    fn get_parameters(&self) -> Vec<(String, String)> {
        if let Some(email) = &self.0 {
            vec![("EMAIL_ADDR".to_owned(), email.clone())]
        } else {
            vec![]
        }
    }
}

//---------- Parameter Types ----------
impl QueryParameter for Center {
    fn get_parameters(&self) -> Vec<(String, String)> {
        vec![(
            "CENTER".to_owned(),
            match self {
                Center::Geocentric(None) => "500".to_owned(),
                Center::Geocentric(Some(id)) => format!("500@{}", id),
            },
        )]
    }
}

impl QueryParameter for RefPlane {
    fn get_parameters(&self) -> Vec<(String, String)> {
        vec![(
            "REF_PLANE".to_owned(),
            match self {
                RefPlane::Ecliptic => "ECLIPTIC",
                RefPlane::Frame => "FRAME",
                RefPlane::BodyEquator => "BODY EQUATOR",
            }
            .to_owned(),
        )]
    }
}

impl QueryParameter for Coord {
    fn get_parameters(&self) -> Vec<(String, String)> {
        match self {
            Coord::Geodetic(elong, lat, h) => vec![
                ("COORD_TYPE".to_owned(), "GEODETIC".to_owned()),
                ("SITE_COORD".to_owned(), format!("{},{},{}", elong, lat, h)),
            ],
            Coord::Cylindrical(elong, dxy, dz) => vec![
                ("COORD_TYPE".to_owned(), "CYLINDRICAL".to_owned()),
                ("SITE_COORD".to_owned(), format!("{},{},{}", elong, dxy, dz)),
            ],
        }
    }
}

impl QueryParameter for StartTime {
    fn get_parameters(&self) -> Vec<(String, String)> {
        if let Some(start_time) = self.0 {
            vec![(
                "START_TIME".to_owned(),
                start_time.format("%Y-%m-%d %T").to_string(),
            )]
        } else {
            vec![]
        }
    }
}

impl QueryParameter for StopTime {
    fn get_parameters(&self) -> Vec<(String, String)> {
        if let Some(stop_time) = self.0 {
            vec![(
                "STOP_TIME".to_owned(),
                stop_time.format("%Y-%m-%d %T").to_string(),
            )]
        } else {
            vec![]
        }
    }
}

impl QueryParameter for StepSize {
    fn get_parameters(&self) -> Vec<(String, String)> {
        vec![(
            "STEP_SIZE".to_owned(),
            match self {
                StepSize::Days(days) => format!("{} d", days),
                StepSize::Hours(hours) => format!("{} h", hours),
                StepSize::Minutes(minutes) => format!("{} m", minutes),
                StepSize::Years(years) => format!("{} y", years),
                StepSize::Months(months) => format!("{} mo", months),
                StepSize::Unitless(unitless) => format!("{}", unitless),
            },
        )]
    }
}

impl QueryParameter for Quantities {
    fn get_parameters(&self) -> Vec<(String, String)> {
        if self.0.is_empty() {
            vec![]
        } else {
            vec![(
                "QUANTITIES".to_owned(),
                self.0
                    .iter()
                    .map(|quantity| match quantity {
                        Quantity::Macro(macro_char) => macro_char.to_string(),
                        Quantity::Select(select_digit) => select_digit.to_string(),
                    })
                    .reduce(|acc, e| acc + "," + &e)
                    .unwrap(),
            )]
        }
    }
}
