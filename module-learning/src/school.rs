pub mod teacher{ 
    #[derive(Debug)]
    pub struct Teacher{
        pub name:String,
        pub field:String
    }
    pub fn give_homework(){

    }   
}
pub mod student{
    #[derive(Debug)]
    pub struct Student{
        pub name:String,
        pub class:String
    }   
    pub fn do_homework(){

    }
}
use crate::school::teacher::Teacher;
use crate::school::student::Student;
pub fn teacher_student(student:&Student,teacher:&Teacher){
    println!("{} is a student, {} is a teacher",student.name,teacher.name);
}
