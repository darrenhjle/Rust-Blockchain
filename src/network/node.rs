use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::blockchain::Blockchain;
use super::message::Message;

pub struct Node {
    pub address: String, 
    pub peers: Vec<String>,
    pub blockchain: Arc<Mutex<Blockchain>>, //shared chain
}

impl Node {
    pub fn new(address: String, peers: Vec<String>)->Self {
        Node {
            address,
            peers,
            blockchain: Arc::new(Mutex::new(Blockchain::load("chain.json"))),
        }
    }

    //TODO: listen fn, send to peer fn, broadcast fn
}

// TODO: Handle incoming connection fn