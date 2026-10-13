use std::{fmt::Display, fs::File};

use jiff::civil::{Date, DateTime};

/// Records of the train service dataset as it is described here: <https://www.rijdendetreinen.nl/en/open-data/train-archive>
/// The dataset is licensed under CC-BY by Rijden de Treinen
#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Record {
    /// This is the ID that Rijden de Treinen uses for this service. It has no
    /// useful meaning beyond uniquely identifying a single service on a single
    /// date.
    ///
    /// This ID will occur more than once in the CSV files because this ID is
    /// unique for each service. There is also a column Stop:RDT-ID with a
    /// unique ID for each stop.
    #[serde(rename = "Service:RDT-ID")]
    pub service_rdt_id: u32,

    /// The scheduled service date for this service.
    ///
    /// The service date is not always the same as the actual date. For example,
    /// a service that departs at 23:59 on 31 July and arrives at 02:00 on 1
    /// August has a service date of 31 July. Delays do not affect the service
    /// date.
    #[serde(rename = "Service:Date")]
    pub service_date: Date,

    /// The service type, for example: Intercity, Sprinter or ICE International.
    #[serde(rename = "Service:Type")]
    pub service_type: TrainType,

    /// Company operating this service, like NS or Arriva.
    #[serde(rename = "Service:Company")]
    pub service_company: String,

    /// The train number (service number) for this service uniquely identifies
    /// this service on this date. This number is sometimes also communicated to
    /// passengers (especially for international trains).
    ///
    /// A single service may sometimes have multiple train numbers. For example,
    /// when a train is split in two parts, or when a train changes a train
    /// number on a major station halfway.
    #[serde(rename = "Service:Train number")]
    pub service_train_number: u32,

    /// This column is true when all stops of this service have been cancelled.
    /// Or in other words: when the train does not run at all.
    #[serde(rename = "Service:Completely cancelled")]
    pub service_completely_cancelled: bool,

    /// This column is true when one or more stops of this service have been
    /// cancelled. Or in other words: when the train does not run on a part of
    /// the route.
    #[serde(rename = "Service:Partly cancelled")]
    pub service_partly_cancelled: bool,

    /// The highest delay (in minutes) of all stops of this service.
    #[serde(rename = "Service:Maximum delay")]
    pub service_maximum_delay: u32,

    /// Unique identifier for this stop. This ID is unique for each stop in the
    /// dataset. It has no further useful meaning.
    #[serde(rename = "Stop:RDT-ID")]
    pub stop_rdt_id: u32,

    /// Code (abbreviation) of the station name. See also the [dataset with railway stations](https://www.rijdendetreinen.nl/en/open-data/stations).
    #[serde(rename = "Stop:Station code")]
    pub stop_station_code: String,

    /// The name of the station.
    #[serde(rename = "Stop:Station name")]
    pub stop_station_name: String,

    ///  Scheduled arrival time in RFC 3339 format. This column is empty when
    /// no arrival was scheduled.
    #[serde(rename = "Stop:Arrival time")]
    pub stop_arrival_time: Option<DateTime>,

    /// Arrival delay in minutes. This column is empty when no arrival was
    /// scheduled.
    #[serde(rename = "Stop:Arrival delay")]
    pub stop_arrival_delay: Option<u32>,

    ///  This column is true when the arrival at this stop has been cancelled.
    /// This column is empty when no arrival was scheduled.
    #[serde(rename = "Stop:Arrival cancelled")]
    pub stop_arrival_cancelled: Option<bool>,

    /// Scheduled departure time in RFC 3339 format. This column is empty when
    /// no departure was scheduled.
    #[serde(rename = "Stop:Departure time")]
    pub stop_departure_time: Option<DateTime>,

    /// Departure delay in minutes. This column is empty when no departure was
    /// scheduled.
    #[serde(rename = "Stop:Departure delay")]
    pub stop_departure_delay: Option<u32>,

    /// This column is true when the departure at this stop has been cancelled.
    /// This column is empty when no departure was scheduled.
    #[serde(rename = "Stop:Departure cancelled")]
    pub stop_departure_cancelled: Option<bool>,

    /// This column is true when the platform of this stop has changed from the
    /// planned platform.
    #[serde(rename = "Stop:Platform change")]
    pub stop_platform_change: bool,

    /// The originally scheduled platform for this service.
    #[serde(rename = "Stop:Planned platform")]
    pub stop_planned_platform: String,

    /// The platform that was actually used for this service.
    #[serde(rename = "Stop:Actual platform")]
    pub stop_actual_platform: String,
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            service_rdt_id: _,
            service_date: _,
            service_type,
            service_company,
            service_train_number,
            service_completely_cancelled,
            service_partly_cancelled,
            service_maximum_delay,
            stop_rdt_id: _,
            stop_station_code: _,
            stop_station_name,
            stop_arrival_time,
            stop_arrival_delay,
            stop_arrival_cancelled,
            stop_departure_time,
            stop_departure_delay,
            stop_departure_cancelled,
            stop_platform_change,
            stop_planned_platform,
            stop_actual_platform,
        } = self;

        writeln!(
            f,
            "{service_type:?} {service_train_number} by {service_company} at {stop_station_name}:"
        )?;

        if *stop_platform_change {
            write!(
                f,
                "- on platform {stop_planned_platform} -> {stop_actual_platform}"
            )?;
        } else if !stop_planned_platform.is_empty() {
            writeln!(f, "- on platform {stop_planned_platform}")?;
        };

        match (service_partly_cancelled, service_completely_cancelled) {
            (false, false) => {}
            (true, false) => writeln!(f, "- partially cancelled")?,
            (_, true) => writeln!(f, "- completely cancelled")?,
        }

        if *service_maximum_delay > 0 {
            writeln!(f, "- worst delay {service_maximum_delay} min")?;
        }

        match (
            stop_arrival_time,
            stop_arrival_delay,
            stop_arrival_cancelled,
        ) {
            (Some(time), Some(0), Some(false)) => writeln!(f, "- arrival at {time}")?,
            (Some(time), Some(delay), Some(false)) => {
                writeln!(f, "- scheduled arrival at {time} delayed by {delay} min")?
            }
            (Some(time), Some(_), Some(true)) => {
                writeln!(f, "- scheduled arrival at {time} cancelled")?
            }
            (None, None, None) => {}
            (_, _, _) => unreachable!(),
        }

        match (
            stop_departure_time,
            stop_departure_delay,
            stop_departure_cancelled,
        ) {
            (Some(time), Some(0), Some(false)) => writeln!(f, "- departure at {time}")?,
            (Some(time), Some(delay), Some(false)) => writeln!(
                f,
                "- scheduled departure at {time:#} delayed by {delay} min"
            )?,
            (Some(time), Some(_), Some(true)) => {
                writeln!(f, "- scheduled departure at {time} cancelled")?
            }
            (None, None, None) => {}
            (_, _, _) => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TrainType {
    Intercity,
    Sprinter,
    EuroCity,
    Nightjet,
    #[serde(rename = "Intercity direct")]
    IntercityDirect,
    Eurostar,
    #[serde(rename = "Eurocity Direct")]
    EurocityDirect,
    ICE,
    Stoptrein,
    Sneltrein,
    #[serde(untagged)]
    Other(String),
}

pub fn read_services(path: impl AsRef<std::path::Path>) -> impl Iterator<Item = Record> {
    let gz_file = File::open(path.as_ref()).unwrap_or_else(|_| {
        panic!(
            "Input file should be at {} but is not",
            path.as_ref().display()
        )
    });
    let gz_reader = flate2::read::GzDecoder::new(gz_file);
    let csv_reader = csv::Reader::from_reader(gz_reader);

    csv_reader
        .into_deserialize()
        .filter_map(|maybe_elem| match maybe_elem {
            Ok(elem) => Some(elem),
            Err(e) => {
                eprintln!("Could not decode line, skipping... {e}");
                None
            }
        })
}
