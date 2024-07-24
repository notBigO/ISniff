use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;
use pnet::datalink::NetworkInterface;

pub fn process_packet(
    packet: &[u8],
    interface: &NetworkInterface,
    bytes_received: &mut u64,
    bytes_sent: &mut u64,
) {
    let ethernet = EthernetPacket::new(packet).expect("Failed to parse Ethernet packet");
    if ethernet.get_ethertype() != EtherTypes::Ipv4 {
        return;
    }

    let ipv4 = Ipv4Packet::new(ethernet.payload()).expect("Failed to parse IPv4 packet");
    let packet_size = ipv4.packet().len() as u64;

    if interface.ips.iter().any(|ip| ip.ip() == ipv4.get_destination()) {
        *bytes_received += packet_size;
    } else {
        *bytes_sent += packet_size;
    }
}
