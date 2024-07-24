use std::time::Instant;
use pnet::datalink;
use pnet::datalink::Channel::Ethernet;

use crate::network::interface::get_interface;
use crate::network::packet_processor::process_packet;

pub struct NetworkMonitor {
    interface_name: String,
    last_time: Instant,
    bytes_received: u64,
    bytes_sent: u64,
}

impl NetworkMonitor {
    pub fn new(interface_name: &str) -> Self {
        Self {
            interface_name: interface_name.to_string(),
            last_time: Instant::now(),
            bytes_received: 0,
            bytes_sent: 0,
        }
    }

    pub fn start(&mut self) {
        let interface = get_interface(&self.interface_name).expect("Interface not found");

        let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unhandled channel type"),
            Err(e) => panic!("An error occurred when creating the datalink: {}", e),
        };

        let mut last_print = Instant::now();

        loop {
            match rx.next() {
                Ok(packet) => {
                    process_packet(
                        packet,
                        &interface,
                        &mut self.bytes_received,
                        &mut self.bytes_sent,
                    );

                    let now = Instant::now();
                    if now.duration_since(last_print).as_secs() >= 5 {
                        let duration = now.duration_since(self.last_time).as_secs_f64();
                        let download_speed = (self.bytes_received as f64 / duration) / 1_000_000.0; // Convert to MBps
                        let upload_speed = (self.bytes_sent as f64 / duration) / 1_000_000.0; // Convert to MBps

                        println!("Download Speed: {:.2} Mbps", download_speed);
                        println!("Upload Speed: {:.2} Mbps", upload_speed);

                        self.bytes_received = 0;
                        self.bytes_sent = 0;
                        self.last_time = now;
                        last_print = now;
                    }
                }
                Err(e) => {
                    eprintln!("Error reading packet: {:?}", e);
                }
            }
        }
    }
}
