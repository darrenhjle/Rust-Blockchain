use clap::{Parser, Subcommand};
use rust_blockchain::{
    blockchain::Blockchain,
    wallet::Wallet,
    transaction::Transaction,
    mempool::Mempool,
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
        #[arg(long)] from_privkey: String,
        #[arg(long)] to: String,
        #[arg(long)] amount: f64,
    },
    /// Print the current chain
    Print,
}
fn main() {
    let cli = Cli::parse();
    let chain_path = "chain.json";
    let mut bc = Blockchain::load(chain_path);
    let mut mempool = Mempool::new();

    match cli.command {
        Commands::Mine => {
            let txs = mempool.drain();
            println!("Mining block with {} transactions...", txs.len());
            bc.add_block(txs);
            bc.save(chain_path);
            println!("Block mined! Chain length: {}", bc.chain.len());
        }
        Commands::Wallet => {
            let w = Wallet::new();
            println!("New wallet address: {}", w.address());
        }
        Commands::Send { from_privkey: _, to, amount } => {
            //simplified does not load private key
            let sender_wallet = Wallet::new();
            let mut tx = Transaction::new(sender_wallet.address(), to, amount);
            tx.sign(&sender_wallet);
            if mempool.add(tx) {
                println!("Transaction added to mempool");
            } else {
                println!("Transaction invalid — rejected");
            }
        }
        Commands::Print => {
            for block in &bc.chain {
                println!("#{} | hash: {}... | data: {}", block.index, &block.hash[..8], &block.data[..block.data.len().min(40)]);
            }
        }
    }
}