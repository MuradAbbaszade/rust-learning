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

}
