use std::rc::Rc;
#[derive(Debug)]
struct City{
    population:i32,
    name:String
}

fn sort_cities(cities:&mut Vec<City>){
    cities.sort_by_key(|p|->i32 {p.population});
}

macro_rules! op{
    ($a:expr,$b:expr,$operation:expr) => {
        {
            match $operation{
                1=>$a+$b,
                2=>$a-$b,
                3=>$a*$b,
                4=>$a/$b,
                5=>$a%$b,
                _=>{
                    println!("Wrong");
                    std::process::exit(1);
                }
            }
        }
    };
}
fn main() {
    println!("{}",op!(5,2,6));
    // Closures
    let example = |x|->i32 {x+1};
    let result = example(5);
    // println!("{}",result);

    let city1 = City{population:100,name:String::from("A")};
    let city2 = City{population:5,name:String::from("B")};
    let city3 = City{population:89,name:String::from("C")};

    let mut cities = vec![city1,city2,city3];
    sort_cities(&mut cities);
    // println!("{:?}",cities);

    // Pointers
    let x = 5;//it stored on the stack
    let y = Box::new(5);//it stored on the heap (y stack)
    assert_eq!(x,*y);

    let a = 5;
    let b = &a;
    assert_eq!(a,*b);


    let s1 = Rc::new(String::from("A"));
    let s2 = s1.clone();
    let s3 = s2.clone();
    // println!("{} {} {}",s1,s2,s3);

    // Stack
    let x = 5;
    // Heap
    let x = Box::new(5);

    let rc_value = String::from("DC");
    let rc = Rc::new(rc_value);
    // println!("Reference counting: {}", Rc::strong_count(&rc));

    let rc1 = rc.clone();
    // println!("Reference counting: {}",Rc::strong_count(&rc1));

    // println!("Reference counting: {}", Rc::strong_count(&rc));

    // Task
    let vec = vec![1,3,5,7,9];

    let result_vector:Vec<i8> = vec.into_iter().map(|x| x+10).collect();
    // println!("{:?}",result_vector);



}
