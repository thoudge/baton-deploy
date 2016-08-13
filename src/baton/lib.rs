#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate amqp;
extern crate env_logger;

extern crate serde;
extern crate serde_json;

extern crate hyper;
extern crate sodiumoxide;
extern crate rustc_serialize;

pub mod message;
pub mod routing_key;
