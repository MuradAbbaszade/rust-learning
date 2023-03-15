struct User{
    active:bool,
    name:String,
    age:i8
}
struct Numbers(i8,i8);

struct Square{
    width:u32,
    height:u32
}

impl Square{
    fn area(&self)->u32{
        return self.width*self.height;
    }
    fn change_width(&mut self,width:u32){
        self.width=width;
    }
}
fn main() {
    let user = user_builder(String::from("Murad"));
    println!("{} {} {}",user.name,user.age,user.active);
    let numbers = Numbers(5,4);
    println!("{}",numbers.0);
    let mut square1 = Square{width:5,height:4};
    println!("{}",square1.area()); 
    square1.change_width(45);
    println!("{}",square1.width); 
    println!("{}",square1.area()); 

}
fn user_builder(name:String)->User{
    return User{active:true,name:name,age:18};
}
