use pnet::datalink;
use pnet::packet::ethernet::EthernetPacket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Import the function to test from the metrics module
use node_exporter::metrics::network::capture_packets;

#[test]
fn test_packet_processing() {
    // Create a dummy network interface for testing
    let interfaces = datalink::interfaces();
    let test_interface = interfaces
        .iter()
        .find(|&iface| iface.name == "lo") // Loopback interface
        .expect("No loopback interface found");

    let (mut tx, mut rx) = match datalink::channel(test_interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e),
    };

    let packet_received = Arc::new(Mutex::new(false));
    let packet_received_clone = packet_received.clone();
    let handler: Arc<Mutex<dyn Fn(EthernetPacket) + Send + Sync>> = Arc::new(Mutex::new(move |packet: EthernetPacket| {
        let mut received = packet_received_clone.lock().unwrap();
        *received = true;
    }));

    thread::spawn(move || {
        capture_packets(handler);
    });

    // Simulate sending a packet
    let packet = [0u8; 64]; // Dummy packet
    tx.send_to(&packet, None).expect("Failed to send packet");

    // Check if the packet was received
    thread::sleep(Duration::from_secs(1)); // Give time for packet to be processed
    let received = packet_received.lock().unwrap();
    assert!(*received, "No packet received");
}
