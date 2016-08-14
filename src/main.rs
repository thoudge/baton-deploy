extern crate env_logger;
extern crate amqp;
extern crate rustc_serialize;
extern crate toml;

use std::collections::HashMap;
use std::str;

use amqp::{Session, Table, Basic, protocol, Channel, Consumer};
use rustc_serialize::json;

struct DeployConsumer {
    app: String
}

impl Consumer for DeployConsumer {
    fn handle_delivery(&mut self, _: &mut Channel, _: protocol::basic::Deliver,
                       _: protocol::basic::BasicProperties, body: Vec<u8>) {
        if let Ok(payload) = str::from_utf8(&body) {
            if let Ok(message) = json::decode::<HashMap<String, String>>(payload) {
                println!("Got message {:?}", message);
            } else {
                println!("Ignoring payload {:?}", payload);
            }
        };
    }
}

fn main() {
    let toml = r#"
        amqp_url = "amqp://localhost//"
        fqdn = "localhost"
        environment = "development"
        apps = ["test"]
    "#;

    let configuration = toml::Parser::new(toml).parse().unwrap();
    let amqp_url = configuration.get("amqp_url").and_then(|value| value.as_str()).unwrap_or("amqp://localhost//");
    let environment = configuration.get("environment").and_then(|value| value.as_str()).unwrap_or("development");
    let fqdn = configuration.get("fqdn").and_then(|value| value.as_str()).unwrap_or("localhost");

    let apps = vec!["test"];

    env_logger::init().expect("Can't initialize logger");

    let mut session = Session::open_url(amqp_url).expect("Can't create AMQP session");
    let mut channel = session.open_channel(1).expect("Error opening AMQP channel 1");

    for app in apps {
        let environment_key = format!("{}.{}", app, environment);
        let instance_key = format!("{}.{}.{}", app, environment, fqdn);

        channel.exchange_declare("baton_deploy_in", "direct", false, false, false, false, false,
                                 Table::new())
            .and_then(|_| channel.queue_declare(app, false, false, true, false, false, Table::new()))
            .and_then(|_| channel.queue_bind(app, "baton_deploy_in", &environment_key, false, Table::new()))
            .and_then(|_| channel.queue_bind(app, "baton_deploy_in", &instance_key, false, Table::new()))
            .and_then(|_| channel.basic_consume(DeployConsumer { app: app.to_string() }, app, "", false, false, false,
                              false, Table::new()))
            .expect("Could not set up exchange, queues and consumers");
    }

    println!("Listening for messages!");
    channel.start_consuming();

    channel.close(200, "Closing channel").expect("Couldn't close AMQP channel 1");
    session.close(200, "Closing session");
}
