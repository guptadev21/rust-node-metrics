use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::arp::ArpPacket;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::Packet;
use std::sync::{Arc, Mutex};
use std::thread;

pub type PacketHandler = dyn Fn(EthernetPacket) + Send + Sync;

pub fn capture_packets(handler: Arc<Mutex<PacketHandler>>) {
    let interfaces = datalink::interfaces()
        .into_iter()
        .filter(|interface| interface.name == "wlan0")
        .collect::<Vec<_>>();

    let mut handles = vec![];

    for interface in interfaces {
        let handler_clone = handler.clone();
        let handle = thread::spawn(move || {
            capture_packets_on_interface(interface, handler_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn capture_packets_on_interface(interface: datalink::NetworkInterface, handler: Arc<Mutex<PacketHandler>>) {
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type: {}", &interface.name),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    println!("Start thread reading packet on interface: {}", &interface.name);

    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                    let handler = handler.lock().unwrap();
                    handler(ethernet_packet); // Call the handler with the packet data

                    // Uncomment for debugging
                    // match ethernet_packet.get_ethertype() {
                    //     EtherTypes::Ipv6 => {
                    //         if let Some(ipv6_packet) = Ipv6Packet::new(ethernet_packet.payload()) {
                    //             println!(
                    //                 "Layer 3: IPv6 packet: source {} destination {} => {} {}",
                    //                 ipv6_packet.get_source(),
                    //                 ipv6_packet.get_destination(),
                    //                 ipv6_packet.get_next_header(),
                    //                 ipv6_packet.get_payload_length()
                    //             );
                    //         }
                    //     }
                    //     EtherTypes::Ipv4 => {
                    //         if let Some(ipv4_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                    //             println!(
                    //                 "Layer 3: IPv4 packet: source {} destination {} => {} {}",
                    //                 ipv4_packet.get_source(),
                    //                 ipv4_packet.get_destination(),
                    //                 ipv4_packet.get_next_level_protocol(),
                    //                 ipv4_packet.get_total_length(),
                    //             );
                    //         }
                    //     }
                    //     EtherTypes::Arp => {
                    //         if let Some(arp_packet) = ArpPacket::new(ethernet_packet.payload()) {
                    //             println!(
                    //                 "Layer 2: ARP packet: source {} destination {} => {:?} {} {} {:?} {}",
                    //                 arp_packet.get_sender_hw_addr(),
                    //                 arp_packet.get_target_hw_addr(),
                    //                 arp_packet.get_operation(),
                    //                 arp_packet.get_target_proto_addr(),
                    //                 arp_packet.get_sender_proto_addr(),
                    //                 arp_packet.get_hardware_type(),
                    //                 arp_packet.get_proto_addr_len()
                    //             );
                    //         }
                    //     }
                    //     _ => {
                    //         println!(
                    //             "Unknown or unsupported packet type: {:?}",
                    //             ethernet_packet.get_ethertype()
                    //         );
                    //     }
                    // }
                }
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
