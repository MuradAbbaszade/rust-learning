fn main() {
    //Variables and mutamility

    let a = 5; //immutable variable and _a is immutable variable ,too
    let mut b = 5; //mutable variable
    const SECONDS :i8 = 60; //immutable variable
    b=7;
    println!("Variable a={}",a);
    println!("Variable b={},SECONDS={}",b,SECONDS);

    //Scalar data types
    //Integer variables can be 8,16,32 and 64 bit
    let mut age:i8 = 10 ;// 8 bit integer
    let mut example1:i64 = 100000000 ;//64 bit integer 
    let mut example = 111; //for default - 64 bit integer
    println!("{} {} {}",age,example1,example);

    //Float variables can be 32 and 64 bit
    let money:f32=1000.0;
    let money2:f64=100000.0;

    //Boolean
    let t =true;
    let t2:bool=true; 
    //Both them will works because rust compiler understand that true is a boolean variable
 
    //Character type
    let character:char = 'a';
    let character2 = 'b';
    println!("{} {}",character,character2); 
     
    //Tuples,arrays,vectors
    let tup = (500,"a",50.0); //tuples
    println!("{}",tup.0); 
    let (x,y,z) = tup;
    println!("{} {} {}",x,y,z);
    
    //Arrays (Elements can be different type in tuples but elements must be same type in arrays)
    let mut array:[i32;3]=[1,2,3];
    let mut array2 =[3,4,5];
    println!("{}",array[0]);
    println!("{:?}",array); //all elements of the array

    //Vector (List)
    let mut nums = vec![1,2,3];
    nums.push(5);
    // nums.pop() is delete the last element from the vectors
    println!("{:?}",nums); 

    let mut vec = Vec::new(); //empty vector
    vec.push("salam");
    vec.push("sa");

    let mut vec2 = Vec::<i32>::with_capacity(3); //empty vector with size 3 and it stores 32 bit integer values

    let v:Vec<i32> = (1..5).collect(); // not included 5

    //Strings
    let name = "Tyler".to_string();
    let name2 = String::from("Tyler");
    //to compare string we can use == and !=


    //Functions
    learn_functions("I learn functions");
    println!("{}",this_function_return_value(5,3));
}
fn learn_functions(phrase:&str){
    println!("{}",phrase);
}
//u32 - is unsigned value - it is can't be negative
fn this_function_return_value(a:u32,b:u32)->u32{
    a+b
}
