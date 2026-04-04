use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::blockchain::Blockchain;
use crate::mempool::Mempool;
use super::message::Message;

pub struct Node {
    pub address: String,
    pub peers: Vec<String>,
    pub blockchain: Arc<Mutex<Blockchain>>,
    pub mempool: Arc<Mutex<Mempool>>,
}

impl Node {
    pub fn new(address: String, peers: Vec<String>) -> Self {
        Node {
            address,
            peers,
            blockchain: Arc::new(Mutex::new(Blockchain::load("chain.json"))),
            mempool: Arc::new(Mutex::new(Mempool::load("mempool.json"))),
        }
    }

    pub async fn listen(&self) {
        let listener = TcpListener::bind(&self.address).await.unwrap();
        println!("Node listening on {}", self.address);

        let blockchain = Arc::clone(&self.blockchain);
        let mempool = Arc::clone(&self.mempool);
        // Wrap peers in Arc so each spawned task can share it without cloning the Vec
        let peers = Arc::new(self.peers.clone());

        loop {
            let (socket, addr) = listener.accept().await.unwrap();
            println!("New connection from {}", addr);

            let blockchain = Arc::clone(&blockchain);
            let mempool = Arc::clone(&mempool);
            let peers = Arc::clone(&peers);

            tokio::spawn(async move {
                handle_connection(socket, blockchain, mempool, peers).await;
            });
        }
    }

    pub async fn send_to_peer(&self, peer: &str, message: &Message) {
        match TcpStream::connect(peer).await {
            Ok(mut stream) => {
                let json = serde_json::to_string(message).unwrap();
                let bytes = json.as_bytes();
                let len = bytes.len() as u32;
                stream.write_all(&len.to_be_bytes()).await.unwrap();
                stream.write_all(bytes).await.unwrap();
            }
            Err(e) => println!("Failed to connect to {}: {}", peer, e),
        }
    }

    pub async fn broadcast(&self, message: &Message) {
        for peer in &self.peers {
            self.send_to_peer(peer, message).await;
        }
    }

    pub async fn sync_with_peers(&self) {
        for peer in &self.peers {
            self.request_chain_from_peer(peer).await;
        }
    }

    // Sends RequestChain and reads the ResponseChain back on the same connection
    async fn request_chain_from_peer(&self, peer: &str) {
        let mut stream = match TcpStream::connect(peer).await {
            Ok(s) => s,
            Err(e) => { println!("Could not connect to peer {}: {}", peer, e); return; }
        };

        // Send the request
        let json = serde_json::to_string(&Message::RequestChain).unwrap();
        let bytes = json.as_bytes();
        let len = bytes.len() as u32;
        if stream.write_all(&len.to_be_bytes()).await.is_err() { return; }
        if stream.write_all(bytes).await.is_err() { return; }

        // Read the response on the same connection
        let mut len_bytes = [0u8; 4];
        if stream.read_exact(&mut len_bytes).await.is_err() { return; }
        let resp_len = u32::from_be_bytes(len_bytes) as usize;
        let mut buf = vec![0u8; resp_len];
        if stream.read_exact(&mut buf).await.is_err() { return; }

        if let Ok(Message::ResponseChain { chain }) = serde_json::from_slice(&buf) {
            let mut bc = self.blockchain.lock().await;
            if chain.len() > bc.chain.len() {
                let candidate = Blockchain { chain: chain.clone(), difficulty: bc.difficulty };
                if candidate.is_valid() {
                    println!("Synced chain from {} ({} blocks)", peer, chain.len());
                    bc.chain = chain;
                    bc.save("chain.json");
                }
            }
        }
    }
}

// Sends a message to all known peers; skips any that are offline
async fn broadcast_to_peers(peers: &[String], message: &Message) {
    for peer in peers {
        match TcpStream::connect(peer).await {
            Ok(mut stream) => {
                let json = serde_json::to_string(message).unwrap();
                let bytes = json.as_bytes();
                let len = bytes.len() as u32;
                let _ = stream.write_all(&len.to_be_bytes()).await;
                let _ = stream.write_all(bytes).await;
            }
            Err(_) => {} // peer offline, skip silently
        }
    }
}

async fn handle_connection(
    mut socket: TcpStream,
    blockchain: Arc<Mutex<Blockchain>>,
    mempool: Arc<Mutex<Mempool>>,
    peers: Arc<Vec<String>>,
) {
    loop {
        let mut len_bytes = [0u8; 4];
        if socket.read_exact(&mut len_bytes).await.is_err() {
            break;
        }
        let len = u32::from_be_bytes(len_bytes) as usize;

        let mut buf = vec![0u8; len];
        if socket.read_exact(&mut buf).await.is_err() {
            break;
        }

        let message: Message = match serde_json::from_slice(&buf) {
            Ok(m) => m,
            Err(_) => break,
        };

        match message {
            Message::RequestChain => {
                // Clone chain and drop the lock before writing to socket
                let chain = {
                    let bc = blockchain.lock().await;
                    bc.chain.clone()
                };
                let response = Message::ResponseChain { chain };
                let json = serde_json::to_string(&response).unwrap();
                let bytes = json.as_bytes();
                let len = bytes.len() as u32;
                if socket.write_all(&len.to_be_bytes()).await.is_err() { break; }
                if socket.write_all(bytes).await.is_err() { break; }
            }

            Message::NewBlock { block } => {
                // Lock only for validation + insert, then drop before broadcasting
                let should_broadcast = {
                    let mut bc = blockchain.lock().await;
                    let last = bc.last_block();
                    // Fix: also verify the hash satisfies PoW difficulty
                    let pow_ok = block.hash.starts_with(&"0".repeat(bc.difficulty));
                    if block.prev_hash == last.hash && block.is_valid() && pow_ok {
                        println!("Received valid new block #{}", block.index);
                        bc.chain.push(block.clone());
                        bc.save("chain.json");
                        true
                    } else {
                        println!("Rejected invalid block #{}", block.index);
                        false
                    }
                };
                // Re-broadcast so the block propagates to the rest of the network
                if should_broadcast {
                    broadcast_to_peers(&peers, &Message::NewBlock { block }).await;
                }
            }

            Message::ResponseChain { chain } => {
                let mut bc = blockchain.lock().await;
                if chain.len() > bc.chain.len() {
                    // Fix: validate the incoming chain before replacing ours
                    let candidate = Blockchain { chain: chain.clone(), difficulty: bc.difficulty };
                    if candidate.is_valid() {
                        println!("Replacing chain with longer valid one from peer ({} blocks)", chain.len());
                        bc.chain = chain;
                        bc.save("chain.json");
                    } else {
                        println!("Rejected longer chain from peer: failed validation");
                    }
                }
            }

            Message::NewTransaction { transaction } => {
                // Deduplicate by signature, then add and re-broadcast
                let added = {
                    let mut mp = mempool.lock().await;
                    let is_duplicate = mp.pending.iter().any(|t| t.signature == transaction.signature);
                    if !is_duplicate && mp.add(transaction.clone()) {
                        mp.save("mempool.json");
                        true
                    } else {
                        false
                    }
                };
                if added {
                    println!("Added transaction to mempool from peer");
                    // Re-broadcast so the tx propagates to the rest of the network
                    broadcast_to_peers(&peers, &Message::NewTransaction { transaction }).await;
                } else {
                    println!("Dropped duplicate or invalid transaction from peer");
                }
            }

            Message::Ping { block_length } => {
                println!("Peer has block height {}", block_length);
            }
        }
    }
}
