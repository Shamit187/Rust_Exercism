use std::thread;
// use std::time::Duration;

fn main() {
    let v = vec![1,2,3];

    let handle = thread::spawn(move||{
        println!("Hello");
        println!("Let me use v");
        println!("{:?}: Hey Vector",v);
    });

    // println!("{:?}: I will still use the vector", v);

    handle.join().unwrap();
}
