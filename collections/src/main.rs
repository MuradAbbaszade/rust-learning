use std::collections::HashMap;
use std::collections::HashSet;
fn main() {
    let mut nums = HashSet::new();
    let mut nums1 = HashSet::new();
    nums.insert(1);
    nums.insert(2);
    nums.insert(4);
    nums.insert(6);

    nums.insert(2);
    nums.insert(4);

    let intersection = &nums & &nums1;
    for x in intersection{
        println!("{}",x);
    }
    let union = &nums | &nums1;
    for x in union{
        println!("{}",x);
    }
    let diff = &nums - &nums1;
    for x in diff{
        println!("{}",x);
    }
    // let mut nums = HashMap::new();
    // nums.insert(1,"Murad");
    // println!("{:?}",nums.remove_entry(&2));
    // let mut nums = Vec::<i32>::new();
    // nums.push(1);
    // nums.insert(1,2);
    // let first = nums.first();//it returns Option<T>
    // println!("{:?}",first);
    // println!("{}",nums.is_empty());
}
