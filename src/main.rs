use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _fut = MyFuture {}.await;
}

struct MyFuture {}

impl Future for MyFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("MyFuture::poll");

        let waker = cx.waker().clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(1));
            waker.wake();
        });

        Poll::Pending
    }
}
