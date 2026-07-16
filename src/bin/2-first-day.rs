use next_next_next::read_services;

fn main() {
    let path = "services-2026-05.csv.gz";
    let service = read_services(path);

    let mut first_date = None;

    let oh_no = service
        .take_while(|stop| first_date.get_or_insert(stop.service_date) == &stop.service_date)
        .max_by_key(
            |stop| match (stop.stop_arrival_delay, stop.stop_departure_delay) {
                (None, None) => 0,
                (None, Some(d)) | (Some(d), None) => d,
                (Some(d1), Some(d2)) => u32::max(d1, d2),
            },
        )
        .unwrap();

    let date = oh_no.service_date;
    println!("The most delayed stop on {date} was:\n{oh_no}");
}
