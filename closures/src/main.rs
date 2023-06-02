use closures::{Inventory,ShirtColor};
fn main() {
    println!("Hello, world!");
    let shirts = vec!(ShirtColor::RED,ShirtColor::BLUE,ShirtColor::RED);
    let inv = Inventory{
        shirts
    };
    let user_preference = Some(ShirtColor::BLUE);
    let color = inv.giveaway(user_preference);
    println!("{:?}",color);

// 1
    let example_closure = |x| x;
    example_closure(String::from("a"));
    // example_closure(1); error

// 2
    let mut a=5;
    let mut aa = || a+=1;
    aa();



}
