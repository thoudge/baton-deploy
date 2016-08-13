#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate amqp;
extern crate env_logger;

extern crate serde;
extern crate serde_json;

use amqp::{Session, Options, Table, Basic, protocol, Channel, ConsumerCallBackFn};
use std::default::Default;
use std::str;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Message {
    #[serde(default,rename="type")]
    msg_type: String,
    sha: Option<String>,
    branch: Option<String>,
}

fn process(message: Message, routing_key: &str) {
    println!("Processing message {:?} with key: {:?}",
             message,
             routing_key);
}

fn consumer_function(_: &mut Channel,
                     deliver: protocol::basic::Deliver,
                     headers: protocol::basic::BasicProperties,
                     body: Vec<u8>) {
    let routing_key = &deliver.routing_key;
    if let Ok(payload) = str::from_utf8(&body) {
        match serde_json::from_str::<Message>(payload) {
            Ok(message) => process(message, routing_key),
            Err(err) => println!("Ignoring invalid payload: {:?}", payload),
        };
    };
}

fn main() {
    env_logger::init().unwrap();
    let mut session = Session::new(Options { vhost: "/", ..Default::default() })
        .ok()
        .expect("Can't create session");
    let mut channel = session.open_channel(1).ok().expect("Error opening channel 1");

    channel.exchange_declare("exchange_in",
                             "direct",
                             false,
                             false,
                             false,
                             false,
                             false,
                             Table::new());
    channel.queue_declare("", false, false, true, false, false, Table::new());
    channel.queue_bind("", "exchange_in", "test.development", false, Table::new());
    channel.queue_bind("",
                       "exchange_in",
                       "test.development.localhost",
                       false,
                       Table::new());
    channel.basic_consume(consumer_function as ConsumerCallBackFn,
                          "",
                          "",
                          false,
                          false,
                          false,
                          false,
                          Table::new());

    channel.start_consuming();

    channel.close(200, "Bye").unwrap();
    session.close(200, "Good Bye");
}
