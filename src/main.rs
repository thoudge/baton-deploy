#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate amqp;
extern crate env_logger;

extern crate serde;
extern crate serde_json;

extern crate hyper;
extern crate sodiumoxide;
extern crate rustc_serialize;

use amqp::{Session, Options, Table, Basic, protocol, Channel, ConsumerCallBackFn};
use std::default::Default;
use std::str;
use hyper::client::Client;
use std::fs::File;
use std::io::{Write, Read};
use rustc_serialize::hex::ToHex;
use sodiumoxide::crypto::hash::sha256;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Message {
    #[serde(default,rename="type")]
    msg_type: String,
    sha: Option<String>,
    branch: Option<String>,
    url: Option<String>,
    checksum_url: Option<String>,
}

fn start(routing_key: &str) {
    println!("Starting {:?}", routing_key);
}

fn stop(routing_key: &str) {
    println!("Stopping {:?}", routing_key);
}

fn deploy(routing_key: &str, url: &str, checksum_url: &str) {
    let client = Client::new();
    let mut result = client.get(url).send().unwrap();

    let mut buffer = File::create("foo.txt").unwrap();
    let mut response = String::new();

    result.read_to_string(&mut response);

    let digest = sha256::hash(response.as_ref());

    println!("got a digest of {:?}", digest.as_ref().to_hex());
    buffer.write(response.as_ref());

    println!("Deploying {:?} from {}, checksum {}",
             routing_key,
             url,
             checksum_url);
}

fn process(message: Message, routing_key: &str) {
    match message.msg_type.as_ref() {
        "deploy" => {
            deploy(routing_key,
                   message.url.unwrap().as_ref(),
                   message.checksum_url.unwrap().as_ref())
        }
        "start" => start(routing_key),
        "stop" => stop(routing_key),
        _ => {
            println!("Processing message {:?} with key: {:?}",
                     message,
                     routing_key)
        }
    };
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
