pub mod parameters;
use parameters::*;

use crate::{client::{HorizonsQueryError, self}, command_old::ParseResultType, ephemeris::{EphemerisVectorParser, EphemerisOrbitalElementsParser}, MajorBody};

trait QueryParameter {
    fn get_parameters(&self) -> Vec<(String, String)>;
}

#[derive(Default)]
pub struct Query {
    format: QueryFormat,
    command: CommandType,
    obj_data: ObjData,
    make_ephem: MakeEphem,
    ephem_type: EphemerisType,
    email_addr: EmailAddressType,
}

//---------- Ephemeris Arguments ----------
#[derive(Default)]
pub struct ObserverArgs {
    center: Center,
    coord: Coord,          //coord_type + site_coord
    start_time: StartTime, //Option<DateTime<Utc>>,
    stop_time: StopTime,   //Option<DateTime<Utc>>,
    step_size: StepSize,   //Duration,
    t_list: Vec<String>,
    t_list_type: Option<TListType>,
    quantities: Quantities,
    ref_system: RefSystem,
    cal_format: CalFormat,
    cal_type: CalType,
    ang_format: AngFormat,
    apparent: Apparent,
    time_digits: TimeDigits,
    time_zone: String,
    range_unit: RangeUnit,
    suppress_range_rate: SuppressRangeRate,
    elev_cut: ElevCut,
    skip_daylt: SkipDaylt,
    solar_elong: SolarElong,
    airmass: Airmass,
    lha_cutoff: LhaCutoff,
    ang_range_cutoff: AngRangeCutoff,
    extra_prec: ExtraPrec,
    csv_format: CsvFormat,
    r_t_s_only: RTSOnly,
}

#[derive(Default)]
pub struct VectorArgs {
    center: Center,
    ref_plane: RefPlane,
    coord: Coord,          //coord_type + site_coord
    start_time: StartTime, //Option<DateTime<Utc>>,
    stop_time: StopTime,   //Option<DateTime<Utc>>,
    step_size: StepSize,   //Duration,
    t_list: Vec<String>,
    t_list_type: Option<TListType>,
    ref_system: RefSystem,
    out_units: OutUnits,
    vec_table: VecTable,
    vec_corr: VecCorr,
    cal_type: CalType,
    time_digits: TimeDigits,
    csv_format: CsvFormat,
    vec_labels: VecLabels,
    vec_delta_t: VecDeltaT,
}

#[derive(Default)]
pub struct ElementArgs {
    center: Center,
    ref_plane: RefPlane,
    coord: Coord,          //coord_type + site_coord
    start_time: StartTime, //Option<DateTime<Utc>>,
    stop_time: StopTime,   //Option<DateTime<Utc>>,
    step_size: StepSize,   //Duration,
    t_list: Vec<String>,
    t_list_type: Option<TListType>,
    ref_system: RefSystem,
    out_units: OutUnits,
    cal_type: CalType,
    time_digits: TimeDigits,
    csv_format: CsvFormat,
    elm_labels: ElmLabels,
    tp_type: TpType,
}

#[derive(Default)]
pub struct SpkArgs {}

#[derive(Default)]
pub struct ApproachArgs {}

//-------------------------------------
//            Struct impl
//-------------------------------------

impl Query {
    pub async fn query(&self) -> Result<Vec<String>, HorizonsQueryError> {
        client::query(&self.get_parameters()).await
    }

    pub async fn parse(&self) -> ParseResultType {
        let query_result = self.query().await.unwrap();

        if matches!(self.command, CommandType(Some(Command::MajorBody))) {
            println!("{:?}", query_result);

            ParseResultType::MajorBodies(query_result.iter()
            .filter_map(|s| MajorBody::try_from(s.as_str()).ok())
            .collect())
        } else if matches!(self.ephem_type, EphemerisType::Vectors(_)) {
            ParseResultType::Vector(EphemerisVectorParser::parse(query_result.iter().map(String::as_str)).collect())
        } else if matches!(self.ephem_type, EphemerisType::Elements(_)) {
            ParseResultType::Elements(EphemerisOrbitalElementsParser::parse(query_result.iter().map(String::as_str)).collect())
        } else {
            todo!()
        }
    }

    // Default Commands
    pub fn major_bodies() -> Self {
        let mut query = Self::default();

        query.command = CommandType(Some(Command::MajorBody));

        query
    }

    pub fn observer() -> Self {
        let mut query = Self::default();

        query.ephem_type = EphemerisType::Observer(ObserverArgs::default());

        query
    }

    pub fn vectors() -> Self {
        let mut query = Self::default();

        query.ephem_type = EphemerisType::Vectors(VectorArgs::default());

        query
    }

    pub fn elements() -> Self {
        let mut query = Self::default();

        query.ephem_type = EphemerisType::Elements(ElementArgs::default());

        query
    }

    pub fn approach() -> Self {
        let mut query = Self::default();

        query.ephem_type = EphemerisType::Approach(ApproachArgs::default());

        query
    }

    pub fn spk() -> Self {
        let mut query = Self::default();

        query.ephem_type = EphemerisType::Spk(SpkArgs::default());

        query
    }


    // Parameter Getter/Setter

    pub fn set_format(&mut self, format: QueryFormat) {
        self.format = format;
    }

    pub fn format(&self) -> &QueryFormat {
        &self.format
    }

    pub fn set_command(&mut self, command: Option<Command>) {
        self.command = CommandType(command);
    }

    pub fn command(&self) -> &Option<Command> {
        &self.command.0
    }

    pub fn set_obj_data(&mut self, obj_data: bool) {
        self.obj_data = ObjData(obj_data);
    }

    pub fn obj_data(&self) -> bool {
        self.obj_data.0
    }

    pub fn set_make_ephem(&mut self, make_ephem: bool) {
        self.make_ephem = MakeEphem(make_ephem);
    }

    pub fn make_ephem(&self) -> bool {
        self.make_ephem.0
    }

    pub fn set_ephem_type(&mut self, ephem_type: EphemerisType) {
        self.ephem_type = ephem_type;
    }

    pub fn ephem_type(&self) -> &EphemerisType {
        &self.ephem_type
    }

    pub fn set_email_addr(&mut self, email_addr: Option<String>) {
        if let Some(addr) = email_addr {
            self.email_addr = EmailAddressType::new(addr);
        } else {
            self.email_addr = EmailAddressType::default();
        }
    }

    pub fn email_addr(&self) -> &Option<String> {
        &self.email_addr.address()
    }
}

//-------------------------------------
//        Query Parameter impl
//-------------------------------------
impl QueryParameter for Query {
    fn get_parameters(&self) -> Vec<(String, String)> {
        let mut parameters = Vec::new();

        parameters.extend(self.format.get_parameters());
        parameters.extend(self.command.get_parameters());
        parameters.extend(self.obj_data.get_parameters());
        parameters.extend(self.make_ephem.get_parameters());
        parameters.extend(self.ephem_type.get_parameters());
        parameters.extend(self.email_addr.get_parameters());

        parameters
    }
}

//---------- Ephemeris Arguments ----------
impl QueryParameter for ObserverArgs {
    fn get_parameters(&self) -> Vec<(String, String)> {
        let mut parameters = Vec::new();

        parameters.extend(self.center.get_parameters());
        //parameters.extend(self.coord.get_parameters()); //TODO: problems for MB
        parameters.extend(self.start_time.get_parameters());
        parameters.extend(self.stop_time.get_parameters());
        //parameters.extend(self.step_size.get_parameters()); //TODO problems for MB

        parameters.extend(self.quantities.get_parameters());

        parameters
    }
}

impl QueryParameter for VectorArgs {
    fn get_parameters(&self) -> Vec<(String, String)> {
        todo!()
    }
}

impl QueryParameter for ElementArgs {
    fn get_parameters(&self) -> Vec<(String, String)> {
        todo!()
    }
}

impl QueryParameter for SpkArgs {
    fn get_parameters(&self) -> Vec<(String, String)> {
        todo!()
    }
}

impl QueryParameter for ApproachArgs {
    fn get_parameters(&self) -> Vec<(String, String)> {
        todo!()
    }
}

//-------------------------------------
//                Tests
//-------------------------------------
#[cfg(test)]
mod test {
    use chrono::{TimeZone, Utc};

    use super::*;

    #[test]
    fn example_query() {
        let mut query = Query::default();

        query.format = QueryFormat::Text;
        query.command = CommandType(Some(Command::Id(499)));

        let mut observer_args = ObserverArgs::default();
        observer_args.center = Center::Geocentric(Some(399));
        observer_args.start_time = StartTime(Utc.with_ymd_and_hms(2006, 1, 1, 0, 0, 0).earliest());
        observer_args.stop_time = StopTime(Utc.with_ymd_and_hms(2006, 1, 20, 0, 0, 0).earliest());
        observer_args.step_size = StepSize::Days(1);
        observer_args.quantities = Quantities(vec![
            Quantity::Select(1),
            Quantity::Select(9),
            Quantity::Select(20),
            Quantity::Select(23),
            Quantity::Select(24),
            Quantity::Select(29),
        ]);

        query.ephem_type = EphemerisType::Observer(observer_args);

        let parameters = query.get_parameters();

        //Example Query from:
        //https://ssd-api.jpl.nasa.gov/doc/horizons.html
        //Slightly modified for easier comparision
        let example_parameters = vec![
            ("format", "text"),
            ("COMMAND", "499"),
            ("OBJ_DATA", "YES"),
            ("MAKE_EPHEM", "YES"),
            ("EPHEM_TYPE", "OBSERVER"),
            ("CENTER", "500@399"),
            ("START_TIME", "2006-01-01 00:00:00"),
            ("STOP_TIME", "2006-01-20 00:00:00"),
            ("STEP_SIZE", "1 d"),
            ("QUANTITIES", "1,9,20,23,24,29"),
        ];

        println!("{:?}", parameters);

        example_parameters
            .iter()
            .map(|(name, value)| ((*name).to_owned(), (*value).to_owned()))
            .for_each(|parameter| {
                assert!(
                    parameters.contains(&parameter),
                    "Missing Parameter. {:?} not found",
                    parameter
                );
            });
    }
}
