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
    println!("{} {}",character,character2) 
}
