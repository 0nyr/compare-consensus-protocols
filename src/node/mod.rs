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
        let mut node = Node {};
        node.blockchain = new Blockchain();
        node 
    }

    fn handle_client(mut stream: TcpStream) {
        let mut buffer = [0 as u8; 512];
        stream.read(&mut buffer).unwrap();
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        stream.shutdown(Shutdown::Both).unwrap();
    
        while match stream.read(&mut data) {
            Ok(size) => {
                // echo everything!
                stream.write(&data[0..size]).unwrap();
                true
            },
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}", 
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        } {}
    }
    
    
    fn init() {
        // connect to the network (other nodes)
        //    + if connection, retreive the chain
        //    + if no connection, create a new chain
        
    
        // allow connections from other nodes
        let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
        println!("Server listening on port 3333");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || {
                        handle_client(stream);
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        // close the socket server
        drop(listener);
    }
}
