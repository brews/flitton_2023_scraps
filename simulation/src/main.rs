use std::time::Instant;
use std::{thread, time};


async fn hello(input_int: i32) -> i32 {
    let five_seconds = time::Duration::from_secs(5);
    tokio::time::sleep(five_seconds).await;
    println!("Hello, world! {}", input_int);
    input_int
}

#[tokio::main]
async fn main() {
    let now = Instant::now();
    let one = tokio::spawn({
        hello(1)
    });
    let two = tokio::spawn({
        hello(2)
    });
    let three = tokio::spawn({
        hello(3)
    });
    one.await;
    two.await;
    three.await;

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}