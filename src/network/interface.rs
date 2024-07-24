use pnet::datalink::{self, NetworkInterface};

pub fn get_interface(interface_name: &str) -> Option<NetworkInterface> {
    let interfaces = datalink::interfaces();
    interfaces.into_iter().find(|iface| iface.name == interface_name)
}
