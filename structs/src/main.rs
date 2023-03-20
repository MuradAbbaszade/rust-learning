// struct User{
//     active:bool,
//     name:String,
//     age:i8
// }
// struct Numbers(i8,i8);

// struct Square{
//     width:u32,
//     height:u32
// }

// impl Square{
//     fn area(&self)->u32{
//         return self.width*self.height;
//     }
//     fn change_width(&mut self,width:u32){
//         self.width=width;
//     }
// }
// fn main() {
//     let user = user_builder(String::from("Murad"));
//     println!("{} {} {}",user.name,user.age,user.active);
//     let numbers = Numbers(5,4);
//     println!("{}",numbers.0);
//     let mut square1 = Square{width:5,height:4};
//     println!("{}",square1.area()); 
//     square1.change_width(45);
//     println!("{}",square1.width); 
//     println!("{}",square1.area()); 

// }
// fn user_builder(name:String)->User{
//     return User{active:true,name:name,age:18};
// }
enum Gender{
    MALE,FEMALE
}
struct Human{
    name:String,
    gender:Gender
}
impl Gender{
    fn toString(self)->String{
        match self{
            Gender::MALE=>String::from("MALE"),
            Gender::FEMALE=>String::from("FEMALE"),
            _=>String::from("NOT MATCHING")
        }
    }
}
fn main(){
    let human1 = Human{
        name:String::from("Murad"),
        gender:Gender::FEMALE,
    };
    let a:Option<i8> = Some(5);
    let b:Option<i8> = None;
    if let Some(Gender::MALE)=Some(Gender::FEMALE) {
        println!("OK")
    }
    else{
        println!("!")
    }
    println!("{}",human1.gender.toString());
    println!("{:?}",b);

    let x =10;
    match x{
        1|2=>println!("Macthes"),
        1..=6=>println!("1-6"),
        _=>println!("not Macthes")
    }
    
}

