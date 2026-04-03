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
    // Mine a new block from pending transactions
    Mine,
    // Create a new wallet and print its address
    Wallet,
    // Send coins (creates and signs a transaction)
    Send {
        #[arg(long)] to: String,
        #[arg(long)] amount: f64,
    },
    // Print the current chain
    Print,
}
fn main() {
    let cli = Cli::parse();
    let chain_path = "chain.json";
    let mempool_path = "mempool.json";
    let mut bc = Blockchain::load(chain_path);
    let mut mempool = Mempool::load(mempool_path);

    match cli.command {
        Commands::Mine => {
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
            let sender_wallet = Wallet::load("wallet.json"); // load saved wallet
            let mut tx = Transaction::new(sender_wallet.address(), to, amount);
            tx.sign(&sender_wallet);
            if mempool.add(tx) {
                mempool.save(mempool_path);
                println!("Transaction sent from {}", &sender_wallet.address()[..8]);
            } else {
                println!("Transaction invalid — rejected");
            }
        }
        Commands::Print => {
            for block in &bc.chain {
                println!("#{} | hash: {}... | merkle_root: {}", block.index, &block.hash[..8], &block.merkle_root[..8]);
            }
        }
    }
}

