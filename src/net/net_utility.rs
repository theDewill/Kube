use serde::{Deserialize, Serialize};
extern crate std;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};

struct Message {
    msg_typ: String,
    data: String, //this will be custmz later
}

struct Node {
    name_uuid: String,
}

pub struct Net {
    node: &Node,
    name: String,
}

pub fn discoverer() {
    println!("Discovering for Nodes...")
}

pub fn Logger() -> () {
    println!("Net utility start-----")
}

impl Node {
    fn new(NName: String, port: String) -> &Self {
        &Node { name_uuid: NName };
    }

    fn start_engine(&self) -> () {}
}
