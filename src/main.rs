use std::{thread::sleep, time::Duration};

fn main() {
    println!("Hello!");
    sleep(Duration::from_millis(500));
    println!("Goodbye!");
}
