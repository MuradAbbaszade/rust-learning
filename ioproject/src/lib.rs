use std::error::Error;
use std::fs;
pub struct Config{
    pub query:String,
    pub file_path:String
}
impl Config{
    pub fn build(args:&Vec<String>)->Result<Config,&str>{
        if args.len()!=3 {
            return Err("not enough arguments");
        }
        Ok(Config{
            query:args[1].clone(),
            file_path:args[2].clone()
        })
    }
}
pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let text = fs::read_to_string(config.file_path)?;
    for line in search(&config.query,&text){
        println!("{}",line);
    }
    Ok(())
}
fn search<'a>(query:&String,context:&'a String)->Vec<&'a str>{
    let mut result = Vec::new();
    for line in context.lines(){
        if line.contains(query){
            result.push(line);
        }
    }
    result
}