use blockchain::Block;
fn main() {
    let mut chain = blockchain::init_chain();
    let second_block = Block::new(blockchain::proof_of_work(1),chain[0].hash(),&chain);
    chain.push(second_block);
    let is_valid = blockchain::chain_is_valid(chain.iter());
    blockchain::mine_block(&mut chain);
    println!("{}",is_valid);
    println!("{:?}",chain);
}
