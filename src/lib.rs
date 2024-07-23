pub mod network {
    use std::time::Instant;
    use pnet::datalink::{self};
    use pnet::datalink::Channel::Ethernet;
    use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
    use pnet::packet::ipv4::Ipv4Packet;
    use pnet::packet::Packet;

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
            let interfaces = datalink::interfaces();
            let interface = interfaces
                .into_iter()
                .find(|iface| iface.name == self.interface_name)
                .expect("Interface not found");

            let (mut _tx, mut rx) = match datalink::channel(&interface, Default::default()) {
                Ok(Ethernet(tx, rx)) => (tx, rx),
                Ok(_) => panic!("Unhandled channel type"),
                Err(e) => panic!("An error occurred when creating the datalink: {}", e),
            };

            let mut last_print = Instant::now();

            loop {
                match rx.next() {
                    Ok(packet) => {
                         let ethernet = EthernetPacket::new(packet).expect("Failed to parse Ethernet packet");
                        if ethernet.get_ethertype() != EtherTypes::Ipv4 {
                            continue;
                        }

                        let ipv4 = Ipv4Packet::new(ethernet.payload()).expect("Failed to parse IPv4 packet");
                        let packet_size = ipv4.packet().len() as u64;

                        if interface.ips.iter().any(|ip| ip.ip() == ipv4.get_destination()) {
                            self.bytes_received += packet_size;
                        } else {
                            self.bytes_sent += packet_size;
                        }

                        let now = Instant::now();
                        if now.duration_since(last_print).as_secs() >= 5 {
                            let duration = now.duration_since(self.last_time).as_secs_f64();
                            let download_speed = (self.bytes_received as f64 / duration) / 125000.0;  // Convert to Mbps
                            let upload_speed = (self.bytes_sent as f64 / duration) / 125000.0;  // Convert to Mbps

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
}