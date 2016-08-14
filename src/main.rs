#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate env_logger;
extern crate amqp;
extern crate serde;
extern crate serde_json;

use amqp::{Session, Table, Basic, protocol, Channel, Consumer};
use std::str;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Message {
    #[serde(default,rename="type")]
    msg_type: String,
    sha: Option<String>,
    branch: Option<String>,
    url: Option<String>,
    checksum_url: Option<String>,
}

fn process(app: &str, message: Message) {
    match message.msg_type.as_ref() {
        "deploy" => println!("Deploying {:?}: {:?}", app, message),
        "start" => println!("Starting {:?}: {:?}", app, message),
        "stop" => println!("Stopping {:?}: {:?}", app, message),
        _ => println!("Unsupported message {:?}: {:?}", app, message),
    };
}

struct DeployConsumer {
    app: String
}

impl Consumer for DeployConsumer {
    fn handle_delivery(&mut self, _: &mut Channel, _: protocol::basic::Deliver,
                       _: protocol::basic::BasicProperties, body: Vec<u8>) {
        if let Ok(payload) = str::from_utf8(&body) {
            match serde_json::from_str::<Message>(payload) {
                Ok(message) => process(self.app.as_ref(), message),
                Err(_) => println!("Ignoring invalid payload: {:?}", payload),
            };
        };
    }
}

fn main() {
    let apps = vec!["test"];
    let environment = "development";
    let fqdn = "localhost";

    env_logger::init().expect("Can't initialize logger");

    let mut session = Session::open_url("amqp://localhost//").expect("Can't create AMQP session");
    let mut channel = session.open_channel(1).expect("Error opening AMQP channel 1");

    for app in &apps {
        channel.exchange_declare("exchange_in", "direct", false, false, false, false, false,
                                 Table::new())
            .and_then(|_| channel.queue_declare("", false, false, true, false, false, Table::new()))
            .and_then(|_| channel.queue_bind("", "exchange_in", format!("{}.{}", app, environment).as_ref(), false, Table::new()))
            .and_then(|_| channel.queue_bind("", "exchange_in", format!("{}.{}.{}", app, environment, fqdn).as_ref(), false, Table::new()))
            .and_then(|_| channel.basic_consume(DeployConsumer { app: app.to_string() }, "", "", false, false, false,
                              false, Table::new()))
            .expect("Could not set up exchange, queues and consumers");
    }

    println!("Listening for messages!");
    channel.start_consuming();

    channel.close(200, "Closing channel").expect("Couldn't close AMQP channel 1");
    session.close(200, "Closing session");
}
