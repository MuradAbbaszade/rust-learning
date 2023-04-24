use std::thread;
fn main() {
    let mut s = String::from("hello");

    let mut r1 = &mut s;
    r1.push_str("asdads");
    println!("{}",&s);
     dbg!(&s);
     dbg!(s);


    // let handle1 = thread::spawn(|| {
    //     // İş parçacığı 1'in çalışması
    //     // println!("thread1");
    //     "thread 1".to_string()
    // });

    // let handle2 = thread::spawn(|| {
    //     // İş parçacığı 2'nin çalışması
    //     // println!("thread2");
    //     "thread 2".to_string()
    // });

    // let result1 = handle1.join().unwrap();
    // let result2 = handle2.join().unwrap();

    // let vector = vec![1,2,3];
    // let handle3 = thread::spawn(move || {
    //     // println!("{:?}",vector);
    // });
    // handle3.join().unwrap();
    // // println!("main {:?}",vector);

    // let mut handles = Vec::new();
    // for e in vector {
    // handles.push(thread::spawn(move || {
    //     println!("Thread {}", e);
    // }));
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("Main thread");


    

    // println!("{} and {}", result1, result2);
}
