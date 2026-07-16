use next_next_next::read_services;

fn main() {
    let path = "services-2026-05.csv.gz";
    let mut services = read_services(path);

    let stop = services.next();
    println!("{stop:#?}");
}
