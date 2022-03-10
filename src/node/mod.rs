use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

use crate::blockchain::{Block, Blockchain};

pub struct Node {
    pub blockchain: Blockchain,
    pub current_transactions: Vec<Transaction>,
    pub nodes: Vec<String>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            blockchain: Blockchain::new(),
            current_transactions: Vec::new(),
            nodes: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        // TODO: build TCP server

        // TODO: build mining thread

    }

}
