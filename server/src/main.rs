use rumqttc::{Client, MqttOptions, QoS};
use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::{thread, time::Duration};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:22").unwrap();

    let args: Vec<String> = env::args().collect();
    // TODO: check args
    let connection_id = &args[1];
    let tx_topic = format!("{}{}", connection_id, "/crx");
    let rx_topic = format!("{}{}", connection_id, "/ctx");

    let mut mqttoptions = MqttOptions::new("rabbitholeserver", "broker.hivemq.com", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(60));

    let (mut client, mut connection) = Client::new(mqttoptions, 10);
    client
        .subscribe(rx_topic.as_str(), QoS::AtMostOnce)
        .unwrap();

    let mut stream_clone = stream.try_clone().expect("clone failed...");
    thread::spawn(move || loop {
        let mut full_msg: [u8; 1] = [0; 1];
        let size = stream_clone.read(&mut full_msg).unwrap();
        if size == 0 {
            continue;
        }
        println!("Read: {:?} {}", full_msg, size);
    });

    // Iterate to poll the eventloop for connection progress
    for (i, notification) in connection.iter().enumerate() {
        //println!("Notification = {:?}", notification);
        match notification {
            Ok(ok_result) => {
                if let rumqttc::Event::Incoming(incomming_request) = ok_result {
                    if let rumqttc::Packet::Publish(publish_packet) = incomming_request {
                        //println!("Notification publish_packet = {:?}", publish_packet.payload);
                        stream.write(&publish_packet.payload);
                    }
                }
            }
            Err(_) => todo!(),
        };
    }
}
