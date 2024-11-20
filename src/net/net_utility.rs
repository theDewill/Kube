use serde::{Deserialize, Serialize};
extern crate std;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};

// fn tst<K : `a> (x : &mut K) where K : Debug + Derive{}

struct Message {
    msg_typ: String,
    data: String, //this will be custmz later
    raw : Vec<u8>
}

struct Node<'a> {
    name_uuid: String,
    version : &'a String,
}

pub struct Net {
    node: &'a mut Node,
    name: String,
}

pub fn discoverer() {
    println!("Discovering for Nodes...")
}

pub fn Logger() -> () {
    println!("Net utility start-----")
}

impl<`a> Node {
    fn new(NName: String, port: String) -> &Self {
        &Node { name_uuid: NName };
    }

    fn start_engine(&self) -> () {}
}
