use std::error::Error;
use std::fs;
use std::env;
pub struct Config{
    pub query:String,
    pub file_path:String,
    pub ignore_case:bool
}
impl Config{
    pub fn build(mut args:impl Iterator<Item=String>)->Result<Config,&'static str>{
        args.next();
        let file_path = match args.next(){
            Some(arg)=>arg,
            None=>{
                return Err("Not enough arguments");
            }
        };
        let query = match args.next(){
            Some(arg)=>arg,
            None=>{
                return Err("Not enough arguments");
            }
        };
        Ok(Config{
            query:query,
            file_path:file_path,
            ignore_case:env::var("IGNORE-CASE").is_ok()
        })
    }
}
pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let text = fs::read_to_string(config.file_path)?;
    let result = if config.ignore_case{
        search_case_insensitive(&config.query,&text)
    }
    else{
        search_case_sensitive(&config.query,&text)
    };
    for line in result{
        println!("{}",line);
    }
    Ok(())
}
fn search_case_sensitive<'a>(query:&String,context:&'a String)->Vec<&'a str>{
    let mut result = Vec::new();
    for line in context.lines(){
        if line.contains(query){
            result.push(line);
        }
    }
    result
}
fn search_case_insensitive<'a>(query:&String,context:&'a String)->Vec<&'a str>{
    let mut result = Vec::new();
    for line in context.lines(){
        if line.to_lowercase().contains(&query.to_lowercase()){
            result.push(line);
        }
    }
    result
}