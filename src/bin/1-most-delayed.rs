use next_next_next::read_services;

fn main() {
    let path = "services-2026-05.csv.gz";
    let service = read_services(path);

    let oh_no = service
        .max_by_key(
            |stop| match (stop.stop_arrival_delay, stop.stop_departure_delay) {
                (None, None) => 0,
                (None, Some(d)) | (Some(d), None) => d,
                (Some(d1), Some(d2)) => u32::max(d1, d2),
            },
        )
        .unwrap();

    println!("The most delayed stop was:\n{oh_no:}");
}
