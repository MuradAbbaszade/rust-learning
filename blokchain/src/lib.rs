use chrono::{DateTime, Local};
use ring::digest::{Context, SHA256};
use hex;

#[derive(Clone,Debug)]
pub struct Block{
    pub proof:i32,
    pub previous_hash:String,
    pub timestamp:DateTime<Local>,
    pub index:usize
}

impl Block{
    pub fn new(proof:i32,previous_hash:String,chain:&Vec<Block>)->Block{
        Block{
            proof:proof,
            previous_hash:previous_hash,
            index:chain.len(),
            timestamp:Local::now()
        }
    }
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
        context.update(&data.to_be_bytes());
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