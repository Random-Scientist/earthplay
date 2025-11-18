use mdns_sd::ServiceDaemon;
use proto::stuff::AirplayFeatures;

fn main() {
    let daemon = ServiceDaemon::new().unwrap();
    let browse = daemon.browse("_airplay._tcp.local.").unwrap();
    loop {
        let resp = browse.recv().unwrap();
        if let mdns_sd::ServiceEvent::ServiceResolved(resolved_service) = resp {
            println!("----------------------------------");
            println!("Device: {}", &resolved_service.host);
            println!("With properties: {:#?}", &resolved_service.txt_properties);
            let features = AirplayFeatures::parse(
                resolved_service.get_property("features").unwrap().val_str(),
            )
            .unwrap();
            println!("Features: ");
            for (name, _) in features.iter_names() {
                println!("{name}");
            }
            println!("Addresses: ");
            for addr in resolved_service.addresses {
                println!("{addr}");
            }
        }
    }
}
