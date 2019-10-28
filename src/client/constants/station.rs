use anyhow::{Result, anyhow};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use std::convert::TryFrom;

pub const STATION_ABBR_OAKLAND_CITY_CENTER12TH_ST: &'static str = "12th";
pub const STATION_ABBR_SF_MISSION16TH_ST: &'static str = "16th";
pub const STATION_ABBR_OAKLAND19TH_ST: &'static str = "19th";
pub const STATION_ABBR_SF_MISSION24TH_ST: &'static str = "24th";
pub const STATION_ABBR_ASHBY: &'static str = "ashb";
pub const STATION_ABBR_ANTIOCH: &'static str = "antc";
pub const STATION_ABBR_BALBOA_PARK: &'static str = "balb";
pub const STATION_ABBR_BAY_FAIR: &'static str = "bayf";
pub const STATION_ABBR_CASTRO_VALLEY: &'static str = "cast";
pub const STATION_ABBR_CIVIC_CENTER: &'static str = "civc";
pub const STATION_ABBR_COLISEUM: &'static str = "cols";
pub const STATION_ABBR_COLMA: &'static str = "colm";
pub const STATION_ABBR_CONCORD: &'static str = "conc";
pub const STATION_ABBR_DALY_CITY: &'static str = "daly";
pub const STATION_ABBR_DOWNTOWN_BERKELEY: &'static str = "dbrk";
pub const STATION_ABBR_DUBLIN_PLEASANTON: &'static str = "dubl";
pub const STATION_ABBR_EL_CERRITO_DEL_NORTE: &'static str = "deln";
pub const STATION_ABBR_EL_CERRITO_PLAZA: &'static str = "plza";
pub const STATION_ABBR_EMBARCADERO: &'static str = "embr";
pub const STATION_ABBR_FREMONT: &'static str = "frmt";
pub const STATION_ABBR_FRUITVALE: &'static str = "ftvl";
pub const STATION_ABBR_GLEN_PARK: &'static str = "glen";
pub const STATION_ABBR_HAYWARD: &'static str = "hayw";
pub const STATION_ABBR_LAFAYETTE: &'static str = "lafy";
pub const STATION_ABBR_LAKE_MERRITT: &'static str = "lake";
pub const STATION_ABBR_MAC_ARTHUR: &'static str = "mcar";
pub const STATION_ABBR_MILLBRAE: &'static str = "mlbr";
pub const STATION_ABBR_MONTGOMERY_ST: &'static str = "mont";
pub const STATION_ABBR_NORTH_BERKELEY: &'static str = "nbrk";
pub const STATION_ABBR_NORTH_CONCORD_MARTINEZ: &'static str = "ncon";
pub const STATION_ABBR_OAKLAND_INTL_AIRPORT: &'static str = "oakl";
pub const STATION_ABBR_ORINDA: &'static str = "orin";
pub const STATION_ABBR_PITTSBURG_BAY_POINT: &'static str = "pitt";
pub const STATION_ABBR_PITTSBURG_CENTER: &'static str = "pctr";
pub const STATION_ABBR_PLEASANT_HILL: &'static str = "phil";
pub const STATION_ABBR_POWELL_ST: &'static str = "powl";
pub const STATION_ABBR_RICHMOND: &'static str = "rich";
pub const STATION_ABBR_ROCKRIDGE: &'static str = "rock";
pub const STATION_ABBR_SAN_BRUNO: &'static str = "sbrn";
pub const STATION_ABBR_SAN_FRANCISCO_INTL_AIRPORT: &'static str = "sfia";
pub const STATION_ABBR_SAN_LEANDRO: &'static str = "sanl";
pub const STATION_ABBR_SOUTH_HAYWARD: &'static str = "shay";
pub const STATION_ABBR_SOUTH_SAN_FRANCISCO: &'static str = "ssan";
pub const STATION_ABBR_UNION_CITY: &'static str = "ucty";
pub const STATION_ABBR_WARM_SPRINGS_SOUTH_FREMONT: &'static str = "warm";
pub const STATION_ABBR_WALNUT_CREEK: &'static str = "wcrk";
pub const STATION_ABBR_WEST_DUBLIN: &'static str = "wdub";
pub const STATION_ABBR_WEST_OAKLAND: &'static str = "woak";

pub const STATION_FULL_OAKLAND_CITY_CENTER12TH_ST: &'static str = "12th St. Oakland City Center";
pub const STATION_FULL_SF_MISSION16TH_ST: &'static str = "16th St. Mission (SF)";
pub const STATION_FULL_OAKLAND19TH_ST: &'static str = "19th St. Oakland";
pub const STATION_FULL_SF_MISSION24TH_ST: &'static str = "24th St. Mission (SF)";
pub const STATION_FULL_ASHBY: &'static str = "Ashby (Berkeley)";
pub const STATION_FULL_ANTIOCH: &'static str = "Antioch";
pub const STATION_FULL_BALBOA_PARK: &'static str = "Balboa Park (SF)";
pub const STATION_FULL_BAY_FAIR: &'static str = "Bay Fair (San Leandro)";
pub const STATION_FULL_CASTRO_VALLEY: &'static str = "Castro Valley";
pub const STATION_FULL_CIVIC_CENTER: &'static str = "Civic Center (SF)";
pub const STATION_FULL_COLISEUM: &'static str = "Coliseum";
pub const STATION_FULL_COLMA: &'static str = "Colma";
pub const STATION_FULL_CONCORD: &'static str = "Concord";
pub const STATION_FULL_DALY_CITY: &'static str = "Daly City";
pub const STATION_FULL_DOWNTOWN_BERKELEY: &'static str = "Downtown Berkeley";
pub const STATION_FULL_DUBLIN_PLEASANTON: &'static str = "Dublin/Pleasanton";
pub const STATION_FULL_EL_CERRITO_DEL_NORTE: &'static str = "El Cerrito del Norte";
pub const STATION_FULL_EL_CERRITO_PLAZA: &'static str = "El Cerrito Plaza";
pub const STATION_FULL_EMBARCADERO: &'static str = "Embarcadero (SF)";
pub const STATION_FULL_FREMONT: &'static str = "Fremont";
pub const STATION_FULL_FRUITVALE: &'static str = "Fruitvale (Oakland)";
pub const STATION_FULL_GLEN_PARK: &'static str = "Glen Park (SF)";
pub const STATION_FULL_HAYWARD: &'static str = "Hayward";
pub const STATION_FULL_LAFAYETTE: &'static str = "Lafayette";
pub const STATION_FULL_LAKE_MERRITT: &'static str = "Lake Merritt (Oakland)";
pub const STATION_FULL_MAC_ARTHUR: &'static str = "MacArthur (Oakland)";
pub const STATION_FULL_MILLBRAE: &'static str = "Millbrae";
pub const STATION_FULL_MONTGOMERY_ST: &'static str = "Montgomery St. (SF)";
pub const STATION_FULL_NORTH_BERKELEY: &'static str = "North Berkeley";
pub const STATION_FULL_NORTH_CONCORD_MARTINEZ: &'static str = "North Concord/Martinez";
pub const STATION_FULL_OAKLAND_INTL_AIRPORT: &'static str = "Oakland Int'l Airport";
pub const STATION_FULL_ORINDA: &'static str = "Orinda";
pub const STATION_FULL_PITTSBURG_BAY_POINT: &'static str = "Pittsburg/Bay Point";
pub const STATION_FULL_PITTSBURG_CENTER: &'static str = "Pittsburg Center";
pub const STATION_FULL_PLEASANT_HILL: &'static str = "Pleasant Hill";
pub const STATION_FULL_POWELL_ST: &'static str = "Powell St. (SF)";
pub const STATION_FULL_RICHMOND: &'static str = "Richmond";
pub const STATION_FULL_ROCKRIDGE: &'static str = "Rockridge (Oakland)";
pub const STATION_FULL_SAN_BRUNO: &'static str = "San Bruno";
pub const STATION_FULL_SAN_FRANCISCO_INTL_AIRPORT: &'static str = "San Francisco Int'l Airport";
pub const STATION_FULL_SAN_LEANDRO: &'static str = "San Leandro";
pub const STATION_FULL_SOUTH_HAYWARD: &'static str = "South Hayward";
pub const STATION_FULL_SOUTH_SAN_FRANCISCO: &'static str = "South San Francisco";
pub const STATION_FULL_UNION_CITY: &'static str = "Union City";
pub const STATION_FULL_WARM_SPRINGS_SOUTH_FREMONT: &'static str = "Warm Springs/South Fremont";
pub const STATION_FULL_WALNUT_CREEK: &'static str = "Walnut Creek";
pub const STATION_FULL_WEST_DUBLIN: &'static str = "West Dublin";
pub const STATION_FULL_WEST_OAKLAND: &'static str = "West Oakland";

#[derive(Debug, Clone, PartialEq)]
pub enum Station {
    OaklandCityCenter12thSt,
    SFMission16thSt,
    Oakland19thSt,
    SFMission24thSt,
    Ashby,
    Antioch,
    BalboaPark,
    BayFair,
    CastroValley,
    CivicCenter,
    Coliseum,
    Colma,
    Concord,
    DalyCity,
    DowntownBerkeley,
    DublinPleasanton,
    ElCerritoDelNorte,
    ElCerritoPlaza,
    Embarcadero,
    Fremont,
    Fruitvale,
    GlenPark,
    Hayward,
    Lafayette,
    LakeMerritt,
    MacArthur,
    Millbrae,
    MontgomerySt,
    NorthBerkeley,
    NorthConcordMartinez,
    OaklandIntlAirport,
    Orinda,
    PittsburgBayPoint,
    PittsburgCenter,
    PleasantHill,
    PowellSt,
    Richmond,
    Rockridge,
    SanBruno,
    SanFranciscoIntlAirport,
    SanLeandro,
    SouthHayward,
    SouthSanFrancisco,
    UnionCity,
    WarmSpringsSouthFremont,
    WalnutCreek,
    WestDublin,
    WestOakland,
}

impl Station {
    pub fn from_abbr<T: AsRef<str>>(abbr: T) -> Result<Station> {
        match abbr.as_ref().to_lowercase().as_ref() {
            STATION_ABBR_OAKLAND_CITY_CENTER12TH_ST => Ok(Station::OaklandCityCenter12thSt),
            STATION_ABBR_SF_MISSION16TH_ST => Ok(Station::SFMission16thSt),
            STATION_ABBR_OAKLAND19TH_ST => Ok(Station::Oakland19thSt),
            STATION_ABBR_SF_MISSION24TH_ST => Ok(Station::SFMission24thSt),
            STATION_ABBR_ASHBY => Ok(Station::Ashby),
            STATION_ABBR_ANTIOCH => Ok(Station::Antioch),
            STATION_ABBR_BALBOA_PARK => Ok(Station::BalboaPark),
            STATION_ABBR_BAY_FAIR => Ok(Station::BayFair),
            STATION_ABBR_CASTRO_VALLEY => Ok(Station::CastroValley),
            STATION_ABBR_CIVIC_CENTER => Ok(Station::CivicCenter),
            STATION_ABBR_COLISEUM => Ok(Station::Coliseum),
            STATION_ABBR_COLMA => Ok(Station::Colma),
            STATION_ABBR_CONCORD => Ok(Station::Concord),
            STATION_ABBR_DALY_CITY => Ok(Station::DalyCity),
            STATION_ABBR_DOWNTOWN_BERKELEY => Ok(Station::DowntownBerkeley),
            STATION_ABBR_DUBLIN_PLEASANTON => Ok(Station::DublinPleasanton),
            STATION_ABBR_EL_CERRITO_DEL_NORTE => Ok(Station::ElCerritoDelNorte),
            STATION_ABBR_EL_CERRITO_PLAZA => Ok(Station::ElCerritoPlaza),
            STATION_ABBR_EMBARCADERO => Ok(Station::Embarcadero),
            STATION_ABBR_FREMONT => Ok(Station::Fremont),
            STATION_ABBR_FRUITVALE => Ok(Station::Fruitvale),
            STATION_ABBR_GLEN_PARK => Ok(Station::GlenPark),
            STATION_ABBR_HAYWARD => Ok(Station::Hayward),
            STATION_ABBR_LAFAYETTE => Ok(Station::Lafayette),
            STATION_ABBR_LAKE_MERRITT => Ok(Station::LakeMerritt),
            STATION_ABBR_MAC_ARTHUR => Ok(Station::MacArthur),
            STATION_ABBR_MILLBRAE => Ok(Station::Millbrae),
            STATION_ABBR_MONTGOMERY_ST => Ok(Station::MontgomerySt),
            STATION_ABBR_NORTH_BERKELEY => Ok(Station::NorthBerkeley),
            STATION_ABBR_NORTH_CONCORD_MARTINEZ => Ok(Station::NorthConcordMartinez),
            STATION_ABBR_OAKLAND_INTL_AIRPORT => Ok(Station::OaklandIntlAirport),
            STATION_ABBR_ORINDA => Ok(Station::Orinda),
            STATION_ABBR_PITTSBURG_BAY_POINT => Ok(Station::PittsburgBayPoint),
            STATION_ABBR_PITTSBURG_CENTER => Ok(Station::PittsburgCenter),
            STATION_ABBR_PLEASANT_HILL => Ok(Station::PleasantHill),
            STATION_ABBR_POWELL_ST => Ok(Station::PowellSt),
            STATION_ABBR_RICHMOND => Ok(Station::Richmond),
            STATION_ABBR_ROCKRIDGE => Ok(Station::Rockridge),
            STATION_ABBR_SAN_BRUNO => Ok(Station::SanBruno),
            STATION_ABBR_SAN_FRANCISCO_INTL_AIRPORT => Ok(Station::SanFranciscoIntlAirport),
            STATION_ABBR_SAN_LEANDRO => Ok(Station::SanLeandro),
            STATION_ABBR_SOUTH_HAYWARD => Ok(Station::SouthHayward),
            STATION_ABBR_SOUTH_SAN_FRANCISCO => Ok(Station::SouthSanFrancisco),
            STATION_ABBR_UNION_CITY => Ok(Station::UnionCity),
            STATION_ABBR_WARM_SPRINGS_SOUTH_FREMONT => Ok(Station::WarmSpringsSouthFremont),
            STATION_ABBR_WALNUT_CREEK => Ok(Station::WalnutCreek),
            STATION_ABBR_WEST_DUBLIN => Ok(Station::WestDublin),
            STATION_ABBR_WEST_OAKLAND => Ok(Station::WestOakland),
            _ => Err(anyhow!("Does not match any station")),
        }
    }

    pub fn from_full<T: AsRef<str>>(full: T) -> Result<Station> {
        match full.as_ref() {
            STATION_FULL_OAKLAND_CITY_CENTER12TH_ST => Ok(Station::OaklandCityCenter12thSt),
            STATION_FULL_SF_MISSION16TH_ST => Ok(Station::SFMission16thSt),
            STATION_FULL_OAKLAND19TH_ST => Ok(Station::Oakland19thSt),
            STATION_FULL_SF_MISSION24TH_ST => Ok(Station::SFMission24thSt),
            STATION_FULL_ASHBY => Ok(Station::Ashby),
            STATION_FULL_ANTIOCH => Ok(Station::Antioch),
            STATION_FULL_BALBOA_PARK => Ok(Station::BalboaPark),
            STATION_FULL_BAY_FAIR => Ok(Station::BayFair),
            STATION_FULL_CASTRO_VALLEY => Ok(Station::CastroValley),
            STATION_FULL_CIVIC_CENTER => Ok(Station::CivicCenter),
            STATION_FULL_COLISEUM => Ok(Station::Coliseum),
            STATION_FULL_COLMA => Ok(Station::Colma),
            STATION_FULL_CONCORD => Ok(Station::Concord),
            STATION_FULL_DALY_CITY => Ok(Station::DalyCity),
            STATION_FULL_DOWNTOWN_BERKELEY => Ok(Station::DowntownBerkeley),
            STATION_FULL_DUBLIN_PLEASANTON => Ok(Station::DublinPleasanton),
            STATION_FULL_EL_CERRITO_DEL_NORTE => Ok(Station::ElCerritoDelNorte),
            STATION_FULL_EL_CERRITO_PLAZA => Ok(Station::ElCerritoPlaza),
            STATION_FULL_EMBARCADERO => Ok(Station::Embarcadero),
            STATION_FULL_FREMONT => Ok(Station::Fremont),
            STATION_FULL_FRUITVALE => Ok(Station::Fruitvale),
            STATION_FULL_GLEN_PARK => Ok(Station::GlenPark),
            STATION_FULL_HAYWARD => Ok(Station::Hayward),
            STATION_FULL_LAFAYETTE => Ok(Station::Lafayette),
            STATION_FULL_LAKE_MERRITT => Ok(Station::LakeMerritt),
            STATION_FULL_MAC_ARTHUR => Ok(Station::MacArthur),
            STATION_FULL_MILLBRAE => Ok(Station::Millbrae),
            STATION_FULL_MONTGOMERY_ST => Ok(Station::MontgomerySt),
            STATION_FULL_NORTH_BERKELEY => Ok(Station::NorthBerkeley),
            STATION_FULL_NORTH_CONCORD_MARTINEZ => Ok(Station::NorthConcordMartinez),
            STATION_FULL_OAKLAND_INTL_AIRPORT => Ok(Station::OaklandIntlAirport),
            STATION_FULL_ORINDA => Ok(Station::Orinda),
            STATION_FULL_PITTSBURG_BAY_POINT => Ok(Station::PittsburgBayPoint),
            STATION_FULL_PITTSBURG_CENTER => Ok(Station::PittsburgCenter),
            STATION_FULL_PLEASANT_HILL => Ok(Station::PleasantHill),
            STATION_FULL_POWELL_ST => Ok(Station::PowellSt),
            STATION_FULL_RICHMOND => Ok(Station::Richmond),
            STATION_FULL_ROCKRIDGE => Ok(Station::Rockridge),
            STATION_FULL_SAN_BRUNO => Ok(Station::SanBruno),
            STATION_FULL_SAN_FRANCISCO_INTL_AIRPORT => Ok(Station::SanFranciscoIntlAirport),
            STATION_FULL_SAN_LEANDRO => Ok(Station::SanLeandro),
            STATION_FULL_SOUTH_HAYWARD => Ok(Station::SouthHayward),
            STATION_FULL_SOUTH_SAN_FRANCISCO => Ok(Station::SouthSanFrancisco),
            STATION_FULL_UNION_CITY => Ok(Station::UnionCity),
            STATION_FULL_WARM_SPRINGS_SOUTH_FREMONT => Ok(Station::WarmSpringsSouthFremont),
            STATION_FULL_WALNUT_CREEK => Ok(Station::WalnutCreek),
            STATION_FULL_WEST_DUBLIN => Ok(Station::WestDublin),
            STATION_FULL_WEST_OAKLAND => Ok(Station::WestOakland),
            _ => Err(anyhow!("Does not match any station")),
        }
    }

    pub fn to_abbr(&self) -> &'static str {
        match self {
            Station::OaklandCityCenter12thSt => STATION_ABBR_OAKLAND_CITY_CENTER12TH_ST,
            Station::SFMission16thSt => STATION_ABBR_SF_MISSION16TH_ST,
            Station::Oakland19thSt => STATION_ABBR_OAKLAND19TH_ST,
            Station::SFMission24thSt => STATION_ABBR_SF_MISSION24TH_ST,
            Station::Ashby => STATION_ABBR_ASHBY,
            Station::Antioch => STATION_ABBR_ANTIOCH,
            Station::BalboaPark => STATION_ABBR_BALBOA_PARK,
            Station::BayFair => STATION_ABBR_BAY_FAIR,
            Station::CastroValley => STATION_ABBR_CASTRO_VALLEY,
            Station::CivicCenter => STATION_ABBR_CIVIC_CENTER,
            Station::Coliseum => STATION_ABBR_COLISEUM,
            Station::Colma => STATION_ABBR_COLMA,
            Station::Concord => STATION_ABBR_CONCORD,
            Station::DalyCity => STATION_ABBR_DALY_CITY,
            Station::DowntownBerkeley => STATION_ABBR_DOWNTOWN_BERKELEY,
            Station::DublinPleasanton => STATION_ABBR_DUBLIN_PLEASANTON,
            Station::ElCerritoDelNorte => STATION_ABBR_EL_CERRITO_DEL_NORTE,
            Station::ElCerritoPlaza => STATION_ABBR_EL_CERRITO_PLAZA,
            Station::Embarcadero => STATION_ABBR_EMBARCADERO,
            Station::Fremont => STATION_ABBR_FREMONT,
            Station::Fruitvale => STATION_ABBR_FRUITVALE,
            Station::GlenPark => STATION_ABBR_GLEN_PARK,
            Station::Hayward => STATION_ABBR_HAYWARD,
            Station::Lafayette => STATION_ABBR_LAFAYETTE,
            Station::LakeMerritt => STATION_ABBR_LAKE_MERRITT,
            Station::MacArthur => STATION_ABBR_MAC_ARTHUR,
            Station::Millbrae => STATION_ABBR_MILLBRAE,
            Station::MontgomerySt => STATION_ABBR_MONTGOMERY_ST,
            Station::NorthBerkeley => STATION_ABBR_NORTH_BERKELEY,
            Station::NorthConcordMartinez => STATION_ABBR_NORTH_CONCORD_MARTINEZ,
            Station::OaklandIntlAirport => STATION_ABBR_OAKLAND_INTL_AIRPORT,
            Station::Orinda => STATION_ABBR_ORINDA,
            Station::PittsburgBayPoint => STATION_ABBR_PITTSBURG_BAY_POINT,
            Station::PittsburgCenter => STATION_ABBR_PITTSBURG_CENTER,
            Station::PleasantHill => STATION_ABBR_PLEASANT_HILL,
            Station::PowellSt => STATION_ABBR_POWELL_ST,
            Station::Richmond => STATION_ABBR_RICHMOND,
            Station::Rockridge => STATION_ABBR_ROCKRIDGE,
            Station::SanBruno => STATION_ABBR_SAN_BRUNO,
            Station::SanFranciscoIntlAirport => STATION_ABBR_SAN_FRANCISCO_INTL_AIRPORT,
            Station::SanLeandro => STATION_ABBR_SAN_LEANDRO,
            Station::SouthHayward => STATION_ABBR_SOUTH_HAYWARD,
            Station::SouthSanFrancisco => STATION_ABBR_SOUTH_SAN_FRANCISCO,
            Station::UnionCity => STATION_ABBR_UNION_CITY,
            Station::WarmSpringsSouthFremont => STATION_ABBR_WARM_SPRINGS_SOUTH_FREMONT,
            Station::WalnutCreek => STATION_ABBR_WALNUT_CREEK,
            Station::WestDublin => STATION_ABBR_WEST_DUBLIN,
            Station::WestOakland => STATION_ABBR_WEST_OAKLAND,
        }
    }

    pub fn to_full(&self) -> &'static str {
        match self {
            Station::OaklandCityCenter12thSt => STATION_FULL_OAKLAND_CITY_CENTER12TH_ST,
            Station::SFMission16thSt => STATION_FULL_SF_MISSION16TH_ST,
            Station::Oakland19thSt => STATION_FULL_OAKLAND19TH_ST,
            Station::SFMission24thSt => STATION_FULL_SF_MISSION24TH_ST,
            Station::Ashby => STATION_FULL_ASHBY,
            Station::Antioch => STATION_FULL_ANTIOCH,
            Station::BalboaPark => STATION_FULL_BALBOA_PARK,
            Station::BayFair => STATION_FULL_BAY_FAIR,
            Station::CastroValley => STATION_FULL_CASTRO_VALLEY,
            Station::CivicCenter => STATION_FULL_CIVIC_CENTER,
            Station::Coliseum => STATION_FULL_COLISEUM,
            Station::Colma => STATION_FULL_COLMA,
            Station::Concord => STATION_FULL_CONCORD,
            Station::DalyCity => STATION_FULL_DALY_CITY,
            Station::DowntownBerkeley => STATION_FULL_DOWNTOWN_BERKELEY,
            Station::DublinPleasanton => STATION_FULL_DUBLIN_PLEASANTON,
            Station::ElCerritoDelNorte => STATION_FULL_EL_CERRITO_DEL_NORTE,
            Station::ElCerritoPlaza => STATION_FULL_EL_CERRITO_PLAZA,
            Station::Embarcadero => STATION_FULL_EMBARCADERO,
            Station::Fremont => STATION_FULL_FREMONT,
            Station::Fruitvale => STATION_FULL_FRUITVALE,
            Station::GlenPark => STATION_FULL_GLEN_PARK,
            Station::Hayward => STATION_FULL_HAYWARD,
            Station::Lafayette => STATION_FULL_LAFAYETTE,
            Station::LakeMerritt => STATION_FULL_LAKE_MERRITT,
            Station::MacArthur => STATION_FULL_MAC_ARTHUR,
            Station::Millbrae => STATION_FULL_MILLBRAE,
            Station::MontgomerySt => STATION_FULL_MONTGOMERY_ST,
            Station::NorthBerkeley => STATION_FULL_NORTH_BERKELEY,
            Station::NorthConcordMartinez => STATION_FULL_NORTH_CONCORD_MARTINEZ,
            Station::OaklandIntlAirport => STATION_FULL_OAKLAND_INTL_AIRPORT,
            Station::Orinda => STATION_FULL_ORINDA,
            Station::PittsburgBayPoint => STATION_FULL_PITTSBURG_BAY_POINT,
            Station::PittsburgCenter => STATION_FULL_PITTSBURG_CENTER,
            Station::PleasantHill => STATION_FULL_PLEASANT_HILL,
            Station::PowellSt => STATION_FULL_POWELL_ST,
            Station::Richmond => STATION_FULL_RICHMOND,
            Station::Rockridge => STATION_FULL_ROCKRIDGE,
            Station::SanBruno => STATION_FULL_SAN_BRUNO,
            Station::SanFranciscoIntlAirport => STATION_FULL_SAN_FRANCISCO_INTL_AIRPORT,
            Station::SanLeandro => STATION_FULL_SAN_LEANDRO,
            Station::SouthHayward => STATION_FULL_SOUTH_HAYWARD,
            Station::SouthSanFrancisco => STATION_FULL_SOUTH_SAN_FRANCISCO,
            Station::UnionCity => STATION_FULL_UNION_CITY,
            Station::WarmSpringsSouthFremont => STATION_FULL_WARM_SPRINGS_SOUTH_FREMONT,
            Station::WalnutCreek => STATION_FULL_WALNUT_CREEK,
            Station::WestDublin => STATION_FULL_WEST_DUBLIN,
            Station::WestOakland => STATION_FULL_WEST_OAKLAND,
        }
    }
}

impl TryFrom<String> for Station {
    type Error = &'static str;

    fn try_from(station_string: String) -> std::result::Result<Self, Self::Error> {
        let abbr = Station::from_abbr(&station_string);
        if abbr.is_ok() {
            return Ok(abbr.unwrap());
        }

        let full = Station::from_full(&station_string);
        if full.is_ok() {
            return Ok(full.unwrap());
        }

        Err("Does not match any station")
    }
}

impl Serialize for Station {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(self.to_abbr())
    }
}

impl<'de> Deserialize<'de> for Station {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Station::try_from(s).map_err(|e| serde::de::Error::custom(e))
    }
}
