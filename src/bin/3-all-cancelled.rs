use jiff::civil::Date;
use next_next_next::read_services;

// Find all the stops at which a train was cancelled, i.e. where the arrival was not cancelled, but the departure was, on 1st May 2025

fn main() {
    let path = "services-2026-05.csv.gz";
    let services = read_services(path);

    let cancelled: Vec<_> = services
        .take_while(|stop| stop.service_date == Date::constant(2026, 05, 01))
        .filter(|stop| {
            stop.stop_arrival_cancelled == Some(false)
                && stop.stop_departure_cancelled == Some(true)
        })
        .collect();

    println!("{}", cancelled.len());

    for stop in cancelled {
        println!("{}", stop.stop_station_name);
    }
}
