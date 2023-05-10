use std::error::Error;
use std::fs;
use std::env;
pub struct Config{
    pub query:String,
    pub file_path:String,
    pub ignore_case:bool
}
impl Config{
    pub fn build(args:&Vec<String>)->Result<Config,&str>{
        if args.len()!=3 {
            return Err("not enough arguments");
        }
        Ok(Config{
            query:args[1].clone(),
            file_path:args[2].clone(),
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