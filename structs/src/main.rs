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
// enum Gender{
//     MALE,FEMALE
// }
// struct Human{
//     name:String,
//     gender:Gender
// }
// impl Gender{
//     fn toString(self)->String{
//         match self{
//             Gender::MALE=>String::from("MALE"),
//             Gender::FEMALE=>String::from("FEMALE"),
//             _=>String::from("NOT MATCHING")
//         }
//     }
// }
// struct Car{
//     mpg:f32,
//     color:String,
//     top_speed:f32,
// }
// impl Car{
//     fn set_mpg(&mut self,mpg:f32){
//         self.mpg=mpg;
//     }
//     fn set_color(&mut self,color:String){
//         self.color=color;
//     }
//     fn set_top_speed(&mut self,top_speed:f32){
//         self.top_speed=top_speed;
//     }
// }
// enum Shape{
//     TRIANGLE,SQUARE,PENTAGON,OCTAGON
// }
// impl Shape{
//     fn corners(self)->i8{
//         match self{
//             Shape::TRIANGLE=>3,
//             Shape::SQUARE=>4,
//             Shape::PENTAGON=>5,
//             Shape::OCTAGON=>8
//         }
//     }
// }
struct Example<T>{
    something:T
}
trait Overview{
    fn overview(&self)->String;
}
trait Overview2{
    fn overview2(&self){
        println!("hello");
    }
}

impl<T> Overview2 for Example<T>{

}

impl<T> Overview for Example<T>{
    fn overview(&self)->String{
        String::from("Hello")
    }
}
fn main(){
    let example = Example{
        something:String::from("Hi")
    };
    let string = example.overview();
    example.overview2();
    println!("{}",string);
    // let human1 = Human{
    //     name:String::from("Murad"),
    //     gender:Gender::FEMALE,
    // };
    // let a:Option<i8> = Some(5);
    // let b:Option<i8> = None;
    // if let Some(Gender::MALE)=Some(Gender::FEMALE) {
    //     println!("OK")
    // }
    // else{
    //     println!("!")
    // }
    // println!("{}",human1.gender.toString());
    // println!("{:?}",b);

    // let x =10;
    // match x{
    //     1|2=>println!("Macthes"),
    //     1..=6=>println!("1-6"),
    //     _=>println!("not Macthes")
    // }
    // let mut car = Car {
    //     mpg: 0.0,
    //     color: String::from("black"),
    //     top_speed: 0.0,
    // };

    // car.set_mpg(30.0);
    // car.set_color(String::from("red"));
    // car.set_top_speed(120.0);
    // println!("{}",car.mpg);
    // let corners = Shape::OCTAGON.corners();
    // println!("{}",corners);
}

