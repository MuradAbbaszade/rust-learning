use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=10);
    loop{
    let mut guess = String::new();
    println!("Please guess the secret number");
    match io::stdin().read_line(&mut guess){
        Ok(input)=>{
            let guess:u32 = guess.trim().parse().expect("Couldn't parse input");
            match compare_values(secret_number,guess){
                true=>break,
                false=>continue,
            }
        },
        Err(err)=>println!("An error occured {}",err),
    }
    }
}
fn compare_values(secret_number:u32,guess_number:u32)->bool{
    match guess_number.cmp(&secret_number){
        Ordering::Greater=>{
            println!("Too big");
            false
        },
        Ordering::Equal=>{
            println!("Win");
            true
        },
        Ordering::Less=>{
            println!("Too less");
            false
        },
    }
}
