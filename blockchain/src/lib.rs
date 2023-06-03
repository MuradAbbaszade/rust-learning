use chrono::{DateTime, Local};
use ring::digest::{Context, SHA256};
use hex;

#[derive(Clone,Debug)]
pub struct Block{
    pub index:usize,
    pub proof:i32,
    pub previous_hash:String,
    pub timestamp:DateTime<Local>,
}

impl Block{
    pub fn new(proof:i32,previous_hash:String,chain:&Vec<Block>)->Block{
        Block{
            index:chain.len(),
            proof:proof,
            previous_hash:previous_hash,
            timestamp:Local::now()
        }
    }
    pub fn hash(&self) -> String {
        let mut context = Context::new(&SHA256);
        context.update(self.index.to_string().as_bytes());
        context.update(self.proof.to_string().as_bytes());
        context.update(self.previous_hash.as_bytes());
        context.update(self.timestamp.to_string().as_bytes());
        let hash = context.finish();
        hex::encode(hash.as_ref())
    }
}
pub fn init_chain()->Vec<Block>{
    let mut blockchain = Vec::<Block>::new();
    let first_block = Block::new(1,"0".to_string(),&blockchain);
    blockchain.push(first_block);
    blockchain
}
pub fn get_previous_block(chain:&Vec<Block>)->Option<Block>{
        if chain.is_empty() {
            return None;
        }
        Some(chain[chain.len() - 1].clone())
}
pub fn proof_of_work(previous_proof:i32)->i32{
    let mut proof:i32=1;
    let mut proof_of_work = false;
    while !proof_of_work {
        let mut context = Context::new(&SHA256);
        let data:i32 = proof.pow(2)-previous_proof.pow(2);
        context.update(&data.to_string().as_bytes());
        let hash = hex::encode(context.finish());
        if &hash[..4]=="0000" {
            proof_of_work=true;
        }
        else{
            proof+=1;
        }
    }
    proof
}
pub fn chain_is_valid<'a>(mut chain: impl Iterator<Item = &'a Block>)->bool{
    while let Some(prev_block) = chain.next() {
        let current_block = chain.next().unwrap();
        if current_block.previous_hash != prev_block.hash() {
            return false;
        }
        let mut context = Context::new(&SHA256);
        let data:i32 = current_block.proof.pow(2)-prev_block.proof.pow(2);
        context.update(&data.to_string().as_bytes());
        let proof = hex::encode(context.finish());
        if &proof[..4]!="0000"{
            return false;
        }
    }
    return true;
}
pub fn mine_block(chain : &mut Vec<Block>)->Block{
    let prev_block = get_previous_block(chain).unwrap();
    let prev_block_hash = prev_block.hash();
    let proof = proof_of_work(prev_block.proof);
    let new_block = Block::new(proof,prev_block_hash,chain);
    chain.push(new_block.clone());
    new_block
}