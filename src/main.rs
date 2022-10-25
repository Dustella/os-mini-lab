use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..100 {
            print!("passed {}", i);
            thread::sleep(Duration::from_secs(1))
        }
    });
    for j in 1..100 {
        println!("Hello, world!{}", j);
        thread::sleep(Duration::from_secs(1));
    }
}
