use std::env;
use sniffme::network::NetworkMonitor;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <interface_name>", args[0]);
        std::process::exit(1);
    }

    let interface_name = &args[1];

    println!("Starting network monitor on interface: {}", interface_name);

    let mut monitor = NetworkMonitor::new(interface_name);
    monitor.start();
}
