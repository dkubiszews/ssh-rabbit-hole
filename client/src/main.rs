use rumqttc::{Client, MqttOptions, QoS};
use std::env;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::{thread, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();
    // TODO: check args
    let connection_id = &args[1];
    let tx_topic = format!("{}{}", connection_id, "/ctx");
    let rx_topic = format!("{}{}", connection_id, "/crx");

    let listener = TcpListener::bind("127.0.0.1:5022").unwrap();
    for stream in listener.incoming() {
        println!("Open connection");
        let mut stream = stream.unwrap();

        let mut mqttoptions = MqttOptions::new("rabbitholeclient", "broker.hivemq.com", 1883);
        mqttoptions.set_keep_alive(Duration::from_secs(60));

        let (mut client, mut connection) = Client::new(mqttoptions, 10);
        client
            .subscribe(rx_topic.as_str(), QoS::AtMostOnce)
            .unwrap();
        let mut stream_clone = stream.try_clone().expect("clone failed...");
        let tx_topic_clone = tx_topic.clone();
        thread::spawn(move || loop {
            let mut full_msg: [u8; 1] = [0; 1];
            stream_clone.read(&mut full_msg).unwrap();
            println!("Read: {}", full_msg.len());
            client.publish(tx_topic_clone.as_str(), QoS::AtLeastOnce, false, full_msg);
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
}
