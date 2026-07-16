use std::collections::BTreeMap;

use jiff::civil::Date;
use next_next_next::read_services;

// Find all the stops at which a train was cancelled, i.e. where the arrival was not cancelled, but the departure was, on 1st May 2025

fn main() {
    let path = "services-2026-05.csv.gz";
    let services = read_services(path);

    let mut cancelled_stations = BTreeMap::<String, u32>::new();
    services
        .take_while(|stop| stop.service_date == Date::constant(2026, 05, 01))
        .filter(|stop| {
            stop.stop_arrival_cancelled == Some(false)
                && stop.stop_departure_cancelled == Some(true)
        })
        .for_each(|stop| {
            *cancelled_stations
                .entry(stop.stop_station_name)
                .or_default() += 1
        });

    for (station, num_cancellations) in &cancelled_stations {
        println!("{station}: {num_cancellations}");
    }

    let worst = cancelled_stations
        .iter()
        .max_by_key(|(_name, count)| *count)
        .unwrap();
    println!("{} was worst with {} cancellations", worst.0, worst.1);
}
