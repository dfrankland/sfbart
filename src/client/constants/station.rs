use anyhow::{anyhow, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;

pub const STATION_ABBR_OAKLAND_CITY_CENTER12TH_ST: &str = "12th";
pub const STATION_ABBR_SF_MISSION16TH_ST: &str = "16th";
pub const STATION_ABBR_OAKLAND19TH_ST: &str = "19th";
pub const STATION_ABBR_SF_MISSION24TH_ST: &str = "24th";
pub const STATION_ABBR_ASHBY: &str = "ashb";
pub const STATION_ABBR_ANTIOCH: &str = "antc";
pub const STATION_ABBR_BALBOA_PARK: &str = "balb";
pub const STATION_ABBR_BAY_FAIR: &str = "bayf";
pub const STATION_ABBR_CASTRO_VALLEY: &str = "cast";
pub const STATION_ABBR_CIVIC_CENTER: &str = "civc";
pub const STATION_ABBR_COLISEUM: &str = "cols";
pub const STATION_ABBR_COLMA: &str = "colm";
pub const STATION_ABBR_CONCORD: &str = "conc";
pub const STATION_ABBR_DALY_CITY: &str = "daly";
pub const STATION_ABBR_DOWNTOWN_BERKELEY: &str = "dbrk";
pub const STATION_ABBR_DUBLIN_PLEASANTON: &str = "dubl";
pub const STATION_ABBR_EL_CERRITO_DEL_NORTE: &str = "deln";
pub const STATION_ABBR_EL_CERRITO_PLAZA: &str = "plza";
pub const STATION_ABBR_EMBARCADERO: &str = "embr";
pub const STATION_ABBR_FREMONT: &str = "frmt";
pub const STATION_ABBR_FRUITVALE: &str = "ftvl";
pub const STATION_ABBR_GLEN_PARK: &str = "glen";
pub const STATION_ABBR_HAYWARD: &str = "hayw";
pub const STATION_ABBR_LAFAYETTE: &str = "lafy";
pub const STATION_ABBR_LAKE_MERRITT: &str = "lake";
pub const STATION_ABBR_MAC_ARTHUR: &str = "mcar";
pub const STATION_ABBR_MILLBRAE: &str = "mlbr";
pub const STATION_ABBR_MONTGOMERY_ST: &str = "mont";
pub const STATION_ABBR_NORTH_BERKELEY: &str = "nbrk";
pub const STATION_ABBR_NORTH_CONCORD_MARTINEZ: &str = "ncon";
pub const STATION_ABBR_OAKLAND_INTL_AIRPORT: &str = "oakl";
pub const STATION_ABBR_ORINDA: &str = "orin";
pub const STATION_ABBR_PITTSBURG_BAY_POINT: &str = "pitt";
pub const STATION_ABBR_PITTSBURG_CENTER: &str = "pctr";
pub const STATION_ABBR_PLEASANT_HILL: &str = "phil";
pub const STATION_ABBR_POWELL_ST: &str = "powl";
pub const STATION_ABBR_RICHMOND: &str = "rich";
pub const STATION_ABBR_ROCKRIDGE: &str = "rock";
pub const STATION_ABBR_SAN_BRUNO: &str = "sbrn";
pub const STATION_ABBR_SAN_FRANCISCO_INTL_AIRPORT: &str = "sfia";
pub const STATION_ABBR_SAN_LEANDRO: &str = "sanl";
pub const STATION_ABBR_SOUTH_HAYWARD: &str = "shay";
pub const STATION_ABBR_SOUTH_SAN_FRANCISCO: &str = "ssan";
pub const STATION_ABBR_UNION_CITY: &str = "ucty";
pub const STATION_ABBR_WARM_SPRINGS_SOUTH_FREMONT: &str = "warm";
pub const STATION_ABBR_WALNUT_CREEK: &str = "wcrk";
pub const STATION_ABBR_WEST_DUBLIN: &str = "wdub";
pub const STATION_ABBR_WEST_OAKLAND: &str = "woak";

pub const STATION_FULL_OAKLAND_CITY_CENTER12TH_ST: &str = "12th St. Oakland City Center";
pub const STATION_FULL_SF_MISSION16TH_ST: &str = "16th St. Mission (SF)";
pub const STATION_FULL_OAKLAND19TH_ST: &str = "19th St. Oakland";
pub const STATION_FULL_SF_MISSION24TH_ST: &str = "24th St. Mission (SF)";
pub const STATION_FULL_ASHBY: &str = "Ashby (Berkeley)";
pub const STATION_FULL_ANTIOCH: &str = "Antioch";
pub const STATION_FULL_BALBOA_PARK: &str = "Balboa Park (SF)";
pub const STATION_FULL_BAY_FAIR: &str = "Bay Fair (San Leandro)";
pub const STATION_FULL_CASTRO_VALLEY: &str = "Castro Valley";
pub const STATION_FULL_CIVIC_CENTER: &str = "Civic Center (SF)";
pub const STATION_FULL_COLISEUM: &str = "Coliseum";
pub const STATION_FULL_COLMA: &str = "Colma";
pub const STATION_FULL_CONCORD: &str = "Concord";
pub const STATION_FULL_DALY_CITY: &str = "Daly City";
pub const STATION_FULL_DOWNTOWN_BERKELEY: &str = "Downtown Berkeley";
pub const STATION_FULL_DUBLIN_PLEASANTON: &str = "Dublin/Pleasanton";
pub const STATION_FULL_EL_CERRITO_DEL_NORTE: &str = "El Cerrito del Norte";
pub const STATION_FULL_EL_CERRITO_PLAZA: &str = "El Cerrito Plaza";
pub const STATION_FULL_EMBARCADERO: &str = "Embarcadero (SF)";
pub const STATION_FULL_FREMONT: &str = "Fremont";
pub const STATION_FULL_FRUITVALE: &str = "Fruitvale (Oakland)";
pub const STATION_FULL_GLEN_PARK: &str = "Glen Park (SF)";
pub const STATION_FULL_HAYWARD: &str = "Hayward";
pub const STATION_FULL_LAFAYETTE: &str = "Lafayette";
pub const STATION_FULL_LAKE_MERRITT: &str = "Lake Merritt (Oakland)";
pub const STATION_FULL_MAC_ARTHUR: &str = "MacArthur (Oakland)";
pub const STATION_FULL_MILLBRAE: &str = "Millbrae";
pub const STATION_FULL_MONTGOMERY_ST: &str = "Montgomery St. (SF)";
pub const STATION_FULL_NORTH_BERKELEY: &str = "North Berkeley";
pub const STATION_FULL_NORTH_CONCORD_MARTINEZ: &str = "North Concord/Martinez";
pub const STATION_FULL_OAKLAND_INTL_AIRPORT: &str = "Oakland Int'l Airport";
pub const STATION_FULL_ORINDA: &str = "Orinda";
pub const STATION_FULL_PITTSBURG_BAY_POINT: &str = "Pittsburg/Bay Point";
pub const STATION_FULL_PITTSBURG_CENTER: &str = "Pittsburg Center";
pub const STATION_FULL_PLEASANT_HILL: &str = "Pleasant Hill";
pub const STATION_FULL_POWELL_ST: &str = "Powell St. (SF)";
pub const STATION_FULL_RICHMOND: &str = "Richmond";
pub const STATION_FULL_ROCKRIDGE: &str = "Rockridge (Oakland)";
pub const STATION_FULL_SAN_BRUNO: &str = "San Bruno";
pub const STATION_FULL_SAN_FRANCISCO_INTL_AIRPORT: &str = "San Francisco Int'l Airport";
pub const STATION_FULL_SAN_LEANDRO: &str = "San Leandro";
pub const STATION_FULL_SOUTH_HAYWARD: &str = "South Hayward";
pub const STATION_FULL_SOUTH_SAN_FRANCISCO: &str = "South San Francisco";
pub const STATION_FULL_UNION_CITY: &str = "Union City";
pub const STATION_FULL_WARM_SPRINGS_SOUTH_FREMONT: &str = "Warm Springs/South Fremont";
pub const STATION_FULL_WALNUT_CREEK: &str = "Walnut Creek";
pub const STATION_FULL_WEST_DUBLIN: &str = "West Dublin";
pub const STATION_FULL_WEST_OAKLAND: &str = "West Oakland";

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

    pub fn to_abbr(&self) -> &str {
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

    pub fn to_full(&self) -> &str {
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
    type Error = anyhow::Error;

    fn try_from(station_string: String) -> std::result::Result<Self, Self::Error> {
        let abbr = Station::from_abbr(&station_string);
        if let Ok(station) = abbr {
            return Ok(station);
        }

        let full = Station::from_full(&station_string);
        if let Ok(station) = full {
            return Ok(station);
        }

        full
    }
}

impl Serialize for Station {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_abbr())
    }
}

impl<'de> Deserialize<'de> for Station {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Station::try_from(s).map_err(serde::de::Error::custom)
    }
}
