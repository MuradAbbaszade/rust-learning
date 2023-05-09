use crate::common::Summary;
use std::fmt::Display;
pub mod common;
fn main() {
    println!("Hello, world!");
    let a = 5;
    let b = 4;
    let c = largest(&a,&b);
    let point = Point{
        x:5,
        y:4
    };
    println!("{}",point.summarize());
    println!("{}",point.x);
}
fn largest<'a,T:std::cmp::PartialOrd>(item1:&'a T,item2:&'a T)->&'a T{
    if item1>item2 {
        return item1;
    }
    return item2;
}
struct Point<T>{
    x:T,
    y:T
}
impl<T> Point<T>{
    fn get_x(&self)->&T{
        &self.x
    } 
}
impl<T:Display> Summary for Point<T>{
    fn summarize(&self)->String{
        format!("{} {}",self.x,self.y)
    }
}
enum Result<T,E>{
    Ok(T),
    Err(E)
}
enum Option<T>{
    Some(T),
    None
}

fn foo<T,U>(item1:T,item2:U)
where T:Display+Summary,U:Display
{

}
fn foo2(item1:impl Display+Summary,item2:impl Display){
    
}