use ipnet::datalink::Channel::Ethernet;
use ipnet::datalink;
use ipnet::packet::ethernet::EthernetPacket;
use ipnet::packet::Packet;
use ipnet::packet::FromPacket;
use core::panic;
use std::thread;

fn main() {
    let interfaces = datalink::interfaces();
    // Creates empty mutable vector of data interfaces
    let mut handles = vec![];

    for interface in interfaces {
        let handle = thread::spawn(move || {
            capture_packets(interface);
        });
        handles.push(handle);
    }
    // Waits for threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}
//Extracts tx and recieves rx for ethernet frames and throws error if an unsupported channel type is transmitted
fn capture_packets(interface: datalink::NetworkInterface) {
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type: {}", &interface),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e),
    };

    println!("Start reading packet: ");
    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                    println!("New packet on {}", interface.name);
                    println!(
                        "{} => {}: {}",
                        //Pulling Packet information
                        ethernet_packet.get_destination(),
                        ethernet_packet.get_source(),
                        ethernet_packet.get_ethertype()
                    );
                    let packet = ethernet_packet.packet();
                    let payload = ethernet_packet.payload();
                    let from_packet = ethernet_packet.from_packet();
                    println!("packet: {:?}", packet);
                    println!("payload: {:?}", payload);
                    println!("from_packet: {:?}", from_packet);
                    println!("---");
                }
            }
            //Error Messages
            Err(e) => {
                panic!("An error has occurred while reading: {}", e);
            }
        };
    }
}
