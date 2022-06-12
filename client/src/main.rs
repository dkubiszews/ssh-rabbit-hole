use rumqttc::{Client, MqttOptions, QoS};
use std::env;
use std::{thread, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();
    // TODO: check args
    let connection_id = &args[1];
    let tx_topic = format!("{}{}", connection_id, "/ctx");
    let rx_topic = format!("{}{}", connection_id, "/crx");

    let mut mqttoptions = MqttOptions::new("rabbitholeclient", "broker.hivemq.com", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(60));

    let (mut client, mut connection) = Client::new(mqttoptions, 10);
    client
        .subscribe(rx_topic.as_str(), QoS::AtMostOnce)
        .unwrap();
    thread::spawn(move || {
        for i in 0..10 {
            println!("send");
            match client.publish(
                tx_topic.as_str(),
                QoS::AtLeastOnce,
                false,
                vec![i; i as usize],
            ) {
                Ok(_) => println!("send ok"),
                Err(_) => println!("send err"),
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Iterate to poll the eventloop for connection progress
    for (i, notification) in connection.iter().enumerate() {
        println!("Notification = {:?}", notification);
    }
}
