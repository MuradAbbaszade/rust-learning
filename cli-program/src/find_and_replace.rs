use std::env;
use std::fs;
use regex::Regex;

fn print_help(){
    println!("You can find and replace strings from file with this cli-program.Please give 4 arguments <input file> <output file> <string1> <string2>");
}
fn check_args()->Vec<String>{
    let args:Vec<String> = env::args().skip(1).collect();
    if args.len()!=4 {
        print_help();
        println!("Expected count of args 4 , got {}",args.len());
        std::process::exit(1);
    }
    args
}
fn find_and_replace(data:&String,rep:&String,target:&String)->Result<String,regex::Error>{
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(data,rep).to_string())
}
fn read_and_write(args:&Vec<String>){
    // read
    let input_file=&args[0];
    let mut data = match fs::read_to_string(input_file){
        Ok(f)=>f,
        Err(e)=>{
            println!("An error occured : {:?}",e);
            std::process::exit(1);
        }
    };

    // find and replace
    let replace_text = &args[3];
    let target_text = &args[2];
    data = match find_and_replace(&data,replace_text,target_text){
        Ok(s)=>s,
        Err(e)=>{
            println!("An error occured {:?}",e);
            std::process::exit(1);
        }
    };

    // write
    let output_file=&args[1];
    match fs::write(output_file,&data){
        Ok(_)=>{},
        Err(e)=>{
            println!("An error occured : {:?}",e);
            std::process::exit(1);
        }
    };

}
pub fn run(){
    let args:Vec<String> = check_args();
    read_and_write(&args);
}