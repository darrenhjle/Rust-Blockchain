use rust_blockchain::blockchain::Blockchain;
fn main() {
    let mut bc = Blockchain::load("chain.json");
    println!("Chain length: {}", bc.chain.len());
    bc.add_block(vec![]);
    bc.save("chain.json");
    println!("Saved. New length: {}", bc.chain.len());
}