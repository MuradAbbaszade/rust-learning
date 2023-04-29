use std::fs::File;
use std::io::Read;
use std::env;

fn main() {
    let args:Vec<String> = env::args().skip(1).collect();
    if args.len()!=2 {
        println!("Please enter file path and word : example.txt example");
    }
    let mut file = File::open(&args[0]).expect("File not found !");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("An error occured");
    
    let mut word_count = 0;
    for word in file_content.split_whitespace(){
        if word==&args[1]{
            word_count+=1;
        } 
    }
    println!("{}",word_count);
}
