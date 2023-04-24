use crate::garden::Vegetable;
pub mod garden;
pub mod school;
use crate::school::teacher::Teacher;
use crate::school::student::Student;
fn main(){
    let v  = Vegetable{
        name:String::from("Cucumber"),
        example:String::from("example")
    };
    garden::print_vegetable(&v);
    println!("asd");
    let s = Student{
        name:String::from("Murad"),
        class:String::from("A"),
    };
    let t = Teacher{
        name:String::from("Murad"),
        field:String::from("IT"),
    };
    school::teacher_student(&s,&t);
    
}