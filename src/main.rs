use clap::{Parser, Subcommand};
use rust_blockchain::{
    blockchain::Blockchain,
    wallet::Wallet,
    transaction::Transaction,
    mempool::Mempool,
    network::node::Node,
    network::message::Message,
};

#[derive(Parser)]
#[command(name = "blockchain", about = "A simple Rust blockchain")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Mine a new block from pending transactions
    Mine,
    /// Create a new wallet and print its address
    Wallet,
    /// Send coins (creates and signs a transaction)
    Send {
        #[arg(long)] to: String,
        #[arg(long)] amount: f64,
    },
    /// Print the current chain
    Print,
    /// Start a P2P node and listen for incoming connections
    Start {
        /// Address to listen on, e.g. 127.0.0.1:8000
        #[arg(long)] address: String,
        /// Known peer addresses, e.g. --peers 127.0.0.1:8001 --peers 127.0.0.1:8002
        #[arg(long)] peers: Vec<String>,
    },

    BroadcastTx {
        #[arg(long)] to: String,
        #[arg(long)] amount: f64,
        #[arg(long)] node: String,
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let chain_path = "chain.json";
    let mempool_path = "mempool.json";

    match cli.command {
        Commands::Mine => {
            let mut bc = Blockchain::load(chain_path);
            let mut mempool = Mempool::load(mempool_path);
            let txs = mempool.drain();
            println!("Mining block with {} transactions...", txs.len());
            bc.add_block(txs);
            bc.save(chain_path);
            mempool.save(mempool_path);
            println!("Block mined! Chain length: {}", bc.chain.len());
        }
        Commands::Wallet => {
            let w = Wallet::new();
            w.save("wallet.json");
            println!("Address:     {}", w.address());
            println!("Wallet saved to wallet.json");
        }
        Commands::Send { to, amount } => {
            let mut mempool = Mempool::load(mempool_path);
            let sender_wallet = Wallet::load("wallet.json");
            let mut tx = Transaction::new(sender_wallet.address(), to, amount);
            tx.sign(&sender_wallet);
            if mempool.add(tx) {
                mempool.save(mempool_path);
                println!("Transaction added from {}", &sender_wallet.address()[..8]);
            } else {
                println!("Transaction invalid — rejected");
            }
        }
        Commands::Print => {
            let bc = Blockchain::load(chain_path);
            for block in &bc.chain {
                let hash_preview = &block.hash[..block.hash.len().min(8)];
                let root_preview = &block.merkle_root[..block.merkle_root.len().min(8)];
                println!(
                    "#{} | hash: {}... | merkle_root: {}",
                    block.index,
                    hash_preview,
                    root_preview
                );
            }
        }
        Commands::Start { address, peers } => {
            let node = Node::new(address, peers);
            node.sync_with_peers().await;
            node.listen().await; // runs forever, ctrl+c to stop
        }
        Commands::BroadcastTx { to, amount, node } => {
            let sender_wallet = Wallet::load("wallet.json");
            let mut tx = Transaction::new(sender_wallet.address(), to, amount);
            tx.sign(&sender_wallet);

            let tmp_node = Node::new("127.0.0.1:0".to_string(), vec![]);
            tmp_node.send_to_peer(&node, &Message::NewTransaction {transaction: tx}).await;
            println!("Transaction sent to node {}", node);
        }
    }
}
