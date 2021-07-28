use std::time::Duration;
use tokio::time::sleep;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let one = tokio::spawn(greet());
    let two = tokio::spawn(greet());
    let (_, _) = tokio::join!(one, two);
}

async fn greet() {
    println!("Hello!");
    sleep(Duration::from_millis(500)).await;
    println!("Goodbye!");
}
