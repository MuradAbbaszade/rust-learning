use std::fs::File;
use std::io::ErrorKind;
use std::io::Error;
fn main() {
    // let file = File::open("error.txt");
    // let file = match file{
    //     Ok(file)=>file,
    //     Err(error)=>match error.kind(){
    //         ErrorKind::NotFound => match File::create("error.txt"){
    //             Ok(created_file)=>created_file,
    //             Err(error)=>panic!("An error occured")
    //         },
    //         _=>panic!("Any other error")
    //     }
    // };
    let file = open_file().unwrap();

}
fn open_file()->Result<File,Error>{
    let file=File::open("error.txt");
    Ok(file)
}
// File::open("error.txt").unwrap() it return Ok value, if Err returns panic!
// enum Result<T,E>{
//     Ok(T),
//     Err(E)
// }