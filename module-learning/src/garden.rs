#[derive(Debug)]
pub struct Vegetable{
        pub name:String,
        pub example:String
}
pub fn print_vegetable(vegetable:&Vegetable){
        println!("name is {} and type is {}",vegetable.name,vegetable.example);
}