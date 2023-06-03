use std::env;
use std::process;
use ioproject::Config;
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|e|{
        println!("An error occured : {}",e);
        process::exit(1);
    });
    if let Err(e) = ioproject::run(config){
        println!("An error occured : {}",e);
        process::exit(1);
    };
    // ioproject::run(config).unwrap_or_else(|e|{
    //     println!("An error occured : {}",e);
    //     process::exit(1);
    // });
}